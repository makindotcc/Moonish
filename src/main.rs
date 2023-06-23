#![cfg_attr(
    all(target_os = "windows", not(feature = "console")),
    windows_subsystem = "windows"
)]

mod cfg;
mod tray;

use crate::cfg::WindowTitlePart;
use crate::tray::TrayMenu;
use auto_launch::AutoLaunch;
use std::env;
use std::process::Command;
use std::sync::RwLock;
use tray_icon::menu::MenuEvent;
use windows::core::{HSTRING, PCWSTR};
use windows::w;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE};
use windows::Win32::UI::Accessibility::{SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK};
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, EnumWindows, GetMessageW, GetWindowTextW, MessageBoxW, TranslateMessage,
    EVENT_OBJECT_CREATE, EVENT_OBJECT_NAMECHANGE, MB_ICONINFORMATION, MSG, OBJID_WINDOW,
    WINEVENT_OUTOFCONTEXT,
};

static WHITELISTED_WINDOWS: RwLock<Vec<WindowTitlePart>> = RwLock::new(Vec::new());

fn main() {
    const AUTOSTART_ARG: &str = "-autostart";

    let app_path = env::current_exe().expect("Could not get current exe path");
    let app_dir = app_path.parent().expect("Invalid program path");
    let app_path = app_path.to_str().expect("Empty current exe path");
    let auto_launch = AutoLaunch::new("Moonish", &app_path, &[AUTOSTART_ARG] as &[&str]);

    if env::args().skip(1).next() == Some(String::from(AUTOSTART_ARG)) {
        println!("Started from autostart.");
        env::set_current_dir(app_dir).expect("Could not set current working directory.");
    }

    *WHITELISTED_WINDOWS.write().unwrap() = cfg::load_or_create_whitelisted_windows();

    unsafe {
        let hook = SetWinEventHook(
            EVENT_OBJECT_CREATE,
            EVENT_OBJECT_NAMECHANGE,
            None,
            Some(on_object_event),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );
        update_all_windows();

        let tray_menu = TrayMenu::new();
        let menu_channel = MenuEvent::receiver();
        let mut msg: MSG = MSG::default();
        while GetMessageW(&mut msg, HWND(0), 0, 0).0 > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);

            while let Ok(event) = menu_channel.try_recv() {
                handle_tray_event(&tray_menu, &event, &auto_launch);
            }
        }
        UnhookWinEvent(hook);
    }
}

fn update_all_windows() {
    unsafe {
        EnumWindows(Some(on_enum_windows_set_dark_mode), LPARAM(0));
    }
}

fn handle_tray_event(tray_menu: &TrayMenu, event: &MenuEvent, auto_launch: &AutoLaunch) {
    if event.id == tray_menu.reload_config.id() {
        println!("Reloading config");
        *WHITELISTED_WINDOWS.write().unwrap() = cfg::load_or_create_whitelisted_windows();
        update_all_windows();
    } else if event.id == tray_menu.open_config.id() {
        println!("Opening config");
        if let Err(err) = Command::new("explorer").arg(cfg::FILE_PATH).spawn() {
            eprintln!("Could not open config: {err}");
        }
    } else if event.id == tray_menu.auto_start.id() {
        let auto_launch_enabled = match auto_launch.is_enabled() {
            Ok(enabled) => enabled,
            Err(err) => {
                eprintln!("Could not check if auto launch is enabled: {err}");
                return;
            }
        };
        let result = if auto_launch_enabled {
            auto_launch.disable()
        } else {
            auto_launch.enable()
        };
        match result {
            Ok(_) => {
                let updated_auto_launch_enabled = !auto_launch_enabled;
                let status = if updated_auto_launch_enabled {
                    "enabled"
                } else {
                    "disabled"
                };
                show_msg_box(&format!("{} auto launch", status));
            }
            Err(err) => {
                eprintln!("Could not toggle auto launch: {err}.");
            }
        }
    }
}

extern "system" fn on_enum_windows_set_dark_mode(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    update_window_theme(hwnd);
    BOOL(1)
}

unsafe extern "system" fn on_object_event(
    _hwineventhook: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    idobject: i32,
    _idchild: i32,
    _ideventthread: u32,
    _dwmseventtime: u32,
) {
    if idobject == OBJID_WINDOW.0 {
        match event {
            EVENT_OBJECT_CREATE => update_window_theme(hwnd),
            EVENT_OBJECT_NAMECHANGE => update_window_theme(hwnd),
            _ => {}
        }
    }
}

fn update_window_theme(hwnd: HWND) {
    let window_title = get_window_title(hwnd);
    let is_whitelisted = WHITELISTED_WINDOWS
        .read()
        .expect("Whitelisted windows poisoned")
        .iter()
        .any(|whitelisted| window_title.contains(whitelisted));
    if is_whitelisted {
        println!(
            "Enabling darkmode for window: [{:x}] {}",
            hwnd.0, window_title
        );
        unsafe {
            let value: u32 = 1;
            let _ = DwmSetWindowAttribute(
                hwnd,
                DWMWA_USE_IMMERSIVE_DARK_MODE,
                &value as *const u32 as *const _,
                std::mem::size_of::<u32>() as u32,
            );
        }
    }
}

fn get_window_title(hwnd: HWND) -> String {
    let mut buffer = [0; 256];
    let _ = unsafe { GetWindowTextW(hwnd, &mut buffer) };
    String::from_utf16_lossy(&buffer)
}

fn show_msg_box(text: &str) {
    unsafe {
        let test_msg = HSTRING::from(text);
        MessageBoxW(
            HWND(0),
            PCWSTR(test_msg.as_ptr()),
            w!("Moonish"),
            MB_ICONINFORMATION,
        );
    }
}
