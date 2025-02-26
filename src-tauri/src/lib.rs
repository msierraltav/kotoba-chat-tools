use windows::{core::Media::Control};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn dotnet_request(request: &str) -> String{
    tauri_dotnet_bridge_host::process_request(request)
}

// a tauri coomand to get the current Media / now playing song information.
#[tauri::command]
fn get_current_media() -> String {
    // get the current media information using the windows crate
    let media: String = windows::Media::Control::GetCurrentSession().unwrap();
    // return the media information as a string
    media
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, dotnet_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
