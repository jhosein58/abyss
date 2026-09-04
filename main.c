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




void sym_1(void);

// Implementations
void sym_1(void) {
    int32_t sym_2 = 100;
    sym_2;
    int32_t* sym_3 = &(sym_2);
    return ;
}

int main(void) {
	sym_1();
	return 0;
}