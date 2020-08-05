use winapi::shared::minwindef::{LPCVOID, LPVOID};
use winapi::um::winnt::{HANDLE};
use winapi::um::memoryapi::{ReadProcessMemory};
use winapi::um::wow64apiset::IsWow64Process;

//Reads bytes in memory to a Vec<u8>
pub unsafe fn read_bytes(handle: HANDLE, address: usize, amount: usize) -> Vec<u8> {
    let bytes: Vec<u8> = vec![0; amount];
    ReadProcessMemory(handle, address as LPCVOID, bytes.as_ptr() as LPVOID, amount, std::ptr::null_mut());
    bytes
}

//Read Memory Functions for some primitive types.
pub unsafe fn read_i64(handle: HANDLE, address: usize) -> i64 {
    read_primitive(handle, address)
}

pub unsafe fn read_i32(handle: HANDLE, address: usize) -> i32 {
    read_primitive(handle, address)
}

pub unsafe fn read_i16(handle: HANDLE, address: usize) -> i16 {
    read_primitive(handle, address)
}

pub unsafe fn read_i8(handle: HANDLE, address: usize) -> i8 {
    read_primitive(handle, address)
}

pub unsafe fn read_u64(handle: HANDLE, address: usize) -> u64 {
    read_primitive(handle, address)
}

pub unsafe fn read_u32(handle: HANDLE, address: usize) -> u32 {
    read_primitive(handle, address)
}

pub unsafe fn read_u16(handle: HANDLE, address: usize) -> u16 {
    read_primitive(handle, address)
}

pub unsafe fn read_u8(handle: HANDLE, address: usize) -> u8 {
    read_primitive(handle, address)
}

pub unsafe fn read_f64(handle: HANDLE, address: usize) -> f64 {
    read_primitive(handle, address)
}

pub unsafe fn read_f32(handle: HANDLE, address: usize) -> f32 {
    read_primitive(handle, address)
}

unsafe fn read_primitive<T: Default>(handle: HANDLE, address: usize) -> T {
    let mut read_result = T::default();
    ReadProcessMemory(handle, address as LPCVOID, &mut read_result as *mut _ as LPVOID, std::mem::size_of::<T>(), std::ptr::null_mut());
    read_result
}

//Resolves multilevel pointer -> returns last address (not value).
pub unsafe fn resolve_multi_level_pointer(handle: HANDLE, base_ptr: usize, offsets: Vec<usize>) -> usize {

    let mut pointer_size = 8;
    let mut is_32_bit: i32 = 0;

    //If 32 bit set pointersize to 4 bytes.
    IsWow64Process(handle, &mut is_32_bit as *mut i32);

    if is_32_bit > 0{
        pointer_size = 4;
    }

    let mut address = base_ptr;
    let mut buffer: usize;

    for i in 0..offsets.len() {
        buffer = 0;
        ReadProcessMemory(handle, address as LPCVOID, &mut buffer as *mut _ as LPVOID, pointer_size, std::ptr::null_mut());
        address = buffer + offsets[i];
    }

    address
}