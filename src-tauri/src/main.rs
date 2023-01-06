#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::any::Any;
use std::fs;
use std::path::Path;
use tauri_plugin_store::PluginBuilder;

//Thanks to Golden_Water. Go see his bilibili profile. (space.bilibili.com/41925356)
#[derive(Debug, serde::Serialize, serde::Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(untagged)]
enum Entry {
    Dir {
        text: String,
        id: i64,
        children: Vec<Entry>,
    },
    File {
        text: String,
        id: i64,
    },
}

#[tauri::command]
fn list_subdirectories(dir: &Path, id: i64) -> String {
    let mut idd: i64 = id;
    let tree: Vec<Entry> = list_subdirectorie(dir, &mut idd);
    let ret_json = serde_json::to_string_pretty(&tree).unwrap();
    ret_json
}

//Thanks to Golden_Water. Go see his bilibili profile. (space.bilibili.com/41925356)
fn list_subdirectorie(dir: &Path, id: &mut i64) -> Vec<Entry> {
    let mut sub_entries = vec![];

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        if path.is_dir() {
            *id += 1;
            let id_old: i64 = *id;
            let children = list_subdirectorie(&path, id);
            sub_entries.push(Entry::Dir {
                text: name,
                id: id_old,
                children,
            });
        } else {
            *id += 1;
            sub_entries.push(Entry::File {
                text: name,
                id: *id,
            });
        }
    }
    sub_entries.sort_by_key(|d| d.type_id());
    sub_entries
}

#[tauri::command]
fn list_files(dir: &Path) -> Vec<String> {
    let mut files = vec![];

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            files.extend(list_files(&path));
        } else {
            files.push(path.to_str().unwrap().to_owned());
        }
    }

    files
}

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .invoke_handler(tauri::generate_handler![list_subdirectories, list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
