#[tauri::command]
pub fn mul(a: f32, b: f32) -> f32 {
    a * b
}
