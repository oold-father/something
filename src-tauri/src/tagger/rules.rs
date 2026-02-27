use crate::db::{File, FileType};
use chrono::{Datelike, Timelike, Utc};

/// 标签规则定义
#[derive(Debug, Clone)]
pub struct TagRule {
    /// 规则名称（生成的标签名）
    pub name: String,
    /// 规则条件
    pub condition: TagCondition,
}

/// 标签条件
#[derive(Debug, Clone)]
pub enum TagCondition {
    /// 文件类型匹配
    FileType(Vec<FileType>),
    /// 文件大小范围
    FileSize { min: Option<u64>, max: Option<u64> },
    /// 日期模式
    DatePattern(DatePattern),
    /// 路径包含指定字符串
    PathContains(String),
    /// 扩展名匹配
    Extension(Vec<String>),
    /// 文件名包含指定字符串
    NameContains(String),
}

/// 日期模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatePattern {
    /// 今天
    Today,
    /// 昨天
    Yesterday,
    /// 本周
    ThisWeek,
    /// 上周
    LastWeek,
    /// 本月
    ThisMonth,
    /// 上月
    LastMonth,
    /// 本年
    ThisYear,
    /// 去年
    LastYear,
}

impl TagRule {
    /// 检查文件是否匹配此规则
    pub fn matches(&self, file: &File) -> bool {
        match &self.condition {
            TagCondition::FileType(types) => types.contains(&file.file_type),
            TagCondition::FileSize { min, max } => {
                let size = file.size as u64;
                match (min, max) {
                    (Some(m), None) => size >= m,
                    (None, Some(m)) => size <= m,
                    (Some(min_val), Some(max_val)) => size >= min_val && size <= max_val,
                    (None, None) => true,
                }
            }
            TagCondition::DatePattern(pattern) => {
                self.check_date_pattern(file, pattern)
            }
            TagCondition::PathContains(s) => {
                file.path.to_lowercase().contains(&s.to_lowercase())
            }
            TagCondition::Extension(exts) => {
                exts.iter().any(|e| e.to_lowercase() == file.extension.to_lowercase())
            }
            TagCondition::NameContains(s) => {
                file.name.to_lowercase().contains(&s.to_lowercase())
            }
        }
    }

    /// 检查日期模式
    fn check_date_pattern(&self, file: &File, pattern: &DatePattern) -> bool {
        let file_date = file.modified_at;
        let now = Utc::now();

        match pattern {
            DatePattern::Today => {
                file_date.date() == now.date()
            }
            DatePattern::Yesterday => {
                let yesterday = now - chrono::Duration::days(1);
                file_date.date() == yesterday.date()
            }
            DatePattern::ThisWeek => {
                let week_start = now - chrono::Duration::days(now.weekday().num_days_from_monday() as i64);
                file_date >= week_start
            }
            DatePattern::LastWeek => {
                let week_start = now - chrono::Duration::days(now.weekday().num_days_from_monday() as i64);
                let last_week_start = week_start - chrono::Duration::weeks(1);
                let last_week_end = week_start;
                file_date >= last_week_start && file_date < last_week_end
            }
            DatePattern::ThisMonth => {
                file_date.year() == now.year() && file_date.month() == now.month()
            }
            DatePattern::LastMonth => {
                let (year, month) = if now.month() == 1 {
                    (now.year() - 1, 12)
                } else {
                    (now.year(), now.month() - 1)
                };
                file_date.year() == year && file_date.month() == month
            }
            DatePattern::ThisYear => {
                file_date.year() == now.year()
            }
            DatePattern::LastYear => {
                file_date.year() == now.year() - 1
            }
        }
    }
}

/// 获取默认标签规则
pub fn default_rules() -> Vec<TagRule> {
    vec![
        // 文件类型规则
        TagRule {
            name: "图片".to_string(),
            condition: TagCondition::FileType(vec![FileType::Image]),
        },
        TagRule {
            name: "音频".to_string(),
            condition: TagCondition::FileType(vec![FileType::Audio]),
        },
        TagRule {
            name: "视频".to_string(),
            condition: TagCondition::FileType(vec![FileType::Video]),
        },
        TagRule {
            name: "文本".to_string(),
            condition: TagCondition::FileType(vec![FileType::Text]),
        },
        TagRule {
            name: "二进制".to_string(),
            condition: TagCondition::FileType(vec![FileType::Binary]),
        },

        // 文件大小规则
        TagRule {
            name: "小文件".to_string(),
            condition: TagCondition::FileSize {
                min: None,
                max: Some(10 * 1024), // < 10KB
            },
        },
        TagRule {
            name: "大文件".to_string(),
            condition: TagCondition::FileSize {
                min: Some(100 * 1024 * 1024), // > 100MB
                max: None,
            },
        },

        // 日期规则
        TagRule {
            name: "今日文件".to_string(),
            condition: TagCondition::DatePattern(DatePattern::Today),
        },
        TagRule {
            name: "本周文件".to_string(),
            condition: TagCondition::DatePattern(DatePattern::ThisWeek),
        },
        TagRule {
            name: "本月文件".to_string(),
            condition: TagCondition::DatePattern(DatePattern::ThisMonth),
        },

        // 路径规则
        TagRule {
            name: "下载".to_string(),
            condition: TagCondition::PathContains("downloads".to_string()),
        },
        TagRule {
            name: "文档".to_string(),
            condition: TagCondition::PathContains("documents".to_string()),
        },
        TagRule {
            name: "桌面".to_string(),
            condition: TagCondition::PathContains("desktop".to_string()),
        },
        TagRule {
            name: "图片".to_string(),
            condition: TagCondition::PathContains("pictures".to_string()),
        },
        TagRule {
            name: "音乐".to_string(),
            condition: TagCondition::PathContains("music".to_string()),
        },
        TagRule {
            name: "视频".to_string(),
            condition: TagCondition::PathContains("videos".to_string()),
        },
    ]
}
