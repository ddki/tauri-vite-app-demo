use tauri::{
    menu::{Menu, MenuItem, Submenu},
    tray::{ClickType, TrayIconBuilder},
    Manager, Runtime,
};
use tauri_plugin_shell::ShellExt;


pub fn create_tray<R: Runtime>(app_handle: &tauri::AppHandle<R>) -> tauri::Result<()> {
    // 创建菜单
    let main = MenuItem::with_id(app_handle, "main", "主页", true, None);
    let setting = MenuItem::with_id(app_handle, "setting", "设置", true, None);
    let about = Submenu::with_id_and_items(
        app_handle,
        "about_group",
        "关于",
        true,
        &[
            &MenuItem::with_id(app_handle, "about", "关于", true, None),
            &MenuItem::with_id(app_handle, "wiki", "文档", true, None),
            &MenuItem::with_id(app_handle, "issues", "Issues", true, None),
            &MenuItem::with_id(app_handle, "github", "Github", true, None),
        ],
    )?;
    let check_update = MenuItem::with_id(app_handle, "check_update", "检查更新", true, None);
    let quit = MenuItem::with_id(app_handle, "quit", "退出", true, None);
    let menu = Menu::with_items(app_handle, &[&main, &setting, &about, &check_update, &quit])?;

    let _ = TrayIconBuilder::with_id("tray")
        .tooltip("Tauri-Vite-App-Template")
        .icon(app_handle.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app_handle, event| match event.id.as_ref() {
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
					_ => {}
				})
        .on_tray_icon_event(|tray, event| {
            if event.click_type == ClickType::Left {
                let app = tray.app_handle();
                if let Some(window) = app.get_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            } else if event.click_type == ClickType::Right {
                println!("tray icon recived a right click.")
            } else if event.click_type == ClickType::Double {
                println!("tray icon recived a double click.")
            }
        })
        .build(app_handle);

    Ok(())
}
