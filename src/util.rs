/// Mask with the lowest `n` bits set (0–16).
pub(crate) const fn mask16(n: u8) -> u16 {
    if n == 16 {
        return u16::MAX;
    }

    (1u16 << n) - 1
}

/// Mask with the lowest `n` bits set (0–32).
pub(crate) const fn mask32(n: u8) -> u32 {
    if n == 32 {
        return u32::MAX;
    }

    (1u32 << n) - 1
}

/// Mask with the lowest `n` bits set (0–64).
pub(crate) const fn mask(n: u8) -> u64 {
    if n == 64 {
        return u64::MAX;
    }

    (1u64 << n) - 1
}

/// Sign-extend the low `bit_count` bits of `val` into a u64.
pub(crate) fn sext(val: u64, bit_count: usize) -> u64 {
    debug_assert_eq!(val >> bit_count, 0, "upper bits must be zero");

    // bit count must be at least 1 and at most 32
    if bit_count == 0 || bit_count > 32 {
        panic!("invalid bit count");
    }

    let val = val;

    // bit_count represents the length of the binary sequence we plan to extend
    // right_shift to erase all elements other than the sign bit
    let sign_bit = val >> (bit_count - 1);

    // pad higher bits withs 1s
    if sign_bit == 1 {
        return val | (u64::MAX << bit_count);
    }

    val
}

/// Copies a slice from src starting at src_start to
/// dest_start.
/// Assumption
///     ORs the mapped slice into `dest` rather than overwriting it.
///     Bits already set in the destination range remain set.
pub(crate) fn map_range(src: u32, dest: u32, src_start: u8, dest_start: u8, len: u8) -> u32 {
    debug_assert!(len > 0 && len <= 32);
    debug_assert!(src_start + 1 >= len);
    debug_assert!(dest_start + 1 >= len);

    let src_slice = (src >> (src_start + 1 - len)) & mask32(len);
    dest | src_slice << (dest_start + 1 - len)
}

pub(crate) fn is_snan_f32(val: f32) -> bool {
    let bits = val.to_bits();
    let exp = (bits >> 23) & 0xFF;
    let frac = bits & 0x7FFFFF;
    exp == 0xFF && frac != 0 && (frac & 0x400000) == 0
}

pub(crate) fn is_subnormal_f32(val: f32) -> bool {
    let bits = val.to_bits();
    let exp = (bits >> 23) & 0xFF;
    let frac = bits & 0x7FFFFF;
    exp == 0 && frac != 0
}

pub(crate) fn is_snan_f64(val: f64) -> bool {
    let bits = val.to_bits();
    let exp = (bits >> 52) & 0x7FF;
    let frac = bits & 0xFFFFFFFFFFFFF;
    // Signaling NaN: exponent all 1s, fraction non-zero, quiet bit (bit 51) is 0
    exp == 0x7FF && frac != 0 && (frac & 0x8000000000000) == 0
}

pub(crate) fn is_subnormal_f64(val: f64) -> bool {
    let bits = val.to_bits();
    let exp = (bits >> 52) & 0x7FF;
    let frac = bits & 0xFFFFFFFFFFFFF;
    exp == 0 && frac != 0
}

pub(crate) fn classify32(val: u32) -> u64 {
    let sign = val >> 31;
    let exponent = (val >> 23) & mask32(8);
    let frac = val & mask32(23);

    match (sign, exponent, frac) {
        (1, 0xff, 0) => 1,
        (0, 0xff, 0) => 1 << 7,

        (_, 0xff, frac) => {
            let quiet_bit = (frac >> 22) & 1;
            if quiet_bit == 0 { 1 << 8 } else { 1 << 9 }
        }

        (1, 0, 0) => 1 << 3,
        (0, 0, 0) => 1 << 4,

        (1, 0, _) => 1 << 2,
        (0, 0, _) => 1 << 5,

        (1, _, _) => 1 << 1,
        (0, _, _) => 1 << 6,

        (_, _, _) => 0,
    }
}

pub(crate) fn classify64(val: u64) -> u64 {
    let sign = val >> 63;
    let exponent = (val >> 52) & mask(11);
    let frac = val & mask(52);

    match (sign, exponent, frac) {
        (1, 0x7ff, 0) => 1,
        (0, 0x7ff, 0) => 1 << 7,

        (_, 0x7ff, frac) => {
            let quiet_bit = (frac >> 51) & 1;
            if quiet_bit == 0 { 1 << 8 } else { 1 << 9 }
        }

        (1, 0, 0) => 1 << 3,
        (0, 0, 0) => 1 << 4,

        (1, 0, _) => 1 << 2,
        (0, 0, _) => 1 << 5,

        (1, _, _) => 1 << 1,
        (0, _, _) => 1 << 6,

        (_, _, _) => 0,
    }
}

#[cfg(test)]
mod test {
    use crate::util::{map_range, mask, sext};

    #[test]
    fn test_mask_basic() {
        assert_eq!(mask(0), 0);
        assert_eq!(mask(1), 0b1);
        assert_eq!(mask(2), 0b11);
        assert_eq!(mask(3), 0b111);
    }

    #[test]
    fn test_mask_midrange() {
        assert_eq!(mask(8), 0xFF);
        assert_eq!(mask(10), 0x3FF);
        assert_eq!(mask(16), 0xFFFF);
    }

    #[test]
    fn test_mask_upper_bits() {
        assert_eq!(mask(63), 0x7FFF_FFFF_FFFF_FFFF);
    }

    #[test]
    fn test_mask_full_width() {
        assert_eq!(mask(64), u64::MAX);
    }
    #[test]

    fn test_sext_positive_values() {
        // bit_count = 1 (sign bit is LSB)
        assert_eq!(sext(0b0, 1), 0);

        // bit_count = 3
        assert_eq!(sext(0b001, 3), 1);
        assert_eq!(sext(0b011, 3), 3);

        // bit_count = 6
        assert_eq!(sext(0b010101, 6), 0b010101);
    }

    #[test]
    fn test_sext_negative_values() {
        // bit_count = 3 (sign bit at position 2)
        // binary: 1_10 = -2 in 3-bit signed
        assert_eq!(sext(0b110, 3), -2_i64 as u64);

        // bit_count = 5 (sign bit at position 4)
        // binary: 1_0001 = -15 in 5-bit signed
        assert_eq!(sext(0b1_0001, 5), -15_i64 as u64);

        // bit_count = 8
        assert_eq!(sext(0xFF, 8), u64::MAX);

        // bit_count = 8: 0b1110_0000 = -32
        assert_eq!(sext(0b1110_0000, 8), -32_i64 as u64);
    }

    #[test]
    fn test_sext_boundary_cases() {
        // Smallest bit_count
        assert_eq!(sext(0b0, 1), 0);
        assert_eq!(sext(0b1, 1), u64::MAX); // 1-bit "-1"

        // Largest bit_count
        assert_eq!(sext(0x7FFF_FFFF, 32), 0x7FFF_FFFF); // positive max
        assert_eq!(sext(0x8000_0000, 32), 0xFFFF_FFFF_8000_0000); // negative
        assert_eq!(sext(0xFFFF_FFFF, 32), u64::MAX); // -1
    }

    #[test]
    fn test_map_range() {
        let val: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        let target_val: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111;

        assert_eq!(
            map_range(target_val, val, 31, 20, 8),
            0b0000_0000_0001_1111_1110_0000_0000_0000
        );

        let val: u32 = 0b0000_0000_0000_1111_1111_0000_0000_0000;

        assert_eq!(
            map_range(target_val, val, 31, 0, 1),
            0b0000_0000_0000_1111_1111_0000_0000_0001
        );
    }
}
