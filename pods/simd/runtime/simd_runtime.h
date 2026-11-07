#ifndef ABYSS_SIMD_RUNTIME_H
#define ABYSS_SIMD_RUNTIME_H

#include <stdint.h>
#include <stdbool.h>

void abyss_simd_sin_f32x4(float *dst, const float *src);
void abyss_simd_sin_fast_f32x4(float *dst, const float *src);
void abyss_simd_cos_f32x4(float *dst, const float *src);
void abyss_simd_cos_fast_f32x4(float *dst, const float *src);
void abyss_simd_tan_f32x4(float *dst, const float *src);
void abyss_simd_tan_fast_f32x4(float *dst, const float *src);
void abyss_simd_asin_f32x4(float *dst, const float *src);
void abyss_simd_acos_f32x4(float *dst, const float *src);
void abyss_simd_atan_f32x4(float *dst, const float *src);
void abyss_simd_atan2_f32x4(float *dst, const float *y, const float *x);
void abyss_simd_exp_f32x4(float *dst, const float *src);
void abyss_simd_exp_fast_f32x4(float *dst, const float *src);
void abyss_simd_exp2_f32x4(float *dst, const float *src);
void abyss_simd_ln_f32x4(float *dst, const float *src);
void abyss_simd_ln_fast_f32x4(float *dst, const float *src);
void abyss_simd_log2_f32x4(float *dst, const float *src);
void abyss_simd_log10_f32x4(float *dst, const float *src);
void abyss_simd_pow_f32x4(float *dst, const float *a, const float *b);
void abyss_simd_sqrt_f32x4(float *dst, const float *src);
void abyss_simd_rsqrt_f32x4(float *dst, const float *src);
void abyss_simd_rsqrt_fast_f32x4(float *dst, const float *src);
void abyss_simd_cbrt_f32x4(float *dst, const float *src);
void abyss_simd_hypot_f32x4(float *dst, const float *a, const float *b);
void abyss_simd_sinh_f32x4(float *dst, const float *src);
void abyss_simd_cosh_f32x4(float *dst, const float *src);
void abyss_simd_tanh_f32x4(float *dst, const float *src);
void abyss_simd_fma_f32x4(float *dst, const float *a, const float *b, const float *c);

void abyss_simd_sin_f64x2(double *dst, const double *src);
void abyss_simd_sin_fast_f64x2(double *dst, const double *src);
void abyss_simd_cos_f64x2(double *dst, const double *src);
void abyss_simd_cos_fast_f64x2(double *dst, const double *src);
void abyss_simd_tan_f64x2(double *dst, const double *src);
void abyss_simd_asin_f64x2(double *dst, const double *src);
void abyss_simd_acos_f64x2(double *dst, const double *src);
void abyss_simd_atan_f64x2(double *dst, const double *src);
void abyss_simd_atan2_f64x2(double *dst, const double *y, const double *x);
void abyss_simd_exp_f64x2(double *dst, const double *src);
void abyss_simd_exp_fast_f64x2(double *dst, const double *src);
void abyss_simd_exp2_f64x2(double *dst, const double *src);
void abyss_simd_ln_f64x2(double *dst, const double *src);
void abyss_simd_ln_fast_f64x2(double *dst, const double *src);
void abyss_simd_log2_f64x2(double *dst, const double *src);
void abyss_simd_log10_f64x2(double *dst, const double *src);
void abyss_simd_pow_f64x2(double *dst, const double *a, const double *b);
void abyss_simd_sqrt_f64x2(double *dst, const double *src);
void abyss_simd_rsqrt_f64x2(double *dst, const double *src);
void abyss_simd_cbrt_f64x2(double *dst, const double *src);
void abyss_simd_hypot_f64x2(double *dst, const double *a, const double *b);
void abyss_simd_sinh_f64x2(double *dst, const double *src);
void abyss_simd_cosh_f64x2(double *dst, const double *src);
void abyss_simd_tanh_f64x2(double *dst, const double *src);
void abyss_simd_fma_f64x2(double *dst, const double *a, const double *b, const double *c);

uint32_t abyss_simd_cpu_features(void);

#endif
