#ifndef _MATH_H
#define _MATH_H

#define HUGE_VAL  (__builtin_huge_val())
#define HUGE_VALF (__builtin_huge_valf())
#define HUGE_VALL (__builtin_huge_vall())
#define INFINITY  (__builtin_inff())
#define NAN       (__builtin_nanf(""))

#define FP_INFINITE  1
#define FP_NAN       2
#define FP_NORMAL    3
#define FP_SUBNORMAL 4
#define FP_ZERO      5

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

/* Trigonometric */
double sin(double x);
double cos(double x);
double tan(double x);
double asin(double x);
double acos(double x);
double atan(double x);
double atan2(double y, double x);
float sinf(float x);
float cosf(float x);
float tanf(float x);

/* Hyperbolic */
double sinh(double x);
double cosh(double x);
double tanh(double x);
double asinh(double x);
double acosh(double x);
double atanh(double x);

/* Exponential and logarithmic */
double exp(double x);
double exp2(double x);
double expm1(double x);
double log(double x);
double log10(double x);
double log2(double x);
double log1p(double x);
float expf(float x);
float logf(float x);

/* Power */
double pow(double x, double y);
double sqrt(double x);
double cbrt(double x);
double hypot(double x, double y);
float powf(float x, float y);
float sqrtf(float x);

/* Rounding */
double ceil(double x);
double floor(double x);
double trunc(double x);
double round(double x);
double nearbyint(double x);
double rint(double x);
long lround(double x);
long long llround(double x);
float ceilf(float x);
float floorf(float x);
float roundf(float x);

/* Remainder */
double fmod(double x, double y);
double remainder(double x, double y);
float fmodf(float x, float y);

/* Manipulation */
double copysign(double x, double y);
double fabs(double x);
double fdim(double x, double y);
double fmax(double x, double y);
double fmin(double x, double y);
float fabsf(float x);

/* Other */
double frexp(double x, int *exp);
double ldexp(double x, int exp);
double modf(double x, double *iptr);
double scalbn(double x, int n);
int ilogb(double x);
double logb(double x);
double nextafter(double x, double y);

/* Classification */
int fpclassify(double x);
int isfinite(double x);
int isinf(double x);
int isnan(double x);
int isnormal(double x);
int signbit(double x);

#endif /* _MATH_H */
