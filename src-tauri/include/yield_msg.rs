#[tauri::command]
pub fn yield_msg() -> String {
    "msg from tauri backend".into()
}
