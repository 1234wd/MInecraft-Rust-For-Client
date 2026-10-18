//! Sub-batch I: ResourceDescriptor and GaussianSampler. Groups `RESDESC`, `GAUSS`.
//! Sub-batch J: ListenerTransform. Groups `REC`/`RECEQ` (its records), `LISTENER`.

use crate::small_probe::*;

use crate::b_records::check_records;

use minecraft_rust::com::mojang::blaze3d::audio::ListenerTransform::ListenerTransform;
use minecraft_rust::com::mojang::blaze3d::resource::ResourceDescriptor::ResourceDescriptor;
use minecraft_rust::net::minecraft::world::attribute::GaussianSampler;
use minecraft_rust::net::minecraft::world::phys::Vec3::Vec3;

/// The probe's `record DescA(int width)`: a descriptor whose `equals` is by value.
#[derive(Debug, PartialEq)]
struct DescA(i32);

impl ResourceDescriptor<String> for DescA {
    fn allocate(&self) -> String {
        format!("a{}", self.0)
    }
    fn free(&self, _resource: String) {}
}

/// The probe's `record DescB(int width)`: same shape, different type.
#[derive(Debug, PartialEq)]
struct DescB(i32);

impl ResourceDescriptor<String> for DescB {
    fn allocate(&self) -> String {
        format!("b{}", self.0)
    }
    fn free(&self, _resource: String) {}
}

#[test]
fn resource_descriptor_defaults_match_the_jar() {
    let r = &rows("RESDESC")[0];
    let (a1, a1b, a2, b1) = (DescA(1), DescA(1), DescA(2), DescB(1));
    a1.prepare(&"x".to_string());
    assert_eq!(a1.can_use_physical_resource(&a1), boolf(r[1]));
    assert_eq!(
        a1.can_use_physical_resource(&a1b),
        boolf(r[2]),
        "equal records"
    );
    assert_eq!(a1.can_use_physical_resource(&a2), boolf(r[3]));
    assert_eq!(
        a1.can_use_physical_resource(&b1),
        boolf(r[4]),
        "a different descriptor type is never equal"
    );
    assert_eq!(a1.allocate(), r[5]);
}

#[test]
fn gaussian_sampler_transcripts_match_the_jar() {
    for r in rows("GAUSS") {
        let pos = Vec3::new(f64f(r[1]), f64f(r[2]), f64f(r[3]));
        let mut calls: Vec<String> = Vec::new();
        GaussianSampler::sample(
            &pos,
            |x: i32, y: i32, z: i32| format!("{x},{y},{z}"),
            |weight: f64, value: String| calls.push(format!("{value}={}", hex64(weight))),
        );
        assert_eq!(calls.len(), 216);
        assert_eq!(calls.join(";"), r[4], "sample({pos})");
    }
}

#[test]
fn listener_transform_matches_the_jar() {
    let v = Vec3::new;
    let (o, back, up) = (Vec3::ZERO, v(0.0, 0.0, -1.0), v(0.0, 1.0, 0.0));
    let odd =
        |p: Vec3| ListenerTransform::new(p, v(1e300, -1e-300, 0.1), v(f64::INFINITY, 2.0, 3.0));
    // The probe's eight transforms, in its order.
    let ts = [
        ListenerTransform::INITIAL,
        ListenerTransform::new(o, back, up),
        ListenerTransform::new(v(-0.0, 0.0, 0.0), back, up),
        ListenerTransform::new(v(1.5, -2.25, 64.0), v(1.0, 0.0, 0.0), v(0.0, 0.0, 1.0)),
        odd(v(f64::NAN, 0.0, 0.0)),
        odd(v(f64::NAN, 0.0, 0.0)),
        ListenerTransform::new(o, o, v(f64::INFINITY, 0.0, 0.0)),
        ListenerTransform::new(o, v(0.3, -0.7, 1e-8), v(-1e8, 0.25, f64::from_bits(1))),
    ];
    check_records("ListenerTransform", &ts, &|t| t.java_hash_code());
    let right = rows("LISTENER");
    assert_eq!(right.len(), ts.len());
    for (r, t) in right.iter().zip(ts.iter()) {
        let got = t.right();
        // `0 * inf` makes a NaN, whose sign is not portable (`nan_canon`); zeros stay exact.
        assert_eq!(
            nan_canon(&[hex64(got.x), hex64(got.y), hex64(got.z)].join(" ")),
            nan_canon(&r[2..5].join(" ")),
            "[{}].right() of {t}",
            r[1]
        );
    }
    let i = ListenerTransform::INITIAL;
    assert_eq!(
        (i.position(), i.forward(), i.up()),
        (o, back, up),
        "INITIAL's components"
    );
}
