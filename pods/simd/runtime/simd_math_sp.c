#include "simd_runtime.h"
#include <math.h>

void abyss_simd_sin_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = sinf(src[i]);
    }
}

void abyss_simd_sin_fast_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        float x = src[i];
        float q = roundf(x * 0.636619772367581343f);
        int k = (int)q;
        float r = x - q * 1.5707963267948966f;
        float r2 = r * r;
        float s = r * (1.0f + r2 * (-0.16666657f + r2 * (0.008333025f - r2 * 0.000198074f)));
        float c = 1.0f + r2 * (-0.5f + r2 * (0.0416664f - r2 * 0.0013886f));
        dst[i] = (k & 1) ? ((k & 2) ? -c : c) : ((k & 2) ? -s : s);
    }
}

void abyss_simd_cos_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = cosf(src[i]);
    }
}

void abyss_simd_cos_fast_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        float x = src[i] + 1.5707963267948966f;
        float q = roundf(x * 0.636619772367581343f);
        int k = (int)q;
        float r = x - q * 1.5707963267948966f;
        float r2 = r * r;
        float s = r * (1.0f + r2 * (-0.16666657f + r2 * (0.008333025f - r2 * 0.000198074f)));
        float c = 1.0f + r2 * (-0.5f + r2 * (0.0416664f - r2 * 0.0013886f));
        dst[i] = (k & 1) ? ((k & 2) ? -c : c) : ((k & 2) ? -s : s);
    }
}

void abyss_simd_tan_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = tanf(src[i]);
    }
}

void abyss_simd_tan_fast_f32x4(float *dst, const float *src) {
    float s[4];
    float c[4];
    abyss_simd_sin_fast_f32x4(s, src);
    abyss_simd_cos_fast_f32x4(c, src);
    for (int i = 0; i < 4; i++) {
        dst[i] = s[i] / c[i];
    }
}

void abyss_simd_asin_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = asinf(src[i]);
    }
}

void abyss_simd_acos_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = acosf(src[i]);
    }
}

void abyss_simd_atan_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = atanf(src[i]);
    }
}

void abyss_simd_atan2_f32x4(float *dst, const float *y, const float *x) {
    for (int i = 0; i < 4; i++) {
        dst[i] = atan2f(y[i], x[i]);
    }
}

void abyss_simd_exp_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = expf(src[i]);
    }
}

void abyss_simd_exp_fast_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        float x = src[i];
        if (x > 88.0f) x = 88.0f;
        if (x < -88.0f) x = -88.0f;
        float k = roundf(x * 1.4426950408889634f);
        float r = x - k * 0.6931471805599453f;
        float p = 1.0f + r * (1.0f + r * (0.5f + r * 0.16666667f));
        union { float f; uint32_t u; } scale;
        scale.u = (uint32_t)((int)k + 127) << 23;
        dst[i] = p * scale.f;
    }
}

void abyss_simd_exp2_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = exp2f(src[i]);
    }
}

void abyss_simd_ln_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = logf(src[i]);
    }
}

void abyss_simd_ln_fast_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        float x = src[i];
        if (x <= 0.0f) {
            dst[i] = 0.0f;
            continue;
        }
        union { float f; uint32_t u; } conv;
        conv.f = x;
        int k = ((conv.u >> 23) & 0xff) - 127;
        conv.u = (conv.u & 0x007fffff) | 0x3f800000;
        float m = conv.f;
        float u = (m - 1.0f) / (m + 1.0f);
        float u2 = u * u;
        float p = u * (2.0f + u2 * 0.6666667f);
        dst[i] = (float)k * 0.6931471805599453f + p;
    }
}

void abyss_simd_log2_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = log2f(src[i]);
    }
}

void abyss_simd_log10_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = log10f(src[i]);
    }
}

void abyss_simd_pow_f32x4(float *dst, const float *a, const float *b) {
    for (int i = 0; i < 4; i++) {
        dst[i] = powf(a[i], b[i]);
    }
}

void abyss_simd_sqrt_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = sqrtf(src[i]);
    }
}

void abyss_simd_rsqrt_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = 1.0f / sqrtf(src[i]);
    }
}

void abyss_simd_rsqrt_fast_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        float x = src[i];
        if (x <= 0.0f) {
            dst[i] = 0.0f;
            continue;
        }
        union { float f; uint32_t u; } conv;
        conv.f = x;
        conv.u = 0x5f3759df - (conv.u >> 1);
        float y = conv.f;
        y = y * (1.5f - 0.5f * x * y * y);
        dst[i] = y;
    }
}

void abyss_simd_cbrt_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = cbrtf(src[i]);
    }
}

void abyss_simd_hypot_f32x4(float *dst, const float *a, const float *b) {
    for (int i = 0; i < 4; i++) {
        dst[i] = hypotf(a[i], b[i]);
    }
}

void abyss_simd_sinh_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = sinhf(src[i]);
    }
}

void abyss_simd_cosh_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = coshf(src[i]);
    }
}

void abyss_simd_tanh_f32x4(float *dst, const float *src) {
    for (int i = 0; i < 4; i++) {
        dst[i] = tanhf(src[i]);
    }
}

void abyss_simd_fma_f32x4(float *dst, const float *a, const float *b, const float *c) {
    for (int i = 0; i < 4; i++) {
        dst[i] = fmaf(a[i], b[i], c[i]);
    }
}
