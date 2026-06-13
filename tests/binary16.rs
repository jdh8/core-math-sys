#![cfg(feature = "f16")]
#![cfg_attr(feature = "f16", feature(f16))]

#[test]
fn sanity() {
    assert!(unsafe { core_math_sys::cr_expf16(0.0) }.eq(&1.0));
    assert!(unsafe { core_math_sys::cr_sqrtf16(4.0) }.eq(&2.0));
    assert!(unsafe { core_math_sys::cr_hypotf16(3.0, 4.0) }.eq(&5.0));

    // sincos(0) == (0, 1), written through the out-pointers
    let mut s = f16::NAN;
    let mut c = f16::NAN;
    unsafe { core_math_sys::cr_sincosf16(0.0, &mut s, &mut c) };
    assert!(s.eq(&0.0));
    assert!(c.eq(&1.0));
}
