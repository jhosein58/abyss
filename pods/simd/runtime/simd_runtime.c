#include "simd_runtime.h"
#include <math.h>

#if defined(__x86_64__) || defined(_M_X64)
#include <immintrin.h>
#endif

void abyss_simd_sin_f32x4(float * __restrict__ dst, const float * __restrict__ src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = sinf(src[i]);
    }
}

void abyss_simd_cos_f32x4(float * __restrict__ dst, const float * __restrict__ src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = cosf(src[i]);
    }
}

void abyss_simd_tan_f32x4(float * __restrict__ dst, const float * __restrict__ src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = tanf(src[i]);
    }
}

void abyss_simd_exp_f32x4(float * __restrict__ dst, const float * __restrict__ src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = expf(src[i]);
    }
}

void abyss_simd_ln_f32x4(float * __restrict__ dst, const float * __restrict__ src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = logf(src[i]);
    }
}

void abyss_simd_sqrt_f32x4(float * __restrict__ dst, const float * __restrict__ src) {
#if defined(__SSE__)
    __m128 v = _mm_loadu_ps(src);
    _mm_storeu_ps(dst, _mm_sqrt_ps(v));
#else
    for (int i = 0; i < 4; i++) {
        dst[i] = sqrtf(src[i]);
    }
#endif
}

void abyss_simd_rsqrt_f32x4(float * __restrict__ dst, const float * __restrict__ src) {
#if defined(__SSE__)
    __m128 v = _mm_loadu_ps(src);
    _mm_storeu_ps(dst, _mm_rsqrt_ps(v));
#else
    for (int i = 0; i < 4; i++) {
        dst[i] = 1.0f / sqrtf(src[i]);
    }
#endif
}

void abyss_simd_pow_f32x4(float * __restrict__ dst, const float * __restrict__ a, const float * __restrict__ b) {
    for (int i = 0; i < 4; i++) {
        dst[i] = powf(a[i], b[i]);
    }
}

void abyss_simd_fma_f32x4(float * __restrict__ dst, const float * __restrict__ a, const float * __restrict__ b, const float * __restrict__ c) {
    for (int i = 0; i < 4; i++) {
        dst[i] = a[i] * b[i] + c[i];
    }
}

void abyss_simd_sin_f64x2(double * __restrict__ dst, const double * __restrict__ src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = sin(src[i]);
    }
}

void abyss_simd_cos_f64x2(double * __restrict__ dst, const double * __restrict__ src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = cos(src[i]);
    }
}

void abyss_simd_exp_f64x2(double * __restrict__ dst, const double * __restrict__ src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = exp(src[i]);
    }
}

void abyss_simd_ln_f64x2(double * __restrict__ dst, const double * __restrict__ src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = log(src[i]);
    }
}

void abyss_simd_sqrt_f64x2(double * __restrict__ dst, const double * __restrict__ src) {
#if defined(__SSE2__)
    __m128d v = _mm_loadu_pd(src);
    _mm_storeu_pd(dst, _mm_sqrt_pd(v));
#else
    for (int i = 0; i < 2; i++) {
        dst[i] = sqrt(src[i]);
    }
#endif
}

void abyss_simd_pow_f64x2(double * __restrict__ dst, const double * __restrict__ a, const double * __restrict__ b) {
    for (int i = 0; i < 2; i++) {
        dst[i] = pow(a[i], b[i]);
    }
}

void abyss_simd_fma_f64x2(double * __restrict__ dst, const double * __restrict__ a, const double * __restrict__ b, const double * __restrict__ c) {
    for (int i = 0; i < 2; i++) {
        dst[i] = a[i] * b[i] + c[i];
    }
}
