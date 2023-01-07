#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::path::Path;
use tauri::regex::Regex;
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

fn modify_dirs(entry: &mut Entry) {
    match entry {
        Entry::Dir { children, .. } => {
            for child in children {
                modify_dirs(child);
            }

            if let Entry::Dir { children, .. } = entry {
                children.sort();
            }
        }
        Entry::File { .. } => {}
    }
}

#[tauri::command]
fn list_subdirectories(dir: &Path, id: i64) -> String {
    let mut idd: i64 = id;
    let tree: Vec<Entry> = list_subdirectorie(dir, &mut idd);
    // for entry in &tree {
    //     // println!("{:?}", entry);
    // }
    let ret_json = serde_json::to_string_pretty(&tree).unwrap();
    ret_json
}

trait FileText {
    fn file_text(&self) -> &str;
}

impl FileText for Entry {
    fn file_text(&self) -> &str {
        match self {
            Entry::Dir { text, .. } => text,
            Entry::File { text, .. } => text,
        }
    }
}

//Thanks to Golden_Water. Go see his bilibili profile. (space.bilibili.com/41925356)
fn list_subdirectorie(dir: &Path, id: &mut i64) -> Vec<Entry> {
    let mut sub_entries = vec![];
    let re = Regex::new(r"(\d+)").unwrap();

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // println!("{:?}", path);
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
    sub_entries.sort_by(|a, b| {
        let a_num: i32 = re
            .captures(a.file_text())
            .and_then(|caps| caps.get(1).map(|m| m.as_str().parse().unwrap()))
            .unwrap_or(std::i32::MAX);
        let b_num: i32 = re
            .captures(b.file_text())
            .and_then(|caps| caps.get(1).map(|m| m.as_str().parse().unwrap()))
            .unwrap_or(std::i32::MAX);
        a_num.cmp(&b_num)
    });
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
