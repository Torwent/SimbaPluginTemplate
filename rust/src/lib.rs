use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::ptr;

use windows::core::PCSTR;
#[cfg(windows)]
use windows::{
    core::BOOL,
    Win32::{
        Foundation::{HINSTANCE, HMODULE, TRUE},
        System::LibraryLoader::GetProcAddress,
    },
};

#[cfg(unix)]
use libc::{dlsym, RTLD_DEFAULT};

// Pascal types as tuples (name, definition)
const PASCAL_TYPES: &[(&str, &str)] = &[("PHelloChar", "^Char;"), ("PTestInt", "^Int32;")];

// Pascal exports as (name, declaration)
//name as to match the dll function name exactly
const PASCAL_EXPORTS: &[(&str, &str)] = &[
    ("TestHelloWorld", "procedure TestHelloWorld();"),
    ("TestSum", "function TestSum(a, b: Integer): Integer;"),
];

// dll functions
#[no_mangle]
pub extern "C" fn TestHelloWorld() {
    println!("Hello World!\r\n");
}

#[no_mangle]
pub extern "C" fn TestSum(a: c_long, b: c_long) -> c_long {
    a + b
}

// Simba plugin helpers, don't touch
#[no_mangle]
pub extern "C" fn GetFunctionCount() -> c_int {
    PASCAL_EXPORTS.len() as c_int
}

#[no_mangle]
pub extern "C" fn GetTypeCount() -> c_int {
    PASCAL_TYPES.len() as c_int
}

#[no_mangle]
pub extern "C" fn GetFunctionInfo(
    index: c_int,
    address: *mut *mut c_void,
    definition: *mut *mut c_char,
) -> c_int {
    if index >= GetFunctionCount() {
        return -1;
    }

    let (name, def) = PASCAL_EXPORTS[index as usize];
    let name = CString::new(name).unwrap();
    unsafe {
        *address = get_proc_address(name.as_ptr());
        let def = CString::new(def).unwrap();
        ptr::copy(def.as_ptr(), *definition, def.as_bytes_with_nul().len());
    }
    index
}

#[no_mangle]
pub extern "C" fn GetTypeInfo(
    index: c_int,
    typ: *mut *mut c_char,
    definition: *mut *mut c_char,
) -> c_int {
    if index >= GetTypeCount() {
        return -1;
    }

    let (name, def) = PASCAL_TYPES[index as usize];
    let name = CString::new(name).unwrap();
    let def = CString::new(def).unwrap();

    unsafe {
        ptr::copy(name.as_ptr(), *typ, name.as_bytes_with_nul().len());
        ptr::copy(def.as_ptr(), *definition, def.as_bytes_with_nul().len());
    }
    index
}

#[cfg(windows)]
unsafe fn get_proc_address(name: *const c_char) -> *mut c_void {
    let name_str = PCSTR::from_raw(name as *const u8);
    let func_ptr = GetProcAddress(MODULE, name_str);
    return std::mem::transmute(func_ptr);
}
#[cfg(unix)]
unsafe fn get_proc_address(name: *const c_char) -> *mut c_void {
    return dlsym(RTLD_DEFAULT, name);
}

#[cfg(windows)]
#[no_mangle]
pub static mut MODULE: HMODULE = HMODULE(ptr::null_mut());

#[cfg(windows)]
#[no_mangle]
pub extern "system" fn DllMain(
    hinst_dll: HINSTANCE,
    _fdw_reason: u32, // DWORD is u32 in windows crate
    _lpv_reserved: *mut std::ffi::c_void,
) -> BOOL {
    unsafe { MODULE = HMODULE(hinst_dll.0) };
    TRUE
}
