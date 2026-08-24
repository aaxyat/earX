/// Computes the CRC-16 (Modbus polynomial 0xA001, initial value 0xFFFF)
/// used by Nothing and CMF earbud RFCOMM communication frames.
pub fn compute_crc16(buffer: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in buffer {
        crc ^= byte as u16;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16_calculation() {
        let sample_header = [0x55, 0x60, 0x01, 0x06, 0xC0, 0x00, 0x00, 0x01];
        let crc = compute_crc16(&sample_header);
        assert_ne!(crc, 0);
    }
}
