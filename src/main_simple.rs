use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ArgAction, ValueEnum};
use std::{fs, path::{Path, PathBuf}, time::SystemTime};
use std::sync::{Arc, Mutex};
use tantivy::{Index, doc, ReloadPolicy, collector::TopDocs, query::{QueryParser, BooleanQuery, TermQuery}, schema::{Field, Value}, directory::MmapDirectory};
use fst::{IntoStreamer, Set, Streamer};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Serialize, Deserialize};
use time::macros::format_description;
use bstr::{ByteSlice};
use xxhash_rust::xxh3::xxh3_64;

use warpseek::search::*;

#[derive(Parser)]
#[command(name="warpseek", version, about="Blazing-fast terminal search: instant filename fuzzy + full-text across your folders.")]
struct Cli {
    /// Index location (defaults to OS data dir)
    #[arg(long)]
    index_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize an index and register one or more root paths
    Init { #[arg(required=true)] roots: Vec<PathBuf> },
    /// Add paths to be indexed
    Add { #[arg(required=true)] paths: Vec<PathBuf> },
    /// Remove paths from config (does not delete files)
    Remove { #[arg(required=true)] paths: Vec<PathBuf> },
    /// Re/build the index now
    Index,
    /// Query by text (content+name)
    Q { query: String, #[arg(long, default_value_t=50)] top: usize, #[arg(long)] names_only: bool },
    /// Fuzzy match filenames (fast, no content)
    F { pattern: String, #[arg(long, default_value_t=50)] top: usize },
    /// Show stats
    Stats,
    /// Purge the index
    Purge,
    /// Watch for changes and keep index fresh
    Watch,
}

fn index_path(cli: &Cli) -> Result<PathBuf> {
    Ok(match &cli.index_dir {
        Some(p) => p.clone(),
        None => project_dirs()?.data_dir().join("index"),
    })
}

fn do_query(cli: &Cli, q: &str, top: usize, names_only: bool) -> Result<()> {
    let index_dir = index_path(cli)?;
    let (index, fields) = open_index(&index_dir)?;
    let reader = index.reader_builder().reload_policy(ReloadPolicy::Manual).try_into()?;
    let searcher = reader.searcher();

    if names_only {
        // filename-only search via QueryParser on 'name'
        let qp = QueryParser::for_index(&index, vec![fields.name]);
        let query = qp.parse_query(q)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(top))?;
        for (_score, docaddr) in top_docs {
            let doc = searcher.doc(docaddr)?;
            let path = doc.get_first(fields.path).and_then(|v| v.text()).unwrap_or("");
            println!("{}", path);
        }
        return Ok(());
    }

    let qp = QueryParser::for_index(&index, vec![fields.content, fields.name]);
    let query = qp.parse_query(q)?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(top))?;
    for (_score, docaddr) in top_docs {
        let doc = searcher.doc(docaddr)?;
        let path = doc.get_first(fields.path).and_then(|v| v.text()).unwrap_or("");
        println!("{}", path);
    }
    Ok(())
}

fn do_fuzzy(cli: &Cli, pat: &str, top: usize) -> Result<()> {
    let index_dir = index_path(cli)?;
    let fst_path = index_dir.join("names.fst");
    let bytes = fs::read(fst_path)?;
    let set = Set::new(bytes)?;
    // simple substring case-insensitive by scanning all terms; fst isn't ideal for substrings, but works
    let mut results = Vec::new();
    let q = pat.to_lowercase();
    let mut stream = set.stream();
    while let Some(b) = stream.next() {
        let s = std::str::from_utf8(b).unwrap_or("");
        if s.to_lowercase().contains(&q) {
            results.push(s.to_string());
            if results.len() >= top { break; }
        }
    }
    for r in results { println!("{}", r); }
    Ok(())
}

fn ensure_index(cli: &Cli) -> Result<()> {
    let p = index_path(cli)?;
    fs::create_dir_all(&p)?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    ensure_index(&cli)?;
    let mut cfg = read_config()?;

    match &cli.command {
        Commands::Init { roots } => {
            cfg.roots = roots.clone();
            write_config(&cfg)?;
            index_all(&mut cfg)?;
        }
        Commands::Add { paths } => {
            for p in paths { if !cfg.roots.contains(p) { cfg.roots.push(p.clone()); } }
            write_config(&cfg)?;
            index_all(&mut cfg)?;
        }
        Commands::Remove { paths } => {
            cfg.roots.retain(|r| !paths.contains(r));
            write_config(&cfg)?;
        }
        Commands::Index => { index_all(&mut cfg)?; }
        Commands::Q { query, top, names_only } => { do_query(&cli, query, *top, *names_only)?; }
        Commands::F { pattern, top } => { do_fuzzy(&cli, pattern, *top)?; }
        Commands::Stats => {
            let index_dir = index_path(&cli)?;
            let (index, _) = open_index(&index_dir)?;
            let reader = index.reader()?;
            println!("Segments: {}", reader.searcher().segment_readers().len());
            println!("Index dir: {}", index_dir.to_string_lossy());
            println!("Roots: {:?}", cfg.roots);
        }
        Commands::Purge => {
            let p = index_path(&cli)?;
            if p.exists() { fs::remove_dir_all(&p)?; }
            println!("Purged {}", p.to_string_lossy());
        }
        Commands::Watch => {
            let roots = cfg.roots.clone();
            if roots.is_empty() { return Err(anyhow!("no roots configured; run `warpseek init <paths>`")); }
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher: RecommendedWatcher = Watcher::new(tx, notify::Config::default())?;
            for r in &roots { watcher.watch(r, RecursiveMode::Recursive)?; }
            println!("Watching for changes... Press Ctrl+C to stop.");
            index_all(&mut cfg)?;
            loop {
                match rx.recv() {
                    Ok(_ev) => {
                        // for simplicity, rebuild fully; can be optimized later
                        let _ = index_all(&mut cfg);
                    }
                    Err(e) => { eprintln!("watch error: {}", e); }
                }
            }
        }
    }
    Ok(())
}