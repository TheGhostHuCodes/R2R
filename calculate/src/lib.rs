use libc::{c_char, c_int};
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn solve(line: *const c_char, solution: *mut c_int) -> c_int {
    if solution.is_null() {
        return 1;
    }

    let c_str = unsafe { CStr::from_ptr(line) };
    let r_str = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("UTF-8 Error: {}", e);
            return 1;
        }
    };

    match evaluate(r_str) {
        Ok(value) => {
            unsafe {
                *solution = value as c_int;
            }
            0
        }
        Err(e) => {
            eprintln!("Error");
            1
        }
    }
}

enum Error {}

fn evaluate(problem: &str) -> Result<i32, Error> {
    Ok(1)
}
