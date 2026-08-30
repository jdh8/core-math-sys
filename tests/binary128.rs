#![cfg(feature = "f128")]
#![feature(f128)]

#[test]
fn sanity() {
    assert!(unsafe { core_math_sys::cr_acosq(1.0) }.eq(&0.0));
    assert!(unsafe { core_math_sys::cr_asinq(0.0) }.eq(&0.0));
    assert!(unsafe { core_math_sys::cr_atanq(0.0) }.eq(&0.0));
    assert!(unsafe { core_math_sys::cr_expq(0.0) }.eq(&1.0));
    assert!(unsafe { core_math_sys::cr_sqrtq(4.0) }.eq(&2.0));
    assert!(unsafe { core_math_sys::cr_hypotq(3.0, 4.0) }.eq(&5.0));
    assert!(unsafe { core_math_sys::cr_atan2q(0.0, 1.0) }.eq(&0.0));
}
