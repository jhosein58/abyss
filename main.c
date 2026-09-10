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
typedef struct _A_i32_3 _A_i32_3;


struct _A_i32_3 {
    int32_t _data[3];
};



void sym_1(void);

// Implementations
void sym_1(void) {
    _A_i32_3 _tmp0 = {
        ._data = {
            (int32_t)1,
            2,
            3,
        }
    };
    _A_i32_3 sym_2 = _tmp0;
    return ;
}

int main(void) {
	sym_1();
	return 0;
}