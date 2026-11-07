#include "simd_runtime.h"
#include <math.h>

void abyss_simd_sin_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = sin(src[i]);
    }
}

void abyss_simd_sin_fast_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        double x = src[i];
        double q = round(x * 0.636619772367581343075535);
        int k = (int)q;
        double r = x - q * 1.57079632679489661923132;
        double r2 = r * r;
        double s = r * (1.0 + r2 * (-0.1666666666666663 + r2 * (0.00833333333330 + r2 * (-0.000198412698 + r2 * 0.00000275573))));
        double c = 1.0 + r2 * (-0.5 + r2 * (0.04166666666666 + r2 * (-0.00138888888888 + r2 * 0.000024801587)));
        dst[i] = (k & 1) ? ((k & 2) ? -c : c) : ((k & 2) ? -s : s);
    }
}

void abyss_simd_cos_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = cos(src[i]);
    }
}

void abyss_simd_cos_fast_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        double x = src[i] + 1.57079632679489661923132;
        double q = round(x * 0.636619772367581343075535);
        int k = (int)q;
        double r = x - q * 1.57079632679489661923132;
        double r2 = r * r;
        double s = r * (1.0 + r2 * (-0.1666666666666663 + r2 * (0.00833333333330 + r2 * (-0.000198412698 + r2 * 0.00000275573))));
        double c = 1.0 + r2 * (-0.5 + r2 * (0.04166666666666 + r2 * (-0.00138888888888 + r2 * 0.000024801587)));
        dst[i] = (k & 1) ? ((k & 2) ? -c : c) : ((k & 2) ? -s : s);
    }
}

void abyss_simd_tan_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = tan(src[i]);
    }
}

void abyss_simd_asin_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = asin(src[i]);
    }
}

void abyss_simd_acos_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = acos(src[i]);
    }
}

void abyss_simd_atan_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = atan(src[i]);
    }
}

void abyss_simd_atan2_f64x2(double *dst, const double *y, const double *x) {
    for (int i = 0; i < 2; i++) {
        dst[i] = atan2(y[i], x[i]);
    }
}

void abyss_simd_exp_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = exp(src[i]);
    }
}

void abyss_simd_exp_fast_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        double x = src[i];
        if (x > 709.0) x = 709.0;
        if (x < -709.0) x = -709.0;
        double k = round(x * 1.44269504088896340735992);
        double r = x - k * 0.69314718055994530941723;
        double p = 1.0 + r * (1.0 + r * (0.5 + r * (0.1666666666666667 + r * 0.04166666666666667)));
        union { double d; uint64_t u; } scale;
        scale.u = ((uint64_t)((int64_t)k + 1023)) << 52;
        dst[i] = p * scale.d;
    }
}

void abyss_simd_exp2_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = exp2(src[i]);
    }
}

void abyss_simd_ln_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = log(src[i]);
    }
}

void abyss_simd_ln_fast_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        double x = src[i];
        if (x <= 0.0) {
            dst[i] = 0.0;
            continue;
        }
        union { double d; uint64_t u; } conv;
        conv.d = x;
        int64_t k = (int64_t)((conv.u >> 52) & 0x7ff) - 1023;
        conv.u = (conv.u & 0x000fffffffffffffULL) | 0x3ff0000000000000ULL;
        double m = conv.d;
        double u = (m - 1.0) / (m + 1.0);
        double u2 = u * u;
        double p = u * (2.0 + u2 * (0.6666666666666667 + u2 * 0.4));
        dst[i] = (double)k * 0.69314718055994530941723 + p;
    }
}

void abyss_simd_log2_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = log2(src[i]);
    }
}

void abyss_simd_log10_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = log10(src[i]);
    }
}

void abyss_simd_pow_f64x2(double *dst, const double *a, const double *b) {
    for (int i = 0; i < 2; i++) {
        dst[i] = pow(a[i], b[i]);
    }
}

void abyss_simd_sqrt_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = sqrt(src[i]);
    }
}

void abyss_simd_rsqrt_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = 1.0 / sqrt(src[i]);
    }
}

void abyss_simd_cbrt_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = cbrt(src[i]);
    }
}

void abyss_simd_hypot_f64x2(double *dst, const double *a, const double *b) {
    for (int i = 0; i < 2; i++) {
        dst[i] = hypot(a[i], b[i]);
    }
}

void abyss_simd_sinh_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = sinh(src[i]);
    }
}

void abyss_simd_cosh_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = cosh(src[i]);
    }
}

void abyss_simd_tanh_f64x2(double *dst, const double *src) {
    for (int i = 0; i < 2; i++) {
        dst[i] = tanh(src[i]);
    }
}

void abyss_simd_fma_f64x2(double *dst, const double *a, const double *b, const double *c) {
    for (int i = 0; i < 2; i++) {
        dst[i] = fma(a[i], b[i], c[i]);
    }
}
