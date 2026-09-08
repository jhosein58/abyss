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
typedef struct _A_i32_5 _A_i32_5;
typedef struct _S_8_ref_i32_9_i32 _S_8_ref_i32_9_i32;
typedef struct _A_i32_5 _A_i32_5;
typedef struct _S_8_ref_i32_9_i32 _S_8_ref_i32_9_i32;


struct _A_i32_5 {
    int32_t _data[5];
};

struct _S_8_ref_i32_9_i32 {
    int32_t* _f8;
    int32_t _f9;
};

struct _A_i32_5 {
    int32_t _data[5];
};

struct _S_8_ref_i32_9_i32 {
    int32_t* _f8;
    int32_t _f9;
};



void sym_2(void);
_A_i32_5 sym_3(void);
void sym_4(_S_8_ref_i32_9_i32 sym_7);

// Implementations
void sym_2(void) {
    _A_i32_5 sym_5 = sym_3();
    sym_5;
    _S_8_ref_i32_9_i32 _tmp0 = {
        ._f8 = &(((sym_5)._data[0])),
        ._f9 = (int32_t)5,
    };
    _S_8_ref_i32_9_i32 sym_6 = _tmp0;
    sym_6;
    sym_4(sym_6);
    return ;
}

_A_i32_5 sym_3(void) {
    _A_i32_5 _tmp1 = {
        ._data = {
            (int32_t)1,
            2,
            3,
            4,
            5,
        }
    };
    return _tmp1;
}

void sym_4(_S_8_ref_i32_9_i32 sym_7) {
    int32_t sym_8 = 0;
    sym_8;
    while ((sym_8 < (sym_7)._f9)) {
        print_i32(*(((sym_7)._f8 + sym_8)));
        sym_8 = (sym_8 + 1);
    }
    return ;
}

int main(void) {
	sym_2();
	return 0;
}