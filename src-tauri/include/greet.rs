// 在后端定义并注册了一个指令，前端可以进行调用
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
