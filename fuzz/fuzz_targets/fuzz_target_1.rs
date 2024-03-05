#![no_main]
use libfuzzer_sys::fuzz_target;
pub use console_calculator::eval::Evaluate;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = s.eval();
    }
});
