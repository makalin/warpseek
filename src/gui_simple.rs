use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub id: String,
    pub path: String,
    pub name: String,
    pub content_preview: String,
    pub modified: i64,
    pub score: f32,
    pub file_type: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchQuery {
    pub query: String,
    pub search_type: SearchType,
    pub file_types: Vec<String>,
    pub max_results: usize,
    pub case_sensitive: bool,
    pub use_regex: bool,
    pub modified_after: Option<i64>,
    pub modified_before: Option<i64>,
    pub size_min: Option<u64>,
    pub size_max: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SearchType {
    FullText,
    Filename,
    Regex,
    Content,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SearchHistory {
    pub queries: Vec<SearchQuery>,
    pub max_entries: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchFilters {
    pub file_extensions: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
    pub max_file_size: u64,
    pub min_file_size: u64,
}

pub struct SearchEngine {
    search_history: Arc<Mutex<SearchHistory>>,
    bookmarks: Arc<Mutex<Vec<Bookmark>>>,
    filters: Arc<Mutex<SearchFilters>>,
}

impl SearchEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            search_history: Arc::new(Mutex::new(SearchHistory {
                queries: Vec::new(),
                max_entries: 100,
            })),
            bookmarks: Arc::new(Mutex::new(Vec::new())),
            filters: Arc::new(Mutex::new(SearchFilters {
                file_extensions: vec!["txt".to_string(), "md".to_string(), "rs".to_string(), "js".to_string(), "py".to_string()],
                exclude_patterns: vec!["node_modules".to_string(), ".git".to_string()],
                include_patterns: Vec::new(),
                max_file_size: 20 * 1024 * 1024, // 20MB
                min_file_size: 0,
            })),
        })
    }

    pub fn search(&self, query: SearchQuery) -> Result<Vec<SearchResult>> {
        // Mock search results for now
        let results = vec![
            SearchResult {
                id: "1".to_string(),
                path: "/Users/test/example.txt".to_string(),
                name: "example.txt".to_string(),
                content_preview: "This is a sample file content...".to_string(),
                modified: 1698000000,
                score: 0.95,
                file_type: "txt".to_string(),
                size: 1024,
            },
            SearchResult {
                id: "2".to_string(),
                path: "/Users/test/README.md".to_string(),
                name: "README.md".to_string(),
                content_preview: "Project documentation...".to_string(),
                modified: 1698000001,
                score: 0.87,
                file_type: "md".to_string(),
                size: 2048,
            },
        ];

        // Add to search history
        {
            let mut history = self.search_history.lock().unwrap();
            history.queries.insert(0, query.clone());
            let max_entries = history.max_entries;
            if history.queries.len() > max_entries {
                history.queries.truncate(max_entries);
            }
        }

        Ok(results)
    }

    pub fn get_search_history(&self) -> Result<Vec<SearchQuery>> {
        let history = self.search_history.lock().unwrap();
        Ok(history.queries.clone())
    }

    pub fn add_bookmark(&self, bookmark: Bookmark) -> Result<()> {
        let mut bookmarks = self.bookmarks.lock().unwrap();
        bookmarks.push(bookmark);
        Ok(())
    }

    pub fn get_bookmarks(&self) -> Result<Vec<Bookmark>> {
        let bookmarks = self.bookmarks.lock().unwrap();
        Ok(bookmarks.clone())
    }

    pub fn update_filters(&self, filters: SearchFilters) -> Result<()> {
        let mut current_filters = self.filters.lock().unwrap();
        *current_filters = filters;
        Ok(())
    }

    pub fn get_filters(&self) -> Result<SearchFilters> {
        let filters = self.filters.lock().unwrap();
        Ok(filters.clone())
    }
}

// Tauri commands
#[tauri::command]
fn search_files(
    engine: tauri::State<'_, Arc<Mutex<SearchEngine>>>,
    query: SearchQuery,
) -> Result<Vec<SearchResult>, String> {
    let engine = engine.lock().unwrap();
    engine.search(query).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_search_history(
    engine: tauri::State<'_, Arc<Mutex<SearchEngine>>>,
) -> Result<Vec<SearchQuery>, String> {
    let engine = engine.lock().unwrap();
    engine.get_search_history().map_err(|e| e.to_string())
}

#[tauri::command]
fn add_bookmark(
    engine: tauri::State<'_, Arc<Mutex<SearchEngine>>>,
    bookmark: Bookmark,
) -> Result<(), String> {
    let engine = engine.lock().unwrap();
    engine.add_bookmark(bookmark).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_bookmarks(
    engine: tauri::State<'_, Arc<Mutex<SearchEngine>>>,
) -> Result<Vec<Bookmark>, String> {
    let engine = engine.lock().unwrap();
    engine.get_bookmarks().map_err(|e| e.to_string())
}

#[tauri::command]
fn update_filters(
    engine: tauri::State<'_, Arc<Mutex<SearchEngine>>>,
    filters: SearchFilters,
) -> Result<(), String> {
    let engine = engine.lock().unwrap();
    engine.update_filters(filters).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_filters(
    engine: tauri::State<'_, Arc<Mutex<SearchEngine>>>,
) -> Result<SearchFilters, String> {
    let engine = engine.lock().unwrap();
    engine.get_filters().map_err(|e| e.to_string())
}

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(SearchEngine::new().expect("Failed to initialize search engine"))))
        .invoke_handler(tauri::generate_handler![
            search_files,
            get_search_history,
            add_bookmark,
            get_bookmarks,
            update_filters,
            get_filters,
            open_file,
            open_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
