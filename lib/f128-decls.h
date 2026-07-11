/* glibc gates its _Float128 API on __GNUC_PREREQ (7, 0) in <bits/floatn.h>,
 * and Clang masquerades as GCC 4.2, so <math.h> never declares the *f128
 * functions for Clang.  The binary128 sources define non-cr_ alias wrappers
 * (sqrtq, expq, ...) that call these functions, and Clang 16+ rejects calls
 * to undeclared functions.  This header is force-included (-include) to give
 * Clang the declarations; the symbols themselves are in libm since glibc 2.26.
 */
#if defined __clang__ && defined __x86_64__
__float128 atan2f128(__float128, __float128);
__float128 cbrtf128(__float128);
__float128 exp10f128(__float128);
__float128 exp2f128(__float128);
__float128 expf128(__float128);
__float128 expm1f128(__float128);
__float128 hypotf128(__float128, __float128);
__float128 sqrtf128(__float128);
#endif
