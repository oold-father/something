use crate::db::{SearchQuery, SearchOperator, FileType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_query_and_operator() {
        let query = SearchQuery {
            keywords: vec!["图片".to_string(), "今日".to_string()],
            operator: SearchOperator::And,
            file_type_filter: None,
            limit: 50,
            offset: 0,
        };

        assert_eq!(query.keywords.len(), 2);
        assert_eq!(query.operator, SearchOperator::And);
    }

    #[test]
    fn test_search_query_or_operator() {
        let query = SearchQuery {
            keywords: vec!["jpg".to_string(), "png".to_string()],
            operator: SearchOperator::Or,
            file_type_filter: None,
            limit: 50,
            offset: 0,
        };

        assert_eq!(query.operator, SearchOperator::Or);
    }

    #[test]
    fn test_search_query_with_filter() {
        let query = SearchQuery {
            keywords: vec!["test".to_string()],
            operator: SearchOperator::And,
            file_type_filter: Some(FileType::Image),
            limit: 50,
            offset: 0,
        };

        assert!(query.file_type_filter.is_some());
        assert_eq!(query.file_type_filter, Some(FileType::Image));
    }

    #[test]
    fn test_search_operator_as_string() {
        assert_eq!(SearchOperator::And.as_str(), "AND");
        assert_eq!(SearchOperator::Or.as_str(), "OR");
    }

    #[test]
    fn test_file_type_from_extension() {
        use crate::db::FileType;

        assert_eq!(FileType::from_extension("jpg"), FileType::Image);
        assert_eq!(FileType::from_extension("mp3"), FileType::Audio);
        assert_eq!(FileType::from_extension("mp4"), FileType::Video);
        assert_eq!(FileType::from_extension("txt"), FileType::Text);
        assert_eq!(FileType::from_extension("exe"), FileType::Binary);
        assert_eq!(FileType::from_extension("unknown"), FileType::Other);
    }
}
