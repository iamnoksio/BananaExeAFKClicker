use rdev::{simulate, Button, EventType, SimulateError};
use std::{thread, time::Duration};
use winapi::um::winuser::{FindWindowA, GetWindowRect};
use winapi::shared::windef::RECT;
use std::ptr::null_mut;
use std::ffi::CString;

fn find_window_pos(window_name: &str) -> Option<RECT> {
    unsafe {
        let cstr = CString::new(window_name).ok()?;
        let h_w_n_d = FindWindowA(null_mut(), cstr.as_ptr());

        if h_w_n_d.is_null() {
            return None;
        }

        let mut rect: RECT = std::mem::zeroed();
        if GetWindowRect(h_w_n_d, &mut rect) != 0 {
            Some(rect)
        } else {
            None
        }
    }
}

fn click_at(x: f64, y: f64) -> Result<(), SimulateError> {
    simulate(&EventType::MouseMove { x, y })?;
    simulate(&EventType::ButtonPress(Button::Left))?;
    simulate(&EventType::ButtonRelease(Button::Left))
}

fn main() {
    let process_name = "Banana";
    let wait_duration = Duration::from_secs(300);

    loop {
        match find_window_pos(process_name) {
            Some(rect) => {
                let x = (rect.left + rect.right) as f64 / 2.0;
                let y = (rect.top + rect.bottom) as f64 / 2.0;

                match click_at(x, y) {
                    Ok(()) => println!("Clicked on {} at ({}, {})", process_name, x, y),
                    Err(e) => println!("Failed to click at ({}, {}): {:?}", x, y, e),
                }
            }
            None => println!("{} not found.", process_name),
        }

        thread::sleep(wait_duration);
    }
}