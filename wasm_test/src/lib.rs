use std::{
    ffi::{c_char, CString},
    panic,
};
use wasm_api::Transform;

extern "C" {
    fn print_name_wasm();
    fn print_wasm(s: *const c_char);
    fn get_transform() -> *mut Transform;
    fn print_transform(s: *const Transform);
}

/// Allocate memory into the module's linear memory
/// and return the offset to the start of the block.
#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    // create a new mutable buffer with capacity `len`
    let mut buf = Vec::with_capacity(len);
    // take a mutable pointer to the buffer
    let ptr = buf.as_mut_ptr();
    // take ownership of the memory block and
    // ensure that its destructor is not
    // called when the object goes out of scope
    // at the end of the function
    std::mem::forget(buf);
    // return the pointer so the runtime
    // can write data at this offset
    ptr
}

#[no_mangle]
pub extern "C" fn main() {
    panic::set_hook(Box::new(|err| unsafe {
        let str = CString::new(err.to_string()).unwrap();
        print_wasm(str.as_ptr());
    }));
    unsafe { print_name_wasm() };
    unsafe {
        let str = CString::new("Hällü, Wörld!").unwrap();
        print_wasm(str.as_ptr());
        let transform = get_transform().read();
        print_transform(&transform as *const Transform);
        let pos = transform.pos;
        let rot = transform.rotation;
        let str = CString::new(format!("x:{}, y:{}, z:{}", pos.x, pos.y, pos.z)).unwrap();
        print_wasm(str.as_ptr());
        let str = CString::new(format!(
            "x:{}, y:{}, z:{}, w:{}",
            rot.x, rot.y, rot.z, rot.w
        ))
        .unwrap();
        print_wasm(str.as_ptr());
    }
}
