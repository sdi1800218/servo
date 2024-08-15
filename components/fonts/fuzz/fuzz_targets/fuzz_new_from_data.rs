#![no_main]
use libfuzzer_sys::fuzz_target;
use fonts::*;
use std::sync::Arc;
use fonts::platform::font::PlatformFont;
//use fonts::FontIdentifier;
use servo_url::ServoUrl;


fuzz_target!(|data: Vec<u8> | {
    let path = "/tmp/hello.ttf";
    let font_data = Arc::new(data);
    let face_index = 0;

    match ServoUrl::from_file_path(path.clone()) {
        Ok(url) => {
            let identifier = FontIdentifier::Web(url);
            //match PlatformFont::new_from_data(identifier, font_data, face_index, None) {
            //    Ok(_) => {}, // Handle success
            //    Err(e) => println!("Failed to create font: {:?}", e),
            //}
            let _ = PlatformFont::new_from_data(identifier, font_data, face_index, None);
        },
        Err(e) => println!("Invalid URL: {:?}", e),
    }
});

/*
fuzz_target!(|data: &[u8]| {
    let path = format!("/tmp/fuzz_font_test.ttf"); // Static path for simplicity
    let identifier = FontIdentifier::Web(ServoUrl::from_file_path(path.clone()).unwrap());

    // Attempt to create a PlatformFont with the provided data
    let _ = PlatformFont::new_from_data(identifier, Arc::new(data.to_vec()), 0, None);
});
*/
