//! Windows registry read helpers for Joplin Cloud.

pub const APP_KEY: &str = "Software\\Joplin\\Joplin Cloud";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Joplin Cloud";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Joplin Cloud") },
        RegValue { name: "Version",     value: String::from("v1.0.3") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Joplin Cloud") },
        RegValue { name: "Publisher",       value: String::from("Joplin") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Joplin Cloud\\uninstall.exe\"") },
    ]
}
