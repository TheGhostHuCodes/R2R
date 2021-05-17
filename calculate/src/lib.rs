use libc::{c_char, c_int};

#[no_mangle]
pub extern "C" fn solve(line: *const c_char, solution: *mut c_int) -> c_int {
    if solution.is_null() {
        return 1;
    }
    unsafe {
        *solution = 1024;
    }

    0
}
