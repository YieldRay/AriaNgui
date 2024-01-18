// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::Debug;
use std::sync::mpsc::sync_channel;
use std::thread;

use tauri::api::process::Command;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

use tauri::{utils::config::AppUrl, WindowUrl};

#[cfg(target_os = "windows")]
use winapi::um::winuser::{MessageBoxA, MB_ICONINFORMATION, MB_OK};

fn main() {
    let (mut _rx, child) = unwrap_or_exit_with_message_box(
        // https://github.com/tauri-apps/tauri/discussions/3273
        Command::new("aria2c")
            .args([
                "--enable-rpc=true",
                "--rpc-allow-origin-all=true",
                "--rpc-listen-all=true",
                "--rpc-listen-all=true",
                "--rpc-listen-port=6800",
            ])
            .spawn(),
        "Please put aria2c.exe to PATH!",
    );
    let (tx, rx) = sync_channel::<i32>(1);
    thread::spawn(
        // this thread waiting for close signal
        move || loop {
            let s = rx.recv();
            if unwrap_or_exit_with_message_box(s, "Failed to call rx.recv()") != 1 {
                unwrap_or_exit_with_message_box(child.kill(), "Failed to call child.kill()");
                // all cleanup done, exit with 0
                std::process::exit(0)
            }
        },
    );

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    let port = portpicker::pick_unused_port().expect("failed to find unused port");
    let mut context = tauri::generate_context!();
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());

    unwrap_or_exit_with_message_box(
        tauri::Builder::default()
            .plugin(tauri_plugin_localhost::Builder::new(port).build())
            .system_tray(SystemTray::new().with_menu(tray_menu))
            .on_system_tray_event(move |app, event| {
                if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                    match id.as_str() {
                        "quit" => {
                            // send signal to exit the app
                            tx.send(-1).unwrap();
                        }
                        "hide" => {
                            if let Some(window) = app.get_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    window.hide().unwrap();
                                } else {
                                    window.show().unwrap();
                                }
                            }
                        }
                        _ => {}
                    }
                } else if let SystemTrayEvent::LeftClick { .. } = event {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.unminimize().unwrap();
                }
            })
            .on_window_event(|event| match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            })
            .run(context),
        "error while running tauri application",
    )
}

/// note that: the `!` type is experimental
/// so we can hardly customize exit behavior
fn unwrap_or_exit<T, E, OnErr>(result: Result<T, E>, on_err: OnErr) -> T
where
    OnErr: Fn(E) -> (),
{
    match result {
        Ok(ok) => return ok,
        Err(err) => {
            on_err(err);
            std::process::exit(-1);
        }
    }
}

fn unwrap_or_exit_with_message_box<T, E: Debug>(result: Result<T, E>, message: &str) -> T {
    match result {
        Ok(ok) => return ok,
        Err(err) => {
            message_box_a(message, &format!("{:#?}", err));
            std::process::exit(-1);
        }
    }
}

fn message_box_a(message: &str, caption: &str) {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            MessageBoxA(
                std::ptr::null_mut(),
                caption.as_ptr() as *const i8,
                message.as_ptr() as *const i8,
                MB_OK | MB_ICONINFORMATION,
            );
        }
    }
}
