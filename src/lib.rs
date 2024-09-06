#![allow(non_snake_case)]

use std::{env, ptr, ffi::CString, io::Error};

use windows::{
    Win32::Foundation::HINSTANCE,
    Win32::System::SystemServices::DLL_PROCESS_ATTACH,
    core::{PCWSTR, w},
    Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxW}
};

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryA(lp_lib_file_name: *const i8) -> *mut u8;
}

#[no_mangle]
#[allow(unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    // https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain
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
    let dll_file_cstring = CString::new(dll_file).expect("CString::new failed");
    let h_dll = unsafe {LoadLibraryA(dll_file_cstring.as_ptr())};

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
    unsafe {
        MessageBoxW(None,
                    PCWSTR::from_raw(msg.as_ptr()),
                    w!("Mirrors Edge Multiplayer Error"),
                    MB_OK);
    };
}
