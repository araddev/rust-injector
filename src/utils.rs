// utils for main

use winapi::{shared::minwindef::{DWORD, HMODULE}, um::{winnt::{PROCESS_QUERY_INFORMATION, LPSTR, PROCESS_VM_READ}, processthreadsapi::OpenProcess, psapi::{EnumProcessModulesEx, EnumProcesses, GetModuleBaseNameA}, handleapi::CloseHandle}};

pub fn get_process_id(process_name: &str) -> Option<DWORD> {
    let mut process_ids =  {[0; 1024]};
    let mut bytes_returned = 0;
    unsafe {
        if EnumProcesses(
            process_ids.as_mut_ptr(),
            std::mem::size_of_val(&process_ids) as DWORD,
            &mut bytes_returned,
        ) == 0
        {
            return None;
        }
    }
    let num_processes =  {bytes_returned / std::mem::size_of::<DWORD>() as u32};
    for i in 0..num_processes {
        let pid =  {process_ids[i as usize]};
        if pid != 0 {
            let handle =  {unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid) }};
            if !handle.is_null() {
                let mut module_handle =  {std::ptr::null_mut()};
                let mut module_name_buf: Vec<u8> =  {Vec::with_capacity(512)};
                let mut module_name_len = 0;
                unsafe {
                    if EnumProcessModulesEx(
                        handle,
                        &mut module_handle,
                        std::mem::size_of::<HMODULE>() as u32,
                        &mut bytes_returned,
                        0x03,
                    ) != 0
                    {
                        let bytes_copied = GetModuleBaseNameA(
                            handle,
                            module_handle,
                            module_name_buf.as_mut_ptr() as LPSTR,
                            module_name_buf.capacity() as DWORD,
                        );
                        if bytes_copied != 0 {
                            module_name_buf.set_len(bytes_copied as usize);
                            module_name_len = bytes_copied as usize;
                        }
                    }
                    CloseHandle(handle);
                }
                let module_name =
                     {String::from_utf8_lossy(&module_name_buf[..module_name_len])};
                if module_name.trim_end_matches('\0') == process_name {
                    return Some(pid);
                }
            }
        }
    }
    None
}
