use crate::db::{Database, Tag, TagType};

/// 获取所有标签
#[tauri::command]
pub fn get_all_tags(state: tauri::State<Database>) -> std::result::Result<Vec<Tag>, String> {
    state.get_all_tags().map_err(|e| e.to_string())
}

/// 获取文件的标签
#[tauri::command]
pub fn get_tags_by_file(
    file_id: i64,
    state: tauri::State<Database>,
) -> std::result::Result<Vec<Tag>, String> {
    state.get_tags_by_file(file_id).map_err(|e| e.to_string())
}

/// 创建标签
#[tauri::command]
pub fn create_tag(
    name: String,
    display_name: String,
    color: String,
    #[allow(dead_code)] icon: Option<String>,
    state: tauri::State<Database>,
) -> std::result::Result<i64, String> {
    println!("[Rust] create_tag called with: name={}, display_name={}, color={}", name, display_name, color);

    let tag = Tag {
        id: None,
        name: name.clone(),
        display_name: display_name.clone(),
        tag_type: TagType::Custom,
        color: color.clone(),
        icon: icon,
        use_count: 0,
        created_at: chrono::Utc::now(),
    };

    println!("[Rust] Tag to create: {:?}", tag);

    match state.create_tag(&tag) {
        Ok(id) => {
            println!("[Rust] Tag created with id: {}", id);
            Ok(id)
        },
        Err(e) => {
            eprintln!("[Rust] Failed to create tag: {}", e);
            Err(e.to_string())
        },
    }
}

/// 添加标签到文件
#[tauri::command]
pub fn add_tag_to_file(
    file_id: i64,
    tag_name: String,
    state: tauri::State<Database>,
) -> std::result::Result<(), String> {
    state.add_tag_to_file_by_name(file_id, &tag_name, false).map_err(|e| e.to_string())
}

/// 从文件移除标签
#[tauri::command]
pub fn remove_tag_from_file(
    file_id: i64,
    tag_id: i64,
    state: tauri::State<Database>,
) -> std::result::Result<(), String> {
    state.remove_tag_from_file(file_id, tag_id).map_err(|e| e.to_string())
}

/// 批量添加标签到文件
#[tauri::command]
pub fn batch_add_tags(
    file_ids: Vec<i64>,
    tag_names: Vec<String>,
    state: tauri::State<Database>,
) -> std::result::Result<(), String> {
    state.batch_add_tags(&file_ids, &tag_names).map_err(|e| e.to_string())
}

/// 根据标签获取文件
#[tauri::command]
pub fn get_files_by_tags(
    tag_names: Vec<String>,
    state: tauri::State<Database>,
) -> std::result::Result<Vec<crate::db::File>, String> {
    state.get_files_by_tags(&tag_names).map_err(|e| e.to_string())
}

/// 删除标签
#[tauri::command]
pub fn delete_tag(
    tag_id: i64,
    state: tauri::State<Database>,
) -> std::result::Result<(), String> {
    use rusqlite::params;

    let conn = state.conn.lock();
    conn.execute("DELETE FROM tags WHERE id = ?1", params![tag_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 更新标签
#[tauri::command]
pub fn update_tag(
    tag_id: i64,
    display_name: String,
    color: String,
    state: tauri::State<Database>,
) -> std::result::Result<(), String> {
    use rusqlite::params;

    let conn = state.conn.lock();
    conn.execute(
        "UPDATE tags SET display_name = ?1, color = ?2 WHERE id = ?3",
        params![display_name, color, tag_id],
        ).map_err(|e| e.to_string())?;
    Ok(())
}
