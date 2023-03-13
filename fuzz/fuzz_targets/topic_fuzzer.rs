#![no_main]
use libfuzzer_sys::fuzz_target;
use mqtt_v5_fork::topic::Topic;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = s.parse::<Topic>();
    }
});
