//! Bindings to CORE-MATH binary64 (`double`) functions.

unsafe extern "C" {
    pub fn cr_acos(x: f64) -> f64;
    pub fn cr_acosh(x: f64) -> f64;
    pub fn cr_acospi(x: f64) -> f64;
    pub fn cr_asin(x: f64) -> f64;
    pub fn cr_asinh(x: f64) -> f64;
    pub fn cr_asinpi(x: f64) -> f64;
    pub fn cr_atan(x: f64) -> f64;
    pub fn cr_atan2(y: f64, x: f64) -> f64;
    pub fn cr_atan2pi(y: f64, x: f64) -> f64;
    pub fn cr_atanh(x: f64) -> f64;
    pub fn cr_atanpi(x: f64) -> f64;
    pub fn cr_cbrt(x: f64) -> f64;
    pub fn cr_cos(x: f64) -> f64;
    pub fn cr_cosh(x: f64) -> f64;
    pub fn cr_cospi(x: f64) -> f64;
    pub fn cr_erf(x: f64) -> f64;
    pub fn cr_erfc(x: f64) -> f64;
    pub fn cr_exp(x: f64) -> f64;
    pub fn cr_exp10(x: f64) -> f64;
    pub fn cr_exp10m1(x: f64) -> f64;
    pub fn cr_exp2(x: f64) -> f64;
    pub fn cr_exp2m1(x: f64) -> f64;
    pub fn cr_expm1(x: f64) -> f64;
    pub fn cr_hypot(x: f64, y: f64) -> f64;
    pub fn cr_lgamma(x: f64) -> f64;
    pub fn cr_log(x: f64) -> f64;
    pub fn cr_log10(x: f64) -> f64;
    pub fn cr_log10p1(x: f64) -> f64;
    pub fn cr_log1p(x: f64) -> f64;
    pub fn cr_log2(x: f64) -> f64;
    pub fn cr_log2p1(x: f64) -> f64;
    pub fn cr_pow(x: f64, y: f64) -> f64;
    pub fn cr_rsqrt(x: f64) -> f64;
    pub fn cr_sin(x: f64) -> f64;
    pub fn cr_sincos(x: f64, s: *mut f64, c: *mut f64);
    pub fn cr_sinh(x: f64) -> f64;
    pub fn cr_sinpi(x: f64) -> f64;
    pub fn cr_tan(x: f64) -> f64;
    pub fn cr_tanh(x: f64) -> f64;
    pub fn cr_tanpi(x: f64) -> f64;
    pub fn cr_tgamma(x: f64) -> f64;
}
