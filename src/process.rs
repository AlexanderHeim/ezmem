use winapi::um::tlhelp32::*;
use winapi::um::handleapi::*;
use std::ffi::{CString, CStr};
use std::mem;
use winapi::um::errhandlingapi::GetLastError;

/// Returns an Option which contains a Process ID if present.
pub fn get_process_id(process_name: &str) -> Option<u32> {
    unsafe {

        let h_process_snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        let process_name = CString::new(process_name).expect("Couldn't convert process_name to CString!");

        if h_process_snap == INVALID_HANDLE_VALUE {
            panic!("INVALID HANDLE VALUE! Last OS Error: {}", GetLastError());
        }

        let mut pe32: PROCESSENTRY32 = std::mem::zeroed();
        pe32.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

        if Process32First(h_process_snap, &mut pe32) != 0 {
            let current_exe_name = CStr::from_ptr(&pe32.szExeFile as *const i8);
            if current_exe_name == process_name.as_c_str() {
                let id = pe32.th32ProcessID;
                CloseHandle(h_process_snap);
                return Some(id);
            }
        }

        while Process32Next(h_process_snap, &mut pe32) != 0 {
            let current_exe_name = CStr::from_ptr(&pe32.szExeFile as *const i8);
            if current_exe_name == process_name.as_c_str() {
                let id = pe32.th32ProcessID;
                CloseHandle(h_process_snap);
                return Some(id);
            }
        }
        CloseHandle(h_process_snap);
    }
    None
}

/// Returns an Option containing  the base address of a module (as usize).
pub fn get_module_base(process_id: u32, module_name: &str) -> Option<usize> {
    unsafe {

        let m_name = CString::new(module_name).expect("Couldn't convert module_name to CString!");
        let h_module_snap = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id);

        if h_module_snap == INVALID_HANDLE_VALUE {
            CloseHandle(h_module_snap);
            panic!("INVALID HANDLE VALUE! Last OS Error: {}", GetLastError());
        }

        let mut module_entry: MODULEENTRY32 = mem::zeroed();
        module_entry.dwSize = mem::size_of::<MODULEENTRY32>() as u32;

        if Module32First(h_module_snap, &mut module_entry) != 0 {
            let current_module_name = CStr::from_ptr(&module_entry.szModule as *const i8);
            if current_module_name == m_name.as_c_str() {
                CloseHandle(h_module_snap);
                return Some(module_entry.modBaseAddr as usize);
            }
        }
        while Module32Next(h_module_snap, &mut module_entry) != 0 {
            let current_module_name = CStr::from_ptr(&module_entry.szModule as *const i8);
            if current_module_name == m_name.as_c_str() {
                CloseHandle(h_module_snap);
                return Some(module_entry.modBaseAddr as usize);
            }
        }
    
    }
    None
}
