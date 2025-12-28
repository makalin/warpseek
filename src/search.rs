use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use ignore::{WalkBuilder, types::TypesBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::{fs, path::{Path, PathBuf}, time::SystemTime};
use std::io::Read;
use tantivy::schema::{Schema, TEXT, STORED, STRING, FAST, Field};
use tantivy::{Index, doc, directory::MmapDirectory};
use serde::{Serialize, Deserialize};
use xxhash_rust::xxh3::xxh3_64;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub roots: Vec<PathBuf>,
    // keep last scan checksum to avoid rebuilds
    pub last_scan: Option<u64>,
}

pub struct Fields {
    pub path: Field,
    pub name: Field,
    pub content: Field,
    pub modified: Field,
}

pub fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from("io", "frangedev", "warpseek").ok_or_else(|| anyhow!("cannot resolve project dirs"))
}

pub fn index_path() -> Result<PathBuf> {
    Ok(project_dirs()?.data_dir().join("index"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(project_dirs()?.config_dir().join("config.json"))
}

pub fn read_config() -> Result<Config> {
    let p = config_path()?;
    if p.exists() {
        let s = fs::read_to_string(&p)?;
        Ok(serde_json::from_str(&s)?)
    } else {
        Ok(Config::default())
    }
}

pub fn write_config(cfg: &Config) -> Result<()> {
    let p = config_path()?;
    fs::create_dir_all(p.parent().unwrap())?;
    fs::write(p, serde_json::to_string_pretty(cfg)?)?;
    Ok(())
}

pub fn build_schema() -> (Schema, Fields) {
    let mut schema = Schema::builder();
    let path = schema.add_text_field("path", STRING | STORED);
    let name = schema.add_text_field("name", TEXT | STORED);
    let content = schema.add_text_field("content", TEXT);
    let modified = schema.add_i64_field("modified", FAST | STORED);
    let built_schema = schema.build();
    let fields = Fields {
        path,
        name,
        content,
        modified,
    };
    (built_schema, fields)
}

pub fn open_index(index_dir: &Path) -> Result<(Index, Fields)> {
    fs::create_dir_all(index_dir)?;
    let (schema, fields) = build_schema();
    let directory = MmapDirectory::open(index_dir)?;
    let index = if Index::exists(&directory)? {
        Index::open(directory)?
    } else {
        Index::create(directory, schema, tantivy::IndexSettings::default())?
    };
    Ok((index, fields))
}

pub fn is_probably_text(bytes: &[u8]) -> bool {
    // simple heuristic: reject if contains many NULs or invalid UTF-8
    let nul = memchr::memchr(0, bytes).is_some();
    if nul { return false; }
    if let Ok(s) = std::str::from_utf8(bytes) {
        !s.bytes().any(|b| b < 0x09 && b != b'\n' && b != b'\r' && b != b'\t')
    } else {
        false
    }
}

pub fn collect_files(roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut types = TypesBuilder::new();
    // index common text types by default
    let _ = types.add_defaults();
    let matcher = types.select("all").build().ok();
    let mut v = Vec::new();
    for r in roots {
        let mut w = WalkBuilder::new(r);
        w.hidden(false).follow_links(false).add_custom_ignore_filename(".ignore");
        if let Some(m) = &matcher { w.types(m.clone()); }
        let walker = w.build();
        for res in walker {
            if let Ok(entry) = res {
                let p = entry.path();
                if p.is_file() {
                    v.push(p.to_path_buf());
                }
            }
        }
    }
    // Sort files lexicographically for FST compatibility
    v.sort();
    v
}

pub fn index_all(cfg: &mut Config) -> Result<()> {
    let index_dir = index_path()?;
    let (index, fields) = open_index(&index_dir)?;
    let mut writer = index.writer(256 * 1024 * 1024)?; // 256MB
    let files = collect_files(&cfg.roots);
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")?.progress_chars("=>-"));

    files.par_iter().for_each(|p| {
        pb.set_message(p.to_string_lossy().to_string());
        if let Ok(meta) = fs::metadata(p) {
            if meta.len() > 20 * 1024 * 1024 { pb.inc(1); return; } // skip >20MB files by default
            if let Ok(mut f) = fs::File::open(p) {
                let mut buf = Vec::new();
                if f.read_to_end(&mut buf).is_ok() {
                    if !is_probably_text(&buf) { pb.inc(1); return; }
                    let text = match String::from_utf8(buf) { Ok(s)=>s, Err(_)=>return };
                    let name = p.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                    let modified = meta.modified().ok().and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok()).map(|d| d.as_secs() as i64).unwrap_or(0);
                    let docu = doc!(
                        fields.path => p.to_string_lossy().to_string(),
                        fields.name => name,
                        fields.content => text,
                        fields.modified => modified
                    );
                    let _ = writer.add_document(docu);
                }
            }
        }
        pb.inc(1);
    });
    pb.finish_with_message("Index built");
    writer.commit()?;
    // Note: reload_searchers is not available in this version of tantivy
    // The index will be automatically reloaded when needed

    // Build FST set for filenames
    let names: Vec<String> = files.iter()
        .filter_map(|p| p.file_name().map(|s| s.to_string_lossy().to_string()))
        .collect();
    let set = fst::Set::from_iter(names.iter()).context("build fst")?;
    let fst_path = index_dir.join("names.fst");
    fs::write(fst_path, set.as_fst().to_vec())?;

    // save checksum of roots list to skip redundant rebuilds later
    let mut hasher_input = String::new();
    for r in &cfg.roots { hasher_input.push_str(&r.to_string_lossy()); }
    cfg.last_scan = Some(xxh3_64(hasher_input.as_bytes()));
    write_config(cfg)?;
    Ok(())
}
