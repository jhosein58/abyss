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
typedef struct _S_6_i32 _S_6_i32;
typedef struct _S_6_i32 _S_6_i32;


struct _S_6_i32 {
    int32_t _f6;
};

struct _S_6_i32 {
    int32_t _f6;
};



void sym_3(void);
_S_6_i32 sym_2(int32_t sym_5);

// Implementations
void sym_3(void) {
    _S_6_i32 sym_4 = sym_2(33);
    sym_4;
    print_i32((sym_4)._f6);
    return ;
}

_S_6_i32 sym_2(int32_t sym_5) {
    _S_6_i32 _tmp0 = {
        ._f6 = sym_5,
    };
    return _tmp0;
}

int main(void) {
	sym_3();
	return 0;
}