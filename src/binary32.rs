//! Bindings to CORE-MATH binary32 (`float`) functions.

unsafe extern "C" {
    pub fn cr_acosf(x: f32) -> f32;
    pub fn cr_acoshf(x: f32) -> f32;
    pub fn cr_acospif(x: f32) -> f32;
    pub fn cr_asinf(x: f32) -> f32;
    pub fn cr_asinhf(x: f32) -> f32;
    pub fn cr_asinpif(x: f32) -> f32;
    pub fn cr_atanf(x: f32) -> f32;
    pub fn cr_atan2f(y: f32, x: f32) -> f32;
    pub fn cr_atan2pif(y: f32, x: f32) -> f32;
    pub fn cr_atanhf(x: f32) -> f32;
    pub fn cr_atanpif(x: f32) -> f32;
    pub fn cr_cbrtf(x: f32) -> f32;
    pub fn cr_compoundf(x: f32, y: f32) -> f32;
    pub fn cr_cosf(x: f32) -> f32;
    pub fn cr_coshf(x: f32) -> f32;
    pub fn cr_cospif(x: f32) -> f32;
    pub fn cr_erff(x: f32) -> f32;
    pub fn cr_erfcf(x: f32) -> f32;
    pub fn cr_expf(x: f32) -> f32;
    pub fn cr_exp10f(x: f32) -> f32;
    pub fn cr_exp10m1f(x: f32) -> f32;
    pub fn cr_exp2f(x: f32) -> f32;
    pub fn cr_exp2m1f(x: f32) -> f32;
    pub fn cr_expm1f(x: f32) -> f32;
    pub fn cr_hypotf(x: f32, y: f32) -> f32;
    pub fn cr_lgammaf(x: f32) -> f32;
    pub fn cr_logf(x: f32) -> f32;
    pub fn cr_log10f(x: f32) -> f32;
    pub fn cr_log10p1f(x: f32) -> f32;
    pub fn cr_log1pf(x: f32) -> f32;
    pub fn cr_log2f(x: f32) -> f32;
    pub fn cr_log2p1f(x: f32) -> f32;
    pub fn cr_powf(x: f32, y: f32) -> f32;
    pub fn cr_rsqrtf(x: f32) -> f32;
    pub fn cr_sinf(x: f32) -> f32;
    pub fn cr_sincosf(x: f32, s: *mut f32, c: *mut f32);
    pub fn cr_sinhf(x: f32) -> f32;
    pub fn cr_sinpif(x: f32) -> f32;
    pub fn cr_tanf(x: f32) -> f32;
    pub fn cr_tanhf(x: f32) -> f32;
    pub fn cr_tanpif(x: f32) -> f32;
    pub fn cr_tgammaf(x: f32) -> f32;
}
