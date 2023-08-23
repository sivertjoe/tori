#[macro_export]
macro_rules! ptr {
    ($p:expr) => {
        $p.as_ptr() as *const std::ffi::c_void
    };
}

#[macro_export]
macro_rules! raw {
    ($p:expr) => {
        $p.as_ptr() as *const std::ffi::c_char
    };
}
