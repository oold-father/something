use crate::db::{Database, SearchQuery, SearchOperator, FileType};
use crate::error::Result;

/// 搜索文件
#[tauri::command]
pub fn search_files(
    keywords: Vec<String>,
    operator: String,
    file_type_filter: Option<String>,
    limit: usize,
    offset: usize,
    state: tauri::State<Database>,
) -> std::result::Result<crate::db::SearchResultResponse, String> {
    let op = match operator.as_str() {
        "AND" | "and" => SearchOperator::And,
        "OR" | "or" => SearchOperator::Or,
        _ => return Err("无效的搜索运算符，请使用 AND 或 OR".to_string()),
    };

    let ft = file_type_filter.and_then(|s| {
        let ext = s.split(',').next().unwrap_or("");
        Some(FileType::from_extension(ext))
    });

    let query = SearchQuery {
        keywords,
        operator: op,
        file_type_filter: ft,
        limit,
        offset,
    };

    state.search_files(&query).map_err(|e| e.to_string())
}
