#![allow(non_snake_case)]

use core::ffi::c_void;

use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

// pub type HMODULE = *mut core::ffi::c_void
// pub type HANDLE = *mut core::ffi::c_void
// pub type PCWSTR = *const u16 (and LPWSTR)

#[link(name = "kernel32")]
extern "system" {
    /// Returns a HANDLE.
    fn GetCurrentProcess() -> *mut c_void;

    /// * HANDLE hProcess:
    ///   A handle to the process
    /// * HMODULE lpHmodule:
    ///   An array that receives the list of module handles.
    /// * DWORD cb:
    ///   The size of the lphModule array, in bytes.
    /// * LPDWORD lpcbNeeded:
    ///   The number of bytes required to store all module handles
    ///   in the lphModule array.
    /// * DWORD dwFilterFlag:
    ///   The filter criteria.
    fn K32EnumProcessModulesEx(
        hProcess: *mut c_void,
        lpHmodule: *mut c_void,
        cb: u32,
        lpcbNeeded: *mut u32,
        dwFilterFlag: u32,
    ) -> i32;

    /// * HANDLE  hProcess:
    ///   A handle to the process that contains the module.
    /// * HMODULE hModule:
    ///   A handle to the module.
    /// * LPWSTR  lpFilename:
    ///   A pointer to a buffer that receives the fully qualified path
    ///   to the module.
    /// * DWORD   nSize:
    ///   The size of the lpFilename buffer, in characters.
    fn K32GetModuleFileNameExW(
        hProcess: *mut c_void,
        hModule: *mut c_void,
        lpFilename: *const u16,
        nSize: u32,
    ) -> u32;

    fn LoadLibraryW(lp_lib_file_name: *const u16) -> *mut u8;
}

#[link(name = "user32")]
extern "system" {
    fn MessageBoxW(hwnd: isize, lptext: *const u16, lpcaption: *const u16, utype: u32) -> i32;
}

#[no_mangle]
#[allow(unused_variables)]
extern "system" fn DllMain(dll_module: isize, call_reason: u32, _: *mut ()) -> bool {
    // https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain
    const DLL_PROCESS_ATTACH: u32 = 1u32;
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        _ => (),
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

    let memp_dll = match find_memp_dll() {
        Ok(path) => path,
        Err(err) => {
            error_message_box(format!("failed to find multiplayer dll - {err}"));
            return;
        }
    };

    let memp_dll_str = memp_dll.display().to_string();
    let mut memp_dll_utf16 = memp_dll_str.encode_utf16().collect::<Vec<_>>();
    memp_dll_utf16.push(0);

    let h_dll = unsafe { LoadLibraryW(memp_dll_utf16.as_ptr()) };
    if h_dll.is_null() {
        error_message_box(format!(
            "failed to load DLL ({memp_dll_str}) - last os error: {err}",
            err = std::io::Error::last_os_error()
        ));
        return;
    }
}

fn find_memp_dll() -> Result<PathBuf, Box<dyn Error>> {
    const MEMP_DLL_NAME: &str = "mmultiplayer.dll";

    let mut def_path = Path::new("C:\\Program Files\\Mirror's Edge Multiplayer\\bin").to_path_buf();
    def_path.push(MEMP_DLL_NAME);

    if def_path.exists() {
        return Ok(def_path);
    }

    let current_process = unsafe { GetCurrentProcess() };
    if current_process.is_null() {
        Err(format!(
            "GetCurrentProcess failed - {err}",
            err = std::io::Error::last_os_error()
        ))?
    }

    // Passing an array (Vec) via FFI by Michael-F-Bryan:
    // https://users.rust-lang.org/t/ffi-how-to-pass-a-array-with-structs-to-a-c-func-that-fills-the-array-out-pointer-and-then-how-to-access-the-items-after-in-my-rust-code/83798/2
    let mut modules: Vec<*mut c_void> = Vec::with_capacity(1024);
    let modules_uninit = modules.spare_capacity_mut();

    let mut num_modules_returned: u32 = 0;

    let enum_modules_res = unsafe {
        K32EnumProcessModulesEx(
            current_process,
            modules_uninit.as_mut_ptr().cast(),
            modules_uninit.len() as u32,
            &mut num_modules_returned,
            0x03,
        )
    };
    if enum_modules_res == 0 {
        Err(format!(
            "K32EnumProcessModulesEx failed - {err}",
            err = std::io::Error::last_os_error()
        ))?
    }

    unsafe { modules.set_len(num_modules_returned as usize) };

    for module_handle in modules {
        const MAX_PATH: usize = 32767;
        let mut filename_raw: Vec<u16> = Vec::with_capacity(MAX_PATH);
        let filename_raw_uninit = filename_raw.spare_capacity_mut();

        let filename_res = unsafe {
            K32GetModuleFileNameExW(
                current_process,
                module_handle,
                filename_raw_uninit.as_mut_ptr().cast(),
                filename_raw_uninit.len() as u32,
            )
        };
        if filename_res == 0 {
            Err(format!(
                "K32GetModuleFileNameExW failed - {err}",
                err = std::io::Error::last_os_error()
            ))?
        }

        unsafe { filename_raw.set_len(filename_res as usize) };

        match String::from_utf16(&filename_raw) {
            Ok(s) => {
                let mut file_path = PathBuf::from(s);

                if !file_path.ends_with("AgPerfMon.dll") {
                    continue;
                }

                file_path.pop();

                file_path.push(MEMP_DLL_NAME);

                if file_path.exists() {
                    return Ok(file_path);
                }

                Err(format!(
                    "multiplayer dll ({MEMP_DLL_NAME}) not present in loader dll directory: {p}",
                    p = file_path.display()
                ))?
            }
            Err(err) => Err(err)?,
        }
    }

    Err(format!(
        "failed to find loader dll in {num_modules_returned} process modules",
    ))?
}

// https://github.com/microsoft/windows-rs/issues/973#issuecomment-1363481060
fn error_message_box(msg: String) {
    let msg = format!("🤕 {msg}\0");
    let msg = msg.encode_utf16().collect::<Vec<_>>();
    let title = "Mirrors Edge Multiplayer Error\0";
    let title = title.encode_utf16().collect::<Vec<_>>();
    unsafe {
        MessageBoxW(0, msg.as_ptr(), title.as_ptr(), 0);
    };
}
