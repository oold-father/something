use crate::db::{SearchQuery, SearchOperator, FileType};

/// 搜索引擎
pub struct SearchEngine {
    // TODO: 实现搜索功能
}

impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {}
    }

    pub fn search(&self, _query: &SearchQuery) -> crate::error::Result<crate::db::SearchResultResponse> {
        Ok(crate::db::SearchResultResponse {
            results: Vec::new(),
            total: 0,
        })
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;

pub use tests::*;
