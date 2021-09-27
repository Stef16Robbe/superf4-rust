use std::io;
use std::mem::size_of;
use std::ffi::OsString;
use winapi::ctypes::wchar_t;
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::psapi::GetProcessImageFileNameW;
use winapi::um::winnt::{HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR};
use winapi::um::handleapi::CloseHandle;
use std::os::windows::ffi::OsStringExt;
use winapi::um::processthreadsapi::{TerminateProcess, OpenProcess};
use winapi::um::tlhelp32::{Process32NextW, Process32FirstW, CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32W};
use winapi;
use winapi::shared::windef::HHOOK;
use winapi::um::winuser;
use winapi::um::winuser::{GetWindowThreadProcessId, GetForegroundWindow, HC_ACTION, KBDLLHOOKSTRUCT, CallNextHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_KEYUP, VK_F4, VK_LMENU, VK_LCONTROL};
use std::convert::TryFrom;


unsafe fn kill_process(process_id: u32) -> HANDLE {
    let h_process: HANDLE;
    h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);
    
    // pretty sure this fucks up your applications lol
    TerminateProcess(h_process, 1);

    h_process
}


/*
*
* Basic SuperF4 (https://github.com/stefansundin/superf4) implementation in Rust 
* Usage: press `ctrl + alt + f4` on any selected program
* 
*/
fn main() {
    // https://github.com/linde12/winapi-testing !!!!
    unsafe {
        let mut pid: winapi::shared::minwindef::DWORD = 0;
        let handle = GetForegroundWindow();
        let kak = GetWindowThreadProcessId(handle, &mut pid);
        let name: [wchar_t; 256] = [0; 256];
        let mut nimma: u16 = 0;
        // TODO:
        // GetLastError
        // why 0?
        let kaas = GetProcessImageFileNameW(handle as HANDLE, &mut nimma, name.len() as u32);
        println!("{:?}", kaas);
    }
}
