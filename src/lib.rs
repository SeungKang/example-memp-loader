#![allow(non_snake_case)]

use std::{env, ptr, io::Error};

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryW(lp_lib_file_name: *const u16) -> *mut u8;
}

#[link(name = "user32")]
extern "system" {
    fn MessageBoxW(hwnd: isize, lptext: *const u16, lpcaption: *const u16, utype: u32) -> i32;
}

#[no_mangle]
#[allow(unused_variables)]
extern "system" fn DllMain(
    dll_module: isize,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    // https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain
    const DLL_PROCESS_ATTACH: u32 = 1u32;
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        _ => ()
    }

    // Unloads the dll
    false
}

fn attach() {
    let exe_path = match env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            error_message_box(format!("failed to get current exe path: {e}"));
            return;
        }
    };

    if !exe_path.ends_with("MirrorsEdge.exe") {
        // error_message_box("skipping loading since process is not Mirrors Edge".into());
        return;
    }

    let dll_file = "C:\\Program Files\\Mirror's Edge Multiplayer\\bin\\mmultiplayer.dll";
    let mut dll_file_u16 = dll_file.encode_utf16().collect::<Vec<_>>();
    dll_file_u16.push(0);

    let h_dll = unsafe {LoadLibraryW(dll_file_u16.as_ptr())};

    // Check if the DLL was loaded successfully
    if h_dll == ptr::null_mut() {
        let os_error = Error::last_os_error();
        error_message_box(format!("failed to load DLL ({dll_file}) - last os error: {os_error}"));

        return;
    }
}

// https://github.com/microsoft/windows-rs/issues/973#issuecomment-1363481060
fn error_message_box(msg: String) {
    let msg = format!("🤕 {msg}\0");
    let msg = msg.encode_utf16().collect::<Vec<_>>();
    let title = "Mirrors Edge Multiplayer Error\0";
    let title = title.encode_utf16().collect::<Vec<_>>();
    unsafe {
        MessageBoxW(0,
                    msg.as_ptr(),
                    title.as_ptr(),
                    0);
    };
}
