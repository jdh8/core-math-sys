#[test]
fn sanity() {
    assert!(unsafe { core_math_sys::cr_expf(0.0) }.eq(&1.0));
}

#[test]
fn newly_exported() {
    // gamma functions on binary64, previously only available for binary32
    assert!(unsafe { core_math_sys::cr_tgamma(5.0) }.eq(&24.0)); // 4!
    assert!(unsafe { core_math_sys::cr_lgamma(1.0) }.eq(&0.0)); // ln(0!) = 0

    // (1 + 0)^10 == 1
    assert!(unsafe { core_math_sys::cr_compoundf(0.0, 10.0) }.eq(&1.0));

    // sincos(0) == (0, 1), written through the out-pointers
    let mut s = f64::NAN;
    let mut c = f64::NAN;
    unsafe { core_math_sys::cr_sincos(0.0, &mut s, &mut c) };
    assert!(s.eq(&0.0));
    assert!(c.eq(&1.0));
}
