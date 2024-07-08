#![no_main]
use libfuzzer_sys::fuzz_target;
use fonts::FallbackFontSelectionOptions;

fuzz_target!(|data: (char, Option<char>)| {
    let (character, next_character) = data;
    let _ = FallbackFontSelectionOptions::new(character, next_character);
});

