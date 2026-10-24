#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

#if defined(_WIN32) || defined(_WIN64)
#include <windows.h>
#define popen _popen
#define pclose _pclose
#else
#include <dlfcn.h>
#endif

void abyss_vm_shift_args(void);
uint64_t abyss_vm_call_dyn(uint8_t *name, uint64_t a0, uint64_t a1, uint64_t a2, uint64_t a3, uint64_t a4, uint64_t a5, uint32_t arg_count);

void print(uint8_t *s) {
    printf("%s", (const char *)s);
}

void print_bytes(uint8_t *s, uint64_t len) {
    if (len) fwrite(s, 1, (size_t)len, stdout);
}

#if defined(__SIZEOF_INT128__)
void print_u128(unsigned __int128 v) {
    uint8_t digits[40];
    uint32_t count = 0;
    do {
        digits[count++] = (uint8_t)('0' + v % 10);
        v /= 10;
    } while (v);
    while (count) putchar(digits[--count]);
}

void print_i128(__int128 v) {
    unsigned __int128 magnitude = (unsigned __int128)v;
    if (v < 0) {
        putchar('-');
        magnitude = 0 - magnitude;
    }
    print_u128(magnitude);
}
#endif

void print_char(uint8_t c) {
    putchar((int)c);
}

void print_i8(int8_t v) {
    printf("%d", (int)v);
}

void print_i16(int16_t v) {
    printf("%d", (int)v);
}

void print_i32(int32_t v) {
    printf("%d", v);
}

void print_i64(int64_t v) {
    printf("%lld", (long long)v);
}

void print_u8(uint8_t v) {
    printf("%u", (unsigned int)v);
}

void print_u16(uint16_t v) {
    printf("%u", (unsigned int)v);
}

void print_u32(uint32_t v) {
    printf("%u", v);
}

void print_u64(uint64_t v) {
    printf("%llu", (unsigned long long)v);
}

void print_f16(_Float16 v) {
    printf("%f", (double)v);
}

void print_f32(float v) {
    printf("%f", (double)v);
}

void print_f64(double v) {
    printf("%f", v);
}


void print_bool(bool b) {
    printf("%s", b ? "true" : "false");
}

// ------> String

uint64_t str_len(uint8_t *s) {
    if (!s) return 0;
    return (uint64_t)strlen((char *)s);
}

bool str_eq(uint8_t *a, uint8_t *b) {
    if (a == b) return true;
    if (!a || !b) return false;
    return strcmp((const char *)a, (const char *)b) == 0;
}

// ------> mem
void mem_copy(uint8_t *dest, uint8_t *src, uint64_t n) {
    if (!dest || !src || n == 0) return;
    if (n > 100 * 1024 * 1024) return; 
    memcpy(dest, src, (size_t)n);
}

void mem_set(uint8_t *dest, uint8_t val, uint64_t n) {
    memset(dest, (int)val, (size_t)n);
}

// ------> File Stream

uint8_t *file_open(uint8_t *path, uint8_t *mode) {
    return (uint8_t*)(fopen(path, mode));
}

int32_t file_seek(uint8_t *handle, int64_t offset, int32_t w) {
    return fseek((FILE*)handle, offset, w);
}

int64_t file_tell(uint8_t *handle) {
    return ftell((FILE*)handle);
}

uint64_t file_read(uint8_t *handle, uint8_t *ptr, uint64_t size, uint64_t count) {
    return fread(ptr, size ,count, (FILE*)handle);
}

int32_t file_close(uint8_t *handle) {
    return fclose((FILE*)handle);
}

uint64_t file_write(uint8_t *handle, uint8_t *ptr, uint64_t size, uint64_t count) {
    return fwrite(ptr, size, count, (FILE*)handle);
}

// ------> Process

int32_t compile_and_run(uint8_t *path) {
    if (!path) return -1;

    const char *p = (const char *)path;
    char out_path[1024];

    const char *last_slash = strrchr(p, '/');
#if defined(_WIN32) || defined(_WIN64)
    const char *last_bslash = strrchr(p, '\\');
    if (!last_slash || (last_bslash && last_bslash > last_slash)) {
        last_slash = last_bslash;
    }
    const char *bin_name = "stg1_out.exe";
#else
    const char *bin_name = "stg1_out";
#endif
    if (last_slash) {
        size_t dir_len = (size_t)(last_slash - p + 1);
        size_t bin_name_len = strlen(bin_name);
        if (dir_len + bin_name_len + 1 >= sizeof(out_path)) return -1;

        memcpy(out_path, p, dir_len);
        strcpy(out_path + dir_len, bin_name);
    } else {
#if defined(_WIN32) || defined(_WIN64)
        snprintf(out_path, sizeof(out_path), "%s", bin_name);
#else
        snprintf(out_path, sizeof(out_path), "./%s", bin_name);
#endif
    }

    char compile_cmd[2048];
    snprintf(compile_cmd, sizeof(compile_cmd), "gcc \"%s\" -o \"%s\" 2>&1", p, out_path);

    fprintf(stderr, "[CMD] %s\n", compile_cmd);
    fflush(stderr);

    FILE *fp = popen(compile_cmd, "r");
    if (!fp) {
        fprintf(stderr, "Failed to run popen for gcc\n");
        fflush(stderr);
        return -1;
    }

    char buffer[256];
    int has_compiler_output = 0;
    while (fgets(buffer, sizeof(buffer), fp) != NULL) {
        if (!has_compiler_output) {
            fprintf(stderr, "\n--- GCC Output ---\n");
            has_compiler_output = 1;
        }
        fputs(buffer, stderr);
        fflush(stderr); 
    }

    int compile_status = pclose(fp);
    if (compile_status != 0) {
        fprintf(stderr, "\n[GCC Exit Code]: %d\n", compile_status);
        fflush(stderr);
        return compile_status;
    }

    char run_cmd[1050];
    snprintf(run_cmd, sizeof(run_cmd), "\"%s\"", out_path);
    return (int32_t)system(run_cmd);
}

int abyss_argc = 0;
char **abyss_argv = NULL;
int abyss_vm_argc = 0;
char **abyss_vm_argv = NULL;

void abyss_vm_shift_args(void) {
    if (abyss_argc > 2 && (strcmp(abyss_argv[1], "r") == 0 || strcmp(abyss_argv[1], "run") == 0)) {
        abyss_vm_argc = abyss_argc - 2;
        abyss_vm_argv = abyss_argv + 2;
    }
}

int32_t get_arg_count(void) {
    if (abyss_vm_argv) return (int32_t)abyss_vm_argc;
    return (int32_t)abyss_argc;
}

uint8_t *get_arg(int32_t idx) {
    if (abyss_vm_argv) {
        if (idx < 0 || idx >= abyss_vm_argc) return NULL;
        return (uint8_t *)abyss_vm_argv[idx];
    }
    if (idx < 0 || idx >= abyss_argc) return NULL;
    return (uint8_t *)abyss_argv[idx];
}

uint8_t *get_env(uint8_t *name) {
    if (!name) return NULL;
    return (uint8_t *)getenv((const char *)name);
}

void abyss_fall(void) {
    fflush(stdout);
    fprintf(stderr, "\n\033[1;35m[fall]\033[0m unfathomable.\n\n");
    fflush(stderr);
    exit(1);
}

uint64_t abyss_vm_call_dyn(uint8_t *name, uint64_t a0, uint64_t a1, uint64_t a2, uint64_t a3, uint64_t a4, uint64_t a5, uint32_t arg_count) {
    if (!name) return 0;
#if defined(_WIN32) || defined(_WIN64)
    HMODULE mod = GetModuleHandle(NULL);
    FARPROC sym = GetProcAddress(mod, (const char *)name);
    if (!sym) {
        HMODULE crt = GetModuleHandleA("msvcrt.dll");
        if (crt) sym = GetProcAddress(crt, (const char *)name);
    }
#else
    void *sym = dlsym(RTLD_DEFAULT, (const char *)name);
#endif
    if (!sym) {
        fprintf(stderr, "\n\033[1;31m[vm error]\033[0m undefined external symbol '%s'\n", (const char *)name);
        abyss_fall();
        return 0;
    }
    typedef uint64_t (*fn_ptr)(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);
    return ((fn_ptr)sym)(a0, a1, a2, a3, a4, a5);
}