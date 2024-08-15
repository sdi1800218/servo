#![no_main]
use libfuzzer_sys::fuzz_target;
use fonts::{FallbackFontSelectionOptions, fallback_font_families};

fuzz_target!(|data: (char, Option<char>)| {
    let (character, next_character) = data;
    let _ = fallback_font_families(FallbackFontSelectionOptions::new(character, next_character));
});

