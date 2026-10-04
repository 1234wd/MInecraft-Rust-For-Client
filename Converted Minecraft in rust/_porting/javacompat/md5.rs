//! Port of: (no Java file -- see DESIGN_DECISIONS.md #javacompat)
//! Java class(es): com.google.common.hash.Hashing#md5 (RFC 1321)
//! Status: VERIFIED
//!
//! `RandomSupport.seedFromHashOf(String)` hashes the UTF-8 bytes of the string with
//! MD5 and reads the digest as two little-endian longs. That makes world-gen seeds
//! depend on MD5 exactly, so we cannot approximate it.
//!
//! We depend on zero crates: the oracle proved (see `_porting/java-oracle/stubs/
//! com/google/common/hash/Hashing.java`) that Guava's `Hashing.md5()` and the JDK's
//! `MessageDigest.getInstance("MD5")` are the same RFC-1321 digest, and the
//! golden data in `_porting/test-data/random.txt` (`seedFromHashOf`) pins the
//! result for ASCII, Latin-1 and CJK inputs.
//!
//! Parity-tested by `_porting/tests/parity_random.rs`.

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

/// `floor(2^32 * abs(sin(i + 1)))` for `i` in `0..64`, i.e. RFC 1321's `K` table.
const K: [u32; 64] = [
    0xd76a_a478, 0xe8c7_b756, 0x2420_70db, 0xc1bd_ceee,
    0xf57c_0faf, 0x4787_c62a, 0xa830_4613, 0xfd46_9501,
    0x6980_98d8, 0x8b44_f7af, 0xffff_5bb1, 0x895c_d7be,
    0x6b90_1122, 0xfd98_7193, 0xa679_438e, 0x49b4_0821,
    0xf61e_2562, 0xc040_b340, 0x265e_5a51, 0xe9b6_c7aa,
    0xd62f_105d, 0x0244_1453, 0xd8a1_e681, 0xe7d3_fbc8,
    0x21e1_cde6, 0xc337_07d6, 0xf4d5_0d87, 0x455a_14ed,
    0xa9e3_e905, 0xfcef_a3f8, 0x676f_02d9, 0x8d2a_4c8a,
    0xfffa_3942, 0x8771_f681, 0x6d9d_6122, 0xfde5_380c,
    0xa4be_ea44, 0x4bde_cfa9, 0xf6bb_4b60, 0xbebf_bc70,
    0x289b_7ec6, 0xeaa1_27fa, 0xd4ef_3085, 0x0488_1d05,
    0xd9d4_d039, 0xe6db_99e5, 0x1fa2_7cf8, 0xc4ac_5665,
    0xf429_2244, 0x432a_ff97, 0xab94_23a7, 0xfc93_a039,
    0x655b_59c3, 0x8f0c_cc92, 0xffef_f47d, 0x8584_5dd1,
    0x6fa8_7e4f, 0xfe2c_e6e0, 0xa301_4314, 0x4e08_11a1,
    0xf753_7e82, 0xbd3a_f235, 0x2ad7_d2bb, 0xeb86_d391,
];

/// RFC 1321 MD5 digest of `data`. Returns the raw 16 bytes.
pub fn md5(data: &[u8]) -> [u8; 16] {
    let mut msg = data.to_vec();
    let bit_len = (data.len() as u64).wrapping_mul(8);

    // Padding: 0x80, then zeroes until length % 64 == 56, then a 64-bit LE bit count.
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_le_bytes());

    let mut a0: u32 = 0x6745_2301;
    let mut b0: u32 = 0xefcd_ab89;
    let mut c0: u32 = 0x98ba_dcfe;
    let mut d0: u32 = 0x1032_5476;

    for chunk in msg.chunks_exact(64) {
        let mut m = [0u32; 16];
        for (i, word) in m.iter_mut().enumerate() {
            *word = u32::from_le_bytes([chunk[4 * i], chunk[4 * i + 1], chunk[4 * i + 2], chunk[4 * i + 3]]);
        }

        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        for i in 0..64 {
            let (f, g) = match i / 16 {
                0 => ((b & c) | (!b & d), i),
                1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                2 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let tmp = d;
            d = c;
            c = b;
            let sum = a.wrapping_add(f).wrapping_add(K[i]).wrapping_add(m[g]);
            b = b.wrapping_add(sum.rotate_left(S[i]));
            a = tmp;
        }

        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&a0.to_le_bytes());
    out[4..8].copy_from_slice(&b0.to_le_bytes());
    out[8..12].copy_from_slice(&c0.to_le_bytes());
    out[12..16].copy_from_slice(&d0.to_le_bytes());
    out
}

/// Port of `RandomSupport.seedFromHashOf(String)`'s reading of the digest:
/// bytes 0..8 become `seedLo`, bytes 8..16 become `seedHi`.
///
/// # THE BYTE ORDER IS BIG-ENDIAN, AND THAT IS EASY TO GET WRONG
///
/// `RandomSupport#seedFromHashOf` reads the digest with
/// `Longs.fromBytes(hashCode[0], hashCode[1], ..., hashCode[7])`, and the name
/// "fromBytes" plus the near-universal little-endian convention on x86 makes
/// little-endian the obvious assumption. It is wrong.
///
/// Guava 33.6.0-jre (the version Minecraft 26.2 resolves to) compiles that method
/// to `b1 << 56 | b2 << 48 | ... | b8` -- the FIRST byte is the MOST significant.
/// Verified with `javap -c`:
///
/// ```text
/// public static long fromBytes(byte, byte, byte, byte, byte, byte, byte, byte);
///    0: iload_0        // b1
///    1: i2l
///    2: ldc2_w  255l
///    5: land
///    6: bipush 56
///    8: lshl           // b1 << 56
/// ```
///
/// Measured, MD5("a") = `0cc175b9 c0f1b6a8 31c399e2 69772661`:
///
/// | reading                    | seedLo            |
/// |----------------------------|-------------------|
/// | big-endian (CORRECT)       | `0x0cc175b9c0f1b6a8` |
/// | little-endian (wrong)      | `0xa8b6f1c0b975c10c` |
///
/// Getting this backwards still produces plausible-looking 128-bit seeds, so
/// nothing downstream would ever flag it -- every world would simply be generated
/// from different seeds. Session 02 shipped a stubbed `Longs` that happened to be
/// little-endian, which made the wrong assumption look correct; the stub was
/// removed in session 03 and the divergence surfaced immediately.
///
/// The values below are asserted against the oracle in `parity_random.rs`, and
/// `md5_long_order_is_big_endian` pins them independently of any golden file.
pub fn md5_lo_hi(data: &[u8]) -> (i64, i64) {
    let digest = md5(data);
    let lo = i64::from_be_bytes([digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7]]);
    let hi = i64::from_be_bytes([digest[8], digest[9], digest[10], digest[11], digest[12], digest[13], digest[14], digest[15]]);
    (lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc1321_test_suite() {
        assert_eq!(hex(&md5(b"")), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(hex(&md5(b"a")), "0cc175b9c0f1b6a831c399e269772661");
        assert_eq!(hex(&md5(b"abc")), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(hex(&md5(b"message digest")), "f96b697d7cb7938d525a2f31aaf161d0");
        assert_eq!(hex(&md5(b"abcdefghijklmnopqrstuvwxyz")), "c3fcd3d76192e4007dfb496cca67e13b");
        assert_eq!(
            hex(&md5(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")),
            "d174ab98d277d9f5a5611c2c9f419d9f"
        );
        assert_eq!(
            hex(&md5(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890")),
            "57edf4a22be3c955ac49da2e2107b67a"
        );
    }

    fn hex(b: &[u8]) -> String {
        b.iter().map(|x| format!("{x:02x}")).collect()
    }
    /// `Longs.fromBytes` is BIG-endian in Guava 33.6.0-jre, so `seedFromHashOf`
    /// must read the MD5 digest most-significant-byte-first.
    ///
    /// These are hand-computed from the published MD5 digests, not copied from a
    /// golden file, so this test stands on its own if the harness is ever wrong.
    #[test]
    fn md5_long_order_is_big_endian() {
        // MD5("")    = d41d8cd98f00b204 e9800998ecf8427e
        assert_eq!(
            md5_lo_hi(b""),
            (0xd41d_8cd9_8f00_b204u64 as i64, 0xe980_0998_ecf8_427eu64 as i64)
        );
        // MD5("a")   = 0cc175b9c0f1b6a8 31c399e269772661
        assert_eq!(
            md5_lo_hi(b"a"),
            (0x0cc1_75b9_c0f1_b6a8u64 as i64, 0x31c3_99e2_6977_2661u64 as i64)
        );
        // MD5("abc") = 900150983cd24fb0 d6963f7d28e17f72
        assert_eq!(
            md5_lo_hi(b"abc"),
            (0x9001_5098_3cd2_4fb0u64 as i64, 0xd696_3f7d_28e1_7f72u64 as i64)
        );

        // The little-endian reading is a genuinely different value, so this test
        // would fail loudly if someone "simplifies" the cast back to `from_le_bytes`.
        assert_ne!(md5_lo_hi(b"a").0, 0xa8b6_f1c0_b975_c10cu64 as i64);
    }
}