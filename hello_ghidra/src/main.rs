use windows_sys::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    println!("Hello, world!");
    unsafe { MessageBoxA(0, s!("Hello Turtle"), s!("Caption"), MB_OK); }
}
