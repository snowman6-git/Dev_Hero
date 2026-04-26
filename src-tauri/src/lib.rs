// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{LogicalSize, Window};

fn init(window: Window){
    center_window(&window);
}
fn center_window(window: &Window){
    let _ = window.center()
    .expect("window init failed!");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn resize_window(width: f32, height: f32, window: Window) {
    println!("!!!");
    
    // 창 크기 변경 (LogicalSize 사용 시 해상도 배율 자동 대응)
    center_window(&window);
    let _ = window.set_size(LogicalSize::new(width, height));
    println!("Resize Window in {}x{}", width, height)
}
#[tauri::command]
fn fullscreen_window(window: Window) {
    let _ = window.set_fullscreen(true);
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, resize_window, fullscreen_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}