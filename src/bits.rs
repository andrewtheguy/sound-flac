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

    /// `count` zero bits, any number of them.
    #[inline]
    pub fn zeros(&mut self, mut count: u32) {
        while count > 63 {
            self.put(0, 63);
            count -= 63;
        }
        self.put(0, count);
    }

    /// The Rice codes of `folded` at parameter `k`: four to a word where they
    /// fit one, which at the parameters sound gives they nearly always do,
    /// two or one where they do not, so the writer's work is a word's
    /// whatever the codes in it. The accumulator and its free bits are
    /// locals through the loop, measured a tenth faster than through `self`.
    pub fn rice_codes(&mut self, folded: &[u32], k: u32) {
        let mask = (1u32 << k) - 1;
        let code = |u: u32| (1u64 << k) | u64::from(u & mask);
        let len = |u: u32| (u >> k) + 1 + k;
        let (mut acc, mut free) = (self.acc, self.free);
        let out = &mut *self.out;
        /// `value`, which has no bit above its `bits`, at most 63 of them.
        #[inline(always)]
        fn fit(out: &mut Vec<u8>, acc: &mut u64, free: &mut u32, value: u64, bits: u32) {
            debug_assert!(bits <= 63 && value >> bits == 0);
            if bits < *free {
                *free -= bits;
                *acc |= value << *free;
            } else {
                let over = bits - *free;
                *acc |= value >> over;
                out.extend_from_slice(&acc.to_be_bytes());
                *free = 64 - over;
                *acc = if over == 0 { 0 } else { value << *free };
            }
        }
        /// One code of any length.
        #[inline(always)]
        fn one(out: &mut Vec<u8>, acc: &mut u64, free: &mut u32, u: u32, k: u32) {
            let total = (u >> k) + 1 + k;
            if total <= 63 {
                fit(out, acc, free, (1u64 << k) | u64::from(u & ((1u32 << k) - 1)), total);
            } else {
                let mut zeros = u >> k;
                while zeros > 63 {
                    fit(out, acc, free, 0, 63);
                    zeros -= 63;
                }
                fit(out, acc, free, 0, zeros);
                fit(out, acc, free, (1u64 << k) | u64::from(u & ((1u32 << k) - 1)), 1 + k);
            }
        }
        let (quads, rest) = folded.as_chunks::<4>();
        for &[u1, u2, u3, u4] in quads {
            let (t1, t2, t3, t4) = (len(u1), len(u2), len(u3), len(u4));
            let total = t1 + t2 + t3 + t4;
            if total <= 63 {
                fit(out, &mut acc, &mut free, (((code(u1) << t2 | code(u2)) << t3 | code(u3)) << t4) | code(u4), total);
            } else {
                for (a, b, ta, tb) in [(u1, u2, t1, t2), (u3, u4, t3, t4)] {
                    if ta + tb <= 63 {
                        fit(out, &mut acc, &mut free, (code(a) << tb) | code(b), ta + tb);
                    } else {
                        one(out, &mut acc, &mut free, a, k);
                        one(out, &mut acc, &mut free, b, k);
                    }
                }
            }
        }
        for &u in rest {
            one(out, &mut acc, &mut free, u, k);
        }
        self.acc = acc;
        self.free = free;
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
        // Quotient 2, a one, then 01: a folded value of 2 * 4 + 1.
        w.rice_codes(&[9], 2);
        w.zeros(70);
        w.put(1, 1);
        w.align();
        assert_eq!(out, [0b1011_1110, 0b0010_1000, 0, 0, 0, 0, 0, 0, 0, 0, 0b0001_0000]);
    }

    /// Rice codes written four to a word, two, or one at a time, and codes
    /// of over 63 bits, all come out as the codes written bit by bit.
    #[test]
    fn rice_codes_come_out_as_written_singly() {
        let mut seed = 0x9E37_79B9u32;
        let folded: Vec<u32> = (0..203)
            .map(|i| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                match i % 50 {
                    49 => seed,
                    48 => seed >> 12,
                    _ => seed >> 23,
                }
            })
            .collect();
        for k in [0u32, 3, 9, 14, 20] {
            let mut fast = Vec::new();
            let mut w = BitWriter::new(&mut fast);
            w.rice_codes(&folded, k);
            w.align();
            let mut slow = Vec::new();
            let mut w = BitWriter::new(&mut slow);
            for &u in &folded {
                w.zeros(u >> k);
                w.put(1, 1);
                for b in (0..k).rev() {
                    w.put(u64::from((u >> b) & 1), 1);
                }
            }
            w.align();
            assert_eq!(fast, slow, "parameter {k}");
        }
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
