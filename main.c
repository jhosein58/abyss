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
void sym_3(void);

// Implementations
void sym_3(void) {
    int32_t sym_4 = 20;
    sym_4;
    int32_t sym_5 = 0;
    sym_5;
    while ((sym_5 < sym_4)) {
        int32_t sym_6 = 0;
        sym_6;
        while ((sym_6 < sym_5)) {
            print_star();
            sym_6 = (sym_6 + 1);
        }
        print_new_line();
        sym_5 = (sym_5 + 1);
    }
    return ;
}

int main(void) {
	sym_3();
	return 0;
}