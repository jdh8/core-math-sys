#[test]
fn sanity() {
    assert!(unsafe { core_math_sys::cr_expf(0.0) }.eq(&1.0));
}