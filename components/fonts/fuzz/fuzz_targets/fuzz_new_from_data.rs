#![no_main]
use libfuzzer_sys::fuzz_target;
use fonts::*;
use std::sync::Arc;
use fonts::platform::font::PlatformFont;
//use fonts::FontIdentifier;
use servo_url::ServoUrl;


// Fuzz target definition
/*
fuzz_target!(|data: (FontIdentifier, Vec<u8>, u32, Option<Au>)| {
    let font_identifier = data.0;
    let font_data = Arc::new(data.1);
    let face_index = data.2;
    let pt_size = data.3;

    // Send them in
    let _ = PlatformFont::new_from_data(font_identifier, font_data, face_index, pt_size);
});
*/

fuzz_target!(|data: &[u8]| {
    let path = format!("/tmp/fuzz_font_{}.ttf", 0); // Static path for simplicity
    let identifier = FontIdentifier::Web(ServoUrl::from_file_path(path.clone()).unwrap());

    // Attempt to create a PlatformFont with the provided data
    let _ = PlatformFont::new_from_data(identifier, Arc::new(data.to_vec()), 0, None);
});
