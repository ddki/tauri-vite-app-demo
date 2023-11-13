#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{thread::sleep, time::Duration};

use tauri::{menu::MenuEvent, window::PageLoadEvent, Manager};

mod app;
pub mod utils;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .on_page_load(|window, payload| {
            if payload.event() == PageLoadEvent::Finished {
                let label = window.label().to_string();
                window.listen("clicked".to_string(), move |_payload| {
                    println!("got 'clicked' event on window '{label}'");
                });
            }
        })
        .invoke_handler(tauri::generate_handler![
            crate::app::event::greet,
            // crate::app::window::close_splashscreen
        ])
        .menu(|handle| app::menu::build_menu(handle))
        .setup(|app| {
            let handle = app.handle();
            // 闪屏
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                splashscreen_window.close().unwrap();
								sleep(Duration::from_secs(5));
                main_window.show().unwrap();
            });
            // 任务栏图标
            #[cfg(desktop)]
            {
                app::tray::create_tray(&handle)?;
            }
            // 菜单事件
            handle.on_menu_event(|handle: &tauri::AppHandle, event: MenuEvent| {
                app::event::build_menu_evnet(handle, event)
            });

            let main_window = handle.get_window("main").unwrap();
            main_window.close_devtools();
            // 事件
            crate::app::event::build_app_event(app.app_handle());
            Ok(())
        })
        // 保持前端在后台运行，以实现系统托盘左击显示窗口
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        // 保持后端在后台运行
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
