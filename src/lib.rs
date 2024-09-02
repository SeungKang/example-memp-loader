#![allow(non_snake_case)]

use std::ffi::CString;
use std::ptr;

use windows::{ Win32::Foundation::*, Win32::System::SystemServices::*, };

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
    let dll_file = "C:\\Program Files\\Mirror's Edge Multiplayer\\bin\\mmultiplayer.dll";
    let dll_file_cstring: CString = CString::new(dll_file).expect("CString::new failed");
    let h_dll: *mut u8 = unsafe {LoadLibraryA(dll_file_cstring.as_ptr())};

    // Check if the DLL was loaded successfully
    if h_dll == ptr::null_mut() {
        println!("[!] Failed to load DLL: {}", dll_file);
        return;
    }

    // Successful load
    println!("DLL {} Loaded successfully", dll_file);
}
