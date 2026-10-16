//! Session 19 batch h: client utilities and model/layout value types. Checks ports against the jar's
//! constant dump and the Java source's behaviour.

#[path = "support/consts_dump.rs"]
mod consts_dump;
#[allow(unused_imports)]
use consts_dump::*;

/// The panic message of `f`, which must panic with a `String` or `&str` payload.
#[allow(dead_code)]
fn panic_text(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let err = std::panic::catch_unwind(f).expect_err("expected a panic");
    if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        panic!("panic payload is not a string")
    }
}

// ============================================================================
// PartNames
// ============================================================================

#[test]
fn part_names_match_the_jar() {
    use minecraft_rust::net::minecraft::client::model::geom::PartNames as P;
    assert_all_consts(
        "net.minecraft.client.model.geom.PartNames",
        &[
            ("LEFT_FIN", strv(P::LEFT_FIN)),
            ("RIGHT_FIN", strv(P::RIGHT_FIN)),
            ("TOP_FIN", strv(P::TOP_FIN)),
            ("BOTTOM_FIN", strv(P::BOTTOM_FIN)),
            ("TAIL_FIN", strv(P::TAIL_FIN)),
            ("LEFT_BLUE_FIN", strv(P::LEFT_BLUE_FIN)),
            ("RIGHT_BLUE_FIN", strv(P::RIGHT_BLUE_FIN)),
            ("LEFT_ARM", strv(P::LEFT_ARM)),
            ("RIGHT_ARM", strv(P::RIGHT_ARM)),
            ("LEFT_WING", strv(P::LEFT_WING)),
            ("RIGHT_WING", strv(P::RIGHT_WING)),
            ("LEFT_WING_BASE", strv(P::LEFT_WING_BASE)),
            ("RIGHT_WING_BASE", strv(P::RIGHT_WING_BASE)),
            ("LEFT_WING_TIP", strv(P::LEFT_WING_TIP)),
            ("RIGHT_WING_TIP", strv(P::RIGHT_WING_TIP)),
            ("LEFT_EAR", strv(P::LEFT_EAR)),
            ("RIGHT_EAR", strv(P::RIGHT_EAR)),
            ("LEFT_LEG", strv(P::LEFT_LEG)),
            ("RIGHT_LEG", strv(P::RIGHT_LEG)),
            ("LEFT_HIND_LEG", strv(P::LEFT_HIND_LEG)),
            ("RIGHT_HIND_LEG", strv(P::RIGHT_HIND_LEG)),
            ("LEFT_MID_LEG", strv(P::LEFT_MID_LEG)),
            ("RIGHT_MID_LEG", strv(P::RIGHT_MID_LEG)),
            ("LEFT_FRONT_LEG", strv(P::LEFT_FRONT_LEG)),
            ("RIGHT_FRONT_LEG", strv(P::RIGHT_FRONT_LEG)),
            ("LEFT_HIND_FOOT", strv(P::LEFT_HIND_FOOT)),
            ("RIGHT_HIND_FOOT", strv(P::RIGHT_HIND_FOOT)),
            ("LEFT_FRONT_FOOT", strv(P::LEFT_FRONT_FOOT)),
            ("RIGHT_FRONT_FOOT", strv(P::RIGHT_FRONT_FOOT)),
            ("FEET", strv(P::FEET)),
            ("LEFT_HIND_LEG_TIP", strv(P::LEFT_HIND_LEG_TIP)),
            ("RIGHT_HIND_LEG_TIP", strv(P::RIGHT_HIND_LEG_TIP)),
            ("LEFT_FRONT_LEG_TIP", strv(P::LEFT_FRONT_LEG_TIP)),
            ("RIGHT_FRONT_LEG_TIP", strv(P::RIGHT_FRONT_LEG_TIP)),
            ("LEFT_LID", strv(P::LEFT_LID)),
            ("RIGHT_LID", strv(P::RIGHT_LID)),
            ("LEFT_CHEST", strv(P::LEFT_CHEST)),
            ("RIGHT_CHEST", strv(P::RIGHT_CHEST)),
            ("LEFT_HORN", strv(P::LEFT_HORN)),
            ("RIGHT_HORN", strv(P::RIGHT_HORN)),
            ("LEFT_EYE", strv(P::LEFT_EYE)),
            ("RIGHT_EYE", strv(P::RIGHT_EYE)),
            ("JAW", strv(P::JAW)),
            ("NOSE", strv(P::NOSE)),
            ("ARMS", strv(P::ARMS)),
            ("TAIL", strv(P::TAIL)),
            ("CUBE", strv(P::CUBE)),
            ("BEAK", strv(P::BEAK)),
            ("BACK_FIN", strv(P::BACK_FIN)),
            ("MANE", strv(P::MANE)),
            ("NECK", strv(P::NECK)),
            ("MOUTH", strv(P::MOUTH)),
            ("UPPER_MOUTH", strv(P::UPPER_MOUTH)),
            ("INNER_MOUTH", strv(P::INNER_MOUTH)),
            ("LOWER_MOUTH", strv(P::LOWER_MOUTH)),
            ("HEAD", strv(P::HEAD)),
            ("HAT", strv(P::HAT)),
            ("BODY", strv(P::BODY)),
            ("UPPER_BODY", strv(P::UPPER_BODY)),
            ("INNER_BODY", strv(P::INNER_BODY)),
            ("WAIST", strv(P::WAIST)),
            ("HAT_RIM", strv(P::HAT_RIM)),
            ("GOGGLES", strv(P::GOGGLES)),
            ("JACKET", strv(P::JACKET)),
            ("HARNESS", strv(P::HARNESS)),
            ("SHELL", strv(P::SHELL)),
            ("TOP_GILLS", strv(P::TOP_GILLS)),
            ("LEFT_GILLS", strv(P::LEFT_GILLS)),
            ("RIGHT_GILLS", strv(P::RIGHT_GILLS)),
            ("ROOT", strv(P::ROOT)),
            ("CROAKING_BODY", strv(P::CROAKING_BODY)),
            ("TONGUE", strv(P::TONGUE)),
            ("TONGUE_R1", strv(P::TONGUE_R1)),
            ("LEFT_HAND", strv(P::LEFT_HAND)),
            ("RIGHT_HAND", strv(P::RIGHT_HAND)),
            ("LEFT_FOOT", strv(P::LEFT_FOOT)),
            ("RIGHT_FOOT", strv(P::RIGHT_FOOT)),
            ("EYES", strv(P::EYES)),
            ("RIGHT_TENDRIL", strv(P::RIGHT_TENDRIL)),
            ("LEFT_TENDRIL", strv(P::LEFT_TENDRIL)),
            ("RIGHT_RIBCAGE", strv(P::RIGHT_RIBCAGE)),
            ("LEFT_RIBCAGE", strv(P::LEFT_RIBCAGE)),
            ("WIND_BODY", strv(P::WIND_BODY)),
            ("WIND_TOP", strv(P::WIND_TOP)),
            ("WIND_MID", strv(P::WIND_MID)),
            ("WIND_BOTTOM", strv(P::WIND_BOTTOM)),
            ("RODS", strv(P::RODS)),
            ("MUSHROOMS", strv(P::MUSHROOMS)),
            ("BONE", strv(P::BONE)),
            ("LEFT_PADDLE", strv(P::LEFT_PADDLE)),
            ("RIGHT_PADDLE", strv(P::RIGHT_PADDLE)),
            ("BOTTOM", strv(P::BOTTOM)),
            ("CHEST_BOTTOM", strv(P::CHEST_BOTTOM)),
            ("CHEST_LID", strv(P::CHEST_LID)),
            ("CHEST_LOCK", strv(P::CHEST_LOCK)),
            ("YELLOW_CORAL", strv(P::YELLOW_CORAL)),
            ("YELLOW_CORAL_FIRST", strv(P::YELLOW_CORAL_FIRST)),
            ("YELLOW_CORAL_SECOND", strv(P::YELLOW_CORAL_SECOND)),
            ("PINK_CORAL", strv(P::PINK_CORAL)),
            ("PINK_CORAL_SECOND", strv(P::PINK_CORAL_SECOND)),
            ("BLUE_CORAL", strv(P::BLUE_CORAL)),
            ("BLUE_CORAL_FIRST", strv(P::BLUE_CORAL_FIRST)),
            ("BLUE_CORAL_SECOND", strv(P::BLUE_CORAL_SECOND)),
            ("RED_CORAL", strv(P::RED_CORAL)),
            ("RED_CORAL_FIRST", strv(P::RED_CORAL_FIRST)),
            ("RED_CORAL_SECOND", strv(P::RED_CORAL_SECOND)),
        ],
    );
}

#[test]
fn part_names_tentacle_concatenates_the_int() {
    use minecraft_rust::net::minecraft::client::model::geom::PartNames::tentacle;
    assert_eq!(tentacle(0), "tentacle0");
    assert_eq!(tentacle(7), "tentacle7");
    assert_eq!(tentacle(-3), "tentacle-3");
    assert_eq!(tentacle(i32::MIN), "tentacle-2147483648");
}

// ============================================================================
// ClientBrandRetriever
// ============================================================================

#[test]
fn client_brand_retriever_matches_the_jar() {
    use minecraft_rust::net::minecraft::client::ClientBrandRetriever as C;
    assert_all_consts("net.minecraft.client.ClientBrandRetriever", &[("VANILLA_NAME", strv(C::VANILLA_NAME))]);
    assert_eq!(C::get_client_mod_name(), "vanilla");
    assert_eq!(C::get_client_mod_name(), C::VANILLA_NAME);
}

// ============================================================================
// EndFlashState
// ============================================================================

/// Independent oracle for `calculateFlashParameters`: `createThreadLocalInstance(seed)` is a
/// `SingleThreadedRandomSource`, whose LCG, seed scrambling, `nextInt(bound)` and `nextFloat` are
/// `java.util.Random`'s -- so the javacompat `JavaRandom` must produce the same draws.
fn end_flash_oracle(seed: i64) -> (i32, i32, f32, f32) {
    use minecraft_rust::javacompat::java_random::JavaRandom;
    let mut r = JavaRandom::new(seed);
    r.next_float();
    let offset = r.next_int_bounded(200 - 0 + 1);
    let max = std::cmp::min(380, 600 - offset);
    let duration = r.next_int_bounded(max - 100 + 1) + 100;
    let x = r.next_float() * (10.0 - -60.0) + -60.0;
    let y = r.next_float() * (180.0 - -180.0) + -180.0;
    (offset, duration, x, y)
}

#[test]
fn end_flash_state_constant_matches_the_jar() {
    use minecraft_rust::net::minecraft::client::renderer::EndFlashState as E;
    assert_all_consts("net.minecraft.client.renderer.EndFlashState", &[("SOUND_DELAY_IN_TICKS", i32v(E::SOUND_DELAY_IN_TICKS))]);
}

#[test]
fn end_flash_state_parameters_follow_the_seeded_random() {
    use minecraft_rust::net::minecraft::client::renderer::EndFlashState::EndFlashState;
    for interval in [1i64, 2, 3, 7, 17, 1000, -1, -2, 123_456_789] {
        let mut s = EndFlashState::new();
        // Exactly `interval * 600`: `x * 600 + 5` with a negative interval would truncate to the
        // next interval toward zero.
        s.tick(interval * 600);
        let (offset, duration, x, y) = end_flash_oracle(interval);
        assert_eq!(s.flash_seed(), interval);
        assert_eq!(s.offset(), offset, "offset for interval {interval}");
        assert_eq!(s.duration(), duration, "duration for interval {interval}");
        assert_eq!(s.get_x_angle().to_bits(), x.to_bits());
        assert_eq!(s.get_y_angle().to_bits(), y.to_bits());
        assert!((0..=200).contains(&offset));
        assert!(duration >= 100 && duration <= std::cmp::min(380, 600 - offset));
        assert!((-60.0..10.0).contains(&x) && (-180.0..180.0).contains(&y));
    }
}

#[test]
fn end_flash_state_intensity_shape_over_one_interval() {
    use minecraft_rust::net::minecraft::client::renderer::EndFlashState::EndFlashState;
    use minecraft_rust::net::minecraft::util::Mth::Mth;
    let base = 7 * 600i64;
    let (offset, duration, _, _) = end_flash_oracle(7);
    let mut s = EndFlashState::new();
    let mut starts = Vec::new();
    let mut prev = 0.0f32;
    for t in 0..600i64 {
        s.tick(base + t);
        let expected = if t >= offset as i64 && t <= (offset + duration) as i64 {
            Mth::sin(((t - offset as i64) as f32 * std::f32::consts::PI / duration as f32) as f64)
        } else {
            0.0
        };
        // `lerp(1, old, new)` is `old + (new - old)`, which is NOT always `new` in float: at the
        // tail (new = sin(PI) ~ 1.2e-16, old ~ 0.008) it rounds to exactly 0. Java does the same.
        for pt in [0.0f32, 0.25, 0.5, 1.0] {
            let want = Mth::lerp_f32(pt, prev, expected);
            assert_eq!(s.get_intensity(pt).to_bits(), want.to_bits(), "t={t} partial={pt}");
        }
        assert_eq!(s.get_intensity(0.0).to_bits(), prev.to_bits(), "old intensity at t={t}");
        if s.flash_started_this_tick() {
            starts.push(t);
        }
        prev = expected;
    }
    // sin(0) at t == offset is exactly 0, so the flash "starts" one tick later.
    assert_eq!(starts, vec![offset as i64 + 1]);
}

#[test]
fn end_flash_state_first_interval_never_computes_parameters() {
    use minecraft_rust::net::minecraft::client::renderer::EndFlashState::EndFlashState;
    // flashSeed starts at 0 and clockTime / 600 == 0 for -599..=599, so nothing is drawn.
    let mut s = EndFlashState::new();
    for t in -599..600i64 {
        s.tick(t);
        assert_eq!(s.offset(), 0);
        assert_eq!(s.duration(), 0);
        assert_eq!(s.get_x_angle(), 0.0);
        assert_eq!(s.get_y_angle(), 0.0);
        // Including t == 0, where the phase is 0 * PI / 0 = NaN and Mth.sin(NaN) is table[0].
        assert_eq!(s.get_intensity(1.0).to_bits(), 0.0f32.to_bits(), "t={t}");
        assert!(!s.flash_started_this_tick());
    }
}

#[test]
fn end_flash_state_negative_clock_never_flashes() {
    use minecraft_rust::net::minecraft::client::renderer::EndFlashState::EndFlashState;
    let mut s = EndFlashState::new();
    for t in (-6000i64..-600).rev() {
        s.tick(t);
        // Java's `%` keeps the dividend's sign, so the in-interval time is <= 0 and can only
        // equal `offset` when both are 0, where sin(0) == 0.
        assert_eq!(s.get_intensity(1.0), 0.0, "t={t}");
        assert!(!s.flash_started_this_tick());
    }
    // ...even though the parameters themselves were drawn (the last tick was -6000 / 600 == -10).
    assert_eq!(s.flash_seed(), -10);
    assert_eq!(s.offset(), end_flash_oracle(-10).0);
    // Truncating division: -601 / 600 == -1, not floor's -2.
    let mut s = EndFlashState::new();
    s.tick(-601);
    assert_eq!(s.flash_seed(), -1);
    assert_eq!(s.duration(), end_flash_oracle(-1).1);
}

// ============================================================================
// Rect2i
// ============================================================================

#[test]
fn rect2i_accessors_and_setters() {
    use minecraft_rust::net::minecraft::client::renderer::Rect2i::Rect2i;
    let mut r = Rect2i::new(1, 2, 3, 4);
    assert_eq!((r.get_x(), r.get_y(), r.get_width(), r.get_height()), (1, 2, 3, 4));
    r.set_x(-5);
    r.set_y(6);
    r.set_width(7);
    r.set_height(8);
    assert_eq!((r.get_x(), r.get_y(), r.get_width(), r.get_height()), (-5, 6, 7, 8));
    r.set_position(10, 20);
    assert_eq!((r.get_x(), r.get_y(), r.get_width(), r.get_height()), (10, 20, 7, 8));
}

#[test]
fn rect2i_contains_is_inclusive_on_every_edge() {
    use minecraft_rust::net::minecraft::client::renderer::Rect2i::Rect2i;
    let r = Rect2i::new(0, 0, 10, 5);
    assert!(r.contains(0, 0));
    assert!(r.contains(10, 5), "far corner is inside: x <= xPos + width");
    assert!(!r.contains(11, 5));
    assert!(!r.contains(10, 6));
    assert!(!r.contains(-1, 0));
    // A zero-size rect still contains its own corner.
    assert!(Rect2i::new(3, 3, 0, 0).contains(3, 3));
    // Negative width: nothing satisfies x >= 3 && x <= 1.
    assert!(!Rect2i::new(3, 3, -2, 0).contains(3, 3));
    // int overflow wraps: xPos + width = MIN_VALUE, so nothing is <= it except MIN itself.
    assert!(!Rect2i::new(i32::MAX, 0, 1, 1).contains(i32::MAX, 0));
}

#[test]
fn rect2i_intersect_mutates_and_clamps() {
    use minecraft_rust::net::minecraft::client::renderer::Rect2i::Rect2i;
    let mut a = Rect2i::new(0, 0, 10, 10);
    let b = Rect2i::new(5, 3, 10, 4);
    let (x, y, w, h) = {
        let r = a.intersect(&b);
        (r.get_x(), r.get_y(), r.get_width(), r.get_height())
    };
    assert_eq!((x, y, w, h), (5, 3, 5, 4));
    // `this` was mutated, `other` was not.
    assert_eq!((a.get_x(), a.get_y(), a.get_width(), a.get_height()), (5, 3, 5, 4));
    assert_eq!((b.get_x(), b.get_y(), b.get_width(), b.get_height()), (5, 3, 10, 4));

    // Disjoint: position is the max of the origins, size clamps to 0.
    let mut c = Rect2i::new(0, 0, 2, 2);
    c.intersect(&Rect2i::new(10, -10, 3, 3));
    assert_eq!((c.get_x(), c.get_y(), c.get_width(), c.get_height()), (10, 0, 0, 0));

    // Contained.
    let mut d = Rect2i::new(-100, -100, 1000, 1000);
    d.intersect(&Rect2i::new(1, 2, 3, 4));
    assert_eq!((d.get_x(), d.get_y(), d.get_width(), d.get_height()), (1, 2, 3, 4));

    // Overflowing right edge wraps negative: min(x1, x3) is the wrapped value, so width clamps to 0.
    let mut e = Rect2i::new(i32::MAX - 1, 0, 10, 10);
    e.intersect(&Rect2i::new(0, 0, i32::MAX, 10));
    assert_eq!((e.get_x(), e.get_width()), (i32::MAX - 1, 0));
}

// ============================================================================
// RunningTrimmedMean
// ============================================================================

#[test]
fn running_trimmed_mean_drops_min_and_max_once_past_two_samples() {
    use minecraft_rust::net::minecraft::client::renderer::RunningTrimmedMean::RunningTrimmedMean;
    let mut m = RunningTrimmedMean::new(5);
    m.register_value_and_get_mean(10);
    m.register_value_and_get_mean(20);
    // [10, 20, 30]: drop 10 and 30 -> 20 / 1.
    assert_eq!(m.register_value_and_get_mean(30), 20);
    // [10, 20, 30, 1]: drop 1 and 30 -> 30 / 2 = 15.
    assert_eq!(m.register_value_and_get_mean(1), 15);
    // [10, 20, 30, 1, 100]: drop 1 and 100 -> 60 / 3 = 20.
    assert_eq!(m.register_value_and_get_mean(100), 20);
    // Ring buffer wraps: [7, 20, 30, 1, 100] -> drop 1 and 100 -> 57 / 3 = 19 (truncated).
    assert_eq!(m.register_value_and_get_mean(7), 19);
    // Truncation toward zero for negative totals: [7, -50, 30, 1, 100] -> drop -50, 100 -> 38/3.
    assert_eq!(m.register_value_and_get_mean(-50), 12);
    let mut n = RunningTrimmedMean::new(3);
    n.register_value_and_get_mean(-1);
    n.register_value_and_get_mean(-100);
    // [-1, -100, -2] -> drop -100 and -1 -> -2 / 1; then [-3, -100, -2] -> -3.
    assert_eq!(n.register_value_and_get_mean(-2), -2);
    assert_eq!(n.register_value_and_get_mean(-3), -3);
    let mut t = RunningTrimmedMean::new(4);
    for v in [-7i64, 0, 0, 0] {
        t.register_value_and_get_mean(v);
    }
    // [-7, 0, 0, 0]: drop -7 and 0 -> 0 / 2. Then [-7, ...] replaced by -5: [-5, 0, 0, 0] -> 0.
    // Now a real negative truncation: [-5, -5, 0, 10] -> drop -5 and 10 -> -5 / 2 = -2 (not -3).
    t.register_value_and_get_mean(-5);
    t.register_value_and_get_mean(-5);
    assert_eq!(t.register_value_and_get_mean(0), 0);
    assert_eq!(t.register_value_and_get_mean(10), -2);
}

#[test]
fn running_trimmed_mean_small_counts_return_count_over_total() {
    use minecraft_rust::net::minecraft::client::renderer::RunningTrimmedMean::RunningTrimmedMean;
    // Vanilla returns `count / total`, the inverse of a mean, for one or two samples.
    let mut m = RunningTrimmedMean::new(10);
    assert_eq!(m.register_value_and_get_mean(16_000_000), 0);
    assert_eq!(m.register_value_and_get_mean(-15_999_998), 1, "count 2 / total 2");
    let mut one = RunningTrimmedMean::new(10);
    assert_eq!(one.register_value_and_get_mean(1), 1);
    let mut neg = RunningTrimmedMean::new(10);
    assert_eq!(neg.register_value_and_get_mean(-5), 0, "total <= 0 gives 0");
    assert_eq!(neg.register_value_and_get_mean(5), 0);
    // maxCount 1 and 2 never get past the small-count branch.
    let mut cap1 = RunningTrimmedMean::new(1);
    for v in [1i64, 2, 3, 1] {
        assert_eq!(cap1.register_value_and_get_mean(v), 1 / v);
    }
}

#[test]
fn running_trimmed_mean_wraps_long_overflow() {
    use minecraft_rust::net::minecraft::client::renderer::RunningTrimmedMean::RunningTrimmedMean;
    let mut m = RunningTrimmedMean::new(3);
    m.register_value_and_get_mean(i64::MAX);
    m.register_value_and_get_mean(i64::MAX);
    // total wraps to -3 + MAX..., min + max wraps too; Java computes it all mod 2^64:
    // total = 3*MAX (wrapped) = MAX - 2, min + max = 2*MAX (wrapped) = -2 -> (MAX - 2) - (-2) = MAX.
    assert_eq!(m.register_value_and_get_mean(i64::MAX), i64::MAX);
}

#[test]
fn running_trimmed_mean_java_exceptions() {
    use minecraft_rust::net::minecraft::client::renderer::RunningTrimmedMean::RunningTrimmedMean;
    assert_eq!(
        panic_text(|| {
            RunningTrimmedMean::new(-1);
        }),
        "java.lang.NegativeArraySizeException: -1"
    );
    let mut empty = RunningTrimmedMean::new(0);
    let empty_ptr = std::panic::AssertUnwindSafe(&mut empty);
    assert_eq!(
        panic_text(move || {
            let e = empty_ptr;
            e.0.register_value_and_get_mean(1);
        }),
        "java.lang.ArrayIndexOutOfBoundsException: Index 0 out of bounds for length 0"
    );
}

// ============================================================================
// FramerateLimiter
// ============================================================================

/// Serialises every test that reads or overrides the process-global clock in `entropy`.
static CLOCK_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn framerate_limiter_waits_out_the_frame_budget() {
    use minecraft_rust::javacompat::entropy;
    use minecraft_rust::net::minecraft::client::FramerateLimiter as F;
    let _guard = CLOCK_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    assert_eq!(panic_text(|| F::limit_display_fps(0)), "java.lang.ArithmeticException: / by zero");

    // Real clock: the second call cannot return before lastFrameTime + 1e9 / 200 ns.
    F::limit_display_fps(200);
    let (first, _, limit) = F::debug_state();
    assert_eq!(limit, 200);
    F::limit_display_fps(200);
    let (second, avg, _) = F::debug_state();
    assert!(second - first >= 5_000_000, "only {} ns passed", second - first);
    assert!((0..=2_000_000).contains(&avg), "average overshoot is clamped to 2 ms: {avg}");

    // A deterministic clock already past the target returns at once and records the new time.
    entropy::with_override(first + 1_000_000_000, || {
        F::limit_display_fps(30);
        assert_eq!(F::debug_state(), (first + 1_000_000_000, 0, 30), "a new limit resets the average");
    });
    // Exactly on target (remaining == 0) also returns: the loop runs while remaining > 0.
    entropy::with_override(first + 1_000_000_000 + 1_000_000_000 / 30, || {
        F::limit_display_fps(30);
        assert_eq!(F::debug_state().0, first + 1_000_000_000 + 33_333_333);
    });
    // NOTE: lastFrameTime is now ~1 s ahead of the real clock. This is the only test that calls
    // limit_display_fps, so nothing waits on it. (Overriding the clock to a value BEFORE the
    // target would spin forever: the fake clock never advances.)
}
