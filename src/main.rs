use std::io;
use std::mem::size_of;
use std::ffi::OsString;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::handleapi::CloseHandle;
use std::os::windows::ffi::OsStringExt;
use winapi::um::processthreadsapi::{TerminateProcess, OpenProcess};
use winapi::um::tlhelp32::{Process32NextW, Process32FirstW, CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, PROCESSENTRY32W};
use winapi;
use winapi::shared::windef::HHOOK;
use winapi::um::winuser;
use winapi::um::winuser::{HC_ACTION, KBDLLHOOKSTRUCT, CallNextHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_KEYUP, VK_F4, VK_LMENU, VK_LCONTROL};
use std::convert::TryFrom;

static mut HOOK_HANDLE: Option<HHOOK> = None;
static mut CTRL_DOWN: bool = false;
static mut ALT_DOWN: bool = false;
static mut F4_DOWN: bool = false;
static mut KILLING: bool = false;

unsafe fn kill_process(process_id: u32) -> HANDLE {
    let h_process: HANDLE;
    h_process = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);
    
    if !KILLING {
        KILLING = true;
        // pretty sure this fucks up your applications lol
        TerminateProcess(h_process, 1);
        
    };

    h_process
}

fn process_handler(process_name: &str) -> u32 {
    let h_process_snap: HANDLE;
    // let mut process_ids = Vec::new();

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
    
                // if exe_file_string == process_name {
                //     process_ids.push(pe32.th32ProcessID)
                // }
                if exe_file_string == process_name {
                    return pe32.th32ProcessID;
                }
            }
        } else {
            println!("can't get a process snapshot");
        }
        CloseHandle(h_process_snap);
    }

    0

}

// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644985(v=vs.85)
extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    let CTRL: u32 = u32::try_from(VK_LCONTROL).unwrap();
    let ALT: u32 = u32::try_from(VK_LMENU).unwrap();
    let F4: u32 = u32::try_from(VK_F4).unwrap();

    if code < HC_ACTION {
        unsafe {
            if let Some(hook_id) = HOOK_HANDLE {
                return CallNextHookEx(hook_id, code, wparam, lparam);
            } else {
                return 0;
            }
        }
    }

    let keypress: KBDLLHOOKSTRUCT = unsafe { *(lparam as *mut KBDLLHOOKSTRUCT) };
    let is_ctrl = keypress.vkCode == CTRL;
    let is_alt = keypress.vkCode == ALT;
    let is_f4 = keypress.vkCode == F4;

    if wparam == WM_KEYDOWN as usize || wparam == WM_SYSKEYDOWN as usize {
        unsafe {
            if is_ctrl {
                CTRL_DOWN = true;
            } else if is_alt {
                ALT_DOWN = true;
            } else if is_f4 {
                F4_DOWN = true;
            }
        }
    }
    
    // this can be neater but cba
    if wparam == WM_KEYUP as usize {
        unsafe {
            KILLING = false;
            if is_ctrl {
                CTRL_DOWN = false
            }
            if is_alt {
                ALT_DOWN = false
            }
            if is_f4 {
                F4_DOWN = false
            }
        }
    }
    if wparam == WM_SYSKEYUP as usize {
        unsafe {
            KILLING = false;
            if is_ctrl {
                CTRL_DOWN = false
            }
            if is_alt {
                ALT_DOWN = false
            }
            if is_f4 {
                F4_DOWN = false
            }
        }
    }

    unsafe {
        if CTRL_DOWN && ALT_DOWN && F4_DOWN {
            println!("Time to kill!");
            // TODO:
            // get process id from top window
            let process_name = "Discord.exe";
            let process_id = process_handler(&process_name);
        
            if process_id == 0 {
                println!("No process found called '{}'", process_name)
            }
        
            kill_process(process_id);

            // prevent this keypress from being propagated
            return 1;
        }
    }

    0
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
        let hook_id = winuser::SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(hook_callback),
            std::ptr::null_mut(),
            0,
        );
        HOOK_HANDLE = Some(hook_id);

        let msg: winuser::LPMSG = std::ptr::null_mut();
        while winuser::GetMessageA(msg, std::ptr::null_mut(), 0, 0) > 0 {
            winuser::TranslateMessage(msg);
            winuser::DispatchMessageA(msg);
        }

        winapi::um::winuser::UnhookWindowsHookEx(hook_id);
    }
}
