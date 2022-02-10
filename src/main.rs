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

    // Next video: 
    checked_arithmetic();
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
    assert_eq!((-128_i8).checked_add(-1), None);
}
