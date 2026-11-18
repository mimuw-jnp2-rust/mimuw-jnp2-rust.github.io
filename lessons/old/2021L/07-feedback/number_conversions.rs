fn main() {
    let small_number: u32 = u32::MAX;
    // dbg!(small_number + 1); // this will panic (in debug builds, in release build it will wrap)
    assert_eq!(small_number as u8, 255);
    assert_eq!(small_number as i8, -1);
    assert_eq!(small_number as i64, 4294967295);
    let converted: Result<i8, _> = small_number.try_into();
    assert!(converted.is_err());
}
