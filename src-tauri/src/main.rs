// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::stdout;
use std::sync::Arc;
use serde_json::Value::Bool;
use tauri::{Manager, Window, CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, WindowBuilder, WindowEvent, PhysicalPosition};
use window_shadows::set_shadow;


fn main() {
    let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let system_tray = SystemTray::new()
        // .with_menu(tray_menu) 这里就不要加默认的菜单了，不然虽然看不见但是会抢我们自定义的菜单的焦点，触发隐藏
        .with_tooltip("custom");
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app,event| {
            match event {
                SystemTrayEvent::MenuItemClick { .. } => {}
                SystemTrayEvent::LeftClick { .. } => {
                    let main_window = app.get_window("main").unwrap();
                    if !main_window.is_visible().unwrap(){
                        main_window.show().unwrap();
                    }else if main_window.is_minimized().unwrap() {
                        main_window.unminimize().unwrap();
                    }
                    main_window.set_focus().unwrap();
                }
                SystemTrayEvent::RightClick {
                    position: p,
                    size: _,
                    ..
                } => {
                    //本来以为要自己build这个窗口的，但是报错，后来发现直接获取就行了
                    //这样写有没有问题呢？每次点击都会弹出这个窗口包装成一个Arc然后move了一个副本到闭包中。
                    let tray_window = Arc::new(app.get_window("TrayPane").expect("没有该窗口"));
                    let size = tray_window.outer_size().unwrap();
                    let position_y = p.y - size.height as f64;
                    let position_x = p.x + 25.0;
                    tray_window.set_position(PhysicalPosition::new(position_x,position_y)).unwrap();
                    tray_window.show().unwrap();
                    // tray_window.set_always_on_top(true).unwrap();//不置顶这个窗口会被挡住
                    tray_window.set_focus().unwrap();
                    let arc_tray = tray_window.clone();
                    tray_window.on_window_event(move |event|match event{
                        WindowEvent::CloseRequested { .. } => {}
                        WindowEvent::Destroyed => {}
                        WindowEvent::Focused(b) => {
                            if !b {
                                // arc_tray.close().unwrap();close后再打开报错，应该要重新build了
                                arc_tray.hide().unwrap();
                            }
                        }
                        _ => {}
                    })
                    // let window = WindowBuilder::new(app, "TrayPane",
                    //                                 tauri::WindowUrl::External("http://localhost:1420/#/trayMenu".parse().unwrap())).build().unwrap();
                    //                                 // tauri::WindowUrl::App("index.html".into())).build().unwrap();
                    // window.set_position(p).expect("TODO: panic message");
                    // window.show().unwrap();
                }
                SystemTrayEvent::DoubleClick { .. } => {}
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            set_window_shadow(&main_window);
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // initialize your app here instead of sleeping :)
                // After it's done, close the splashscreen and display the main window
                main_window.show().unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}

// pub fn set_window_shadow<R: Runtime>(app: &tauri::App<R>) {
pub fn set_window_shadow(window: &Window) {
    // let window = app.get_window("main").unwrap();
    set_shadow(window, true).expect("Unsupported platform!");
}