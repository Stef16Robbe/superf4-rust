use winapi::um::processthreadsapi::{TerminateProcess, OpenProcess};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::tlhelp32::{Process32NextW, Process32FirstW, CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32W};
use winapi::um::winnt::HANDLE;
use winapi::um::handleapi::CloseHandle;
use std::mem::size_of;
use std::os::windows::ffi::OsStringExt;
use std::ffi::OsString;
// use winapi::ctypes::{__int64, __uint64, c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void, wchar_t};
// use winapi::shared::minwindef::{
//     BOOL, DWORD, LPBYTE, LPCVOID, LPDWORD, LPFILETIME, LPVOID, PBOOL, PDWORD, PULONG, UINT, WORD
// };

unsafe fn kill_process() -> HANDLE {
    let h_process: HANDLE;

    // Powershell: "Get-Process"
    // example, spotify Id  = 1736...
    // 419 | 53 | 63152 | 105304 | 16.20 | 1488 | 1 | Spotify
    h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, 1736);

    println!("{:?}", h_process);

    TerminateProcess(h_process, 1);

    // returns same id cause process hasn't exited yet when this is run...
    // println!("{:?}", h_process);
    h_process
}

fn get_processes() {
    let h_process_snap: HANDLE;
    let mut pe32 = &mut PROCESSENTRY32W{
        dwSize: 0,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; 260],
    };

    unsafe {
        h_process_snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    }
    println!("{:?}", h_process_snap);

    pe32.dwSize = size_of::<PROCESSENTRY32W>() as u32;
    println!("{:?}", pe32.dwSize);

    unsafe {
        if Process32FirstW(h_process_snap, pe32) == 0 {
            CloseHandle(h_process_snap);
            println!("can't get a process snapshot");
            // return false
        }

        while Process32NextW(h_process_snap, pe32) != 0 {
            // https://stackoverflow.com/questions/69146231/printing-tchar-array-as-unicode-string-rust-winapi
            let os_string = OsString::from_wide(&pe32.szExeFile[..]);
            // ExeFile names have a lot of trailing 0's, remove them...
            let exe_files: String = os_string.into_string().unwrap().replace("\u{0}", "");
            println!("{:?}", exe_files);

        }
    }

}

fn main() {
    get_processes()

}
