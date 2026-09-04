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
typedef struct _S_5_i32 _S_5_i32;
typedef struct _S_5_i32 _S_5_i32;


struct _S_5_i32 {
    int32_t _f5;
};

struct _S_5_i32 {
    int32_t _f5;
};



void sym_1(void);
void sym_2(_S_5_i32* sym_4);

// Implementations
void sym_1(void) {
    _S_5_i32 _tmp0 = {
        ._f5 = (int32_t)50,
    };
    _S_5_i32 sym_3 = _tmp0;
    sym_3;
    sym_2(&(sym_3));
    sym_2(&(sym_3));
    sym_2(&(sym_3));
    print_i32((sym_3)._f5);
    return ;
}

void sym_2(_S_5_i32* sym_4) {
    (*(sym_4))._f5 = ((*(sym_4))._f5 * 2);
    return ;
}

int main(void) {
	sym_1();
	return 0;
}