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

uint64_t abyss_vm_call_dyn(uint8_t *name, uint64_t a0, uint64_t a1, uint64_t a2, uint64_t a3, uint64_t a4, uint64_t a5, uint32_t arg_count) {
    (void)arg_count;
    if (!name) return 0;
    const char *s = (const char *)name;
    if (strcmp(s, "print_cstr") == 0 || strcmp(s, "print") == 0) { print_cstr((uint8_t *)a0); return 0; }
    if (strcmp(s, "print_bytes") == 0) { print_bytes((uint8_t *)a0, a1); return 0; }
    if (strcmp(s, "print_char") == 0) { print_char((uint8_t)a0); return 0; }
    if (strcmp(s, "print_bool") == 0) { print_bool((bool)a0); return 0; }
    if (strcmp(s, "print_i8") == 0) { print_i8((int8_t)a0); return 0; }
    if (strcmp(s, "print_i16") == 0) { print_i16((int16_t)a0); return 0; }
    if (strcmp(s, "print_i32") == 0) { print_i32((int32_t)a0); return 0; }
    if (strcmp(s, "print_i64") == 0) { print_i64((int64_t)a0); return 0; }
    if (strcmp(s, "print_u8") == 0) { print_u8((uint8_t)a0); return 0; }
    if (strcmp(s, "print_u16") == 0) { print_u16((uint16_t)a0); return 0; }
    if (strcmp(s, "print_u32") == 0) { print_u32((uint32_t)a0); return 0; }
    if (strcmp(s, "print_u64") == 0) { print_u64(a0); return 0; }
    if (strcmp(s, "print_f32") == 0) {
        float f; memcpy(&f, &a0, sizeof(float)); print_f32(f); return 0;
    }
    if (strcmp(s, "print_f64") == 0) {
        double d; memcpy(&d, &a0, sizeof(double)); print_f64(d); return 0;
    }
    if (strcmp(s, "malloc") == 0) return (uint64_t)(uintptr_t)malloc((size_t)a0);
    if (strcmp(s, "free") == 0) { free((void *)(uintptr_t)a0); return 0; }
    if (strcmp(s, "realloc") == 0) return (uint64_t)(uintptr_t)realloc((void *)(uintptr_t)a0, (size_t)a1);
    if (strcmp(s, "mem_copy") == 0) { mem_copy((uint8_t *)a0, (uint8_t *)a1, a2); return 0; }
    if (strcmp(s, "mem_set") == 0) { mem_set((uint8_t *)a0, (uint8_t)a1, a2); return 0; }
    if (strcmp(s, "str_len") == 0) return (uint64_t)strlen((const char *)a0);
    if (strcmp(s, "str_eq") == 0) return (uint64_t)(strcmp((const char *)a0, (const char *)a1) == 0);
    if (strcmp(s, "abyss_fall") == 0) { abyss_fall(); return 0; }
    print_cstr((uint8_t *)"\n[vm error] undefined external symbol '");
    print_cstr(name);
    print_cstr((uint8_t *)"'\n");
    abyss_fall();
    return 0;
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
#define popen _popen
#define pclose _pclose
#else
#include <dlfcn.h>
#endif

void abyss_vm_shift_args(void);
uint64_t abyss_vm_call_dyn(uint8_t *name, uint64_t a0, uint64_t a1, uint64_t a2, uint64_t a3, uint64_t a4, uint64_t a5, uint32_t arg_count);

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

#endif
