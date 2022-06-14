#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod dao;
use crate::dao::imageview_dao::{ImageViewDao, ImagesMetaList, ImagesMeta, BrowseSettings};
use serde_with::serde_as;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      init_table,
      add_images_meta,
      get_images_meta_list,
      get_images_folder_info,
      get_images_meta,
      update_browse_settings,
      get_browse_settings,
      delete_images_meta,
      update_images_meta,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn get_dao() -> ImageViewDao {
  // let dir = std::env::current_dir().unwrap().join("data");
  let dir = dire::Directories::with_prefix("imageview", "ImageView").unwrap();
  let path = dir.data_home();
  let create_result = std::fs::create_dir_all(&path);
  match create_result {
    Ok(_) => (),
    // Err(error) => println!("创建data目录失败 {:?}", error),
    Err(_) => (),
  }
  ImageViewDao::new(path.join("images.db"))
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
  _window: tauri:: Window,
  search: &str,
  page: i64,
  page_size: i64,
) -> Result<ImagesMetaList, String> {
  let dao = get_dao();
  dao.get_images_meta_list(search, page, page_size)
}

#[serde_as]
#[derive(serde::Serialize)]
struct ImagesFolderInfoFile {
  is_dir: bool,
  path: String,
  name: String,
  files: Vec<String>,
}

#[serde_as]
#[derive(serde::Serialize)]
struct ImagesFolderInfo {
  name: String,
  files: Vec<ImagesFolderInfoFile>, // 2度文件目录
}

fn is_image_by_suffix(s: &String) -> bool {
  let s = s.to_lowercase();
  for suffix in [".jpeg", ".jpg", ".webp", ".gif", ".png", ".bmp"] {
    if s.ends_with(suffix) {
      return true
    }
  }
  false
}

#[tauri::command]
fn get_images_folder_info(_window: tauri:: Window, path_str: &str) -> Result<ImagesFolderInfo, String> {
  // 前端传个文件夹路径过来，后台返回目录相关信息
  let path = std::path::PathBuf::from(path_str);
  let name = String::from(path.file_name().unwrap().to_str().unwrap());

  // 读二级目录里面的图片文件
  let mut info = ImagesFolderInfo {
    name: name,
    files: vec![],
  };
  for entry in path.read_dir().expect("read_dir call failed") {
    if let Ok(entry) = entry {
      let mut files2: Vec<String> = vec![];
      if entry.path().is_dir() {
        for entry2 in entry.path().read_dir().expect("read_dir call failed") {
          if let Ok(entry2) = entry2 {
            let fp = String::from(entry2.path().to_str().unwrap());
            if is_image_by_suffix(&fp) {
              files2.push(fp);
            }
          }
        }
        info.files.push(ImagesFolderInfoFile {
          is_dir: entry.path().is_dir(),
          path: String::from(entry.path().to_str().unwrap()),
          name: String::from(entry.path().file_name().unwrap().to_str().unwrap()),
          files: files2,
        });
      } else {
        let fp = String::from(entry.path().to_str().unwrap());
        if is_image_by_suffix(&fp) {
          info.files.push(ImagesFolderInfoFile {
            is_dir: entry.path().is_dir(),
            path: fp,
            name: String::from(entry.path().file_name().unwrap().to_str().unwrap()),
            files: files2,
          });
        }
      }
    }
  }

  Ok(info)
}

#[tauri::command]
fn get_images_meta(_window: tauri:: Window, id: i64) -> Result<ImagesMeta, String> {
  let dao = get_dao();
  dao.get_images_meta(id)
}

#[tauri::command]
fn update_browse_settings(
  _window: tauri:: Window,
  meta_id: i64,
  browse_type: &str, 
  home_page: bool,
  current_path: &str,
  current_index: i64,
) -> Result<(), String> {
  let dao = get_dao();
  dao.update_browse_settings(meta_id, browse_type, home_page, current_path, current_index);
  Ok(())
}

#[tauri::command]
fn get_browse_settings(_window: tauri:: Window, meta_id: i64) -> Result<BrowseSettings, String> {
  let dao = get_dao();
  dao.get_browse_settings(meta_id)
}

#[tauri::command]
fn delete_images_meta(_window: tauri:: Window, id: i64) -> Result<(), String> {
  let dao = get_dao();
  dao.delete_images_meta(id)
}

#[tauri::command]
fn update_images_meta(
  _window: tauri:: Window,
  id: i64,
  path: &str, 
  title: &str, 
  author: &str, 
  intro: &str,
  cover: &str,
) -> Result<(), String> {
  let dao = get_dao();
  dao.update_images_meta(id, path, title, author, intro, cover);
  Ok(())
}
