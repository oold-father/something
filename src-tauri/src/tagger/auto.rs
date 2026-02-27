use super::rules::{TagRule, default_rules};
use crate::db::File;

/// 自动标签生成器
pub struct AutoTagger {
    rules: Vec<TagRule>,
}

impl AutoTagger {
    /// 创建新的自动标签生成器（使用默认规则）
    pub fn new() -> Self {
        AutoTagger {
            rules: default_rules(),
        }
    }

    /// 使用自定义规则创建
    pub fn with_rules(rules: Vec<TagRule>) -> Self {
        AutoTagger { rules }
    }

    /// 为文件生成标签
    pub fn generate_tags(&self, file: &File) -> Vec<String> {
        self.rules
            .iter()
            .filter(|rule| rule.matches(file))
            .map(|rule| rule.name.clone())
            .collect()
    }

    /// 获取所有规则
    pub fn rules(&self) -> &[TagRule] {
        &self.rules
    }

    /// 添加规则
    pub fn add_rule(&mut self, rule: TagRule) {
        self.rules.push(rule);
    }

    /// 移除规则（按名称）
    pub fn remove_rule(&mut self, name: &str) {
        self.rules.retain(|r| r.name != name);
    }

    /// 重置为默认规则
    pub fn reset_to_default(&mut self) {
        self.rules = default_rules();
    }
}

impl Default for AutoTagger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{FileType, FileStatus};
    use chrono::{Utc, Duration};

    fn create_test_file(size: i64, file_type: FileType) -> File {
        File {
            id: None,
            path: "/test/file.jpg".to_string(),
            name: "file.jpg".to_string(),
            extension: "jpg".to_string(),
            size,
            file_type,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            accessed_at: Utc::now(),
            status: FileStatus::Active,
            indexed_at: Utc::now(),
            metadata: None,
        }
    }

    #[test]
    fn test_generate_tags_for_image() {
        let tagger = AutoTagger::new();
        let file = create_test_file(1024 * 1024, FileType::Image);
        let tags = tagger.generate_tags(&file);

        assert!(tags.contains(&"图片".to_string()));
    }

    #[test]
    fn test_small_file_tag() {
        let tagger = AutoTagger::new();
        let file = create_test_file(5 * 1024, FileType::Image);
        let tags = tagger.generate_tags(&file);

        assert!(tags.contains(&"小文件".to_string()));
    }

    #[test]
    fn test_large_file_tag() {
        let tagger = AutoTagger::new();
        let file = create_test_file(200 * 1024 * 1024, FileType::Video);
        let tags = tagger.generate_tags(&file);

        assert!(tags.contains(&"大文件".to_string()));
        assert!(tags.contains(&"视频".to_string()));
    }

    #[test]
    fn test_today_file_tag() {
        let tagger = AutoTagger::new();
        let mut file = create_test_file(1024, FileType::Text);
        file.modified_at = Utc::now();

        let tags = tagger.generate_tags(&file);
        assert!(tags.contains(&"今日文件".to_string()));
    }
}
