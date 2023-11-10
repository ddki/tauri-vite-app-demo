use tauri::{ Manager, menu::Menu, Runtime};

#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) {
    println!("close-splashscreen");
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

pub fn open_about<R: Runtime>(app: &tauri::AppHandle<R>) {
    println!("open_about...");
    let windows = app.windows();
    let about_window = windows.get("about");
    match about_window {
        Some(about_window) => about_window.show().unwrap(),
        None => crate::utils::create_window(app, "about", "关于", "#/about", Menu::default(app).unwrap()),
    }
}

pub fn open_wiki<R: Runtime>(app: &tauri::AppHandle<R>) {
    println!("open_wiki...");
    let windows = app.windows();
    let wiki_window = windows.get("wiki");
    match wiki_window {
        Some(wiki_window) => wiki_window.show().unwrap(),
        None => crate::utils::create_window(app, "wiki", "文档", "#/wiki", Menu::default(app).unwrap()),
    }
}

