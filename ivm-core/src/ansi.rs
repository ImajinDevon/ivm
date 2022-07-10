use std::os::windows::raw::HANDLE;

type Dword = u32;

const STD_OUTPUT_HANDLE: Dword = Dword::MAX - 11;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: Dword = 0x0004;

#[cfg(windows)]
extern "C" {
    fn SetConsoleMode(handle: HANDLE, dw_mode: Dword) -> bool;
    fn GetStdHandle(handle: u32) -> HANDLE;
}

/// Initialize the Windows ANSI terminal.
///
/// Calling this method has no effect on non-Windows platforms.
#[cfg(target_os = "windows")]
pub fn init_ansi_terminal() {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        SetConsoleMode(handle, ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
}

#[cfg(not(target_os = "windows"))]
pub fn init_ansi_terminal() {}
