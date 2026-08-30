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
typedef struct _S_4_i32 _S_4_i32;


struct _S_4_i32 {
    int32_t _f4;
};



void sym_1(void);

// Implementations
void sym_1(void) {
    _S_4_i32 _tmp0 = {
        ._f4 = (int32_t)100,
    };
    _S_4_i32 sym_2 = _tmp0;
    return ;
}

int main(void) {
	sym_1();
	return 0;
}