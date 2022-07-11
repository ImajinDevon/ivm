#[cfg(target_os = "windows")]
mod windows {
    use std::os::windows::raw::HANDLE;

    type Dword = u32;

    pub const STD_OUTPUT_HANDLE: Dword = Dword::MAX - 11;
    pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: Dword = 0x0004;

    #[cfg(target_os = "windows")]
    extern "C" {
        pub fn SetConsoleMode(handle: HANDLE, dw_mode: Dword) -> bool;
        pub fn GetStdHandle(handle: u32) -> HANDLE;
    }
}

/// Initialize the Windows ANSI terminal.
///
/// Calling this method has no effect on non-Windows platforms.
#[cfg(target_os = "windows")]
pub fn init_ansi_terminal() {
    unsafe {
        let handle = windows::GetStdHandle(windows::STD_OUTPUT_HANDLE);
        windows::SetConsoleMode(handle, windows::ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
}

#[cfg(not(target_os = "windows"))]
pub fn init_ansi_terminal() {}
