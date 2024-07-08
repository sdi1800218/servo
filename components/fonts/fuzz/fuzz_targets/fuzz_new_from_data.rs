#![no_main]
use libfuzzer_sys::fuzz_target;
use fonts::*;
use std::sync::Arc;

// Fuzz target definition
fuzz_target!(|data: (FontIdentifier, Vec<u8>, u32, Option<Au>)| {
    let font_identifier = data.0;
    let font_data = Arc::new(data.1);
    let face_index = data.2;
    let pt_size = data.3;

    // Send them in
    let _ = YourPlatformFont::new_from_data(font_identifier, font_data, face_index, pt_size);
});

