#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>



void print_i32(int32_t v) {
    printf("%d\n", v);
}


// Forward Declarations
void sym_1(void);

// Implementations
void sym_1(void) {
    int32_t sym_2 = 0;
    sym_2;
    while ((sym_2 < 5)) {
        if ((sym_2 != 2)) {
            print_i32(sym_2);
        } else {
        }
        ;
        sym_2 = (sym_2 + 1);
    }
    return ;
}

int main(void) {
	sym_1();
	return 0;
}