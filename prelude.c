#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

void print(const uint8_t *s) {
    printf("%s", (const char *)s);
}

void print_char(uint8_t c) {
    putchar((int)c);
}

void print_i8(int8_t v) {
    printf("%d\n", (int)v);
}

void print_i16(int16_t v) {
    printf("%d\n", (int)v);
}

void print_i32(int32_t v) {
    printf("%d\n", v);
}

void print_i64(int64_t v) {
    printf("%lld\n", (long long)v);
}

void print_u8(uint8_t v) {
    printf("%u\n", (unsigned int)v);
}

void print_u16(uint16_t v) {
    printf("%u\n", (unsigned int)v);
}

void print_u32(uint32_t v) {
    printf("%u\n", v);
}

void print_u64(uint64_t v) {
    printf("%llu\n", (unsigned long long)v);
}

void print_f16(_Float16 v) {
    printf("%f\n", (double)v);
}

void print_f32(float v) {
    printf("%f\n", (double)v);
}

void print_f64(double v) {
    printf("%f\n", v);
}


void print_bool(bool b) {
    printf("%s\n", b ? "true" : "false");
}

void print_ptr(const void *p) {
    printf("%p\n", p);
}

// ------> String

uint64_t str_len(const uint8_t *s) {
    if (!s) return 0;
    return (uint64_t)strlen((const char *)s);
}

bool str_eq(const uint8_t *a, const uint8_t *b) {
    if (a == b) return true;
    if (!a || !b) return false;
    return strcmp((const char *)a, (const char *)b) == 0;
}

// ------> mem
void mem_copy(uint8_t *dest, const uint8_t *src, uint64_t n) {
    memcpy(dest, src, (size_t)n);
}