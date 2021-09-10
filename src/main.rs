use winapi::um::processthreadsapi::{TerminateProcess, OpenProcess};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::winnt::HANDLE;
// use winapi::ctypes::{__int64, __uint64, c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void, wchar_t};
// use winapi::shared::minwindef::{
//     BOOL, DWORD, LPBYTE, LPCVOID, LPDWORD, LPFILETIME, LPVOID, PBOOL, PDWORD, PULONG, UINT, WORD
// };

fn main() {
    let h_process: HANDLE;
    unsafe {
        // Powershell: "Get-Process"
        // example, spotify Id  = 1488...
        // 419 | 53 | 63152 | 105304 | 16.20 | 1488 | 1 | Spotify
        h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, 1736);
        println!("{:?}", h_process);
        TerminateProcess(h_process, 1);
        // returns same id cause process hasn't exited yet when this is run...
        println!("{:?}", h_process);
    }
}
