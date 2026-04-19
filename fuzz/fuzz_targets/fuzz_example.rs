#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use ci_battery_pack_test::add;

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    left: u64,
    right: u64,
}

fuzz_target!(|input: FuzzInput| {
    // Exercise the crate's API with arbitrary inputs.
    let _ = add(input.left, input.right);
});