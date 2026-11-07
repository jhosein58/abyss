#include "simd_runtime.h"

uint32_t abyss_simd_cpu_features(void) {
    uint32_t flags = 1;
#if defined(__x86_64__) || defined(_M_X64) || defined(__i386__) || defined(_M_IX86)
    flags |= 2;
#if defined(__AVX__)
    flags |= 4;
#endif
#if defined(__AVX2__)
    flags |= 8;
#endif
#elif defined(__ARM_NEON) || defined(__aarch64__)
    flags |= 16;
#endif
    return flags;
}
