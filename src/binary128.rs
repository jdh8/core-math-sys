//! Bindings to CORE-MATH binary128 (`__float128`) functions, handwritten
//! because bindgen cannot yet emit the unstable `f128` primitive.

unsafe extern "C" {
    pub fn cr_acosq(x: f128) -> f128;
    pub fn cr_asinq(x: f128) -> f128;
    pub fn cr_atanq(x: f128) -> f128;
    pub fn cr_atan2q(y: f128, x: f128) -> f128;
    pub fn cr_cbrtq(x: f128) -> f128;
    pub fn cr_expq(x: f128) -> f128;
    pub fn cr_exp10q(x: f128) -> f128;
    pub fn cr_exp2q(x: f128) -> f128;
    pub fn cr_expm1q(x: f128) -> f128;
    pub fn cr_hypotq(x: f128, y: f128) -> f128;
    pub fn cr_logq(x: f128) -> f128;
    pub fn cr_rsqrtq(x: f128) -> f128;
    pub fn cr_sqrtq(x: f128) -> f128;
}
