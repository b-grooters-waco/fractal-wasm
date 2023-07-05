pub fn set_panic_hook() {
    #[cfg(feature = "panic_hook")]
    console_error_panic_hook::set_once();
}
