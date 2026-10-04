//! Parity tests: the Minecraft 26.2 random sources.
//!
//! Golden data: `_porting/test-data/random.txt`, produced by `_porting/java-oracle`
//! running the ORIGINAL, UNMODIFIED `RandomSource.java` and the
//! `net.minecraft.world.level.levelgen` random classes.
//!
//! # These groups are TRANSCRIPTS, not independent cases
//!
//! For a stateful source, a row's expected value depends on every draw before it.
//! A fresh source per row would compare unrelated stream positions and fail for
//! reasons that have nothing to do with the port. So each test below walks the rows
//! IN FILE ORDER and either keeps advancing one source or restarts it when the
//! oracle restarts (i.e. when the key arguments change).
//!
//! # The most dangerous thing in this batch
//!
//! `BitRandomSource#nextDouble` multiplies by a *float* literal, which narrows the
//! 53-bit `long` to 24 bits before scaling. See
//! `legacy_next_double_has_only_24_bits_of_entropy` below. Porting it "correctly"
//! desynchronises every legacy world, and no ordinary test would notice -- the
//! values still look random.

#![allow(non_snake_case)]

use minecraft_rust::javacompat::golden::{self, Golden, Row};
use minecraft_rust::net::minecraft::util::Mth::Mth;
use minecraft_rust::net::minecraft::util::RandomSource::RandomSource;
use minecraft_rust::net::minecraft::world::level::levelgen::BitRandomSource::BitRandomSource;
use minecraft_rust::net::minecraft::world::level::levelgen::LegacyRandomSource::{
    LegacyPositionalRandomFactory, LegacyRandomSource,
};
use minecraft_rust::net::minecraft::world::level::levelgen::PositionalRandomFactory::PositionalRandomFactory;
use minecraft_rust::net::minecraft::world::level::levelgen::RandomSupport::{
    self, Seed128bit,
};
use minecraft_rust::net::minecraft::world::level::levelgen::SingleThreadedRandomSource::SingleThreadedRandomSource;
use minecraft_rust::net::minecraft::world::level::levelgen::ThreadSafeLegacyRandomSource::ThreadSafeLegacyRandomSource;
use minecraft_rust::net::minecraft::world::level::levelgen::WorldgenRandom::{Algorithm, WorldgenRandom};
use minecraft_rust::net::minecraft::world::level::levelgen::Xoroshiro128PlusPlus::Xoroshiro128PlusPlus;
use minecraft_rust::net::minecraft::world::level::levelgen::XoroshiroRandomSource::{
    XoroshiroPositionalRandomFactory, XoroshiroRandomSource,
};

/// How a given implementation is constructed from a seed.
#[derive(Clone, Copy)]
enum Kind {
    Legacy,
    Xoroshiro,
    Single,
    ThreadSafe,
}

impl Kind {
    fn build(self, seed: i64) -> Box<dyn RandomSource> {
        match self {
            Kind::Legacy => Box::new(LegacyRandomSource::new(seed)),
            Kind::Xoroshiro => Box::new(XoroshiroRandomSource::new(seed)),
            Kind::Single => Box::new(SingleThreadedRandomSource::new(seed)),
            Kind::ThreadSafe => Box::new(ThreadSafeLegacyRandomSource::new(seed)),
        }
    }
}

/// The four golden transcripts share their exact shape, so one generic driver
/// covers all of them. `prefix` is the golden group prefix ("legacy", "xoroshiro",
/// "single", "threadsafe").
fn transcript(g: &Golden, kind: Kind, prefix: &str) {
    // The oracle emits ONE source per seed and then a fixed number of draws per
    // group, EXCEPT for `nextIntBound` / `nextIntBetweenInclusive` / the triangles /
    // `setSeed` / `fork`, where the key arguments also change. Restart whenever the
    // seed changes; that matches the oracle for every group except the ones whose
    // oracle loop re-creates the source per key (handled below by key comparison).

    // nextInt(): one source per seed, 24 draws.
    let mut rng: Option<(i64, Box<dyn RandomSource>)> = None;
    for r in g.rows(&format!("{prefix}.nextInt")) {
        let seed = r.arg(0).as_i64();
        let s = restart(&mut rng, seed, kind);
        golden::assert_i32(&format!("{prefix}.nextInt"), r, RandomSource::next_int(&mut *s));
    }

    // nextLong()
    rng = None;
    for r in g.rows(&format!("{prefix}.nextLong")) {
        let seed = r.arg(0).as_i64();
        let s = restart(&mut rng, seed, kind);
        golden::assert_i64(&format!("{prefix}.nextLong"), r, RandomSource::next_long(&mut *s));
    }

    // nextBoolean()
    rng = None;
    for r in g.rows(&format!("{prefix}.nextBoolean")) {
        let seed = r.arg(0).as_i64();
        let s = restart(&mut rng, seed, kind);
        golden::assert_bool(&format!("{prefix}.nextBoolean"), r, RandomSource::next_boolean(s));
    }

    // nextFloat()
    rng = None;
    for r in g.rows(&format!("{prefix}.nextFloat")) {
        let seed = r.arg(0).as_i64();
        let s = restart(&mut rng, seed, kind);
        golden::assert_f32_bits(&format!("{prefix}.nextFloat"), r, RandomSource::next_float(s));
    }

    // nextDouble() -- the float-narrowing path for the BitRandomSource kinds.
    rng = None;
    for r in g.rows(&format!("{prefix}.nextDouble")) {
        let seed = r.arg(0).as_i64();
        let s = restart(&mut rng, seed, kind);
        golden::assert_f64_bits(&format!("{prefix}.nextDouble"), r, RandomSource::next_double(s));
    }

    // nextGaussian() -- exercises the cached spare value.
    rng = None;
    for r in g.rows(&format!("{prefix}.nextGaussian")) {
        let seed = r.arg(0).as_i64();
        let s = restart(&mut rng, seed, kind);
        golden::assert_f64_bits(&format!("{prefix}.nextGaussian"), r, RandomSource::next_gaussian(s));
    }

    // nextInt(bound): the oracle keeps ONE source per seed across ALL bounds, so
    // the key is the seed only -- restarting per bound would desync it.
    rng = None;
    for r in g.rows(&format!("{prefix}.nextIntBound")) {
        let seed = r.arg(0).as_i64();
        let s = restart(&mut rng, seed, kind);
        golden::assert_i32(&format!("{prefix}.nextIntBound"), r, RandomSource::next_int_bounded(&mut *s,r.arg(1).as_i32()));
    }

    // nextIntBetweenInclusive(): one source per seed, six draws per range.
    rng = None;
    for r in g.rows(&format!("{prefix}.nextIntBetweenInclusive")) {
        let seed = r.arg(0).as_i64();
        let s = restart(&mut rng, seed, kind);
        golden::assert_i32(
            &format!("{prefix}.nextIntBetweenInclusive"),
            r,
            s.next_int_between_inclusive(r.arg(1).as_i32(), r.arg(2).as_i32()),
        );
    }

    // triangle(double,double)
    rng = None;
    for r in g.rows(&format!("{prefix}.triangleD")) {
        let seed = r.arg(0).as_i64();
        let s = restart(&mut rng, seed, kind);
        golden::assert_f64_bits(&format!("{prefix}.triangleD"), r, s.triangle_f64(r.arg(1).as_f64(), r.arg(2).as_f64()));
    }

    // triangle(float,float)
    rng = None;
    for r in g.rows(&format!("{prefix}.triangleF")) {
        let seed = r.arg(0).as_i64();
        let s = restart(&mut rng, seed, kind);
        golden::assert_f32_bits(&format!("{prefix}.triangleF"), r, s.triangle_f32(r.arg(1).as_f32(), r.arg(2).as_f32()));
    }

    // consumeCount(rounds): the oracle builds a FRESH source per (seed, rounds).
    each(g, &format!("{prefix}.consumeCount"), |r| {
        let mut s = kind.build(r.arg(0).as_i64());
        RandomSource::consume_count(&mut *s, r.arg(1).as_i32());
        golden::assert_i32(&format!("{prefix}.consumeCount"), r, RandomSource::next_int(&mut *s));
    });

    // setSeed(): fresh source per (seed, newSeed).
    each(g, &format!("{prefix}.setSeed"), |r| {
        let mut s = kind.build(r.arg(0).as_i64());
        RandomSource::set_seed(&mut *s, r.arg(1).as_i64());
        golden::assert_i32(&format!("{prefix}.setSeed"), r, RandomSource::next_int(&mut *s));
    });

    // fork(): fresh source per seed, then three draws from the fork.
    each(g, &format!("{prefix}.fork"), |r| {
        let mut s = kind.build(r.arg(0).as_i64());
        let mut f = RandomSource::fork(&mut *s);
        let a = RandomSource::next_int(&mut *f);
        let b = RandomSource::next_int(&mut *f);
        let c = RandomSource::next_long(&mut *f);
        assert_eq!(
            (a, b, c),
            (r.exp(0).as_i32(), r.exp(1).as_i32(), r.exp(2).as_i64()),
            "{prefix}.fork (golden line {}) seed {}", r.line, r.arg(0).as_i64()
        );
    });

    // nextBits(bits) only exists for the BitRandomSource implementations.
    if matches!(kind, Kind::Legacy | Kind::Single | Kind::ThreadSafe) {
        let mut b: Option<(i64, Box<dyn RandomSource>)> = None;
        for r in g.rows(&format!("{prefix}.nextBits")) {
            let seed = r.arg(0).as_i64();
            let s = restart(&mut b, seed, kind);
            let bits = s.as_bit_source().expect("BitRandomSource kind must expose itself").next(r.arg(1).as_i32());
            golden::assert_i32(&format!("{prefix}.nextBits"), r, bits);
        }
    }
}

/// Returns the source to keep drawing from, restarting it when the seed changes.
///
/// Note the return type: `&mut dyn RandomSource`, obtained by auto-deref'ing the
/// `Box`. Returning the `Box` itself would force `&mut Box<..>` at every call site
/// and make the trait-object indirection noisier than it needs to be.
fn restart<'a>(slot: &'a mut Option<(i64, Box<dyn RandomSource>)>, seed: i64, kind: Kind) -> &'a mut (dyn RandomSource + 'a) {
    let keep = slot.as_ref().map(|(s, _)| *s) == Some(seed);
    if !keep {
        *slot = Some((seed, kind.build(seed)));
    }
    slot.as_mut().unwrap().1.as_mut()
}

fn each(g: &Golden, method: &str, mut f: impl FnMut(&Row)) {
    let rows = g.rows(method);
    assert!(!rows.is_empty(), "golden group `{method}` is empty");
    for row in rows {
        f(row);
    }
}

// ---------------------------------------------------------------------------
// The four transcripts
// ---------------------------------------------------------------------------

#[test]
fn legacy_random_source_transcript() {
    transcript(&Golden::load("random.txt"), Kind::Legacy, "legacy");
}

#[test]
fn xoroshiro_random_source_transcript() {
    transcript(&Golden::load("random.txt"), Kind::Xoroshiro, "xoroshiro");
}

#[test]
fn single_threaded_random_source_transcript() {
    transcript(&Golden::load("random.txt"), Kind::Single, "single");
}

#[test]
fn thread_safe_legacy_random_source_transcript() {
    transcript(&Golden::load("random.txt"), Kind::ThreadSafe, "threadsafe");
}

/// `RandomSource#createThreadLocalInstance(seed)` must equal a `SingleThreadedRandomSource`
/// with that seed -- `Mth#wobble` depends on it, and wobbled block positions are
/// world-stable.
#[test]
fn create_thread_local_instance_matches_single_threaded() {
    let g = Golden::load("random.txt");
    each(&g, "wobble_check", |r| {
        let mut a = minecraft_rust::net::minecraft::util::RandomSource::create_thread_local_instance(r.arg(0).as_i64());
        golden::assert_i32("wobble_check", r, RandomSource::next_int(&mut *a));
    });
}

// ---------------------------------------------------------------------------
// The float-narrowing quirk, pinned explicitly
// ---------------------------------------------------------------------------

/// If this ever "fixes" itself, every legacy world desyncs silently. The values are
/// HotSpot 21's, verified through the oracle.
#[test]
fn legacy_next_double_has_only_24_bits_of_entropy() {
    use minecraft_rust::net::minecraft::world::level::levelgen::BitRandomSource::BitRandomSource;

    let g = Golden::load("random.txt");

    let mut r = LegacyRandomSource::new(0);
    assert_eq!(BitRandomSource::next(&mut r, 26), 0x02ec82d1);
    assert_eq!(BitRandomSource::next(&mut r, 27), 0x006a6ca89);

    let mut r = LegacyRandomSource::new(0);
    let got = BitRandomSource::next_double(&mut r);
    assert_eq!(got.to_bits(), 0x3fe7641680000000, "the float narrowing is mandatory");
    // The "obvious" f64 implementation genuinely differs, so this is not a no-op.
    assert_ne!(got.to_bits(), 0x3fe764168ea6ca89);

    // And the golden transcript agrees.
    let mut r = LegacyRandomSource::new(0);
    golden::assert_f64_bits("legacy.nextDouble", &g.rows("legacy.nextDouble")[0], BitRandomSource::next_double(&mut r));
}

// ---------------------------------------------------------------------------
// Xoroshiro specifics
// ---------------------------------------------------------------------------

#[test]
fn xoroshiro_constructors_and_raw_generator() {
    let g = Golden::load("random.txt");

    each(&g, "xoroshiro.seed128", |r| {
        let mut X = XoroshiroRandomSource::new_from_lo_hi(r.arg(0).as_i64(), r.arg(1).as_i64());
        golden::assert_i64("xoroshiro.seed128", r, X.next_long());
    });

    each(&g, "xoroshiro.seed128bit", |r| {
        let seed = Seed128bit { seed_lo: r.arg(0).as_i64(), seed_hi: r.arg(1).as_i64() };
        let mut X = XoroshiroRandomSource::from_seed(seed);
        golden::assert_i64("xoroshiro.seed128bit", r, X.next_long());
    });

    each(&g, "xoroshiro128pp", |r| {
        let mut gen = Xoroshiro128PlusPlus::new(r.arg(0).as_i64(), r.arg(1).as_i64());
        let got: Vec<i64> = (0..8).map(|_| gen.next_long()).collect();
        let want: Vec<i64> = (0..8).map(|i| r.exp(i).as_i64()).collect();
        assert_eq!(got, want, "xoroshiro128pp (golden line {})", r.line);
    });

    // The all-zero state must be replaced by the golden/silver ratio pair, because
    // xoroshiro's all-zero state is a fixed point that would emit zeros forever.
    if let Some(row) = g.rows("xoroshiro128pp.zero").first() {
        let mut gen = Xoroshiro128PlusPlus::new(0, 0);
        // Check the substituted state BEFORE drawing, since nextLong advances it.
        assert_eq!(gen.seed_lo(), RandomSupport::GOLDEN_RATIO_64, "the all-zero state must be replaced by the golden ratio");
        assert_eq!(gen.seed_hi(), RandomSupport::SILVER_RATIO_64, "...and by the silver ratio");
        golden::assert_i64("xoroshiro128pp.zero", row, gen.next_long());
    }
}

// ---------------------------------------------------------------------------
// RandomSupport
// ---------------------------------------------------------------------------

#[test]
fn random_support() {
    let g = Golden::load("random.txt");

    each(&g, "mixStafford13", |r| golden::assert_i64("mixStafford13", r, RandomSupport::mix_stafford13(r.arg(0).as_i64())));

    each(&g, "upgradeSeedTo128bitUnmixed", |r| {
        let s = RandomSupport::upgrade_seed_to_128bit_unmixed(r.arg(0).as_i64());
        assert_eq!((s.seed_lo, s.seed_hi), (r.exp(0).as_i64(), r.exp(1).as_i64()), "line {}", r.line);
    });

    each(&g, "upgradeSeedTo128bit", |r| {
        let s = RandomSupport::upgrade_seed_to_128bit(r.arg(0).as_i64());
        assert_eq!((s.seed_lo, s.seed_hi), (r.exp(0).as_i64(), r.exp(1).as_i64()), "line {}", r.line);
    });

    // `seedFromHashOf` depends on MD5 over UTF-8 AND on Java's UTF-16-based
    // `String.hashCode` for the legacy path, so this covers two compat modules.
    each(&g, "seedFromHashOf", |r| {
        let name = r.arg(0).as_opt_str().expect("seedFromHashOf has no null names");
        let s = RandomSupport::seed_from_hash_of(name);
        assert_eq!((s.seed_lo, s.seed_hi), (r.exp(0).as_i64(), r.exp(1).as_i64()), "name {name:?}");
    });

    each(&g, "seed128bit_xor", |r| {
        let s = Seed128bit { seed_lo: r.arg(0).as_i64(), seed_hi: r.arg(1).as_i64() }.xor(r.arg(2).as_i64(), r.arg(3).as_i64());
        assert_eq!((s.seed_lo, s.seed_hi), (r.exp(0).as_i64(), r.exp(1).as_i64()), "line {}", r.line);
    });

    each(&g, "seed128bit_mixed", |r| {
        let s = Seed128bit { seed_lo: r.arg(0).as_i64(), seed_hi: r.arg(1).as_i64() }.mixed();
        assert_eq!((s.seed_lo, s.seed_hi), (r.exp(0).as_i64(), r.exp(1).as_i64()), "line {}", r.line);
    });
}

// ---------------------------------------------------------------------------
// Positional factories -- the "same seed, same world" machinery
// ---------------------------------------------------------------------------

#[test]
fn legacy_positional_random_factory() {
    let g = Golden::load("random.txt");
    positional(&g, "legacyPos", |seed| Box::new(LegacyPositionalRandomFactory::new(seed)) as Box<dyn PositionalRandomFactory>);
}

#[test]
fn xoroshiro_positional_random_factory() {
    let g = Golden::load("random.txt");
    // The oracle builds the xoroshiro factory with `seed` and `seed ^ 0x9E37...`.
    positional(&g, "xoroshiroPos", |seed| {
        Box::new(XoroshiroPositionalRandomFactory::new(seed, seed ^ 0x9E37_79B9_7F4A_7C15u64 as i64)) as Box<dyn PositionalRandomFactory>
    });
}

fn positional(g: &Golden, prefix: &str, make: impl Fn(i64) -> Box<dyn PositionalRandomFactory>) {
    each(g, &format!("{prefix}.at"), |r| {
        let f = make(r.arg(0).as_i64());
        let mut s = (*f).at(r.arg(1).as_i32(), r.arg(2).as_i32(), r.arg(3).as_i32());
        let a = RandomSource::next_int(&mut *s);
        let b = RandomSource::next_int(&mut *s);
        let c = RandomSource::next_long(&mut *s);
        assert_eq!((a, b, c), (r.exp(0).as_i32(), r.exp(1).as_i32(), r.exp(2).as_i64()), "line {}", r.line);
    });

    each(g, &format!("{prefix}.fromHashOf"), |r| {
        let f = make(r.arg(0).as_i64());
        let name = r.arg(1).as_opt_str().expect("no null names here");
        let mut s = f.from_hash_of(name);
        let a = RandomSource::next_int(&mut *s);
        let b = RandomSource::next_int(&mut *s);
        let c = RandomSource::next_long(&mut *s);
        assert_eq!((a, b, c), (r.exp(0).as_i32(), r.exp(1).as_i32(), r.exp(2).as_i64()), "name {name:?}");
    });

    each(g, &format!("{prefix}.fromSeed"), |r| {
        let f = make(r.arg(0).as_i64());
        let mut s = f.from_seed(r.arg(1).as_i64());
        let a = RandomSource::next_int(&mut *s);
        let b = RandomSource::next_int(&mut *s);
        let c = RandomSource::next_long(&mut *s);
        assert_eq!((a, b, c), (r.exp(0).as_i32(), r.exp(1).as_i32(), r.exp(2).as_i64()), "line {}", r.line);
    });

    // `parityConfigString` is what vanilla's own config tests compare.
    each(g, &format!("{prefix}.parityConfigString"), |r| {
        golden::assert_str(&format!("{prefix}.parityConfigString"), r, &make(r.arg(0).as_i64()).parity_config_string());
    });
}

/// `forkPositional()` draws two longs from the SOURCE before wrapping it in a
/// factory, so the source's own stream must be consumed in that order.
#[test]
fn fork_positional_consumes_from_the_source_first() {
    let g = Golden::load("random.txt");
    for variant in 0..4 {
        let kind = match variant {
            0 => Kind::Legacy,
            1 => Kind::Xoroshiro,
            2 => Kind::Single,
            _ => Kind::ThreadSafe,
        };
        let mut r: Option<(i64, Box<dyn RandomSource>)> = None;
        for row in g.rows("forkPositional") {
            let seed = row.arg(0).as_i64();
            let v = row.arg(1).as_i32();
            if v != variant {
                continue;
            }
            let s = restart(&mut r, seed, kind);
            let mut at = RandomSource::fork_positional(s).at(3, -4, 5);
            let a = at.next_int();
            let c = at.next_long();
            assert_eq!((a, c), (row.exp(0).as_i32(), row.exp(1).as_i64()), "variant {variant} line {}", row.line);
        }
    }
}

// ---------------------------------------------------------------------------
// WorldgenRandom
// ---------------------------------------------------------------------------

#[test]
fn worldgen_random_seeding() {
    let g = Golden::load("random.txt");

    each(&g, "worldgen.setDecorationSeed", |r| {
        let mut w = WorldgenRandom::new(Kind::Legacy.build(r.arg(0).as_i64()));
        let seed = r.arg(0).as_i64();
        let got = w.set_decoration_seed(seed, r.arg(1).as_i32(), r.arg(2).as_i32());
        golden::assert_i64("worldgen.setDecorationSeed", r, got);
    });

    each(&g, "worldgen.afterDecoration", |r| {
        use minecraft_rust::net::minecraft::world::level::levelgen::BitRandomSource::BitRandomSource;
        let seed = r.arg(0).as_i64();
        let mut w = WorldgenRandom::new(Kind::Legacy.build(seed));
        w.set_decoration_seed(seed, r.arg(1).as_i32(), r.arg(2).as_i32());
        let a = BitRandomSource::next_int(&mut w);
        let b = BitRandomSource::next_int(&mut w);
        let count = w.get_count();
        assert_eq!(
            (a, b, count as i64),
            (r.exp(0).as_i32(), r.exp(1).as_i32(), r.exp(2).as_i64()),
            "expected (nextInt, nextInt, getCount); the call counter must advance \
             exactly once per next(bits) call"
        );
    });

    each(&g, "worldgen.setFeatureSeed", |r| {
        use minecraft_rust::net::minecraft::world::level::levelgen::BitRandomSource::BitRandomSource;
        let seed = r.arg(0).as_i64();
        let mut w = WorldgenRandom::new(Kind::Legacy.build(seed));
        w.set_feature_seed(seed, 3, 7);
        golden::assert_i32("worldgen.setFeatureSeed", r, BitRandomSource::next_int(&mut w));
    });

    each(&g, "worldgen.setLargeFeatureSeed", |r| {
        use minecraft_rust::net::minecraft::world::level::levelgen::BitRandomSource::BitRandomSource;
        let seed = r.arg(0).as_i64();
        let mut w = WorldgenRandom::new(Kind::Legacy.build(seed));
        w.set_large_feature_seed(seed, r.arg(1).as_i32(), r.arg(2).as_i32());
        golden::assert_i32("worldgen.setLargeFeatureSeed", r, BitRandomSource::next_int(&mut w));
    });

    each(&g, "worldgen.setLargeFeatureWithSalt", |r| {
        use minecraft_rust::net::minecraft::world::level::levelgen::BitRandomSource::BitRandomSource;
        let seed = r.arg(0).as_i64();
        let mut w = WorldgenRandom::new(Kind::Legacy.build(seed));
        w.set_large_feature_with_salt(seed, r.arg(1).as_i32(), r.arg(2).as_i32(), r.arg(3).as_i32());
        golden::assert_i32("worldgen.setLargeFeatureWithSalt", r, BitRandomSource::next_int(&mut w));
    });

    // `next(bits)` dispatches on whether the wrapped source is a BitRandomSource.
    // Both branches are covered: legacy -> `legacy.next(bits)` (this group), and
    // xoroshiro -> `(int)(nextLong() >>> 64 - bits)` (`overXoroshiro`, below).
    //
    // The oracle builds ONE source per seed and loops bits inside it, so the
    // WorldgenRandom has to borrow-and-return the same stream. `take_..` swaps a
    // placeholder in so the wrapper can be constructed around the live source.
    let mut w: Option<(i64, Box<dyn RandomSource>)> = None;
    for r in g.rows("worldgen.nextBits") {
        let seed = r.arg(0).as_i64();
        let keep = w.as_ref().map(|(s, _)| *s) == Some(seed);
        if !keep {
            w = Some((seed, Kind::Legacy.build(seed)));
        }
        let mut source = w.take().unwrap().1;
        let mut rng = WorldgenRandom::new(source);
        let got = rng.next(r.arg(1).as_i32());
        source = rng.take_random_source_for_test();
        w = Some((seed, source));
        golden::assert_i32("worldgen.nextBits", r, got);
    }

    // One XoroshiroRandomSource per SEED, then all five `bits` groups are drawn from
    // that same stream -- the oracle's loop is `for bits { for k { r.next(bits) } }`
    // around a single `WorldgenRandom`, so the stream carries across groups.
    let mut over: Option<(i64, Box<dyn RandomSource>)> = None;
    for r in g.rows("worldgen.overXoroshiro") {
        let seed = r.arg(0).as_i64();
        let keep = over.as_ref().map(|(s, _)| *s) == Some(seed);
        if !keep {
            over = Some((seed, Box::new(XoroshiroRandomSource::new(seed))));
        }
        let mut source = over.take().unwrap().1;
        let mut w = WorldgenRandom::new(source);
        let got = BitRandomSource::next(&mut w, r.arg(1).as_i32());
        over = Some((seed, w.take_random_source_for_test()));
        golden::assert_i32("worldgen.overXoroshiro", r, got);
    }

    each(&g, "worldgen.seedSlimeChunk", |r| {
        let mut s: Box<dyn RandomSource> = WorldgenRandom::seed_slime_chunk(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i64(), r.arg(3).as_i64());
        golden::assert_i32("worldgen.seedSlimeChunk", r, RandomSource::next_int(&mut *s));
    });

    each(&g, "worldgen.algorithm", |r| {
        let v = r.arg(1).as_i32();
        let alg = Algorithm::VALUES[v as usize];
        assert_eq!(alg.ordinal(), v, "Algorithm ordinal must match the declaration order");
        let mut s: Box<dyn RandomSource> = alg.new_instance(r.arg(0).as_i64());
        golden::assert_i32("worldgen.algorithm", r, RandomSource::next_int(&mut *s));
    });
}

// ---------------------------------------------------------------------------
// LinearCongruentialGenerator + Mth#getSeed, the two remaining helpers
// ---------------------------------------------------------------------------

#[test]
fn linear_congruential_generator() {
    let g = Golden::load("random.txt");
    each(&g, "lcg.next", |r| {
        golden::assert_i64(
            "lcg.next",
            r,
            minecraft_rust::net::minecraft::util::LinearCongruentialGenerator::next(r.arg(0).as_i64(), r.arg(1).as_i64()),
        );
    });
}

#[test]
fn get_seed() {
    let g = Golden::load("random.txt");
    each(&g, "mth.getSeed", |r| {
        golden::assert_i64("mth.getSeed", r, Mth::get_seed(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32()));
    });
}

/// BitRandomSource's power-of-two fast path and the bounded loop, spelled out.
#[test]
fn bit_random_source_bounded_paths() {
    let g = Golden::load("random.txt");
    let mut r: Option<(i64, LegacyRandomSource)> = None;
    for row in g.rows("bit.nextIntPow2") {
        let seed = row.arg(0).as_i64();
        let keep = r.as_ref().map(|(s, _)| *s) == Some(seed);
        let s = if keep {
            &mut r.as_mut().unwrap().1
        } else {
            r = Some((seed, LegacyRandomSource::new(seed)));
            &mut r.as_mut().unwrap().1
        };
        golden::assert_i32("bit.nextIntPow2", row, RandomSource::next_int_bounded(s, row.arg(1).as_i32()));
    }

    each(&g, "bit.seed", |r| {
        let mut s = LegacyRandomSource::new(r.arg(0).as_i64());
        golden::assert_i64("bit.seed", r, RandomSource::next_long(&mut s));
    });
}