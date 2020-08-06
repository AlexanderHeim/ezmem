use winapi::um::memoryapi::{WriteProcessMemory, VirtualProtectEx};
use winapi::shared::minwindef::{LPCVOID, LPVOID};
use winapi::um::winnt::{HANDLE, PAGE_EXECUTE_READWRITE};



///Writes bytes in Vec<u8> to memory location.
pub unsafe fn write_bytes(handle: HANDLE, address: usize, bytes: Vec<u8>) {
    WriteProcessMemory(handle, address as LPVOID, bytes.as_ptr() as LPVOID, bytes.len(), std::ptr::null_mut());
}

//writemem functions for some primitive types.
pub unsafe fn write_i64(handle: HANDLE, address: usize, to_write: i64) {
    write_primitive(handle, address, to_write);
}

pub unsafe fn write_i32(handle: HANDLE, address: usize, to_write: i32) {
    write_primitive(handle, address, to_write);
}

pub unsafe fn write_i16(handle: HANDLE, address: usize, to_write: i16) {
    write_primitive(handle, address, to_write);
}

pub unsafe fn write_i8(handle: HANDLE, address: usize, to_write: i8) {
    write_primitive(handle, address, to_write);
}

pub unsafe fn write_u64(handle: HANDLE, address: usize, to_write: u64) {
    write_primitive(handle, address, to_write);
}

pub unsafe fn write_u32(handle: HANDLE, address: usize, to_write: u32) {
    write_primitive(handle, address, to_write);
}

pub unsafe fn write_u16(handle: HANDLE, address: usize, to_write: u16) {
    write_primitive(handle, address, to_write);
}

pub unsafe fn write_u8(handle: HANDLE, address: usize, to_write: u8) {
    write_primitive(handle, address, to_write);
}

pub unsafe fn write_f32(handle: HANDLE, address: usize, to_write: f32) {
    write_primitive(handle, address, to_write);
}

pub unsafe fn write_f64(handle: HANDLE, address: usize, to_write: f64) {
    write_primitive(handle, address, to_write);
}

unsafe fn write_primitive<T>(handle: HANDLE, address: usize, to_write: T) {
    let mut buffer = to_write;
    WriteProcessMemory(handle, address as LPVOID, &mut buffer as *mut _ as LPCVOID, std::mem::size_of::<T>(), std::ptr::null_mut());
}

///Changes memory protection to writeable and overrides memory. Then sets memory protection to original state.
///Used to inject/change code in code loaded into memory.
pub unsafe fn patch_ex(handle: HANDLE, address: usize, bytes: Vec<u8>) {

    let mut last_protection: u32 = 0;
    let byte_amount = bytes.len();

    VirtualProtectEx(handle, address as LPVOID, byte_amount, PAGE_EXECUTE_READWRITE, &mut last_protection as *mut _);
    write_bytes(handle, address, bytes);
    VirtualProtectEx(handle, address as LPVOID, byte_amount, last_protection, &mut last_protection as *mut _);
}
