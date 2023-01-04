#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use tauri_plugin_store::PluginBuilder;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn liste_fichiers(dossier: &str) -> Vec<String> {
    let mut chemins = Vec::new();

    // Récupère l'itérable de tous les éléments du dossier
    for entry in fs::read_dir(dossier).unwrap() {
        // Récupère le chemin du fichier ou du dossier
        let entry = entry.unwrap();
        let chemin = entry.path();

        // Ajoute le chemin du fichier ou du dossier à la liste
        chemins.push(chemin.to_str().unwrap().to_string());
    }
    chemins
}

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .invoke_handler(tauri::generate_handler![liste_fichiers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
