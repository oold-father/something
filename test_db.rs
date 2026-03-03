use rusqlite::Connection;

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    // 测试单个语句
    conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY);", []).unwrap();
    println!("CREATE TABLE 成功");

    // 测试批量执行
    let sql = "
CREATE TABLE test2 (id INTEGER PRIMARY KEY);
CREATE TABLE test3 (id INTEGER PRIMARY KEY);
";
    conn.execute_batch(sql).unwrap();
    println!("execute_batch 成功");
}
