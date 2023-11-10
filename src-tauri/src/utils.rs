use tauri::{Manager, WindowBuilder, Runtime, menu::Menu};

//创建新的窗口 label唯一标识
pub fn create_window<R: Runtime>(app: &tauri::AppHandle<R>, label: &str, title: &str, router: &str, menu: Menu<R>) {
    match app.get_window(label) {
        Some(js) => {
            let _r = js.set_focus();
        }
        None => {
            WindowBuilder::new(app, label, tauri::WindowUrl::App(router.into()))
                .center()
                .menu(menu)
                .title(title)
                .build()
                .unwrap();
        }
    }
}
