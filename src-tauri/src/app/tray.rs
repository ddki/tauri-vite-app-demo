use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};
use tauri_plugin_shell::ShellExt;

pub fn build() -> SystemTray {
    SystemTray::new().with_menu(build_menu())
}

pub fn handle_menu_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
            let window: tauri::Window = app.get_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "main" => {
                let window: tauri::Window = app.get_window("main").unwrap();
                window.eval("window.location.replace('#/')").unwrap();
            }
            "setting" => {
                let window: tauri::Window = app.get_window("main").unwrap();
                window.eval("window.location.replace('#/setting')").unwrap();
            }
            "about" => {
                let window: tauri::Window = app.get_window("main").unwrap();
                window.menu_handle().hide().unwrap();
                super::window::open_about(window.app_handle());
            }
            "wiki" => {
                let window: tauri::Window = app.get_window("main").unwrap();
                window.menu_handle().hide().unwrap();
                super::window::open_wiki(window.app_handle());
            }
            "issues" => {
                app.get_window("main")
                    .unwrap()
                    .shell()
                    .open("https://github.com/ddki/tauri-vite-app-demo/issues", None)
                    .unwrap();
            }
            "github" => {
                app.get_window("main")
                    .unwrap()
                    .shell()
                    .open("https://github.com/ddki/tauri-vite-app-demo", None)
                    .unwrap();
            }
            "check_update" => {
                println!("check_update...");
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}

fn build_menu() -> SystemTrayMenu {
    let main = CustomMenuItem::new("main", "主页");
    let setting = CustomMenuItem::new("setting".to_string(), "设置");
    let about = SystemTraySubmenu::new(
        "关于",
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("about", "关于"))
            .add_item(CustomMenuItem::new("wiki", "文档"))
            .add_item(CustomMenuItem::new("issues", "Issues"))
            .add_item(CustomMenuItem::new("github", "Github")),
    );

    let check_update = CustomMenuItem::new("check_update", "检查更新");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu: SystemTrayMenu = SystemTrayMenu::new()
        .add_item(main)
        .add_item(setting)
        .add_submenu(about)
        .add_item(check_update)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    return tray_menu;
}
