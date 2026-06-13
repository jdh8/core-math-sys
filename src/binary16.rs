//! Bindings to CORE-MATH binary16 (`_Float16`) functions, handwritten because
//! bindgen cannot yet emit the unstable `f16` primitive.

unsafe extern "C" {
    pub fn cr_acosf16(x: f16) -> f16;
    pub fn cr_acoshf16(x: f16) -> f16;
    pub fn cr_acospif16(x: f16) -> f16;
    pub fn cr_asinf16(x: f16) -> f16;
    pub fn cr_asinhf16(x: f16) -> f16;
    pub fn cr_asinpif16(x: f16) -> f16;
    pub fn cr_atanf16(x: f16) -> f16;
    pub fn cr_atan2f16(y: f16, x: f16) -> f16;
    pub fn cr_atan2pif16(y: f16, x: f16) -> f16;
    pub fn cr_atanhf16(x: f16) -> f16;
    pub fn cr_atanpif16(x: f16) -> f16;
    pub fn cr_cbrtf16(x: f16) -> f16;
    pub fn cr_compoundf16(x: f16, y: f16) -> f16;
    pub fn cr_cosf16(x: f16) -> f16;
    pub fn cr_coshf16(x: f16) -> f16;
    pub fn cr_cospif16(x: f16) -> f16;
    pub fn cr_erff16(x: f16) -> f16;
    pub fn cr_erfcf16(x: f16) -> f16;
    pub fn cr_expf16(x: f16) -> f16;
    pub fn cr_exp10f16(x: f16) -> f16;
    pub fn cr_exp10m1f16(x: f16) -> f16;
    pub fn cr_exp2f16(x: f16) -> f16;
    pub fn cr_exp2m1f16(x: f16) -> f16;
    pub fn cr_expm1f16(x: f16) -> f16;
    pub fn cr_hypotf16(x: f16, y: f16) -> f16;
    pub fn cr_lgammaf16(x: f16) -> f16;
    pub fn cr_logf16(x: f16) -> f16;
    pub fn cr_log10f16(x: f16) -> f16;
    pub fn cr_log10p1f16(x: f16) -> f16;
    pub fn cr_log1pf16(x: f16) -> f16;
    pub fn cr_log2f16(x: f16) -> f16;
    pub fn cr_log2p1f16(x: f16) -> f16;
    pub fn cr_powf16(x: f16, y: f16) -> f16;
    pub fn cr_rsqrtf16(x: f16) -> f16;
    pub fn cr_sinf16(x: f16) -> f16;
    pub fn cr_sincosf16(x: f16, s: *mut f16, c: *mut f16);
    pub fn cr_sinhf16(x: f16) -> f16;
    pub fn cr_sinpif16(x: f16) -> f16;
    pub fn cr_sqrtf16(x: f16) -> f16;
    pub fn cr_tanf16(x: f16) -> f16;
    pub fn cr_tanhf16(x: f16) -> f16;
    pub fn cr_tanpif16(x: f16) -> f16;
    pub fn cr_tgammaf16(x: f16) -> f16;
}
