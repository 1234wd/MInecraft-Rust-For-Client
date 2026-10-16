//! Session 19 batch f: pure-logic utilities and bit storage. Checks ports against the jar's constant dump and the
//! Java source's behaviour. See the file headers of the ported .rs files for status.
//!
//! Where a test pins a literal (checksums, exception texts, formatted strings) the value was PRINTED BY THE
//! 26.2 JAR on JDK 25.0.4 (a throwaway probe calling the real classes), not derived by hand. Checksums fold a
//! `long[]` as `h = h * 31 + x` starting from 7 (Java `long` arithmetic), exactly as the probe did.

#[path = "support/consts_dump.rs"]
mod consts_dump;
#[allow(unused_imports)]
use consts_dump::*;

use std::panic::{catch_unwind, AssertUnwindSafe};

/// Runs `f`, expects a panic, returns its message.
fn panic_msg<R>(f: impl FnOnce() -> R) -> String {
    let r = catch_unwind(AssertUnwindSafe(f));
    match r {
        Ok(_) => panic!("expected a panic, got none"),
        Err(e) => {
            if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "<non-string panic>".to_string()
            }
        }
    }
}

/// `h = h * 31 + x` from 7, Java long arithmetic.
fn fold_longs(xs: &[i64]) -> i64 {
    xs.iter().fold(7i64, |h, &x| h.wrapping_mul(31).wrapping_add(x))
}

/// The probe's value generator: `(int)((i * 2654435761L + 12345L) & mask & 0x7FFFFFFFL)`.
fn probe_val(i: i32, mask: i64) -> i32 {
    ((i as i64).wrapping_mul(2654435761).wrapping_add(12345) & mask & 0x7FFF_FFFF) as i32
}

// =====================================================================================================
// BitStorage / SimpleBitStorage / ZeroBitStorage / datafix.PackedBitStorage
// =====================================================================================================

mod bit_storage {
    use super::*;
    use minecraft_rust::net::minecraft::util::BitStorage::BitStorage;
    use minecraft_rust::net::minecraft::util::SimpleBitStorage::{SimpleBitStorage, MAGIC};
    use minecraft_rust::net::minecraft::util::ZeroBitStorage::{ZeroBitStorage, RAW};
    use minecraft_rust::net::minecraft::util::datafix::PackedBitStorage::PackedBitStorage;

    /// Per bits value: (bits, raw length, fold(from int[] ctor), fold(set one by one), fold of getAndSet
    /// results, fold after the getAndSet pass). Printed by the jar for size 77.
    const SIMPLE_JAR: [(i32, usize, i64, i64, i64, i64); 32] = [
        (1, 2, 6148914691236529383, 6148914691236529383, 7482814974868461253, 6148914691236529383),
        (2, 3, -4774451407292324686, -4774451407292324686, 933657865674815945, 4610486750196403252),
        (3, 4, 8371348084274974297, 8371348084274974297, 2755920187700600781, 8092670519076992733),
        (4, 5, -4015536500279484990, -4015536500279484990, -8883738162058954307, 8006836844240375996),
        (5, 7, -117812113250568458, -117812113250568458, 8489603223545632733, -4631962346074527904),
        (6, 8, 440171343372425071, 440171343372425071, 8430797318886814685, -2999736259584128397),
        (7, 9, -787467145247898122, -787467145247898122, -6654436415112201315, -8978733701063259828),
        (8, 10, 3627316295666442383, 3627316295666442383, -2447132924114588643, 349589464742559007),
        (9, 11, -4887384909548130363, -4887384909548130363, 6574441441291496221, -4663620437829335023),
        (10, 13, 233656567245982262, 233656567245982262, 5531915283938895645, -8330954537324156966),
        (11, 16, 6942973040406421167, 6942973040406421167, 8381791104842168093, -7196880561092108573),
        (12, 16, 3956249623558299311, 3956249623558299311, 1372681534658019101, 8856746488287471331),
        (13, 20, -1370883908627240593, -1370883908627240593, -5080037528226656483, 800138612758467581),
        (14, 20, -4293385513638581905, -4293385513638581905, -6709573725166620899, 4388480477018444797),
        (15, 20, 6868586662031010159, 6868586662031010159, -1621903868829438179, -7349177079598525443),
        (16, 20, 2417090848066972015, 2417090848066972015, 3257925528614268701, 2029143935512433661),
        (17, 26, -8339036705553607714, -8339036705553607714, 4683628727861112605, 4433379804582179450),
        (18, 26, 5645972908388553694, 5645972908388553694, 8920223482424192797, 7475742582697292410),
        (19, 26, 6053919006213444574, 6053919006213444574, -2246783951816757475, 4687675044002198138),
        (20, 26, 5571886185808040926, 5571886185808040926, -1534348447367172323, 7579077967347118714),
        (21, 26, 2430424207732937694, 2430424207732937694, -4029883761713615075, 3207572318138593914),
        (22, 39, -3189796741684427784, -3189796741684427784, -8233583548201570531, 1016517526712070428),
        (23, 39, 5311698399910921208, 5311698399910921208, 3556090776296484637, 4044029687807193372),
        (24, 39, -1122094293780823048, -1122094293780823048, -5832003935838015715, -4075740671277682404),
        (25, 39, -7219495580820339720, -7219495580820339720, -1215661775040418019, 2736693418886541596),
        (26, 39, 7303718471407469560, 7303718471407469560, 696422305746775837, -9199681512312557284),
        (27, 39, -4924586681766280200, -4924586681766280200, -6640016945251750115, 311527811127030044),
        (28, 39, -636674158118075400, -636674158118075400, -3297952641527451875, 3341114609814843676),
        (29, 39, -732776483928177672, -732776483928177672, 3397817716637298461, 2600397243938877724),
        (30, 39, 8306316182193144824, 8306316182193144824, -2843962032339685603, 6492486343675982108),
        (31, 39, 18664784731014136, 18664784731014136, -2038951434041299171, -5638507476230754020),
        (32, 39, -5728985670547700744, -5728985670547700744, -2038951434041299171, -8456074780538195684),
    ];

    /// Per bits value: (bits, raw length, fold after set, fold of get results, fold after the overwrite pass).
    const PACKED_JAR: [(i32, usize, i64, i64, i64); 32] = [
        (1, 2, 6148914691236529383, 8084141759252400871, 6148914691236529383),
        (2, 3, -4774451407292324686, -2998334161824350297, 4610486750196403252),
        (3, 4, -8485735098864470034, 8026865222006647079, -8756164129842569044),
        (4, 5, -4015536500279484990, 552160958030262319, 8006836844240375996),
        (5, 7, -1687794205536004400, 6243572702859418175, 7266337632264785850),
        (6, 8, -8036959738449786982, -461834708117678529, -8803066857068133608),
        (7, 9, -4960942480536122822, -7010388787490857345, 4055123187596259108),
        (8, 10, 3627316295666442383, -7875112249584710017, 349589464742559007),
        (9, 11, -3758649491580088684, 5886090118068506239, -4366346073234014386),
        (10, 13, 4790757138841304505, 2577982183350425215, -9108801545481927670),
        (11, 14, -3927792769647836219, -8104434836442196353, 1163525724020749867),
        (12, 15, 9181958625064517133, 8470876663911145087, 2431378034471794271),
        (13, 16, 1571970639939921074, -1945858570568339841, -7988986397407529238),
        (14, 17, -8066767883828448276, -7277616123246293377, 9095636149666991382),
        (15, 19, 4573405763795116523, 7888547170574929535, -3767081439213373640),
        (16, 20, 2417090848066972015, 580303897181621887, 2029143935512433661),
        (17, 21, 5162962107116013481, 7383018441087258239, -486032501570420743),
        (18, 22, 8514291753793800047, -6363960145853542785, -4745919987667603415),
        (19, 23, -2572620295384171319, -1083965048870400385, 4567328728708204786),
        (20, 25, -569658109578502741, 3650450505733508735, 2386281156283489369),
        (21, 26, -8005244510275853738, 8543290253759483519, 5184850015480755162),
        (22, 27, -2441047313104403686, 1522154561784254079, -1280802975502822221),
        (23, 28, -5529569921333106290, 9063243512239361663, 2441091275781016988),
        (24, 29, -2002859708149164448, 2229482724914368127, 2314911749219658928),
        (25, 31, 8588596100351257370, 2325848502424571519, -4816738150997525535),
        (26, 32, -3301906574629313186, -1588677073815990657, -8948215173958049616),
        (27, 33, -4233628122633711867, -39321487116333441, -6766181490694757975),
        (28, 34, -5382006021135548507, 3480229965132467839, -6067211957669091687),
        (29, 35, 4156637830856518206, 636261966081691263, 4770270055527534992),
        (30, 37, -4094995720074207000, -4393759363460493697, -2147986224717109104),
        (31, 38, -5592968415600258282, 3244950988790173311, 4159649255269218889),
        (32, 39, -5728985670547700744, 3244950988790173311, -8456074780538195684),
    ];

    #[test]
    fn magic_table_matches_the_jar_by_reflection() {
        // Probe: `SimpleBitStorage.MAGIC` read by reflection, folded as ints.
        assert_eq!(MAGIC.len(), 192);
        let h = MAGIC.iter().fold(7i64, |h, &x| h.wrapping_mul(31).wrapping_add(x as i64));
        assert_eq!(h, -7060619042545238026);
    }

    #[test]
    fn magic_cell_index_is_true_division_for_every_values_per_long() {
        for bits in 1..=32 {
            let s = SimpleBitStorage::new(bits, 1);
            let vpl = 64 / bits;
            for i in (0..200_000).chain([i32::MAX / 2, i32::MAX - 1, i32::MAX]) {
                assert_eq!(s.cell_index(i), i / vpl, "bits {bits} index {i}");
            }
        }
    }

    #[test]
    fn simple_bit_storage_packs_like_the_jar_for_every_bit_width() {
        let size = 77;
        for &(bits, len, h, h2, g, h3) in SIMPLE_JAR.iter() {
            let mask = (1i64 << bits) - 1;
            let values: Vec<i32> = (0..size).map(|i| probe_val(i, mask)).collect();
            let s = SimpleBitStorage::from_values(bits, size, &values);
            assert_eq!(s.get_raw().len(), len, "bits {bits}");
            assert_eq!(fold_longs(s.get_raw()), h, "int[] ctor, bits {bits}");

            let mut s2 = SimpleBitStorage::new(bits, size);
            for i in 0..size {
                s2.set(i, values[i as usize]);
            }
            assert_eq!(fold_longs(s2.get_raw()), h2, "set(), bits {bits}");

            let mut gacc = 0i64;
            for i in 0..size {
                let old = s2.get_and_set((i * 7) % size, probe_val(i + 1000, mask));
                gacc = gacc.wrapping_mul(31).wrapping_add(old as i64);
            }
            assert_eq!(gacc, g, "getAndSet results, bits {bits}");
            assert_eq!(fold_longs(s2.get_raw()), h3, "after getAndSet, bits {bits}");

            // get / unpack / getAll all agree with what was written.
            let mut unpacked = vec![0; size as usize];
            s.unpack(&mut unpacked);
            assert_eq!(unpacked, values, "unpack, bits {bits}");
            let mut all = Vec::new();
            s.get_all(&mut |v| all.push(v));
            assert_eq!(all, values, "getAll, bits {bits}");
            for i in 0..size {
                assert_eq!(s.get(i), values[i as usize]);
            }
            assert_eq!(s.get_bits(), bits);
            assert_eq!(s.get_size(), size);
        }
    }

    #[test]
    fn simple_bit_storage_small_cases_match_the_jar() {
        let small = SimpleBitStorage::from_values(5, 13, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 31]);
        assert_eq!(small.get_raw(), &[445092485129178177i64, 31]);

        // 7 bits -> 9 values per long, 1 spare bit per word that unpack must ignore.
        let s7 = SimpleBitStorage::with_data(7, 20, Some(vec![-1, -1, 0x0123456789abcdef]));
        let mut out = [0; 20];
        s7.unpack(&mut out);
        let expect = [127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 111, 27];
        assert_eq!(out, expect);
        let mut all = Vec::new();
        s7.get_all(&mut |v| all.push(v));
        assert_eq!(all, expect);

        let mut s32 = SimpleBitStorage::new(32, 3);
        s32.set(1, i32::MAX);
        assert_eq!(s32.get_raw(), &[9223372032559808512i64, 0]);
        assert_eq!(s32.get(1), i32::MAX);

        let copy = s7.copy();
        assert_eq!(copy.get_raw(), s7.get_raw());
        assert_eq!(copy.get_bits(), 7);
    }

    #[test]
    fn simple_bit_storage_exceptions_match_the_jar() {
        assert_eq!(
            panic_msg(|| SimpleBitStorage::new(0, 10)),
            "java.lang.IllegalArgumentException: The value 0 is not in the specified inclusive range of 1 to 32"
        );
        assert_eq!(
            panic_msg(|| SimpleBitStorage::new(33, 10)),
            "java.lang.IllegalArgumentException: The value 33 is not in the specified inclusive range of 1 to 32"
        );
        assert_eq!(
            panic_msg(|| SimpleBitStorage::with_data(5, 13, Some(vec![0; 3]))),
            "net.minecraft.util.SimpleBitStorage$InitializationException: Invalid length given for storage, got: 3 but expected: 2"
        );
        let err = SimpleBitStorage::try_with_data(5, 13, Some(vec![0; 3])).unwrap_err();
        assert_eq!(err.get_message(), "Invalid length given for storage, got: 3 but expected: 2");
        assert_eq!(
            panic_msg(|| SimpleBitStorage::new(5, 13).get(-1)),
            "java.lang.IllegalArgumentException: The value -1 is not in the specified inclusive range of 0 to 12"
        );
        assert_eq!(
            panic_msg(|| SimpleBitStorage::new(5, 13).get(13)),
            "java.lang.IllegalArgumentException: The value 13 is not in the specified inclusive range of 0 to 12"
        );
        assert_eq!(
            panic_msg(|| SimpleBitStorage::new(5, 13).set(0, 32)),
            "java.lang.IllegalArgumentException: The value 32 is not in the specified inclusive range of 0 to 31"
        );
        assert_eq!(
            panic_msg(|| SimpleBitStorage::new(5, 13).set(0, -1)),
            "java.lang.IllegalArgumentException: The value -1 is not in the specified inclusive range of 0 to 31"
        );
        assert_eq!(
            panic_msg(|| SimpleBitStorage::new(32, 13).set(0, -1)),
            "java.lang.IllegalArgumentException: The value -1 is not in the specified inclusive range of 0 to 4294967295"
        );
        assert_eq!(panic_msg(|| SimpleBitStorage::new(5, -100)), "java.lang.NegativeArraySizeException: -7");
    }

    #[test]
    fn zero_bit_storage_matches_the_jar() {
        let mut z = ZeroBitStorage::new(4);
        assert_eq!(z.get(3), 0);
        assert_eq!(z.get_and_set(2, 0), 0);
        assert_eq!(z.get_bits(), 0);
        assert_eq!(z.get_size(), 4);
        assert!(z.get_raw().is_empty());
        assert!(RAW.is_empty());
        assert_eq!(
            panic_msg(|| ZeroBitStorage::new(4).set(0, 1)),
            "java.lang.IllegalArgumentException: The value 1 is not in the specified inclusive range of 0 to 0"
        );
        assert_eq!(
            panic_msg(|| ZeroBitStorage::new(4).get(4)),
            "java.lang.IllegalArgumentException: The value 4 is not in the specified inclusive range of 0 to 3"
        );
        let mut out = [9, 9, 9, 9];
        ZeroBitStorage::new(2).unpack(&mut out);
        assert_eq!(out, [0, 0, 9, 9]);
        assert_eq!(
            panic_msg(|| ZeroBitStorage::new(5).unpack(&mut [0; 3])),
            "java.lang.ArrayIndexOutOfBoundsException: Array index out of range: 5"
        );
        assert_eq!(
            panic_msg(|| ZeroBitStorage::new(-2).unpack(&mut [0; 3])),
            "java.lang.IllegalArgumentException: fromIndex(0) > toIndex(-2)"
        );
        let mut all = Vec::new();
        ZeroBitStorage::new(3).get_all(&mut |v| all.push(v));
        assert_eq!(all, [0, 0, 0]);
        let c = z.copy();
        assert_eq!(c.get_size(), 4);
        z.set(0, 0);
    }

    #[test]
    fn packed_bit_storage_matches_the_jar_for_every_bit_width() {
        let size = 77;
        for &(bits, len, h, g, h3) in PACKED_JAR.iter() {
            let mask = (1i64 << bits) - 1;
            let mut p = PackedBitStorage::new(bits, size);
            assert_eq!(p.get_raw().len(), len, "bits {bits}");
            for i in 0..size {
                p.set(i, probe_val(i, mask));
            }
            assert_eq!(fold_longs(p.get_raw()), h, "after set, bits {bits}");
            let mut gacc = 0i64;
            for i in 0..size {
                let v = p.get(i);
                assert_eq!(v, probe_val(i, mask), "get({i}), bits {bits}");
                gacc = gacc.wrapping_mul(31).wrapping_add(v as i64);
            }
            assert_eq!(gacc, g, "bits {bits}");
            for i in 0..size {
                p.set((i * 7) % size, probe_val(i + 1000, mask));
            }
            assert_eq!(fold_longs(p.get_raw()), h3, "after overwrite, bits {bits}");
            assert_eq!(p.get_bits(), bits);
        }
    }

    #[test]
    fn packed_bit_storage_exceptions_match_the_jar() {
        assert_eq!(
            panic_msg(|| PackedBitStorage::with_data(5, 13, vec![0; 3])),
            "java.lang.IllegalArgumentException: Invalid length given for storage, got: 3 but expected: 2"
        );
        assert_eq!(
            panic_msg(|| PackedBitStorage::new(0, 13)),
            "java.lang.IllegalArgumentException: The value 0 is not in the specified inclusive range of 1 to 32"
        );
        assert_eq!(
            panic_msg(|| PackedBitStorage::new(5, 13).set(3, 99)),
            "java.lang.IllegalArgumentException: The value 99 is not in the specified inclusive range of 0 to 31"
        );
    }
}
