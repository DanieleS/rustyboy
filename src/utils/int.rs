#[inline(always)]
pub fn activate_rightmost_zeros(x: u16) -> u16 {
    x | x.wrapping_sub(1)
}

#[inline(always)]
pub fn test_add_carry_bit(bit: usize, a: u16, b: u16) -> bool {
    let mask = activate_rightmost_zeros(1 << bit);
    (a & mask) + (b & mask) > mask
}
