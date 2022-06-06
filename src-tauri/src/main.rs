#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hello])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn hello(_window: tauri::Window, a: i32, b: String) -> Result<i32, String> {
  // println!("{:?}", window);
  println!("{}, {}", a, b);
  Ok(a)
}

