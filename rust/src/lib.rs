use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::ptr;

#[cfg(windows)]
use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, HMODULE, LPVOID, TRUE},
    um::libloaderapi::GetProcAddress,
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
    println!("Hello World!");
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

unsafe fn get_proc_address(name: *const c_char) -> *mut c_void {
    #[cfg(windows)]
    return GetProcAddress(MODULE, name) as _;
    #[cfg(unix)]
    return dlsym(RTLD_DEFAULT, name);
}

#[cfg(windows)]
#[no_mangle]
pub static mut MODULE: HMODULE = ptr::null_mut();

#[cfg(windows)]
#[no_mangle]
pub extern "system" fn DllMain(
    hinst_dll: HINSTANCE,
    _fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> BOOL {
    unsafe { MODULE = hinst_dll };
    TRUE
}
