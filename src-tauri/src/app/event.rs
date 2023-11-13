use tauri::{menu::MenuEvent, AppHandle, Manager, Runtime};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub fn greet(name: &str) -> String {
    println!("arg = {}", name);
    format!("Hello, {}!", name)
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[derive(Clone, serde::Serialize)]
struct Setting {
    user: String,
}

// #[tauri::command]
// pub fn save_setting(setting: Setting) -> String {}

pub fn build_app_event<R: Runtime>(app_handle: &AppHandle<R>) {
    // listen to the `event-name` (emitted on any window)
    let id = app_handle.listen_global("listen_global", |event| {
        println!("got click with payload {:?}", event.payload());
    });
    // unlisten to the event using the `id` returned on the `listen_global` function
    // a `once_global` API is also exposed on the `App` struct
    app_handle.unlisten(id);

    // emit the `event-name` event to all webview windows on the frontend
    app_handle.emit(
        "click_emit",
        Payload {
            message: "Tauri is awesome!".into(),
        },
    )
    .unwrap();
}

pub fn build_menu_evnet<R: Runtime>(app_handle: &AppHandle<R>, event: MenuEvent) {
    match event.id.as_ref() {
        // 主程序和任务栏图标共有
        "main" => {
            let window = app_handle.get_window("main").unwrap();
						if !window.is_visible().unwrap_or_default() {
							let _ = window.show();
							let _ = window.set_focus();
						}
            window.eval("window.location.replace('#/')").unwrap();
        }
        "setting" => {
            let window = app_handle.get_window("main").unwrap();
						if !window.is_visible().unwrap_or_default() {
							let _ = window.show();
							let _ = window.set_focus();
						}
            let _ = window.eval("window.location.replace('#/setting')").unwrap();
        }
        "about" => {
            let window = app_handle.get_window("main").unwrap();
            window.hide().unwrap();
            super::window::open_about(window.app_handle());
            println!("tray menu: click about menu");
        }
        "wiki" => {
            let window = app_handle.get_window("main").unwrap();
            window.hide().unwrap();
            super::window::open_wiki(window.app_handle());
            println!("tray menu: click wiki menu");
        }
        "issues" => {
            app_handle
                .get_window("main")
                .unwrap()
                .shell()
                .open("https://github.com/ddki/tauri-vite-app-template/issues", None)
                .unwrap();
        }
        "github" => {
            app_handle
                .get_window("main")
                .unwrap()
                .shell()
                .open("https://github.com/ddki/tauri-vite-app-template", None)
                .unwrap();
        }
        "check_update" => {
            println!("check_update...");
        }
        "quit" => {
            std::process::exit(0);
        }
        // 主程序菜单独有
        "close" => {
						let window = app_handle.get_window("main").unwrap();
						let _ = window.close();
        }
        // 例子
        "call_font" => {
						let _ = app_handle.emit("fontGrobalListenEvent", "rust 调用 vue");
				}
        "dialog_1" => {
            DialogExt::dialog(app_handle)
                .message("我是弹窗-1")
                .blocking_show();
        }
        "dialog_2" => {
            DialogExt::dialog(app_handle)
                .message("我是弹窗-2-Error")
                .kind(MessageDialogKind::Error)
                .blocking_show();
        }
        "dialog_3" => {
            DialogExt::dialog(app_handle)
                .message("我是弹窗-3-Info")
                .kind(MessageDialogKind::Info)
                .blocking_show();
        }
        "dialog_4" => {
            DialogExt::dialog(app_handle)
                .message("我是弹窗-4-Warning")
                .kind(MessageDialogKind::Warning)
                .blocking_show();
        }
        "dialog_5" => {
            DialogExt::dialog(app_handle)
                .message("我是弹窗-5-default")
                .kind(MessageDialogKind::default())
                .blocking_show();
        }
        "dialog_6" => {
            DialogExt::dialog(app_handle)
                .message("我是弹窗-6-button")
                .kind(MessageDialogKind::default())
                .cancel_button_label("取消")
                .ok_button_label("确定")
                .title("我是弹窗标题")
                .blocking_show();
        }
        _ => {}
    }
}
