extern crate systray;

#[cfg(not(target_os = "windows"))]
fn main() {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!")
    }
    app.set_icon_from_file(&"/usr/share/gxkb/flags/ua.png".to_string()).ok();
    app.add_menu_item(&"Print a thing".to_string(), |_| {
        println!("Printing a thing!");
    }).ok();
    app.add_menu_item(&"Add Menu Item".to_string(), |window| {
        window.add_menu_item(&"Interior item".to_string(), |_| {
            println!("what");
        }).ok();
        window.add_menu_separator().ok();
    }).ok();
    app.add_menu_separator().ok();
    app.add_menu_item(&"Quit".to_string(), |window| {
        window.quit();
    }).ok();
    println!("Waiting on message!");
    app.wait_for_message();
}

#[cfg(target_os = "macos")]
fn main() {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create tray icon app!")
    }

    const ICON_BUFFER: &'static [u8] = include_bytes!("rust-logo.png");
    app.set_icon_from_buffer(ICON_BUFFER, 256, 256).unwrap();
    app.wait_for_message();
}
