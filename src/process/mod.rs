use winapi::shared::minwindef::{DWORD};
use winapi::um::tlhelp32::*;
use winapi::um::handleapi::*;
use winapi::um::winbase::{lstrcmpiA};
use std::ffi::CString;
use std::mem;

//GETS THE PROCESS ID OF A PROCESS BY EXECUTABLE NAME
pub unsafe fn get_process_id(process_name: &str) -> Result<DWORD, &str> {

    let h_process_snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    let process_name = CString::new(process_name).expect("Couldn't convert String to CString!");

    if h_process_snap == INVALID_HANDLE_VALUE {
        return Err("INVALID HANDLE VALUE!");
    }

    let mut pe32: PROCESSENTRY32 = std::mem::zeroed();
    pe32.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

    if Process32First(h_process_snap, &mut pe32) != 0 {
        if lstrcmpiA(&pe32.szExeFile as *const i8, process_name.as_ptr()) == 0 {
            let id = pe32.th32ProcessID;
            CloseHandle(h_process_snap);
            return Ok(id);
        }
    }
    
    while Process32Next(h_process_snap, &mut pe32) != 0 {
        if lstrcmpiA(&pe32.szExeFile as *const i8, process_name.as_ptr()) == 0 {
            let id = pe32.th32ProcessID;
            CloseHandle(h_process_snap);
            return Ok(id);
        }
    }
    Err("Couldn't get Process ID")
}

//GETS THE MODULE BASE ADDRESS OF A MODULE
pub unsafe fn get_module_base(process_id: DWORD, name: &str) -> Result<usize, &str> {

    let module_name = CString::new(name).expect("Couldn't create CString!");
    let handle_snap = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id);

    if handle_snap == INVALID_HANDLE_VALUE {
        CloseHandle(handle_snap);
        return Err("Couldn't get Snapshot!");
    }

    let mut module_entry: MODULEENTRY32 = mem::zeroed();
    module_entry.dwSize = mem::size_of::<MODULEENTRY32>() as u32;

    if Module32First(handle_snap, &mut module_entry) != 0 {
        if lstrcmpiA(&module_entry.szModule[0], module_name.as_ptr()) == 0 {
            CloseHandle(handle_snap);
            return Ok(module_entry.modBaseAddr as usize);
        }
    }
    while Module32Next(handle_snap, &mut module_entry) != 0 {
        if lstrcmpiA(&module_entry.szModule[0], module_name.as_ptr()) == 0 {
            CloseHandle(handle_snap);
            return Ok(module_entry.modBaseAddr as usize);
        }
    }

    Err("Couldn't get Modulebase!")
}
