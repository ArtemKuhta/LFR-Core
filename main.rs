use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK, MB_ICONINFORMATION};
use std::iter::once;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

fn main() {
    let title = to_wstring("LFR: Longhorn First Rebirth");
    let message = to_wstring("System pre v0.6.0 successfully initialised!\n\nStable Rust core is active. Ready for global localization. ");

    unsafe {
        MessageBoxW(
            0,
            message.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        );
    }
    
    println!("LFR v0.5.9: Статус системи - СТАБІЛЬНО.");
}

fn to_wstring(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}