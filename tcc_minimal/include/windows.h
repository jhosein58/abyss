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
