#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate winapi;

use std::mem;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{VirtualAllocEx, VirtualQueryEx};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{HANDLE, MEMORY_BASIC_INFORMATION, PROCESS_ALL_ACCESS};
use winapi::um::winnt::{MEM_FREE, MEM_RESERVE, PAGE_NOACCESS};

mod hebro_helper;

pub fn align_top<T>(any_pointer: *const (), alignment: usize) -> *const T {
    let mut as_uintptr_t: usize = any_pointer as usize;
    let mask: usize = alignment - 1;
    as_uintptr_t += mask;
    as_uintptr_t &= !mask;

    as_uintptr_t as *const T
}

pub fn offset<T, U>(any_pointer: *const (), how_many_bytes: U) -> *const T
where
    U: Into<usize>,
{
    let as_char: *const u8 = any_pointer as *const u8;
    let as_char: *const u8 = unsafe { as_char.offset(how_many_bytes.into() as isize) };
    as_char as *const T
}

fn main() {
    let pid: usize = hebro_helper::get_pid("explorer.exe");

    if pid != 0 {
        let proc: HANDLE = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid as u32) };

        let alignment: usize = 64 * 1024;

        let address_start: *const () = 0x00007FFF7FF00000 as *const ();
        let address_end: *const () = 0x000080007FF00000 as *const ();

        let mut address: *const () = address_start;

        while address < address_end {
            address = align_top(address, alignment);

            let mut memory_info: MEMORY_BASIC_INFORMATION =
                unsafe { mem::MaybeUninit::zeroed().assume_init() };

            let bytes_returned: usize = unsafe {
                VirtualQueryEx(
                    proc,
                    address as *const c_void,
                    &mut memory_info,
                    (mem::size_of::<MEMORY_BASIC_INFORMATION>() as u32)
                        .try_into()
                        .unwrap(),
                )
            };

            if bytes_returned > 0 && memory_info.State == MEM_FREE {
                let bytes_left: usize =
                    (address_end as usize).wrapping_sub(memory_info.BaseAddress as usize);
                let size: usize = bytes_left.min(memory_info.RegionSize);

                let base_address: *mut c_void = unsafe {
                    VirtualAllocEx(
                        proc,
                        memory_info.BaseAddress,
                        size,
                        MEM_RESERVE,
                        PAGE_NOACCESS,
                    )
                };

                if !base_address.is_null() {}
            }

            address = offset(memory_info.BaseAddress as *const (), memory_info.RegionSize);
        }

        unsafe {
            CloseHandle(proc);
        }
    }
}
