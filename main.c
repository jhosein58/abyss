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


struct _S_5_i32 {
    int32_t _f5;
};



void sym_2(void);

// Implementations
void sym_2(void) {
    _S_5_i32 _tmp0 = {
        ._f5 = (int32_t)100,
    };
    _S_5_i32 sym_3 = _tmp0;
    sym_3;
    (sym_3)._f5 = 399;
    (sym_3)._f5;
    print_i32((sym_3)._f5);
    return ;
}

int main(void) {
	sym_2();
	return 0;
}