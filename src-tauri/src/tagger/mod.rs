mod rules;
mod auto;

pub use rules::{TagRule, TagCondition, DatePattern, default_rules};
pub use auto::AutoTagger;

use crate::db::{Database, File};
use crate::error::Result;

/// 标签生成管理器
pub struct TagGenerator {
    tagger: AutoTagger,
    db: Database,
}

impl TagGenerator {
    /// 创建新的标签生成器
    pub fn new(db: Database) -> Self {
        TagGenerator {
            tagger: AutoTagger::new(),
            db,
        }
    }

    /// 为文件生成并添加自动标签
    pub fn process_file(&self, file: &File) -> Result<usize> {
        let tags = self.tagger.generate_tags(file);
        let mut added_count = 0;

        if let Some(file_id) = file.id {
            for tag_name in tags {
                self.db.add_tag_to_file_by_name(file_id, &tag_name, true)?;
                added_count += 1;
            }
        }

        Ok(added_count)
    }

    /// 批量处理文件
    pub fn process_files(&self, files: &[File]) -> Result<usize> {
        let mut total_count = 0;

        for file in files {
            total_count += self.process_file(file)?;
        }

        Ok(total_count)
    }

    /// 获取生成器（用于自定义规则）
    pub fn tagger(&self) -> &AutoTagger {
        &self.tagger
    }

    /// 获取可变生成器（用于修改规则）
    pub fn tagger_mut(&mut self) -> &mut AutoTagger {
        &mut self.tagger
    }
}
