#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;
use std::{cmp::Ordering, fs};
use tauri::regex::{self, Regex};
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

fn compare_last_number(a: &str, b: &str, re: &regex::Regex) -> Ordering {
    // Utilisation de la méthode `find_iter` pour récupérer un iterator sur tous les nombres de chaque chaîne
    let a_number = re.find_iter(a).last().map(|m| m.as_str()).unwrap_or("");
    let b_number = re.find_iter(b).last().map(|m| m.as_str()).unwrap_or("");

    // Conversion des nombres en entiers et comparaison
    let a_int = a_number.parse::<i32>().unwrap_or(0);
    let b_int = b_number.parse::<i32>().unwrap_or(0);
    a_int.cmp(&b_int)
}

//Thanks to Golden_Water. Go see his bilibili profile. (space.bilibili.com/41925356)
fn list_subdirectorie(dir: &Path, id: &mut i64) -> Vec<Entry> {
    let mut sub_entries = vec![];
    let re = Regex::new(r"(\d+)").unwrap();

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // println!("{:?}", path);
        let mut name = path.file_name().unwrap().to_str().unwrap().to_string();

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
            name = re
                .replace_all(&name, |caps: &regex::Captures| {
                    format!("{:03}", caps[0].parse::<i32>().unwrap())
                })
                .to_string();
            sub_entries.push(Entry::File {
                text: name,
                id: *id,
            });
        }
    }
    sub_entries.sort_by(|a, b| compare_last_number(a.file_text(), b.file_text(), &re));
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
