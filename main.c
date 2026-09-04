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


// Forward Declarations
typedef struct _S_7_i32 _S_7_i32;
typedef struct _S_7_i32 _S_7_i32;


struct _S_7_i32 {
    int32_t _f7;
};

struct _S_7_i32 {
    int32_t _f7;
};



void sym_3(void);
_S_7_i32 sym_2(int32_t sym_6);

// Implementations
void sym_3(void) {
    int32_t sym_4 = 0;
    sym_4;
    while ((sym_4 < 1)) {
        _S_7_i32 sym_5 = sym_2(sym_4);
        sym_5;
        print_i32((sym_5)._f7);
        sym_4 = (sym_4 + 1);
    }
    return ;
}

_S_7_i32 sym_2(int32_t sym_6) {
    _S_7_i32 _tmp0 = {
        ._f7 = (sym_6 * 10),
    };
    return _tmp0;
}

int main(void) {
	sym_3();
	return 0;
}