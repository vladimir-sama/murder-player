// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn get_playlist(playlist_url: &str) -> String {
    let mut return_html: String = String::new();

    let output = Command::new("yt-dlp")
        .args(&[
            "--flat-playlist",
            "--print",
            "%(title)s %(url)s",
            "--skip-download",
            playlist_url,
        ])
        .output()
        .expect("Failed to execute yt-dlp");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        let mut index: u32 = 0;

        for line in result.lines() {
            if let Some(last_space_index) = line.rfind(' ') {
                let title = &line[..last_space_index].trim();
                let video_url = &line[last_space_index + 1..].trim();
                index += 1;

                return_html += &format!(
                    "<p class='element_parent'>{}. <button onclick='play_track(\"{}\", \"{}\")' class='element' type='button'>{}</button></p>",
                    index, video_url, title, title
                );
            }
        }
    } else {
        eprintln!(
            "Error fetching playlist: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    return_html
}

#[tauri::command]
fn get_url(video_url: &str) -> String {
    let mut audio_url: String = String::new();
    let audio_output = Command::new("yt-dlp")
        .args(&["-f", "bestaudio", "--get-url", video_url])
        .output()
        .expect("Failed to execute yt-dlp for audio");

    if audio_output.status.success() {
        audio_url = String::from_utf8_lossy(&audio_output.stdout)
            .trim()
            .to_string();
    }

    audio_url
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_playlist, get_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
