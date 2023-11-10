use tauri::{
    menu::{Menu, MenuItem, Submenu},
    tray::{ClickType, TrayIconBuilder},
    Manager, Runtime,
};


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
        .on_menu_event(move |app, event| super::event::build_menu_evnet(app, event))
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
