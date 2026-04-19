
#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

/// Example structured input for fuzzing.
/// Replace with types from your crate.
#[derive(Debug, Arbitrary)]
struct FuzzInput {
    data: Vec<u8>,
    offset: usize,
}

fuzz_target!(|input: FuzzInput| {
    // Example: fuzz a parser or data processing function.
    // Replace with calls to your crate's API.
    if let Some(slice) = input.data.get(..input.offset.min(input.data.len())) {
        let _ = std::str::from_utf8(slice);
    }
});