local host_os = package.config:sub(1, 1) == '\\' and "windows" or "linux"
local target_os = arg[1] or host_os

print("========================================")
print(">> Building Minimal TCC for: " .. target_os)
print(">> Host OS: " .. host_os)
print("========================================")

local src_dir = "third_party/tcc"
local libs_dir = "libs"
local minimal_dir = "tcc_minimal"
local include_out_dir = minimal_dir .. "/include"

local cc = "zig cc"
local ar = "zig ar"

--------------------------------------------------------------------------------
-- Utility Functions
--------------------------------------------------------------------------------

function exec(cmd)
    print("   [CMD] " .. cmd)
    local ret = os.execute(cmd)
    if ret ~= 0 and ret ~= true then
        print("!! Error executing command: " .. cmd)
        os.exit(1)
    end
end

function mkdir(path)
    if host_os == "windows" then
        os.execute('if not exist "' .. path:gsub("/", "\\") .. '" mkdir "' .. path:gsub("/", "\\") .. '"')
    else
        os.execute("mkdir -p '" .. path .. "'")
    end
end

function copy_file(src, dest)
    local input = io.open(src, "rb")
    if not input then
        print("   [WARN] Source file not found: " .. src)
        return false
    end
    local data = input:read("*all")
    input:close()

    local output = io.open(dest, "wb")
    if not output then
        print("   [ERR] Cannot write to: " .. dest)
        return false
    end
    output:write(data)
    output:close()
    print("   [CP] " .. src .. " -> " .. dest)
    return true
end

function write_file(path, content)
    local output = io.open(path, "wb")
    if not output then
        print("   [ERR] Cannot write to: " .. path)
        return false
    end
    output:write(content)
    output:close()
    print("   [GEN] " .. path)
    return true
end

function file_exists(path)
    local f = io.open(path, "r")
    if f then
        f:close()
        return true
    end
    return false
end

--------------------------------------------------------------------------------
-- Standard Headers Generation (Self-contained, no conflicts)
--------------------------------------------------------------------------------

local standard_headers = {}

-- stddef.h - Base definitions (TCC's own, modified to avoid conflicts)
standard_headers["stddef.h"] = [[
#ifndef _STDDEF_H
#define _STDDEF_H

#ifndef _SIZE_T_DEFINED
#define _SIZE_T_DEFINED
typedef unsigned long size_t;
#endif

#ifndef _PTRDIFF_T_DEFINED
#define _PTRDIFF_T_DEFINED
typedef long ptrdiff_t;
#endif

#ifndef _WCHAR_T_DEFINED
#define _WCHAR_T_DEFINED
#ifndef __cplusplus
typedef int wchar_t;
#endif
#endif

#ifndef NULL
#define NULL ((void*)0)
#endif

#define offsetof(type, member) ((size_t)&((type*)0)->member)

#endif /* _STDDEF_H */
]]

-- stdint.h - Integer types
standard_headers["stdint.h"] = [[
#ifndef _STDINT_H
#define _STDINT_H

/* Exact-width integer types */
typedef signed char        int8_t;
typedef unsigned char      uint8_t;
typedef short              int16_t;
typedef unsigned short     uint16_t;
typedef int                int32_t;
typedef unsigned int       uint32_t;
typedef long long          int64_t;
typedef unsigned long long uint64_t;

/* Minimum-width integer types */
typedef int8_t   int_least8_t;
typedef uint8_t  uint_least8_t;
typedef int16_t  int_least16_t;
typedef uint16_t uint_least16_t;
typedef int32_t  int_least32_t;
typedef uint32_t uint_least32_t;
typedef int64_t  int_least64_t;
typedef uint64_t uint_least64_t;

/* Fastest minimum-width integer types */
typedef int8_t   int_fast8_t;
typedef uint8_t  uint_fast8_t;
typedef int      int_fast16_t;
typedef unsigned uint_fast16_t;
typedef int      int_fast32_t;
typedef unsigned uint_fast32_t;
typedef int64_t  int_fast64_t;
typedef uint64_t uint_fast64_t;

/* Integer types capable of holding object pointers */
#ifndef _INTPTR_T_DEFINED
#define _INTPTR_T_DEFINED
#if defined(__x86_64__) || defined(_WIN64) || defined(__LP64__)
typedef long          intptr_t;
typedef unsigned long uintptr_t;
#else
typedef int           intptr_t;
typedef unsigned int  uintptr_t;
#endif
#endif

/* Greatest-width integer types */
typedef long long          intmax_t;
typedef unsigned long long uintmax_t;

/* Limits */
#define INT8_MIN   (-128)
#define INT8_MAX   127
#define UINT8_MAX  255
#define INT16_MIN  (-32768)
#define INT16_MAX  32767
#define UINT16_MAX 65535
#define INT32_MIN  (-2147483647-1)
#define INT32_MAX  2147483647
#define UINT32_MAX 4294967295U
#define INT64_MIN  (-9223372036854775807LL-1)
#define INT64_MAX  9223372036854775807LL
#define UINT64_MAX 18446744073709551615ULL

#define INTPTR_MIN  INT64_MIN
#define INTPTR_MAX  INT64_MAX
#define UINTPTR_MAX UINT64_MAX

#define SIZE_MAX    UINT64_MAX
#define PTRDIFF_MIN INT64_MIN
#define PTRDIFF_MAX INT64_MAX

#endif /* _STDINT_H */
]]

-- stdbool.h
standard_headers["stdbool.h"] = [[
#ifndef _STDBOOL_H
#define _STDBOOL_H

#ifndef __cplusplus
#define bool    _Bool
#define true    1
#define false   0
#endif

#define __bool_true_false_are_defined 1

#endif /* _STDBOOL_H */
]]

-- stdarg.h - Variable arguments
standard_headers["stdarg.h"] = [[
#ifndef _STDARG_H
#define _STDARG_H

typedef __builtin_va_list va_list;
#define va_start(ap, last) __builtin_va_start(ap, last)
#define va_arg(ap, type)   __builtin_va_arg(ap, type)
#define va_end(ap)         __builtin_va_end(ap)
#define va_copy(dest, src) __builtin_va_copy(dest, src)

#endif /* _STDARG_H */
]]

-- limits.h
standard_headers["limits.h"] = [[
#ifndef _LIMITS_H
#define _LIMITS_H

#define CHAR_BIT    8
#define SCHAR_MIN   (-128)
#define SCHAR_MAX   127
#define UCHAR_MAX   255
#define CHAR_MIN    SCHAR_MIN
#define CHAR_MAX    SCHAR_MAX
#define MB_LEN_MAX  16
#define SHRT_MIN    (-32768)
#define SHRT_MAX    32767
#define USHRT_MAX   65535
#define INT_MIN     (-2147483647-1)
#define INT_MAX     2147483647
#define UINT_MAX    4294967295U
#define LONG_MIN    (-9223372036854775807L-1)
#define LONG_MAX    9223372036854775807L
#define ULONG_MAX   18446744073709551615UL
#define LLONG_MIN   (-9223372036854775807LL-1)
#define LLONG_MAX   9223372036854775807LL
#define ULLONG_MAX  18446744073709551615ULL

#endif /* _LIMITS_H */
]]

-- float.h
standard_headers["float.h"] = [[
#ifndef _FLOAT_H
#define _FLOAT_H

#define FLT_RADIX       2
#define FLT_ROUNDS      1

#define FLT_DIG         6
#define FLT_EPSILON     1.19209290e-7F
#define FLT_MANT_DIG    24
#define FLT_MAX         3.40282347e+38F
#define FLT_MAX_10_EXP  38
#define FLT_MAX_EXP     128
#define FLT_MIN         1.17549435e-38F
#define FLT_MIN_10_EXP  (-37)
#define FLT_MIN_EXP     (-125)

#define DBL_DIG         15
#define DBL_EPSILON     2.2204460492503131e-16
#define DBL_MANT_DIG    53
#define DBL_MAX         1.7976931348623157e+308
#define DBL_MAX_10_EXP  308
#define DBL_MAX_EXP     1024
#define DBL_MIN         2.2250738585072014e-308
#define DBL_MIN_10_EXP  (-307)
#define DBL_MIN_EXP     (-1021)

#define LDBL_DIG        DBL_DIG
#define LDBL_EPSILON    DBL_EPSILON
#define LDBL_MANT_DIG   DBL_MANT_DIG
#define LDBL_MAX        DBL_MAX
#define LDBL_MAX_10_EXP DBL_MAX_10_EXP
#define LDBL_MAX_EXP    DBL_MAX_EXP
#define LDBL_MIN        DBL_MIN
#define LDBL_MIN_10_EXP DBL_MIN_10_EXP
#define LDBL_MIN_EXP    DBL_MIN_EXP

#endif /* _FLOAT_H */
]]

-- string.h
standard_headers["string.h"] = [[
#ifndef _STRING_H
#define _STRING_H

#include <stddef.h>

/* Copying */
void *memcpy(void *dest, const void *src, size_t n);
void *memmove(void *dest, const void *src, size_t n);
char *strcpy(char *dest, const char *src);
char *strncpy(char *dest, const char *src, size_t n);

/* Concatenation */
char *strcat(char *dest, const char *src);
char *strncat(char *dest, const char *src, size_t n);

/* Comparison */
int memcmp(const void *s1, const void *s2, size_t n);
int strcmp(const char *s1, const char *s2);
int strncmp(const char *s1, const char *s2, size_t n);
int strcoll(const char *s1, const char *s2);
size_t strxfrm(char *dest, const char *src, size_t n);

/* Search */
void *memchr(const void *s, int c, size_t n);
char *strchr(const char *s, int c);
size_t strcspn(const char *s1, const char *s2);
char *strpbrk(const char *s1, const char *s2);
char *strrchr(const char *s, int c);
size_t strspn(const char *s1, const char *s2);
char *strstr(const char *haystack, const char *needle);
char *strtok(char *str, const char *delim);

/* Other */
void *memset(void *s, int c, size_t n);
char *strerror(int errnum);
size_t strlen(const char *s);

/* Non-standard but common */
char *strdup(const char *s);
char *strndup(const char *s, size_t n);
int strcasecmp(const char *s1, const char *s2);
int strncasecmp(const char *s1, const char *s2, size_t n);

#endif /* _STRING_H */
]]

-- stdlib.h
standard_headers["stdlib.h"] = [[
#ifndef _STDLIB_H
#define _STDLIB_H

#include <stddef.h>

#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1
#define RAND_MAX     2147483647

typedef struct {
    int quot;
    int rem;
} div_t;

typedef struct {
    long quot;
    long rem;
} ldiv_t;

typedef struct {
    long long quot;
    long long rem;
} lldiv_t;

/* String conversion */
double atof(const char *str);
int atoi(const char *str);
long atol(const char *str);
long long atoll(const char *str);
double strtod(const char *str, char **endptr);
float strtof(const char *str, char **endptr);
long double strtold(const char *str, char **endptr);
long strtol(const char *str, char **endptr, int base);
long long strtoll(const char *str, char **endptr, int base);
unsigned long strtoul(const char *str, char **endptr, int base);
unsigned long long strtoull(const char *str, char **endptr, int base);

/* Pseudo-random */
int rand(void);
void srand(unsigned int seed);

/* Memory */
void *malloc(size_t size);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
void *aligned_alloc(size_t alignment, size_t size);

/* Environment */
void abort(void);
int atexit(void (*func)(void));
void exit(int status);
void _Exit(int status);
char *getenv(const char *name);
int system(const char *command);

/* Search and sort */
void *bsearch(const void *key, const void *base, size_t nmemb, size_t size,
              int (*compar)(const void *, const void *));
void qsort(void *base, size_t nmemb, size_t size,
           int (*compar)(const void *, const void *));

/* Integer arithmetic */
int abs(int n);
long labs(long n);
long long llabs(long long n);
div_t div(int numer, int denom);
ldiv_t ldiv(long numer, long denom);
lldiv_t lldiv(long long numer, long long denom);

/* Multibyte/wide char */
int mblen(const char *s, size_t n);
int mbtowc(wchar_t *pwc, const char *s, size_t n);
int wctomb(char *s, wchar_t wc);
size_t mbstowcs(wchar_t *dest, const char *src, size_t n);
size_t wcstombs(char *dest, const wchar_t *src, size_t n);

#endif /* _STDLIB_H */
]]

-- stdio.h
standard_headers["stdio.h"] = [[
#ifndef _STDIO_H
#define _STDIO_H

#include <stddef.h>
#include <stdarg.h>

#define EOF (-1)
#define BUFSIZ 8192
#define FILENAME_MAX 4096
#define FOPEN_MAX 16
#define L_tmpnam 20
#define TMP_MAX 238328

#define _IOFBF 0
#define _IOLBF 1
#define _IONBF 2

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

typedef struct _IO_FILE FILE;
typedef long fpos_t;

extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;

/* File operations */
int remove(const char *filename);
int rename(const char *oldname, const char *newname);
FILE *tmpfile(void);
char *tmpnam(char *s);

/* File access */
int fclose(FILE *stream);
int fflush(FILE *stream);
FILE *fopen(const char *filename, const char *mode);
FILE *freopen(const char *filename, const char *mode, FILE *stream);
void setbuf(FILE *stream, char *buf);
int setvbuf(FILE *stream, char *buf, int mode, size_t size);

/* Formatted I/O */
int fprintf(FILE *stream, const char *format, ...);
int fscanf(FILE *stream, const char *format, ...);
int printf(const char *format, ...);
int scanf(const char *format, ...);
int snprintf(char *s, size_t n, const char *format, ...);
int sprintf(char *s, const char *format, ...);
int sscanf(const char *s, const char *format, ...);
int vfprintf(FILE *stream, const char *format, va_list arg);
int vfscanf(FILE *stream, const char *format, va_list arg);
int vprintf(const char *format, va_list arg);
int vscanf(const char *format, va_list arg);
int vsnprintf(char *s, size_t n, const char *format, va_list arg);
int vsprintf(char *s, const char *format, va_list arg);
int vsscanf(const char *s, const char *format, va_list arg);

/* Character I/O */
int fgetc(FILE *stream);
char *fgets(char *s, int n, FILE *stream);
int fputc(int c, FILE *stream);
int fputs(const char *s, FILE *stream);
int getc(FILE *stream);
int getchar(void);
int putc(int c, FILE *stream);
int putchar(int c);
int puts(const char *s);
int ungetc(int c, FILE *stream);

/* Direct I/O */
size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream);
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream);

/* File positioning */
int fgetpos(FILE *stream, fpos_t *pos);
int fseek(FILE *stream, long offset, int whence);
int fsetpos(FILE *stream, const fpos_t *pos);
long ftell(FILE *stream);
void rewind(FILE *stream);

/* Error handling */
void clearerr(FILE *stream);
int feof(FILE *stream);
int ferror(FILE *stream);
void perror(const char *s);

#endif /* _STDIO_H */
]]

-- errno.h
standard_headers["errno.h"] = [[
#ifndef _ERRNO_H
#define _ERRNO_H

extern int errno;

#define EPERM        1
#define ENOENT       2
#define ESRCH        3
#define EINTR        4
#define EIO          5
#define ENXIO        6
#define E2BIG        7
#define ENOEXEC      8
#define EBADF        9
#define ECHILD      10
#define EAGAIN      11
#define ENOMEM      12
#define EACCES      13
#define EFAULT      14
#define EBUSY       16
#define EEXIST      17
#define EXDEV       18
#define ENODEV      19
#define ENOTDIR     20
#define EISDIR      21
#define EINVAL      22
#define ENFILE      23
#define EMFILE      24
#define ENOTTY      25
#define EFBIG       27
#define ENOSPC      28
#define ESPIPE      29
#define EROFS       30
#define EMLINK      31
#define EPIPE       32
#define EDOM        33
#define ERANGE      34
#define EDEADLK     35
#define ENAMETOOLONG 36
#define ENOLCK      37
#define ENOSYS      38
#define ENOTEMPTY   39
#define ELOOP       40
#define EWOULDBLOCK EAGAIN
#define EILSEQ      84

#endif /* _ERRNO_H */
]]

-- ctype.h
standard_headers["ctype.h"] = [[
#ifndef _CTYPE_H
#define _CTYPE_H

int isalnum(int c);
int isalpha(int c);
int isblank(int c);
int iscntrl(int c);
int isdigit(int c);
int isgraph(int c);
int islower(int c);
int isprint(int c);
int ispunct(int c);
int isspace(int c);
int isupper(int c);
int isxdigit(int c);
int tolower(int c);
int toupper(int c);

#endif /* _CTYPE_H */
]]

-- math.h
standard_headers["math.h"] = [[
#ifndef _MATH_H
#define _MATH_H

#define HUGE_VAL  (__builtin_huge_val())
#define HUGE_VALF (__builtin_huge_valf())
#define HUGE_VALL (__builtin_huge_vall())
#define INFINITY  (__builtin_inff())
#define NAN       (__builtin_nanf(""))

#define FP_INFINITE  1
#define FP_NAN       2
#define FP_NORMAL    3
#define FP_SUBNORMAL 4
#define FP_ZERO      5

#define M_E        2.71828182845904523536
#define M_LOG2E    1.44269504088896340736
#define M_LOG10E   0.43429448190325182765
#define M_LN2      0.69314718055994530942
#define M_LN10     2.30258509299404568402
#define M_PI       3.14159265358979323846
#define M_PI_2     1.57079632679489661923
#define M_PI_4     0.78539816339744830962
#define M_1_PI     0.31830988618379067154
#define M_2_PI     0.63661977236758134308
#define M_2_SQRTPI 1.12837916709551257390
#define M_SQRT2    1.41421356237309504880
#define M_SQRT1_2  0.70710678118654752440

/* Trigonometric */
double sin(double x);
double cos(double x);
double tan(double x);
double asin(double x);
double acos(double x);
double atan(double x);
double atan2(double y, double x);
float sinf(float x);
float cosf(float x);
float tanf(float x);

/* Hyperbolic */
double sinh(double x);
double cosh(double x);
double tanh(double x);
double asinh(double x);
double acosh(double x);
double atanh(double x);

/* Exponential and logarithmic */
double exp(double x);
double exp2(double x);
double expm1(double x);
double log(double x);
double log10(double x);
double log2(double x);
double log1p(double x);
float expf(float x);
float logf(float x);

/* Power */
double pow(double x, double y);
double sqrt(double x);
double cbrt(double x);
double hypot(double x, double y);
float powf(float x, float y);
float sqrtf(float x);

/* Rounding */
double ceil(double x);
double floor(double x);
double trunc(double x);
double round(double x);
double nearbyint(double x);
double rint(double x);
long lround(double x);
long long llround(double x);
float ceilf(float x);
float floorf(float x);
float roundf(float x);

/* Remainder */
double fmod(double x, double y);
double remainder(double x, double y);
float fmodf(float x, float y);

/* Manipulation */
double copysign(double x, double y);
double fabs(double x);
double fdim(double x, double y);
double fmax(double x, double y);
double fmin(double x, double y);
float fabsf(float x);

/* Other */
double frexp(double x, int *exp);
double ldexp(double x, int exp);
double modf(double x, double *iptr);
double scalbn(double x, int n);
int ilogb(double x);
double logb(double x);
double nextafter(double x, double y);

/* Classification */
int fpclassify(double x);
int isfinite(double x);
int isinf(double x);
int isnan(double x);
int isnormal(double x);
int signbit(double x);

#endif /* _MATH_H */
]]

-- time.h
standard_headers["time.h"] = [[
#ifndef _TIME_H
#define _TIME_H

#include <stddef.h>

#define CLOCKS_PER_SEC 1000000L

typedef long clock_t;
typedef long time_t;

struct tm {
    int tm_sec;
    int tm_min;
    int tm_hour;
    int tm_mday;
    int tm_mon;
    int tm_year;
    int tm_wday;
    int tm_yday;
    int tm_isdst;
};

struct timespec {
    time_t tv_sec;
    long   tv_nsec;
};

clock_t clock(void);
double difftime(time_t time1, time_t time0);
time_t mktime(struct tm *timeptr);
time_t time(time_t *timer);
char *asctime(const struct tm *timeptr);
char *ctime(const time_t *timer);
struct tm *gmtime(const time_t *timer);
struct tm *localtime(const time_t *timer);
size_t strftime(char *s, size_t maxsize, const char *format, const struct tm *timeptr);

#endif /* _TIME_H */
]]

-- assert.h
standard_headers["assert.h"] = [[
#ifndef _ASSERT_H
#define _ASSERT_H

#ifdef NDEBUG
#define assert(expression) ((void)0)
#else
void __assert_fail(const char *expr, const char *file, int line, const char *func);
#define assert(expression) \
    ((expression) ? (void)0 : __assert_fail(#expression, __FILE__, __LINE__, __func__))
#endif

#define static_assert _Static_assert

#endif /* _ASSERT_H */
]]

-- signal.h
standard_headers["signal.h"] = [[
#ifndef _SIGNAL_H
#define _SIGNAL_H

typedef int sig_atomic_t;
typedef void (*sighandler_t)(int);

#define SIG_DFL ((sighandler_t)0)
#define SIG_IGN ((sighandler_t)1)
#define SIG_ERR ((sighandler_t)-1)

#define SIGABRT  6
#define SIGFPE   8
#define SIGILL   4
#define SIGINT   2
#define SIGSEGV 11
#define SIGTERM 15

sighandler_t signal(int signum, sighandler_t handler);
int raise(int sig);

#endif /* _SIGNAL_H */
]]

-- setjmp.h
standard_headers["setjmp.h"] = [[
#ifndef _SETJMP_H
#define _SETJMP_H

typedef long jmp_buf[8];

int setjmp(jmp_buf env);
void longjmp(jmp_buf env, int val);

#endif /* _SETJMP_H */
]]

-- locale.h
standard_headers["locale.h"] = [[
#ifndef _LOCALE_H
#define _LOCALE_H

#include <stddef.h>

#define LC_ALL      0
#define LC_COLLATE  1
#define LC_CTYPE    2
#define LC_MONETARY 3
#define LC_NUMERIC  4
#define LC_TIME     5

struct lconv {
    char *decimal_point;
    char *thousands_sep;
    char *grouping;
    char *int_curr_symbol;
    char *currency_symbol;
    char *mon_decimal_point;
    char *mon_thousands_sep;
    char *mon_grouping;
    char *positive_sign;
    char *negative_sign;
    char int_frac_digits;
    char frac_digits;
    char p_cs_precedes;
    char p_sep_by_space;
    char n_cs_precedes;
    char n_sep_by_space;
    char p_sign_posn;
    char n_sign_posn;
};

char *setlocale(int category, const char *locale);
struct lconv *localeconv(void);

#endif /* _LOCALE_H */
]]

-- inttypes.h
standard_headers["inttypes.h"] = [[
#ifndef _INTTYPES_H
#define _INTTYPES_H

#include <stdint.h>

typedef struct {
    intmax_t quot;
    intmax_t rem;
} imaxdiv_t;

intmax_t imaxabs(intmax_t n);
imaxdiv_t imaxdiv(intmax_t numer, intmax_t denom);
intmax_t strtoimax(const char *nptr, char **endptr, int base);
uintmax_t strtoumax(const char *nptr, char **endptr, int base);

/* Format macros for printf */
#define PRId8  "d"
#define PRId16 "d"
#define PRId32 "d"
#define PRId64 "lld"
#define PRIi8  "i"
#define PRIi16 "i"
#define PRIi32 "i"
#define PRIi64 "lli"
#define PRIu8  "u"
#define PRIu16 "u"
#define PRIu32 "u"
#define PRIu64 "llu"
#define PRIx8  "x"
#define PRIx16 "x"
#define PRIx32 "x"
#define PRIx64 "llx"
#define PRIX8  "X"
#define PRIX16 "X"
#define PRIX32 "X"
#define PRIX64 "llX"

/* Format macros for scanf */
#define SCNd8  "hhd"
#define SCNd16 "hd"
#define SCNd32 "d"
#define SCNd64 "lld"
#define SCNi8  "hhi"
#define SCNi16 "hi"
#define SCNi32 "i"
#define SCNi64 "lli"
#define SCNu8  "hhu"
#define SCNu16 "hu"
#define SCNu32 "u"
#define SCNu64 "llu"

#endif /* _INTTYPES_H */
]]

-- fenv.h
standard_headers["fenv.h"] = [[
#ifndef _FENV_H
#define _FENV_H

typedef unsigned int fexcept_t;
typedef struct {
    unsigned int __control;
    unsigned int __status;
} fenv_t;

#define FE_INVALID    1
#define FE_DIVBYZERO  4
#define FE_OVERFLOW   8
#define FE_UNDERFLOW 16
#define FE_INEXACT   32
#define FE_ALL_EXCEPT (FE_INVALID|FE_DIVBYZERO|FE_OVERFLOW|FE_UNDERFLOW|FE_INEXACT)

#define FE_TONEAREST  0
#define FE_DOWNWARD   1
#define FE_UPWARD     2
#define FE_TOWARDZERO 3

extern const fenv_t __fe_dfl_env;
#define FE_DFL_ENV (&__fe_dfl_env)

int feclearexcept(int excepts);
int fegetexceptflag(fexcept_t *flagp, int excepts);
int feraiseexcept(int excepts);
int fesetexceptflag(const fexcept_t *flagp, int excepts);
int fetestexcept(int excepts);
int fegetround(void);
int fesetround(int round);
int fegetenv(fenv_t *envp);
int feholdexcept(fenv_t *envp);
int fesetenv(const fenv_t *envp);
int feupdateenv(const fenv_t *envp);

#endif /* _FENV_H */
]]

-- stdalign.h
standard_headers["stdalign.h"] = [[
#ifndef _STDALIGN_H
#define _STDALIGN_H

#define alignas _Alignas
#define alignof _Alignof

#define __alignas_is_defined 1
#define __alignof_is_defined 1

#endif /* _STDALIGN_H */
]]

-- stdnoreturn.h
standard_headers["stdnoreturn.h"] = [[
#ifndef _STDNORETURN_H
#define _STDNORETURN_H

#define noreturn _Noreturn

#endif /* _STDNORETURN_H */
]]

--------------------------------------------------------------------------------
-- Windows-specific headers
--------------------------------------------------------------------------------

local windows_headers = {}

windows_headers["windows.h"] = [[
#ifndef _WINDOWS_H
#define _WINDOWS_H

#include <stddef.h>
#include <stdint.h>

/* Basic Windows types */
typedef void *HANDLE;
typedef void *PVOID;
typedef void *LPVOID;
typedef const void *LPCVOID;
typedef int BOOL;
typedef unsigned char BYTE;
typedef unsigned short WORD;
typedef unsigned long DWORD;
typedef unsigned int UINT;
typedef long LONG;
typedef unsigned long ULONG;
typedef long long LONGLONG;
typedef unsigned long long ULONGLONG;
typedef wchar_t WCHAR;
typedef char *LPSTR;
typedef const char *LPCSTR;
typedef wchar_t *LPWSTR;
typedef const wchar_t *LPCWSTR;
typedef DWORD *LPDWORD;

typedef intptr_t INT_PTR;
typedef uintptr_t UINT_PTR;
typedef intptr_t LONG_PTR;
typedef uintptr_t ULONG_PTR;
typedef ULONG_PTR SIZE_T;
typedef LONG_PTR SSIZE_T;

#define TRUE  1
#define FALSE 0
#define NULL  ((void*)0)

#define WINAPI __attribute__((stdcall))
#define CALLBACK __attribute__((stdcall))
#define APIENTRY WINAPI

#define INVALID_HANDLE_VALUE ((HANDLE)(LONG_PTR)-1)

/* Error codes */
#define ERROR_SUCCESS 0
#define ERROR_FILE_NOT_FOUND 2
#define ERROR_ACCESS_DENIED 5
#define ERROR_INVALID_HANDLE 6
#define ERROR_NOT_ENOUGH_MEMORY 8

DWORD WINAPI GetLastError(void);
void WINAPI SetLastError(DWORD dwErrCode);

/* Memory */
LPVOID WINAPI VirtualAlloc(LPVOID lpAddress, SIZE_T dwSize, DWORD flAllocationType, DWORD flProtect);
BOOL WINAPI VirtualFree(LPVOID lpAddress, SIZE_T dwSize, DWORD dwFreeType);

#define MEM_COMMIT  0x1000
#define MEM_RESERVE 0x2000
#define MEM_RELEASE 0x8000
#define PAGE_READWRITE 0x04
#define PAGE_EXECUTE_READWRITE 0x40

/* Console */
HANDLE WINAPI GetStdHandle(DWORD nStdHandle);
#define STD_INPUT_HANDLE  ((DWORD)-10)
#define STD_OUTPUT_HANDLE ((DWORD)-11)
#define STD_ERROR_HANDLE  ((DWORD)-12)

BOOL WINAPI WriteConsoleA(HANDLE hConsoleOutput, LPCVOID lpBuffer, DWORD nNumberOfCharsToWrite,
                          LPDWORD lpNumberOfCharsWritten, LPVOID lpReserved);
BOOL WINAPI ReadConsoleA(HANDLE hConsoleInput, LPVOID lpBuffer, DWORD nNumberOfCharsToRead,
                         LPDWORD lpNumberOfCharsRead, LPVOID pInputControl);

/* Process */
void WINAPI ExitProcess(UINT uExitCode);
HANDLE WINAPI GetCurrentProcess(void);
DWORD WINAPI GetCurrentProcessId(void);

#endif /* _WINDOWS_H */
]]

--------------------------------------------------------------------------------
-- Linux-specific headers
--------------------------------------------------------------------------------

local linux_headers = {}

linux_headers["unistd.h"] = [[
#ifndef _UNISTD_H
#define _UNISTD_H

#include <stddef.h>

typedef int pid_t;
typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef long ssize_t;
typedef long off_t;

#define STDIN_FILENO  0
#define STDOUT_FILENO 1
#define STDERR_FILENO 2

/* File access */
#define R_OK 4
#define W_OK 2
#define X_OK 1
#define F_OK 0

ssize_t read(int fd, void *buf, size_t count);
ssize_t write(int fd, const void *buf, size_t count);
int close(int fd);
off_t lseek(int fd, off_t offset, int whence);
int access(const char *pathname, int mode);
int unlink(const char *pathname);

/* Process */
pid_t fork(void);
pid_t getpid(void);
pid_t getppid(void);
uid_t getuid(void);
gid_t getgid(void);
int execv(const char *pathname, char *const argv[]);
int execve(const char *pathname, char *const argv[], char *const envp[]);
int execvp(const char *file, char *const argv[]);
void _exit(int status);

/* Directory */
int chdir(const char *path);
char *getcwd(char *buf, size_t size);

/* Sleep */
unsigned int sleep(unsigned int seconds);
int usleep(unsigned int usec);

/* Other */
int isatty(int fd);
int dup(int oldfd);
int dup2(int oldfd, int newfd);
int pipe(int pipefd[2]);

#endif /* _UNISTD_H */
]]

linux_headers["fcntl.h"] = [[
#ifndef _FCNTL_H
#define _FCNTL_H

#include <stddef.h>

#define O_RDONLY    0x0000
#define O_WRONLY    0x0001
#define O_RDWR      0x0002
#define O_CREAT     0x0040
#define O_EXCL      0x0080
#define O_TRUNC     0x0200
#define O_APPEND    0x0400
#define O_NONBLOCK  0x0800

#define F_DUPFD  0
#define F_GETFD  1
#define F_SETFD  2
#define F_GETFL  3
#define F_SETFL  4

#define FD_CLOEXEC 1

int open(const char *pathname, int flags, ...);
int fcntl(int fd, int cmd, ...);
int creat(const char *pathname, unsigned int mode);

#endif /* _FCNTL_H */
]]

linux_headers["sys/types.h"] = [[
#ifndef _SYS_TYPES_H
#define _SYS_TYPES_H

#include <stddef.h>
#include <stdint.h>

typedef int pid_t;
typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef unsigned int mode_t;
typedef long ssize_t;
typedef long off_t;
typedef long long off64_t;
typedef unsigned long dev_t;
typedef unsigned long ino_t;
typedef unsigned long nlink_t;
typedef long blksize_t;
typedef long blkcnt_t;
typedef long time_t;
typedef long suseconds_t;

#endif /* _SYS_TYPES_H */
]]

linux_headers["sys/stat.h"] = [[
#ifndef _SYS_STAT_H
#define _SYS_STAT_H

#include <sys/types.h>

struct stat {
    dev_t     st_dev;
    ino_t     st_ino;
    mode_t    st_mode;
    nlink_t   st_nlink;
    uid_t     st_uid;
    gid_t     st_gid;
    dev_t     st_rdev;
    off_t     st_size;
    blksize_t st_blksize;
    blkcnt_t  st_blocks;
    time_t    st_atime;
    time_t    st_mtime;
    time_t    st_ctime;
};

#define S_IFMT   0170000
#define S_IFSOCK 0140000
#define S_IFLNK  0120000
#define S_IFREG  0100000
#define S_IFBLK  0060000
#define S_IFDIR  0040000
#define S_IFCHR  0020000
#define S_IFIFO  0010000

#define S_ISREG(m)  (((m) & S_IFMT) == S_IFREG)
#define S_ISDIR(m)  (((m) & S_IFMT) == S_IFDIR)
#define S_ISCHR(m)  (((m) & S_IFMT) == S_IFCHR)
#define S_ISBLK(m)  (((m) & S_IFMT) == S_IFBLK)
#define S_ISFIFO(m) (((m) & S_IFMT) == S_IFIFO)
#define S_ISLNK(m)  (((m) & S_IFMT) == S_IFLNK)
#define S_ISSOCK(m) (((m) & S_IFMT) == S_IFSOCK)

#define S_IRWXU 0700
#define S_IRUSR 0400
#define S_IWUSR 0200
#define S_IXUSR 0100
#define S_IRWXG 0070
#define S_IRGRP 0040
#define S_IWGRP 0020
#define S_IXGRP 0010
#define S_IRWXO 0007
#define S_IROTH 0004
#define S_IWOTH 0002
#define S_IXOTH 0001

int stat(const char *pathname, struct stat *statbuf);
int fstat(int fd, struct stat *statbuf);
int lstat(const char *pathname, struct stat *statbuf);
int chmod(const char *pathname, mode_t mode);
int fchmod(int fd, mode_t mode);
int mkdir(const char *pathname, mode_t mode);
mode_t umask(mode_t mask);

#endif /* _SYS_STAT_H */
]]

linux_headers["sys/mman.h"] = [[
#ifndef _SYS_MMAN_H
#define _SYS_MMAN_H

#include <stddef.h>

#define PROT_NONE  0x0
#define PROT_READ  0x1
#define PROT_WRITE 0x2
#define PROT_EXEC  0x4

#define MAP_SHARED    0x01
#define MAP_PRIVATE   0x02
#define MAP_FIXED     0x10
#define MAP_ANONYMOUS 0x20
#define MAP_ANON      MAP_ANONYMOUS

#define MAP_FAILED ((void*)-1)

#define MS_ASYNC      1
#define MS_SYNC       4
#define MS_INVALIDATE 2

void *mmap(void *addr, size_t length, int prot, int flags, int fd, long offset);
int munmap(void *addr, size_t length);
int mprotect(void *addr, size_t len, int prot);
int msync(void *addr, size_t length, int flags);

#endif /* _SYS_MMAN_H */
]]

linux_headers["dlfcn.h"] = [[
#ifndef _DLFCN_H
#define _DLFCN_H

#define RTLD_LAZY     0x00001
#define RTLD_NOW      0x00002
#define RTLD_GLOBAL   0x00100
#define RTLD_LOCAL    0x00000
#define RTLD_NOLOAD   0x00004
#define RTLD_NODELETE 0x01000
#define RTLD_DEEPBIND 0x00008

#define RTLD_DEFAULT  ((void*)0)
#define RTLD_NEXT     ((void*)-1)

void *dlopen(const char *filename, int flags);
int dlclose(void *handle);
void *dlsym(void *handle, const char *symbol);
char *dlerror(void);

#endif /* _DLFCN_H */
]]

--------------------------------------------------------------------------------
-- Build Process
--------------------------------------------------------------------------------

mkdir(libs_dir)
mkdir(minimal_dir)
mkdir(include_out_dir)

-- Create sys subdirectory for Linux
if target_os == "linux" then
    mkdir(include_out_dir .. "/sys")
end

print("\n>> [1/4] Generating Standard Headers...")

-- Write all standard headers
for name, content in pairs(standard_headers) do
    write_file(include_out_dir .. "/" .. name, content)
end

-- Write OS-specific headers
if target_os == "windows" then
    for name, content in pairs(windows_headers) do
        write_file(include_out_dir .. "/" .. name, content)
    end
elseif target_os == "linux" then
    for name, content in pairs(linux_headers) do
        write_file(include_out_dir .. "/" .. name, content)
    end
end

print("\n>> [2/4] Building libtcc.a (Host Library)...")

local build_cc = cc
local cflags_host = "-c -Os -fno-stack-protector -w"

-- Architecture detection
cflags_host = cflags_host .. " -DTCC_TARGET_X86_64"

if target_os == "linux" then
    cflags_host = cflags_host .. " -DTCC_IS_NATIVE"
    cflags_host = cflags_host .. " -D_GNU_SOURCE"
    cflags_host = cflags_host .. " -DCONFIG_TCC_BACKTRACE"
    -- Use morecore instead of selinux for simplicity
    cflags_host = cflags_host .. " -DHAVE_MORECORE"
elseif target_os == "windows" then
    cflags_host = cflags_host .. " -DTCC_TARGET_PE"
    cflags_host = cflags_host .. " -DTCC_IS_NATIVE"
    if host_os == "linux" then
        build_cc = cc .. " -target x86_64-windows-gnu"
    end
end

exec(string.format("%s %s -I%s -o tcc.o %s/libtcc.c", build_cc, cflags_host, src_dir, src_dir))

local libtcc_path = libs_dir .. "/libtcc.a"
exec(string.format("%s rcs %s tcc.o", ar, libtcc_path))

if host_os == "windows" then
    os.execute('del tcc.o 2>nul')
else
    os.remove("tcc.o")
end

print("\n>> [3/4] Building libtcc1.a (Runtime Support)...")

local libtcc1_src = src_dir .. "/lib/libtcc1.c"

-- Check if runtime source exists
if not file_exists(libtcc1_src) then
    print("   [WARN] libtcc1.c not found, skipping runtime library")
else
    local cflags_rt = "-c -Os -fno-stack-protector -w -DTCC_TARGET_X86_64"

    if target_os == "windows" then
        cflags_rt = cflags_rt .. " -DTCC_TARGET_PE"
    end

    exec(string.format("%s %s -I%s -o libtcc1.o %s", build_cc, cflags_rt, src_dir, libtcc1_src))

    local rt_lib_path = minimal_dir .. "/libtcc1.a"
    exec(string.format("%s rcs %s libtcc1.o", ar, rt_lib_path))

    if host_os == "windows" then
        os.execute('del libtcc1.o 2>nul')
    else
        os.remove("libtcc1.o")
    end
end

print("\n>> [4/4] Copying TCC internal headers...")

-- Copy TCC's own headers that might be needed
local tcc_internal_headers = {
    "tccdefs.h"
}

for _, header in ipairs(tcc_internal_headers) do
    local src_h = src_dir .. "/" .. header
    if file_exists(src_h) then
        copy_file(src_h, include_out_dir .. "/" .. header)
    end
end

-- Copy libtcc.h for API usage
local libtcc_h = src_dir .. "/libtcc.h"
if file_exists(libtcc_h) then
    copy_file(libtcc_h, minimal_dir .. "/libtcc.h")
end

--------------------------------------------------------------------------------
-- Summary
--------------------------------------------------------------------------------

print("\n========================================")
print(">> BUILD SUCCESSFUL!")
print("========================================")
print("   Target OS:      " .. target_os)
print("   Host OS:        " .. host_os)
print("   Library:        " .. libtcc_path)
print("   Runtime:        " .. minimal_dir .. "/libtcc1.a")
print("   Headers:        " .. include_out_dir .. "/")
print("   TCC API:        " .. minimal_dir .. "/libtcc.h")
print("========================================")
print("\nUsage in Rust:")
print('   println!("cargo:rustc-link-search=native=' .. libs_dir .. '");')
print('   println!("cargo:rustc-link-lib=static=tcc");')
if target_os == "linux" then
    print('   println!("cargo:rustc-link-lib=dylib=dl");')
    print('   println!("cargo:rustc-link-lib=dylib=pthread");')
end
print("========================================")
