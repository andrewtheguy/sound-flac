//! Bits into bytes, and the two CRCs a frame carries.

/// Bits appended to a `Vec<u8>`, most significant first, as FLAC lays them.
pub struct BitWriter<'a> {
    out: &'a mut Vec<u8>,
    /// The bits not yet written, at the top; `free` bits are open below them.
    acc: u64,
    free: u32,
}

impl<'a> BitWriter<'a> {
    pub fn new(out: &'a mut Vec<u8>) -> Self {
        Self { out, acc: 0, free: 64 }
    }

    /// The low `bits` of `value`, at most 63 of them; anything above is masked
    /// off, so a negative sample's two's complement goes in as it should.
    #[inline(always)]
    pub fn put(&mut self, value: u64, bits: u32) {
        debug_assert!(bits <= 63);
        if bits == 0 {
            return;
        }
        let value = value & (u64::MAX >> (64 - bits));
        if bits < self.free {
            self.free -= bits;
            self.acc |= value << self.free;
        } else {
            let over = bits - self.free;
            self.acc |= value >> over;
            self.out.extend_from_slice(&self.acc.to_be_bytes());
            self.free = 64 - over;
            self.acc = if over == 0 { 0 } else { value << self.free };
        }
    }

    /// `value`, which has no bit above its `bits`, at most 63 of them: what
    /// [`BitWriter::put`] does without the masking, for a value made to fit.
    #[inline(always)]
    pub fn fitted(&mut self, value: u64, bits: u32) {
        debug_assert!(bits <= 63 && value >> bits == 0);
        if bits < self.free {
            self.free -= bits;
            self.acc |= value << self.free;
        } else {
            let over = bits - self.free;
            self.acc |= value >> over;
            self.out.extend_from_slice(&self.acc.to_be_bytes());
            self.free = 64 - over;
            self.acc = if over == 0 { 0 } else { value << self.free };
        }
    }

    /// `count` zero bits, any number of them.
    #[inline]
    pub fn zeros(&mut self, mut count: u32) {
        while count > 63 {
            self.put(0, 63);
            count -= 63;
        }
        self.put(0, count);
    }

    /// One Rice code: `quotient` zeros, a one, then the low `k` bits of
    /// `remainder`.
    #[inline(always)]
    pub fn rice(&mut self, quotient: u32, remainder: u32, k: u32) {
        let total = quotient + 1 + k;
        if total <= 63 {
            self.put((1u64 << k) | u64::from(remainder), total);
        } else {
            self.zeros(quotient);
            self.put((1u64 << k) | u64::from(remainder), 1 + k);
        }
    }

    /// Zero bits up to the next byte, and every byte out.
    pub fn align(&mut self) {
        let held = (64 - self.free).div_ceil(8);
        self.out.extend_from_slice(&self.acc.to_be_bytes()[..held as usize]);
        self.acc = 0;
        self.free = 64;
    }
}

const fn table8() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u8;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 0x80 != 0 { (crc << 1) ^ 0x07 } else { crc << 1 };
            bit += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

/// Eight tables: the CRC of a byte followed by 0 to 7 zero bytes, so eight
/// bytes go in at a time.
const fn tables16() -> [[u16; 256]; 8] {
    let mut tables = [[0u16; 256]; 8];
    let mut i = 0;
    while i < 256 {
        let mut crc = (i as u16) << 8;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 0x8000 != 0 { (crc << 1) ^ 0x8005 } else { crc << 1 };
            bit += 1;
        }
        tables[0][i] = crc;
        i += 1;
    }
    let mut k = 1;
    while k < 8 {
        let mut i = 0;
        while i < 256 {
            let before = tables[k - 1][i];
            tables[k][i] = (before << 8) ^ tables[0][(before >> 8) as usize];
            i += 1;
        }
        k += 1;
    }
    tables
}

static CRC8: [u8; 256] = table8();
static CRC16: [[u16; 256]; 8] = tables16();

/// The frame header's CRC: polynomial 0x07, from zero.
pub fn crc8(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0u8, |crc, &byte| CRC8[usize::from(crc ^ byte)])
}

/// The frame's CRC: polynomial 0x8005, from zero.
pub fn crc16(bytes: &[u8]) -> u16 {
    let (eights, rest) = bytes.as_chunks::<8>();
    let mut crc = 0u16;
    for &[b0, b1, b2, b3, b4, b5, b6, b7] in eights {
        crc = CRC16[7][usize::from(b0 ^ (crc >> 8) as u8)]
            ^ CRC16[6][usize::from(b1 ^ crc as u8)]
            ^ CRC16[5][usize::from(b2)]
            ^ CRC16[4][usize::from(b3)]
            ^ CRC16[3][usize::from(b4)]
            ^ CRC16[2][usize::from(b5)]
            ^ CRC16[1][usize::from(b6)]
            ^ CRC16[0][usize::from(b7)];
    }
    rest.iter().fold(crc, |crc, &byte| (crc << 8) ^ CRC16[0][usize::from((crc >> 8) as u8 ^ byte)])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The check values the CRC catalogue gives these two polynomials for
    /// "123456789": CRC-8 (SMBUS) 0xF4 and CRC-16/UMTS (BUYPASS) 0xFEE8; and
    /// the eight-at-a-time path agrees with the byte-at-a-time one on every
    /// length.
    #[test]
    fn the_crcs_match_the_catalogue() {
        assert_eq!(crc8(b"123456789"), 0xF4);
        assert_eq!(crc16(b"123456789"), 0xFEE8);
        let bytes: Vec<u8> = (0..100u32).map(|i| (i * 37 + 11) as u8).collect();
        for len in 0..bytes.len() {
            let slow = bytes[..len].iter().fold(0u16, |crc, &byte| (crc << 8) ^ CRC16[0][usize::from((crc >> 8) as u8 ^ byte)]);
            assert_eq!(crc16(&bytes[..len]), slow, "{len} bytes");
        }
    }

    #[test]
    fn bits_land_most_significant_first() {
        let mut out = Vec::new();
        let mut w = BitWriter::new(&mut out);
        w.put(0b101, 3);
        w.put(-2i32 as u32 as u64, 5);
        w.rice(2, 0b01, 2);
        w.zeros(70);
        w.put(1, 1);
        w.align();
        assert_eq!(out, [0b1011_1110, 0b0010_1000, 0, 0, 0, 0, 0, 0, 0, 0, 0b0001_0000]);
    }

    /// Writes that straddle the accumulator's edge come out the same as the
    /// bits written one at a time.
    #[test]
    fn writes_across_the_word_boundary_hold() {
        let values: Vec<(u64, u32)> = (1..200u64).map(|i| (i.wrapping_mul(0x9E37_79B9_7F4A_7C15), (i % 63 + 1) as u32)).collect();
        let mut fast = Vec::new();
        let mut w = BitWriter::new(&mut fast);
        for &(v, bits) in &values {
            w.put(v, bits);
        }
        w.align();
        let mut slow = Vec::new();
        let mut w = BitWriter::new(&mut slow);
        for &(v, bits) in &values {
            for b in (0..bits).rev() {
                w.put((v >> b) & 1, 1);
            }
        }
        w.align();
        assert_eq!(fast, slow);
    }
}
