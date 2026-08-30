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
typedef struct _S_3_f32 _S_3_f32;
typedef struct _S_2__S_3_f32 _S_2__S_3_f32;


struct _S_3_f32 {
    float _f3;
};

struct _S_2__S_3_f32 {
    _S_3_f32 _f2;
};



void sym_0(void);

// Implementations
void sym_0(void) {
    _S_3_f32 _tmp0 = {
        ._f3 = (float)200,
    };
    _S_2__S_3_f32 _tmp1 = {
        ._f2 = _tmp0,
    };
    _S_2__S_3_f32 sym_1 = _tmp1;
    return ;
}

int main(void) {
	sym_0();
	return 0;
}