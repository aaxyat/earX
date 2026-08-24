/// Formats a float value (gain in dB, typically -6.0 .. +6.0) into the 4-byte representation
/// expected by Nothing earbud firmware for Custom EQ.
///
/// If `is_preamp` is true (the total gain compensation value), non-negative values
/// are represented as `[0x00, 0x00, 0x00, 0x80]`.
pub fn format_float_for_eq(f: f32, is_preamp: bool) -> [u8; 4] {
    if is_preamp && f >= 0.0 {
        return [0x00, 0x00, 0x00, 0x80];
    }

    let mut bytes = f.to_be_bytes();
    if f != 0.0 && bytes[0] == 0 && bytes[1] == 0 && bytes[2] == 0 {
        bytes[3] = bytes[3] | 0x80;
    }

    // Reverse byte order (big-endian to little-endian representation)
    bytes.reverse();
    bytes
}

/// Decodes the 4-byte EQ float representation returned by Nothing earbuds back into a standard f32.
pub fn from_format_float_for_eq(mut bytes: [u8; 4]) -> f32 {
    bytes.reverse();

    if bytes[0] == 0 && bytes[1] == 0 && bytes[2] == 0 && (bytes[3] & 0x80) != 0 {
        bytes[3] &= 0x7F;
        let val = f32::from_be_bytes(bytes);
        -val
    } else {
        f32::from_be_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_roundtrip() {
        let test_values = [0.0, 1.0, -1.0, 3.5, -4.2, 6.0, -6.0];
        for &val in &test_values {
            let encoded = format_float_for_eq(val, false);
            let decoded = from_format_float_for_eq(encoded);
            assert!((val - decoded).abs() < 1e-4, "Failed for {}: got {}", val, decoded);
        }
    }

    #[test]
    fn test_preamp_format() {
        let preamp = format_float_for_eq(0.0, true);
        assert_eq!(preamp, [0x00, 0x00, 0x00, 0x80]);
    }
}
