// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use tauri::{tray::ClickType, Manager};
use tauri_plugin_positioner::{Position,WindowExt};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_positioner::init())
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Focused(focused) => {
              // hide window whenever it loses focus
                if !focused {
                    window.hide().unwrap();
                }
            }
            _ => {}
          })
         .setup(move |app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let icon = tauri::Icon::File(PathBuf::from(
                "./icons/icon.ico",
            ));
            let _tray = tauri::tray::TrayIconBuilder::new()
                .icon(icon)
                .on_tray_icon_event(|tray, event| {
                    let app = tray.app_handle();
                    tauri_plugin_positioner::on_tray_event(app, &event);
                    match event.click_type {
                        ClickType::Left  => {
                                let window = app.get_window("main").unwrap();
                                let _ = window.move_window(Position::TrayBottomCenter);
                                let _ = match window.is_visible() {
                                    Ok(true) => {
                                        let _ = window.hide();
                                        Ok(())
                                    }
                                    Ok(false) => {
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                        Ok(())
                                    }
                                    Err(e) => Err(e),
                                };
                        }
                        ClickType::Right => {
                            dbg!("right click");
                        }
                        _ => (),
                    }
                })
                .icon_as_template(true)
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
