mod tag;
mod search;
mod file;
mod watcher;

pub use tag::*;
pub use search::*;
pub use file::*;
pub use watcher::*;

use crate::db::Database;
use tauri::Manager;

/// 注册所有 Tauri 命令
pub fn register_commands(db: Database) {
    tauri::Builder::default()
        .setup(move |app| {
            // 将数据库实例存储到 app state 中
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 标签相关
            get_all_tags,
            get_tags_by_file,
            create_tag,
            add_tag_to_file,
            remove_tag_from_file,
            batch_add_tags,
            get_files_by_tags,
            delete_tag,
            update_tag,

            // 搜索相关
            search_files,

            // 文件相关
            get_files,
            get_file_by_id,
            get_file_by_path,
            add_file,
            delete_file,
            get_stats,

            // 监控目录相关
            get_watched_directories,
            add_watched_directory,
            remove_watched_directory,
            update_watched_directory,
            scan_directory,
        ]);
}
