#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

void print_i32(int32_t v) {
    printf("%d\n", v);
}

void print_star() {
    printf("*");
}

void print_new_line() {
    printf("\n");
}

void print(const uint8_t *s) {
    printf("%s", (const char *)s);
}
