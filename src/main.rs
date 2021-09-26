use std::io;
use std::mem::size_of;
use std::ffi::OsString;
use structopt::StructOpt;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::handleapi::CloseHandle;
use std::os::windows::ffi::OsStringExt;
use winapi::um::processthreadsapi::{TerminateProcess, OpenProcess};
use winapi::um::tlhelp32::{Process32NextW, Process32FirstW, CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32W};

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "")]
    process_name: String,
}

unsafe fn kill_process(process_id: u32) -> HANDLE {
    let h_process: HANDLE;

    // Powershell: "Get-Process"
    // example, Spotify id  = 1736...
    // 419 | 53 | 63152 | 105304 | 16.20 | 1488 | 1 | Spotify
    h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);

    TerminateProcess(h_process, 1);

    h_process
}

fn get_all_processes() {
    let h_process_snap: HANDLE;
    let mut pe32 = &mut PROCESSENTRY32W {
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

    pe32.dwSize = size_of::<PROCESSENTRY32W>() as u32;
    
    unsafe {
        h_process_snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if Process32FirstW(h_process_snap, pe32) != 0 {
            while Process32NextW(h_process_snap, pe32) != 0 {
                // https://stackoverflow.com/questions/69146231/printing-tchar-array-as-unicode-string-rust-winapi
                let os_string = OsString::from_wide(&pe32.szExeFile[..]);
                // ExeFile names have a lot of trailing 0's, remove them...
                let exe_files: String = os_string.into_string().unwrap().replace("\u{0}", "");
                
                for file_name in exe_files.split("\n") {
                    println!("{}", file_name);
                }
            }
        } else {
            println!("can't get a process snapshot");
        }
        CloseHandle(h_process_snap);
    }
}

// TODO:
// duplicate code
fn get_process_ids(process_name: &str) -> Vec<u32> {
    let h_process_snap: HANDLE;
    let mut process_ids = Vec::new();

    let mut pe32 = &mut PROCESSENTRY32W {
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

    pe32.dwSize = size_of::<PROCESSENTRY32W>() as u32;
    
    unsafe {
        h_process_snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if Process32FirstW(h_process_snap, pe32) != 0 {
            while Process32NextW(h_process_snap, pe32) != 0 {
                let os_string = OsString::from_wide(&pe32.szExeFile[..]);
                let exe_file_string: String = os_string.into_string().unwrap().replace("\u{0}", "");
    
                if exe_file_string == process_name {
                    process_ids.push(pe32.th32ProcessID)
                }
            }
        } else {
            println!("can't get a process snapshot");
        }
        CloseHandle(h_process_snap);
    }

    process_ids

}

/*
*
* Basic SuperF4 (https://github.com/stefansundin/superf4) implementation in Rust 
* Usage: `cargo run -- {process name}
* 
*/
fn main() {
    let args = Cli::from_args();
    let mut process_name = String::new();

    if &args.process_name == "" {
        get_all_processes();
        println!("Enter a process name...");
        io::stdin().read_line(&mut process_name).unwrap();
        process_name = process_name.replace("\r", "").replace("\n", "");
    } else {
        process_name = args.process_name;
    }

    let process_ids = get_process_ids(&process_name);

    if process_ids == Vec::new() {
        println!("No process found called '{}'", process_name)
    }

    unsafe {
        for process_id in process_ids {
            kill_process(process_id);
        }
    }
}
