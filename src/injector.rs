// a collaboration between arad and allahu akbar (alawapar)
// this code is INSANE.

use winapi::um::{winnt::{HANDLE, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, MEM_RELEASE}, memoryapi::{WriteProcessMemory, VirtualFreeEx}, libloaderapi::GetProcAddress, synchapi::WaitForSingleObject, handleapi::CloseHandle};
use widestring::U16CString;

pub unsafe fn inject(process: Process, path: Path) {
let dllpath_address = winapi::um::memoryapi::VirtualAllocEx(process.handle, std::ptr::null_mut(), path.len(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE); // basically the address of the dll path
if dllpath_address.is_null() {
    panic!("OH NO 💀💀") 
}
let dllpath_address_written_to = WriteProcessMemory(process.handle, dllpath_address, path.as_ptr(), path.len(), std::ptr::null_mut());
if dllpath_address_written_to == 0 {
    panic!("OH NO 💀💀") 
}

let kernel_module = winapi::um::libloaderapi::GetModuleHandleA("kernel32.dll\0".as_ptr().cast()); 
if kernel_module.is_null() {
    panic!("OH NO 💀💀💀") 
}

let loadlibrary_address = GetProcAddress(kernel_module, "LoadLibraryW\0".as_ptr().cast()); 
let remote_thread_handle = winapi::um::processthreadsapi::CreateRemoteThread(process.handle, std::ptr::null_mut(), 0, std::mem::transmute(loadlibrary_address), dllpath_address, 0, std::ptr::null_mut());
if remote_thread_handle.is_null() {
    VirtualFreeEx(process.handle, dllpath_address, 0, MEM_RELEASE);
    panic!("NUH UH, NO MORE INJECTION 💀💀💀💀💀💀") 
}

WaitForSingleObject(remote_thread_handle, winapi::um::winbase::INFINITE); 
CloseHandle(remote_thread_handle);

VirtualFreeEx(process.handle, dllpath_address, 0, MEM_RELEASE); 

}

pub struct Process {
   handle: HANDLE 
}


pub struct Path (U16CString); 

impl Path { 
    pub fn len(&self) -> usize { 
      self.0.len() * 2 + 1 
    }

    pub fn as_ptr(&self) -> *const std::ffi::c_void { 
        self.0.as_ptr() as _ 
    }

    pub fn new(u16cstring: U16CString) -> Self {
        Self(u16cstring)
    }
}

impl Process {
    pub fn new(handle: HANDLE) -> Self {
        Self { handle }
    }
}