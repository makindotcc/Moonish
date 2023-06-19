use tray_icon::menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem};
use tray_icon::{TrayIcon, TrayIconBuilder};

pub struct TrayMenu {
    _tray_icon: TrayIcon,
    pub reload_config: MenuItem,
    pub open_config: MenuItem,
    pub auto_start: MenuItem,
}

impl TrayMenu {
    pub fn new() -> Self {
        let tray_menu = Menu::new();
        let reload_config = MenuItem::new("Reload config", true, None);
        let open_config = MenuItem::new("Open config", true, None);
        let auto_start = MenuItem::new("Toggle auto start", true, None);
        tray_menu.append_items(&[
            &PredefinedMenuItem::about(
                None,
                Some(AboutMetadata {
                    name: Some("Moon".to_string()),
                    comments: Some("Make all windows titles dark...".to_string()),
                    ..Default::default()
                }),
            ),
            &reload_config,
            &open_config,
            &auto_start,
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::quit(None),
        ]);
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("Moon")
            .with_icon(tray_icon::icon::Icon::from_resource(1, Some((96, 96))).unwrap())
            .with_menu(Box::new(tray_menu))
            .build()
            .expect("Could not create tray icon");
        Self {
            _tray_icon: tray_icon,
            reload_config,
            open_config,
            auto_start,
        }
    }
}
