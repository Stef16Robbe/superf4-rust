use std::io;
use std::mem::size_of;
use std::ffi::OsString;
use winapi::ctypes::{wchar_t, c_char, c_void};
use winapi::um::winnt::{PROCESS_ALL_ACCESS, PTOKEN_PRIVILEGES};
use winapi::um::psapi::{GetProcessImageFileNameW, GetProcessImageFileNameA};
use winapi::um::winnt::{HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR};
use winapi::um::handleapi::CloseHandle;
use std::os::windows::ffi::OsStringExt;
use winapi::um::processthreadsapi::{TerminateProcess, OpenProcess, GetProcessId};
use winapi::um::tlhelp32::{Process32NextW, Process32FirstW, CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32W};
use winapi;
use winapi::shared::windef::HWND;
use winapi::um::winuser;
use winapi::um::winuser::{GetWindowThreadProcessId, GetForegroundWindow, HC_ACTION, KBDLLHOOKSTRUCT, CallNextHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_KEYUP, VK_F4, VK_LMENU, VK_LCONTROL};
use std::convert::TryFrom;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::securitybaseapi::AdjustTokenPrivileges;


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
        /*
        1. get HWND from getforegroundwindow
        2. get DWORD processid from GetWindowThreadProcessId using hwnd
        3. apparently, give ourselves permission using AdjustTokenPrivileges
        4. give that pid to kill_process()
        */

        // 1
        let handle: HWND = GetForegroundWindow();
        // 2
        let mut pid: winapi::shared::minwindef::DWORD = 0;
        pid = GetWindowThreadProcessId(handle, &mut pid);
        // 3
        // TODO:
        // initialize these fuckers
        let hToken: HANDLE;
        let tkp: PTOKEN_PRIVILEGES;
        let kak: winapi::shared::minwindef::PDWORD = &mut 0;

        AdjustTokenPrivileges(hToken, winapi::shared::minwindef::FALSE, tkp, 0, tkp, kak);


        // 4
        kill_process(pid);
    }
}
