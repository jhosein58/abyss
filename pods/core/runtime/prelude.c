#if defined(__wasm__)

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;
typedef signed char int8_t;
typedef signed short int16_t;
typedef signed int int32_t;
typedef signed long long int64_t;
typedef unsigned long size_t;
typedef signed long intptr_t;
typedef unsigned long uintptr_t;
typedef _Bool bool;
#define true 1
#define false 0
#define NULL ((void *)0)

__attribute__((import_module("env"), import_name("js_print_bytes")))
extern void js_print_bytes(const uint8_t *ptr, uint32_t len);

__attribute__((import_module("env"), import_name("js_trap")))
extern void js_trap(void);

extern unsigned char __heap_base;
static uint32_t bump_ptr = 0;

void *malloc(size_t size) {
    if (!bump_ptr) bump_ptr = (uint32_t)(uintptr_t)&__heap_base;
    size = (size + 7) & ~7;
    uint32_t curr = bump_ptr;
    bump_ptr += (uint32_t)size;
    uint32_t mem_size = (uint32_t)__builtin_wasm_memory_size(0) * 65536;
    if (bump_ptr > mem_size) {
        uint32_t needed_pages = (bump_ptr - mem_size + 65535) / 65536;
        if (__builtin_wasm_memory_grow(0, needed_pages) == (size_t)-1) return NULL;
    }
    return (void *)(uintptr_t)curr;
}

void free(void *ptr) {
    (void)ptr;
}

void *memcpy(void *dest, const void *src, size_t n) {
    uint8_t *d = (uint8_t *)dest;
    const uint8_t *s = (const uint8_t *)src;
    for (size_t i = 0; i < n; i++) d[i] = s[i];
    return dest;
}

void *memset(void *dest, int val, size_t n) {
    uint8_t *d = (uint8_t *)dest;
    for (size_t i = 0; i < n; i++) d[i] = (uint8_t)val;
    return dest;
}

void *memmove(void *dest, const void *src, size_t n) {
    uint8_t *d = (uint8_t *)dest;
    const uint8_t *s = (const uint8_t *)src;
    if (d < s) {
        for (size_t i = 0; i < n; i++) d[i] = s[i];
    } else {
        for (size_t i = n; i > 0; i--) d[i - 1] = s[i - 1];
    }
    return dest;
}

size_t strlen(const char *s) {
    size_t len = 0;
    if (!s) return 0;
    while (s[len]) len++;
    return len;
}

int strcmp(const char *s1, const char *s2) {
    if (s1 == s2) return 0;
    if (!s1) return -1;
    if (!s2) return 1;
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(const unsigned char *)s1 - *(const unsigned char *)s2;
}

int strncmp(const char *s1, const char *s2, size_t n) {
    for (size_t i = 0; i < n; i++) {
        if (s1[i] != s2[i] || !s1[i]) return (unsigned char)s1[i] - (unsigned char)s2[i];
    }
    return 0;
}

void *realloc(void *ptr, size_t size) {
    void *new_p = malloc(size);
    if (ptr && new_p) memcpy(new_p, ptr, size);
    return new_p;
}

void print_bytes(uint8_t *s, uint64_t len) {
    if (len) js_print_bytes(s, (uint32_t)len);
}

void print_cstr(uint8_t *s) {
    if (!s) return;
    print_bytes(s, strlen((const char *)s));
}

void print(uint8_t *s) {
    print_cstr(s);
}

void print_char(uint8_t c) {
    print_bytes(&c, 1);
}

static void print_uint(uint64_t v) {
    uint8_t buf[32];
    uint32_t idx = 0;
    if (v == 0) {
        buf[idx++] = '0';
    } else {
        while (v > 0) {
            buf[idx++] = (uint8_t)('0' + (v % 10));
            v /= 10;
        }
    }
    for (uint32_t i = 0; i < idx / 2; i++) {
        uint8_t tmp = buf[i];
        buf[i] = buf[idx - 1 - i];
        buf[idx - 1 - i] = tmp;
    }
    print_bytes(buf, idx);
}

static void print_int(int64_t v) {
    if (v < 0) {
        print_char('-');
        v = -v;
    }
    print_uint((uint64_t)v);
}

void print_i8(int8_t v) { print_int((int64_t)v); }
void print_i16(int16_t v) { print_int((int64_t)v); }
void print_i32(int32_t v) { print_int((int64_t)v); }
void print_i64(int64_t v) { print_int(v); }
void print_u8(uint8_t v) { print_uint((uint64_t)v); }
void print_u16(uint16_t v) { print_uint((uint64_t)v); }
void print_u32(uint32_t v) { print_uint((uint64_t)v); }
void print_u64(uint64_t v) { print_uint(v); }
void print_bool(bool b) {
    if (b) print_bytes((uint8_t *)"true", 4);
    else print_bytes((uint8_t *)"false", 5);
}

uint64_t string_float64(uint8_t *ptr, uint64_t capacity, double value) {
    if (value < 0) {
        if (capacity > 0) *ptr = '-';
        return 1 + string_float64(ptr + 1, capacity > 1 ? capacity - 1 : 0, -value);
    }
    uint64_t int_part = (uint64_t)value;
    double frac = value - (double)int_part;
    uint32_t frac_int = (uint32_t)(frac * 1000000.0 + 0.5);
    uint8_t buf[64];
    uint32_t idx = 0;
    if (int_part == 0) buf[idx++] = '0';
    else {
        uint64_t t = int_part;
        while (t > 0) { buf[idx++] = (uint8_t)('0' + (t % 10)); t /= 10; }
        for (uint32_t i = 0; i < idx / 2; i++) {
            uint8_t tmp = buf[i]; buf[i] = buf[idx - 1 - i]; buf[idx - 1 - i] = tmp;
        }
    }
    buf[idx++] = '.';
    for (int p = 5; p >= 0; p--) {
        uint32_t div = 1;
        for (int k = 0; k < p; k++) div *= 10;
        buf[idx++] = (uint8_t)('0' + ((frac_int / div) % 10));
    }
    if (idx > capacity) idx = (uint32_t)capacity;
    memcpy(ptr, buf, idx);
    return idx;
}

void print_f64(double v) {
    uint8_t buf[64];
    uint64_t len = string_float64(buf, sizeof(buf), v);
    print_bytes(buf, len);
}
void print_f32(float v) {
    print_f64((double)v);
}
void print_f16(uint16_t v) {
    (void)v;
}

uint64_t str_len(uint8_t *s) {
    if (!s) return 0;
    return (uint64_t)strlen((char *)s);
}

bool str_eq(uint8_t *a, uint8_t *b) {
    if (a == b) return true;
    if (!a || !b) return false;
    return strcmp((const char *)a, (const char *)b) == 0;
}

void mem_copy(uint8_t *dest, uint8_t *src, uint64_t n) {
    if (!dest || !src || n == 0) return;
    if (n > 100 * 1024 * 1024) return;
    memcpy(dest, src, (size_t)n);
}

void mem_set(uint8_t *dest, uint8_t val, uint64_t n) {
    memset(dest, (int)val, (size_t)n);
}

typedef struct {
    char *path;
    uint8_t *data;
    uint32_t size;
    uint32_t capacity;
} WasmVfsFile;

#define WASM_VFS_MAX 1024
static WasmVfsFile wasm_vfs[WASM_VFS_MAX];
static uint32_t wasm_vfs_count = 0;

static const char *wasm_vfs_norm(const char *p) {
    if (!p) return "";
    while (1) {
        if (p[0] == '.' && p[1] == '/') { p += 2; continue; }
        if (p[0] == '/') { p += 1; continue; }
        break;
    }
    return p;
}

static int wasm_vfs_find(const char *path) {
    if (!path) return -1;
    const char *norm = wasm_vfs_norm(path);
    for (uint32_t i = 0; i < wasm_vfs_count; i++) {
        if (strcmp(wasm_vfs[i].path, norm) == 0) return (int)i;
    }
    return -1;
}

__attribute__((export_name("abyss_vfs_write")))
int abyss_vfs_write(const uint8_t *path_ptr, const uint8_t *data_ptr, uint32_t size) {
    if (!path_ptr) return -1;
    const char *norm = wasm_vfs_norm((const char *)path_ptr);
    int idx = wasm_vfs_find(norm);
    if (idx >= 0) {
        if (size > wasm_vfs[idx].capacity) {
            wasm_vfs[idx].data = (uint8_t *)realloc(wasm_vfs[idx].data, size + 1);
            wasm_vfs[idx].capacity = size;
        }
        if (size > 0 && data_ptr) memcpy(wasm_vfs[idx].data, data_ptr, size);
        wasm_vfs[idx].data[size] = 0;
        wasm_vfs[idx].size = size;
        return 0;
    }
    if (wasm_vfs_count >= WASM_VFS_MAX) return -1;
    size_t plen = strlen(norm);
    char *new_path = (char *)malloc(plen + 1);
    memcpy(new_path, norm, plen + 1);
    uint8_t *new_data = (uint8_t *)malloc(size + 1);
    if (size > 0 && data_ptr) memcpy(new_data, data_ptr, size);
    new_data[size] = 0;
    wasm_vfs[wasm_vfs_count].path = new_path;
    wasm_vfs[wasm_vfs_count].data = new_data;
    wasm_vfs[wasm_vfs_count].size = size;
    wasm_vfs[wasm_vfs_count].capacity = size;
    wasm_vfs_count++;
    return 0;
}

__attribute__((export_name("abyss_vfs_exists")))
int abyss_vfs_exists(const uint8_t *path_ptr) {
    return wasm_vfs_find((const char *)path_ptr) >= 0 ? 1 : 0;
}

__attribute__((export_name("abyss_vfs_read_size")))
uint32_t abyss_vfs_read_size(const uint8_t *path_ptr) {
    int idx = wasm_vfs_find((const char *)path_ptr);
    if (idx >= 0) return wasm_vfs[idx].size;
    return 0;
}

__attribute__((export_name("abyss_vfs_read_ptr")))
uint8_t *abyss_vfs_read_ptr(const uint8_t *path_ptr) {
    int idx = wasm_vfs_find((const char *)path_ptr);
    if (idx >= 0) return wasm_vfs[idx].data;
    return NULL;
}

__attribute__((export_name("abyss_vfs_file_count")))
uint32_t abyss_vfs_file_count(void) {
    return wasm_vfs_count;
}

__attribute__((export_name("abyss_vfs_reset")))
void abyss_vfs_reset(void) {
    wasm_vfs_count = 0;
}

typedef struct {
    int file_idx;
    uint32_t pos;
    bool is_open;
    bool is_write;
} WasmOpenFile;

#define WASM_MAX_OPEN_FILES 32
static WasmOpenFile wasm_open_files[WASM_MAX_OPEN_FILES];

uint8_t *file_open(uint8_t *path, uint8_t *mode) {
    if (!path) return NULL;
    const char *p = (const char *)path;
    const char *m = mode ? (const char *)mode : "r";
    bool is_write = (m[0] == 'w' || m[0] == 'a');
    int idx = wasm_vfs_find(p);
    if (idx < 0) {
        if (is_write) {
            if (abyss_vfs_write(path, (const uint8_t *)"", 0) != 0) return NULL;
            idx = wasm_vfs_find(p);
        } else {
            return NULL;
        }
    }
    for (int i = 0; i < WASM_MAX_OPEN_FILES; i++) {
        if (!wasm_open_files[i].is_open) {
            wasm_open_files[i].file_idx = idx;
            wasm_open_files[i].pos = (m[0] == 'a') ? wasm_vfs[idx].size : 0;
            wasm_open_files[i].is_open = true;
            wasm_open_files[i].is_write = is_write;
            return (uint8_t *)(uintptr_t)(i + 1);
        }
    }
    return NULL;
}

int32_t file_close(uint8_t *handle) {
    uintptr_t fd = (uintptr_t)handle;
    if (fd >= 1 && fd <= WASM_MAX_OPEN_FILES) {
        wasm_open_files[fd - 1].is_open = false;
        return 0;
    }
    return -1;
}

int32_t file_seek(uint8_t *handle, int64_t offset, int32_t w) {
    uintptr_t fd = (uintptr_t)handle;
    if (fd < 1 || fd > WASM_MAX_OPEN_FILES) return -1;
    WasmOpenFile *of = &wasm_open_files[fd - 1];
    if (!of->is_open) return -1;
    uint32_t fsize = wasm_vfs[of->file_idx].size;
    if (w == 0) of->pos = (uint32_t)offset;
    else if (w == 1) of->pos += (uint32_t)offset;
    else if (w == 2) of->pos = fsize + (uint32_t)offset;
    if (of->pos > fsize) of->pos = fsize;
    return 0;
}

int64_t file_tell(uint8_t *handle) {
    uintptr_t fd = (uintptr_t)handle;
    if (fd < 1 || fd > WASM_MAX_OPEN_FILES) return -1;
    return (int64_t)wasm_open_files[fd - 1].pos;
}

uint64_t file_read(uint8_t *handle, uint8_t *ptr, uint64_t size, uint64_t count) {
    uintptr_t fd = (uintptr_t)handle;
    if (fd < 1 || fd > WASM_MAX_OPEN_FILES) return 0;
    WasmOpenFile *of = &wasm_open_files[fd - 1];
    if (!of->is_open) return 0;
    uint32_t fsize = wasm_vfs[of->file_idx].size;
    uint64_t to_read = size * count;
    if (of->pos + to_read > fsize) to_read = (fsize > of->pos) ? (fsize - of->pos) : 0;
    if (to_read > 0) {
        memcpy(ptr, wasm_vfs[of->file_idx].data + of->pos, to_read);
        of->pos += (uint32_t)to_read;
    }
    return to_read / (size ? size : 1);
}

uint64_t file_write(uint8_t *handle, uint8_t *ptr, uint64_t size, uint64_t count) {
    uintptr_t fd = (uintptr_t)handle;
    if (fd < 1 || fd > WASM_MAX_OPEN_FILES) return 0;
    WasmOpenFile *of = &wasm_open_files[fd - 1];
    if (!of->is_open || !of->is_write) return 0;
    uint64_t to_write = size * count;
    if (to_write == 0) return 0;
    WasmVfsFile *vf = &wasm_vfs[of->file_idx];
    if (of->pos + to_write > vf->capacity) {
        uint32_t new_cap = (of->pos + (uint32_t)to_write) * 2;
        vf->data = (uint8_t *)realloc(vf->data, new_cap + 1);
        vf->capacity = new_cap;
    }
    memcpy(vf->data + of->pos, ptr, to_write);
    of->pos += (uint32_t)to_write;
    if (of->pos > vf->size) {
        vf->size = of->pos;
        vf->data[vf->size] = 0;
    }
    return count;
}

typedef struct {
    char prefix[128];
    uint32_t next_idx;
    bool is_open;
    char entry_name[128];
} WasmDir;

#define WASM_MAX_OPEN_DIRS 8
static WasmDir wasm_dirs[WASM_MAX_OPEN_DIRS];

uint8_t *opendir(uint8_t *path) {
    if (!path) return NULL;
    const char *norm = wasm_vfs_norm((const char *)path);
    for (int i = 0; i < WASM_MAX_OPEN_DIRS; i++) {
        if (!wasm_dirs[i].is_open) {
            wasm_dirs[i].is_open = true;
            wasm_dirs[i].next_idx = 0;
            size_t plen = strlen(norm);
            if (plen >= sizeof(wasm_dirs[i].prefix)) plen = sizeof(wasm_dirs[i].prefix) - 1;
            memcpy(wasm_dirs[i].prefix, norm, plen);
            wasm_dirs[i].prefix[plen] = 0;
            return (uint8_t *)(uintptr_t)(i + 1);
        }
    }
    return NULL;
}

uint8_t *readdir(uint8_t *dir) {
    uintptr_t dd = (uintptr_t)dir;
    if (dd < 1 || dd > WASM_MAX_OPEN_DIRS) return NULL;
    WasmDir *wd = &wasm_dirs[dd - 1];
    if (!wd->is_open) return NULL;
    size_t plen = strlen(wd->prefix);
    while (wd->next_idx < wasm_vfs_count) {
        const char *vp = wasm_vfs[wd->next_idx++].path;
        if (plen == 0 || (strncmp(vp, wd->prefix, plen) == 0 && (vp[plen] == '/' || vp[plen] == 0))) {
            const char *sub = (plen == 0) ? vp : (vp + plen + (vp[plen] == '/' ? 1 : 0));
            size_t k = 0;
            while (sub[k] && sub[k] != '/' && k < sizeof(wd->entry_name) - 1) {
                wd->entry_name[k] = sub[k];
                k++;
            }
            wd->entry_name[k] = 0;
            return (uint8_t *)wd->entry_name;
        }
    }
    return NULL;
}

int32_t closedir(uint8_t *dir) {
    uintptr_t dd = (uintptr_t)dir;
    if (dd >= 1 && dd <= WASM_MAX_OPEN_DIRS) {
        wasm_dirs[dd - 1].is_open = false;
        return 0;
    }
    return -1;
}

uint8_t *dir_open(uint8_t *path) { return opendir(path); }
uint8_t *dir_read(uint8_t *dir) { return readdir(dir); }
int32_t dir_close(uint8_t *dir) { return closedir(dir); }
int32_t dir_create(uint8_t *path) { (void)path; return 0; }

int32_t compile_and_run(uint8_t *path) { (void)path; return -1; }

static char arg0[] = "abyssc";
static char arg1[] = "r";
static char arg2[] = "main.a";
static char *wasm_args[3] = { arg0, arg1, arg2 };
int abyss_argc = 3;
char **abyss_argv = wasm_args;
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
    (void)name;
    return NULL;
}

void abyss_fall(void) {
    print_cstr((uint8_t *)"\n\033[1;35m[fall]\033[0m unfathomable.\n\n");
    js_trap();
}



int main(int argc, char **argv);

__attribute__((export_name("alloc_source_buffer")))
uint8_t *alloc_source_buffer(uint32_t size) {
    return (uint8_t *)malloc(size);
}

__attribute__((export_name("run_abyss_file")))
int run_abyss_file(const uint8_t *path, uint32_t path_len) {
    static char file_arg[256];
    if (path_len >= sizeof(file_arg)) path_len = sizeof(file_arg) - 1;
    memcpy(file_arg, path, path_len);
    file_arg[path_len] = 0;
    wasm_args[2] = file_arg;
    abyss_argc = 3;
    abyss_argv = wasm_args;
    abyss_vm_argc = 0;
    abyss_vm_argv = NULL;
    return main(abyss_argc, abyss_argv);
}

__attribute__((export_name("run_abyss")))
int run_abyss(const uint8_t *src, uint32_t len) {
    abyss_vfs_write((const uint8_t *)"main.a", src, len);
    return run_abyss_file((const uint8_t *)"main.a", 6);
}

#else

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

#include <dirent.h>
#include <sys/stat.h>
#if defined(_WIN32) || defined(_WIN64)
#include <windows.h>
#include <direct.h>
#define popen _popen
#define pclose _pclose
#else
#include <dlfcn.h>
#endif

void abyss_vm_shift_args(void);


void print_cstr(uint8_t *s) {
    printf("%s", (const char *)s);
}

void print(uint8_t *s) {
    print_cstr(s);
}

void print_bytes(uint8_t *s, uint64_t len) {
    if (len) fwrite(s, 1, (size_t)len, stdout);
}

uint64_t string_float64(uint8_t *ptr, uint64_t capacity, double value) {
    int length = snprintf((char *)ptr, (size_t)capacity, "%f", value);
    if (length < 0 || (uint64_t)length >= capacity) abort();
    return (uint64_t)length;
}

#ifdef __TINYC__
#define __int128 long long
#endif

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

#ifdef __TINYC__
typedef uint16_t _Float16;
#endif

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

uint64_t str_len(uint8_t *s) {
    if (!s) return 0;
    return (uint64_t)strlen((char *)s);
}

bool str_eq(uint8_t *a, uint8_t *b) {
    if (a == b) return true;
    if (!a || !b) return false;
    return strcmp((const char *)a, (const char *)b) == 0;
}

void mem_copy(uint8_t *dest, uint8_t *src, uint64_t n) {
    if (!dest || !src || n == 0) return;
    if (n > 100 * 1024 * 1024) return; 
    memcpy(dest, src, (size_t)n);
}

void mem_set(uint8_t *dest, uint8_t val, uint64_t n) {
    memset(dest, (int)val, (size_t)n);
}

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

uint8_t *dir_open(uint8_t *path) {
    return (uint8_t *)opendir((const char *)path);
}

uint8_t *dir_read(uint8_t *dir) {
    return (uint8_t *)readdir((DIR *)dir);
}

int32_t dir_close(uint8_t *dir) {
    return closedir((DIR *)dir);
}

int32_t dir_create(uint8_t *path) {
    if (!path) return -1;
#if defined(_WIN32) || defined(_WIN64)
    return _mkdir((const char *)path);
#else
    return mkdir((const char *)path, 0755);
#endif
}

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



#endif

typedef uint64_t (*AbyssVmCallback)(uint8_t **, uint8_t *, uint32_t, uint64_t *, uint32_t);

#if !defined(__wasm__) && !defined(ABYSS_HAS_LIBFFI) && defined(__has_include)
#if __has_include(<ffi.h>)
#include <ffi.h>
#define ABYSS_HAS_LIBFFI 1
#endif
#endif
#ifndef ABYSS_HAS_LIBFFI
#define ABYSS_HAS_LIBFFI 0
#endif

typedef struct AbyssFfiAllocation {
    struct AbyssFfiAllocation *next;
    union {
        uint64_t integer;
        double floating;
        void *pointer;
    } alignment;
} AbyssFfiAllocation;

typedef struct {
    AbyssFfiAllocation *allocations;
#if ABYSS_HAS_LIBFFI
    struct AbyssFfiSignature *signatures;
    struct AbyssFfiClosure *closures;
#endif
} AbyssFfiScope;

static void abyss_ffi_error(const char *message, const char *name) {
#if defined(__wasm__)
    print_cstr((uint8_t *)message);
    if (name) print_cstr((uint8_t *)name);
#else
    fprintf(stderr, "\n\033[1;31m[vm error]\033[0m %s%s%s\n", message, name ? ": " : "", name ? name : "");
#endif
    abyss_fall();
}

static AbyssFfiScope *abyss_ffi_scope(uint8_t **slot) {
    if (!*slot) {
        *slot = (uint8_t *)malloc(sizeof(AbyssFfiScope));
        if (!*slot) abyss_ffi_error("foreign call allocation failed", NULL);
        memset(*slot, 0, sizeof(AbyssFfiScope));
    }
    return (AbyssFfiScope *)*slot;
}

uint8_t *abyss_ffi_alloc(uint8_t **slot, uint64_t size) {
    AbyssFfiScope *scope = abyss_ffi_scope(slot);
    AbyssFfiAllocation *item = (AbyssFfiAllocation *)malloc(sizeof(*item) + (size_t)size);
    if (!item) abyss_ffi_error("foreign call allocation failed", NULL);
    item->next = scope->allocations;
    scope->allocations = item;
    memset(item + 1, 0, (size_t)size);
    return (uint8_t *)(item + 1);
}

#if !defined(__wasm__)
static void *abyss_ffi_symbol(const char *name) {
#if defined(_WIN32)
    HMODULE module = GetModuleHandleA(NULL);
    FARPROC symbol = GetProcAddress(module, name);
    if (!symbol) {
        module = GetModuleHandleA("msvcrt.dll");
        if (module) symbol = GetProcAddress(module, name);
    }
    return (void *)symbol;
#else
    return dlsym(RTLD_DEFAULT, name);
#endif
}
#endif

uint64_t abyss_ffi_symbol_address(uint8_t *name) {
#if !defined(__wasm__)
    void *symbol = abyss_ffi_symbol((const char *)name);
    if (symbol) return (uintptr_t)symbol;
#endif
    abyss_ffi_error("undefined external symbol", (const char *)name);
    return 0;
}

#if ABYSS_HAS_LIBFFI

enum {
    ABYSS_FFI_VOID, ABYSS_FFI_I8, ABYSS_FFI_U8, ABYSS_FFI_I16, ABYSS_FFI_U16,
    ABYSS_FFI_I32, ABYSS_FFI_U32, ABYSS_FFI_I64, ABYSS_FFI_U64, ABYSS_FFI_F32,
    ABYSS_FFI_F64, ABYSS_FFI_POINTER, ABYSS_FFI_RECORD, ABYSS_FFI_CALLBACK
};

typedef struct AbyssFfiSignature {
    struct AbyssFfiSignature *next;
    const uint32_t *key;
    const uint32_t *result;
    const uint32_t **params;
    ffi_cif cif;
} AbyssFfiSignature;

typedef struct AbyssFfiClosure {
    struct AbyssFfiClosure *next;
    AbyssFfiSignature *signature;
    ffi_closure *closure;
    void *code;
    uint8_t **scope;
    uint8_t *chunk;
    AbyssVmCallback invoke;
    uint32_t entry;
} AbyssFfiClosure;

static struct {
    ffi_status (*prep_cif)(ffi_cif *, ffi_abi, unsigned, ffi_type *, ffi_type **);
    void (*call)(ffi_cif *, void (*)(void), void *, void **);
    void *(*closure_alloc)(size_t, void **);
    void (*closure_free)(void *);
    ffi_status (*prep_closure)(ffi_closure *, ffi_cif *, void (*)(ffi_cif *, void *, void **, void *), void *, void *);
    ffi_type *types[ABYSS_FFI_POINTER + 1];
} abyss_libffi;

static void *abyss_ffi_library(void) {
#if defined(_WIN32)
    const char *names[] = {"libffi-8.dll", "libffi.dll"};
#elif defined(__APPLE__)
    const char *names[] = {"libffi.dylib", "/usr/lib/libffi.dylib"};
#else
    const char *names[] = {"libffi.so.8", "libffi.so.7", "libffi.so.6", "libffi.so"};
#endif
    for (size_t i = 0; i < sizeof(names) / sizeof(*names); ++i) {
#if defined(_WIN32)
        void *handle = (void *)LoadLibraryA(names[i]);
#else
        void *handle = dlopen(names[i], RTLD_LOCAL | RTLD_NOW);
#endif
        if (handle) return handle;
    }
    abyss_ffi_error("VM foreign calls require libffi", NULL);
    return NULL;
}

static void *abyss_ffi_load(void *handle, const char *name) {
#if defined(_WIN32)
    void *symbol = (void *)GetProcAddress((HMODULE)handle, name);
#else
    void *symbol = dlsym(handle, name);
#endif
    if (!symbol) abyss_ffi_error("missing libffi symbol", name);
    return symbol;
}

static void abyss_ffi_init(void) {
    if (abyss_libffi.call) return;
    void *handle = abyss_ffi_library();
    const char *types[] = {"void", "sint8", "uint8", "sint16", "uint16", "sint32", "uint32",
        "sint64", "uint64", "float", "double", "pointer"};
    for (size_t i = 0; i < sizeof(types) / sizeof(*types); ++i) {
        char name[32];
        snprintf(name, sizeof(name), "ffi_type_%s", types[i]);
        abyss_libffi.types[i] = (ffi_type *)abyss_ffi_load(handle, name);
    }
    abyss_libffi.prep_cif = (ffi_status (*)(ffi_cif *, ffi_abi, unsigned, ffi_type *, ffi_type **))abyss_ffi_load(handle, "ffi_prep_cif");
    abyss_libffi.closure_alloc = (void *(*)(size_t, void **))abyss_ffi_load(handle, "ffi_closure_alloc");
    abyss_libffi.closure_free = (void (*)(void *))abyss_ffi_load(handle, "ffi_closure_free");
    abyss_libffi.prep_closure = (ffi_status (*)(ffi_closure *, ffi_cif *, void (*)(ffi_cif *, void *, void **, void *), void *, void *))abyss_ffi_load(handle, "ffi_prep_closure_loc");
    abyss_libffi.call = (void (*)(ffi_cif *, void (*)(void), void *, void **))abyss_ffi_load(handle, "ffi_call");
}

static void abyss_ffi_skip(const uint32_t **cursor) {
    uint32_t kind = *(*cursor)++;
    if (kind == ABYSS_FFI_RECORD || kind == ABYSS_FFI_CALLBACK) {
        uint32_t count = *(*cursor)++ + (kind == ABYSS_FFI_CALLBACK);
        for (uint32_t i = 0; i < count; ++i) abyss_ffi_skip(cursor);
    }
}

static ffi_type *abyss_ffi_type(uint8_t **scope, const uint32_t **cursor) {
    uint32_t kind = **cursor;
    if (kind <= ABYSS_FFI_POINTER) {
        ++*cursor;
        return abyss_libffi.types[kind];
    }
    if (kind == ABYSS_FFI_CALLBACK) {
        abyss_ffi_skip(cursor);
        return abyss_libffi.types[ABYSS_FFI_POINTER];
    }
    if (kind == ABYSS_FFI_RECORD) {
        ++*cursor;
        uint32_t count = *(*cursor)++;
        ffi_type *type = (ffi_type *)abyss_ffi_alloc(scope, sizeof(*type));
        type->type = FFI_TYPE_STRUCT;
        type->elements = (ffi_type **)abyss_ffi_alloc(scope, (count + 1) * sizeof(*type->elements));
        for (uint32_t i = 0; i < count; ++i) type->elements[i] = abyss_ffi_type(scope, cursor);
        return type;
    }
    abyss_ffi_error("unsupported C ABI type in VM foreign call", NULL);
    return NULL;
}

static AbyssFfiSignature *abyss_ffi_signature(uint8_t **slot, const uint32_t *key) {
    if (!key) abyss_ffi_error("foreign call has no function signature", NULL);
    abyss_ffi_init();
    AbyssFfiScope *scope = abyss_ffi_scope(slot);
    for (AbyssFfiSignature *sig = scope->signatures; sig; sig = sig->next)
        if (sig->key == key) return sig;
    AbyssFfiSignature *sig = (AbyssFfiSignature *)abyss_ffi_alloc(slot, sizeof(*sig));
    const uint32_t *cursor = key;
    uint32_t count = *cursor++;
    if (count > UINT8_MAX) abyss_ffi_error("too many VM foreign call parameters", NULL);
    sig->key = key;
    sig->result = cursor;
    ffi_type *result = abyss_ffi_type(slot, &cursor);
    sig->params = (const uint32_t **)abyss_ffi_alloc(slot, count * sizeof(*sig->params));
    ffi_type **params = (ffi_type **)abyss_ffi_alloc(slot, count * sizeof(*params));
    for (uint32_t i = 0; i < count; ++i) {
        sig->params[i] = cursor;
        params[i] = abyss_ffi_type(slot, &cursor);
    }
    if (abyss_libffi.prep_cif(&sig->cif, FFI_DEFAULT_ABI, count, result, params) != FFI_OK)
        abyss_ffi_error("invalid C ABI signature", NULL);
    sig->next = scope->signatures;
    scope->signatures = sig;
    return sig;
}

uint64_t abyss_ffi_callback(uint8_t **slot, AbyssVmCallback invoke, uint8_t *chunk, uint32_t entry, uint32_t *key);

static uint64_t abyss_ffi_read(uint8_t **slot, uint32_t kind, const void *data, size_t size) {
    uint64_t value = 0;
    switch (kind) {
        case ABYSS_FFI_VOID: return 0;
        case ABYSS_FFI_I8: return (uint64_t)*(const int8_t *)data;
        case ABYSS_FFI_I16: return (uint64_t)*(const int16_t *)data;
        case ABYSS_FFI_I32: return (uint64_t)*(const int32_t *)data;
        case ABYSS_FFI_U8: return *(const uint8_t *)data;
        case ABYSS_FFI_U16: return *(const uint16_t *)data;
        case ABYSS_FFI_U32: return *(const uint32_t *)data;
        case ABYSS_FFI_F32: {
            uint32_t bits;
            memcpy(&bits, data, sizeof(bits));
            return bits;
        }
        case ABYSS_FFI_POINTER:
        case ABYSS_FFI_CALLBACK: return (uintptr_t)*(void *const *)data;
        case ABYSS_FFI_RECORD: {
            void *copy = abyss_ffi_alloc(slot, size);
            memcpy(copy, data, size);
            return (uintptr_t)copy;
        }
        default: memcpy(&value, data, sizeof(value)); return value;
    }
}

static void abyss_ffi_write(uint8_t **slot, AbyssVmCallback invoke, uint8_t *chunk, const uint32_t *type, void *out, uint64_t value, size_t size) {
    switch (*type) {
        case ABYSS_FFI_VOID: return;
        case ABYSS_FFI_I8:
        case ABYSS_FFI_U8: *(uint8_t *)out = (uint8_t)value; return;
        case ABYSS_FFI_I16:
        case ABYSS_FFI_U16: *(uint16_t *)out = (uint16_t)value; return;
        case ABYSS_FFI_I32:
        case ABYSS_FFI_U32:
        case ABYSS_FFI_F32: {
            uint32_t bits = (uint32_t)value;
            memcpy(out, &bits, sizeof(bits));
            return;
        }
        case ABYSS_FFI_CALLBACK:
            if (value >> 63) value = abyss_ffi_callback(slot, invoke, chunk, (uint32_t)value, (uint32_t *)(type + 1));
            if (value >> 62) abyss_ffi_error("foreign function value is not a native callback", NULL);
            *(void **)out = (void *)(uintptr_t)value;
            return;
        case ABYSS_FFI_POINTER: *(void **)out = (void *)(uintptr_t)value; return;
        case ABYSS_FFI_RECORD: memcpy(out, (void *)(uintptr_t)value, size); return;
        default: memcpy(out, &value, sizeof(value)); return;
    }
}

static void abyss_ffi_invoke(ffi_cif *cif, void *out, void **args, void *data) {
    AbyssFfiClosure *closure = (AbyssFfiClosure *)data;
    uint64_t values[256];
    for (unsigned i = 0; i < cif->nargs; ++i)
        values[i] = abyss_ffi_read(closure->scope, *closure->signature->params[i], args[i], cif->arg_types[i]->size);
    uint64_t result = closure->invoke(closure->scope, closure->chunk, closure->entry, values, cif->nargs);
    uint32_t kind = *closure->signature->result;
    if (kind >= ABYSS_FFI_I8 && kind <= ABYSS_FFI_U32)
        *(ffi_arg *)out = (ffi_arg)result;
    else
        abyss_ffi_write(closure->scope, closure->invoke, closure->chunk, closure->signature->result, out, result, cif->rtype->size);
}

uint64_t abyss_ffi_callback(uint8_t **slot, AbyssVmCallback invoke, uint8_t *chunk, uint32_t entry, uint32_t *key) {
    AbyssFfiSignature *sig = abyss_ffi_signature(slot, key);
    AbyssFfiScope *scope = abyss_ffi_scope(slot);
    for (AbyssFfiClosure *item = scope->closures; item; item = item->next)
        if (item->chunk == chunk && item->entry == entry && item->signature == sig) return (uintptr_t)item->code;
    AbyssFfiClosure *item = (AbyssFfiClosure *)abyss_ffi_alloc(slot, sizeof(*item));
    item->signature = sig;
    item->scope = slot;
    item->chunk = chunk;
    item->invoke = invoke;
    item->entry = entry;
    item->closure = (ffi_closure *)abyss_libffi.closure_alloc(sizeof(ffi_closure), &item->code);
    if (!item->closure || abyss_libffi.prep_closure(item->closure, &sig->cif, abyss_ffi_invoke, item, item->code) != FFI_OK)
        abyss_ffi_error("could not create a VM callback", NULL);
    item->next = scope->closures;
    scope->closures = item;
    return (uintptr_t)item->code;
}
#else
uint64_t abyss_ffi_callback(uint8_t **slot, AbyssVmCallback invoke, uint8_t *chunk, uint32_t entry, uint32_t *key) {
    (void)slot; (void)invoke; (void)chunk; (void)entry; (void)key;
    abyss_ffi_error("VM native callbacks require libffi", NULL);
    return 0;
}
#endif

uint64_t abyss_ffi_call(uint8_t **slot, AbyssVmCallback invoke, uint8_t *chunk, uint8_t *name, uint64_t address, uint32_t *key, uint64_t *args) {
#if !defined(__wasm__)
    if (name) address = abyss_ffi_symbol_address(name);
    if (!address) abyss_ffi_error("null foreign function pointer", NULL);
#endif
#if !defined(__wasm__) && !ABYSS_HAS_LIBFFI
    if (name && key && key[0] == 1 && key[1] == 5 && key[2] == 11 &&
        (strcmp((const char *)name, "puts") == 0 || strcmp((const char *)name, "remove") == 0)) {
        return (uint64_t)((int (*)(const char *))(uintptr_t)address)((const char *)(uintptr_t)args[0]);
    }
#endif
#if ABYSS_HAS_LIBFFI
    AbyssFfiSignature *sig = abyss_ffi_signature(slot, key);
    ffi_cif *cif = &sig->cif;
    uint64_t slots[256];
    void *values[256];
    for (unsigned i = 0; i < cif->nargs; ++i) {
        if (*sig->params[i] == ABYSS_FFI_RECORD) {
            values[i] = (void *)(uintptr_t)args[i];
        } else {
            values[i] = &slots[i];
            abyss_ffi_write(slot, invoke, chunk, sig->params[i], values[i], args[i], cif->arg_types[i]->size);
        }
    }
    uint32_t kind = *sig->result;
    uint64_t scalar = 0;
    void *result = kind == ABYSS_FFI_RECORD ? (void *)abyss_ffi_alloc(slot, cif->rtype->size) : (void *)&scalar;
    abyss_libffi.call(cif, (void (*)(void))(uintptr_t)address, result, values);
    if (kind == ABYSS_FFI_RECORD) return (uintptr_t)result;
    if (kind >= ABYSS_FFI_I8 && kind <= ABYSS_FFI_U32) {
        ffi_arg bits = *(ffi_arg *)result;
        if (kind == ABYSS_FFI_I8) return (uint64_t)(int8_t)bits;
        if (kind == ABYSS_FFI_I16) return (uint64_t)(int16_t)bits;
        if (kind == ABYSS_FFI_I32) return (uint64_t)(int32_t)bits;
        return (uint64_t)bits;
    }
    return abyss_ffi_read(slot, kind, result, cif->rtype->size);
#else
    (void)slot; (void)invoke; (void)chunk; (void)name; (void)address; (void)key; (void)args;
    abyss_ffi_error("VM foreign calls require libffi", NULL);
    return 0;
#endif
}

void abyss_ffi_destroy(uint8_t *ptr) {
    if (!ptr) return;
    AbyssFfiScope *scope = (AbyssFfiScope *)ptr;
#if ABYSS_HAS_LIBFFI
    for (AbyssFfiClosure *item = scope->closures; item; item = item->next)
        abyss_libffi.closure_free(item->closure);
#endif
    while (scope->allocations) {
        AbyssFfiAllocation *item = scope->allocations;
        scope->allocations = item->next;
        free(item);
    }
    free(scope);
}
