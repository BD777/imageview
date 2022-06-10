#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod dao;
use crate::dao::imageview_dao::{ImageViewDao, ImagesMetaList};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      hello,
      init_table,
      add_images_meta,
      get_images_meta_list,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn get_dao() -> ImageViewDao {
  ImageViewDao::new(std::env::current_dir().unwrap().join("images.db"))
}

#[tauri::command]
fn hello(_window: tauri::Window, a: i32, b_c: String) -> Result<i32, String> {
  // println!("{:?}", window);
  // println!("{}, {}", a, b);
  // Ok(a)
  Err(b_c)
}

#[tauri::command]
fn init_table(_window: tauri::Window) -> Result<(), String> {
  let dao = get_dao();
  dao.init_table();
  Ok(())
}

#[tauri::command]
fn add_images_meta(
  _window: tauri:: Window,
  path: &str, 
  title: &str, 
  author: &str, 
  intro: &str,
  cover: &str,
) -> Result<(), String> {
  let dao = get_dao();
  dao.add_images_meta(path, title, author, intro, cover);
  Ok(())
}

#[tauri::command]
fn get_images_meta_list(
  // _window: tauri:: Window,
  page: i64,
  page_size: i64,
) -> Result<ImagesMetaList, String> {
  let dao = get_dao();
  dao.get_images_meta_list(page, page_size)
}
