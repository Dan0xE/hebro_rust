extern crate winapi;

use std::ffi::CString;
use std::ptr::null_mut;
use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};

pub fn get_pid(process_name: String) -> usize {
    let h_snapshot: *mut winapi::ctypes::c_void;
    let mut pe: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    let mut pid: usize = 0;
    let process_cstring: CString = CString::new(process_name).unwrap();

    h_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };

    if h_snapshot == null_mut() {
        return 0;
    }

    pe.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

    let mut h_result: i32 = unsafe { Process32First(h_snapshot, &mut pe) };

    while h_result != FALSE {
        let szexe_u8: Vec<u8> = pe
            .szExeFile
            .iter()
            .take_while(|&i| *i != 0)
            .map(|&c| c as u8)
            .collect();
        let current_proc_name: &str = std::str::from_utf8(&szexe_u8)
            .unwrap_or("")
            .trim_end_matches(char::from(0));

        if current_proc_name == process_cstring.to_str().unwrap() {
            pid = pe.th32ProcessID as usize;
            break;
        }
        h_result = unsafe { Process32Next(h_snapshot, &mut pe) };
    }

    unsafe { CloseHandle(h_snapshot) };

    pid
}
