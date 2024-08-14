#ifndef _MATH_H
#define _MATH_H

extern int signgam;

#define NAN      __builtin_nanf("")
#define INFINITY __builtin_inff()
#define HUGE_VALF __builtin_huge_valf()
#define HUGE_VAL  __builtin_huge_val()
#define HUGE_VALL __builtin_huge_vall()
#define FP_ILOGBNAN __INT_MAX__
#define FP_ILOGB0   (-__INT_MAX__ - (-1 == ~0))  // INT_MIN

#define M_E        2.71828182845904523536
#define M_LOG2E    1.44269504088896340736
#define M_LOG10E   0.43429448190325182765
#define M_LN2      0.69314718055994530942
#define M_LN10     2.30258509299404568402
#define M_PI       3.14159265358979323846
#define M_PI_2     1.57079632679489661923
#define M_PI_4     0.78539816339744830962
#define M_1_PI     0.31830988618379067154
#define M_2_PI     0.63661977236758134308
#define M_2_SQRTPI 1.12837916709551257390
#define M_SQRT2    1.41421356237309504880
#define M_SQRT1_2  0.70710678118654752440

float acosf(float);
float acoshf(float);
float acospif(float);
float asinf(float);
float asinhf(float);
float asinpif(float);
float atanf(float);
float atan2f(float, float);
float atan2pif(float, float);
float atanhf(float);
float atanpif(float);
float cbrtf(float);
float cosf(float);
float coshf(float);
float cospif(float);
float erff(float);
float erfcf(float);
float expf(float);
float exp10f(float);
float exp10m1f(float);
float exp2f(float);
float exp2m1f(float);
float expm1f(float);
float hypotf(float, float);
float lgammaf(float);
float logf(float);
float log10f(float);
float log10p1f(float);
float log1pf(float);
float log2f(float);
float log2p1f(float);
float powf(float, float);
float rsqrtf(float);
float sinf(float);
float sincosf(float);
float sinhf(float);
float sinpif(float);
float sqrtf(float);
float tanf(float);
float tanhf(float);
float tanpif(float);
float tgammaf(float);

double acos(double);
double acosh(double);
double acospi(double);
double asin(double);
double asinh(double);
double asinpi(double);
double atan(double);
double atan2(double, double);
double atan2pi(double, double);
double atanh(double);
double atanpi(double);
double cbrt(double);
double cos(double);
double cosh(double);
double cospi(double);
double erf(double);
double erfc(double);
double exp(double);
double exp10(double);
double exp10m1(double);
double exp2(double);
double exp2m1(double);
double expm1(double);
double hypot(double, double);
double log(double);
double log10(double);
double log10p1(double);
double log1p(double);
double log2(double);
double log2p1(double);
double pow(double, double);
double rsqrt(double);
double sin(double);
double sinh(double);
double sinpi(double);
double sqrt(double);
double tan(double);
double tanh(double);
double tanpi(double);

#endif // math.h