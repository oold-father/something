use crate::db::{Database, File, Tag, FileType, FileStatus, TagType};
use chrono::Utc;
use rusqlite::{Connection, Result};

#[cfg(test)]
mod tests {
    use super::*;

    /// 创建内存测试数据库
    fn create_test_db() -> Database {
        let conn = Connection::open_in_memory().unwrap();
        // 执行 schema
        let schema = include_str!("schema.sql");
        conn.execute_batch(schema).unwrap();

        // 将连接包装成 Database（需要手动实现）
        Database {
            conn: std::sync::Arc::new(std::sync::Mutex::new(conn)),
        }
    }

    #[test]
    fn test_create_and_get_file() {
        let db = create_test_db();

        let file = File {
            id: None,
            path: "/test/file.jpg".to_string(),
            name: "file.jpg".to_string(),
            extension: "jpg".to_string(),
            size: 1024,
            file_type: FileType::Image,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            accessed_at: Utc::now(),
            status: FileStatus::Active,
            indexed_at: Utc::now(),
            metadata: None,
        };

        let file_id = db.create_file(&file).unwrap();
        assert!(file_id > 0);

        let retrieved = db.get_file_by_id(file_id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().path, "/test/file.jpg");
    }

    #[test]
    fn test_create_and_get_tag() {
        let db = create_test_db();

        let tag = Tag {
            id: None,
            name: "test".to_string(),
            display_name: "测试标签".to_string(),
            tag_type: TagType::Custom,
            color: "#FF0000".to_string(),
            icon: None,
            use_count: 0,
            created_at: Utc::now(),
        };

        let tag_id = db.create_tag(&tag).unwrap();
        assert!(tag_id > 0);

        let all_tags = db.get_all_tags().unwrap();
        assert!(!all_tags.is_empty());
        assert_eq!(all_tags.len(), 1);
    }

    #[test]
    fn test_add_tag_to_file() {
        let db = create_test_db();

        // 创建文件
        let file = File {
            id: None,
            path: "/test/file.txt".to_string(),
            name: "file.txt".to_string(),
            extension: "txt".to_string(),
            size: 100,
            file_type: FileType::Text,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            accessed_at: Utc::now(),
            status: FileStatus::Active,
            indexed_at: Utc::now(),
            metadata: None,
        };

        let file_id = db.create_file(&file).unwrap();

        // 添加标签
        db.add_tag_to_file_by_name(file_id, "文本", false).unwrap();

        // 查询文件标签
        let tags = db.get_tags_by_file(file_id).unwrap();
        assert!(!tags.is_empty());
        assert_eq!(tags[0].name, "文本");
    }

    #[test]
    fn test_file_type_from_extension() {
        assert_eq!(FileType::from_extension("jpg"), FileType::Image);
        assert_eq!(FileType::from_extension("mp3"), FileType::Audio);
        assert_eq!(FileType::from_extension("mp4"), FileType::Video);
        assert_eq!(FileType::from_extension("txt"), FileType::Text);
        assert_eq!(FileType::from_extension("exe"), FileType::Binary);
        assert_eq!(FileType::from_extension("unknown"), FileType::Other);
    }
}
