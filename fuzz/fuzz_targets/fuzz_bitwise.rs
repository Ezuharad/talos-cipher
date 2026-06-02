// 2026 Steven Chiacchira
#![no_main]

use libfuzzer_sys::fuzz_target;
use talos::bits::{Bit, BitWise};

fuzz_target!(|data: &[u8]| {
    for byte in data {
        for idx in 0..=8 {
            let _ = byte.get_bit(idx);
        }

        let mut byte = *byte;
        for idx in 0..=u8::n_bits() {
            let _ = byte.set_bit(idx as usize, Bit::ONE);
            let _ = byte.set_bit(idx as usize, Bit::ZERO);
        }
    }
});
