#ifndef ABYSS_SIMD_RUNTIME_H
#define ABYSS_SIMD_RUNTIME_H

#include <stdint.h>
#include <stdbool.h>

void abyss_simd_sin_f32x4(float *dst, const float *src);
void abyss_simd_cos_f32x4(float *dst, const float *src);
void abyss_simd_tan_f32x4(float *dst, const float *src);
void abyss_simd_exp_f32x4(float *dst, const float *src);
void abyss_simd_ln_f32x4(float *dst, const float *src);
void abyss_simd_sqrt_f32x4(float *dst, const float *src);
void abyss_simd_rsqrt_f32x4(float *dst, const float *src);
void abyss_simd_pow_f32x4(float *dst, const float *a, const float *b);
void abyss_simd_fma_f32x4(float *dst, const float *a, const float *b, const float *c);

void abyss_simd_sin_f64x2(double *dst, const double *src);
void abyss_simd_cos_f64x2(double *dst, const double *src);
void abyss_simd_exp_f64x2(double *dst, const double *src);
void abyss_simd_ln_f64x2(double *dst, const double *src);
void abyss_simd_sqrt_f64x2(double *dst, const double *src);
void abyss_simd_pow_f64x2(double *dst, const double *a, const double *b);
void abyss_simd_fma_f64x2(double *dst, const double *a, const double *b, const double *c);

#endif
