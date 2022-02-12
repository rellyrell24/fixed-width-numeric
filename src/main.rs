// Fixed-Width Numeric Types

fn main() {
    // Explicit Typing is Required

    // Exponentiation
    assert_eq!(2_u16.pow(4), 16);

    // Absolute Value
    // Note: Parenthesis takes precedence
    assert_eq!((-4_i32).abs(), 4);

    // Population count
    assert_eq!(0b101101_u8.count_ones(), 4);

    // Type of arithmetic functions
    checked_arithmetic();
    wrapping_arithmetic();
    saturating_arithmetic();
    overflowing_arithmetic();
}

fn checked_arithmetic() {
    // Checked operations return an Option 
    // of the result: Some(v) or None

    // The sum of 10 and 20 can be represented as u8
    assert_eq!(10u8.checked_add(20), Some(30));

    // Unfortunately, the sum of 100 and 200 cannot
    assert_eq!(100u8.checked_add(200), None);

    // Do the addition; panic if it overflows
    let sum = 100u16.checked_add(200).unwrap();
    eprintln!("The sum is: {}", sum);

    // Oddly, signed division can overflow too, in 
    // one particular case. A signed n-bit can
    // represent -2^(n-1), but not 2^(n-1)
    assert_eq!((-128i8).checked_div(-1), None);
}

fn wrapping_arithmetic() {
    // The first product can be represented as a u16
    // the second cannot, so we get 250000 modulo 2^16
    assert_eq!(100u16.wrapping_mul(200), 20000);
    assert_eq!(500u16.wrapping_mul(500), 53392);

    // Operation on signed types may wap to negative value
    assert_eq!(500i16.wrapping_mul(500), -12144);

    // In bitwise shift operations, the shift distance
    // is wrapped to fall within the size of the value
    // So a shift of 17 bits in a bit type is a shift of 1
    assert_eq!(5i16.wrapping_shl(17), 10);
}

fn saturating_arithmetic() {
    // the result is clamped to the maximum and minimum values 
    assert_eq!(32760i16.saturating_add(10), 32767);
    assert_eq!((-32760i16).saturating_add(10), -32768);

    // there are no saturating division, remainder,
    // or bitwise shift methods
}

fn overflowing_arithmetic() {
    // returns a tuple (result, overflowed)
    assert_eq!(255u8.overflowing_sub(2), (253, false));
    assert_eq!(255u8.overflowing_add(2), (1, true));

    // a shift of 17 bits is too large for `u16`, and 17 % 16 = 1
    assert_eq!(5u16.overflowing_shl(17), (10, true))

}