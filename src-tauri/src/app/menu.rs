use tauri::{
    menu::{Menu, MenuItem, Submenu},
    Runtime,
};

pub fn build_menu<R: Runtime>(app_handle: &tauri::AppHandle<R>) -> tauri::Result<Menu<R>> {
    let main = MenuItem::with_id(app_handle, "main", "主页", true, None);
    let setting = MenuItem::with_id(app_handle, "setting", "设置", true, None);
    let about = about_menu(app_handle);
    let example = example_menu(app_handle);

    Ok(Menu::with_items(app_handle, &[&main, &setting, &about, &example])?)
}

fn about_menu<R: Runtime>(app_handle: &tauri::AppHandle<R>) -> Submenu<R> {
    Submenu::with_items(
        app_handle,
        "关于",
        true,
        &[
            &MenuItem::with_id(app_handle, "about", "关于", true, None),
            &MenuItem::with_id(app_handle, "wiki", "文档", true, None),
            &MenuItem::with_id(app_handle, "issues", "Issues", true, None),
            &MenuItem::with_id(app_handle, "github", "Github", true, None),
            &MenuItem::with_id(app_handle, "check_update", "检查更新", true, None),
        ],
    ).unwrap()
}

fn example_menu<R: Runtime>(app_handle: &tauri::AppHandle<R>) -> Submenu<R> {
    let event = Submenu::with_items(
        app_handle,
        "事件",
        true,
        &[&MenuItem::with_id(
            app_handle,
            "call_font",
            "调用前端",
            true,
            None,
        )],
    ).unwrap();
    let dialog = Submenu::with_items(
        app_handle,
        "弹窗",
        true,
        &[
            &MenuItem::with_id(app_handle, "dialog_1", "弹窗-1", true, None),
            &MenuItem::with_id(app_handle, "dialog_2", "弹窗-2-Error", true, None),
            &MenuItem::with_id(app_handle, "dialog_3", "弹窗-3-Info", true, None),
            &MenuItem::with_id(app_handle, "dialog_4", "弹窗-4-Warning", true, None),
            &MenuItem::with_id(app_handle, "dialog_5", "弹窗-5-default", true, None),
            &MenuItem::with_id(app_handle, "dialog_6", "弹窗-6-button", true, None),
        ],
    ).unwrap();
    Submenu::with_items(app_handle, "例子", true, &[&event, &dialog]).unwrap()
}
