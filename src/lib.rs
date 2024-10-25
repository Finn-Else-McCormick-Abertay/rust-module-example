extern "C" {
    fn alert();
}

#[no_mangle]
pub fn run() {
    unsafe {
        alert();
    }
}