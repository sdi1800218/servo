#![no_main]
use libfuzzer_sys::fuzz_target;
use servo_url::ServoUrl;
use std::sync::Arc;
use style::stylesheets::DocumentStyleSheet;
use fonts::*;

fuzz_target!(|data: (String, Vec<u8>)| {
    // Unpack the datas
    let url_string = data.0;
    let font_data = Arc::new(data.1);
    let stylesheet_flag = Option::None;

    // Create a basic CSSFontFaceDescriptors
    // TODO: Arbitrary it
    let css_font_template_descriptors = CSSFontFaceDescriptors {
        family_name: LowercaseString::new("hello"),
        weight: None,
        style: None,
        stretch: None,
        unicode_range: None,
    };

    // Try to create a ServoUrl from the string
    if let Ok(url) = ServoUrl::parse(&url_string) {
        // Fuzz the new_for_remote_web_font function
        let _ = FontTemplate::new_for_remote_web_font(url, font_data, &css_font_template_descriptors, stylesheet_flag);
    }
});
