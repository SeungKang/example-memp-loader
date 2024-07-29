use windows::{ Win32::Foundation::*, Win32::System::SystemServices::*, };
// use windows::{ core::*, Win32::UI::WindowsAndMessaging::MessageBoxA, };
// use windows::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION;

extern crate user32;
extern crate winapi;
#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryA(lp_lib_file_name: *const i8) -> *mut u8;
}

use std::ffi::CString;
use user32::MessageBoxA;
use winapi::winuser::{MB_OK, MB_ICONINFORMATION};
use std::ptr;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    // https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain
    match call_reason {
        // case DLL_PROCESS_ATTACH:
        //   attach()
        DLL_PROCESS_ATTACH => attach(),
        _ => ()
    }

    // unloads the dll
    false
}

fn attach() {
    // unsafe {
    //     // Create a message box
    //     MessageBoxA(HWND(0),
    //                 s!("ZOMG!"),
    //                 s!("hello.dll"),
    //                 MB_OK|MB_ICONINFORMATION
    //     );
    // };

    // let lp_text = CString::new("buh").unwrap();
    // let lp_caption = CString::new("MessageBox Example").unwrap();
    //
    // unsafe {
    //     MessageBoxA(
    //         std::ptr::null_mut(),
    //         lp_text.as_ptr(),
    //         lp_caption.as_ptr(),
    //         MB_OK | MB_ICONINFORMATION
    //     );
    // }
    let dll_file = "C:\\Program Files\\Mirror's Edge Multiplayer\\bin\\mmultiplayer.dll";
    let dll_file_cstring: CString = CString::new(dll_file.clone()).expect("CString::new failed");
    let h_dll: *mut u8 = unsafe {LoadLibraryA(dll_file_cstring.as_ptr())};

    // Check if the DLL was loaded successfully
    if h_dll == ptr::null_mut() {
        println!("[!] Failed to load DLL: {}", dll_file);
        return;
    }
    // Successful load
    println!("DLL {} Loaded successfully", dll_file);
}

// fn detach() {
//     unsafe {
//         // Create a message box
//         MessageBoxA(HWND(0),
//                     s!("GOODBYE!"),
//                     s!("hello.dll"),
//                     Default::default()
//         );
//     };
// }
