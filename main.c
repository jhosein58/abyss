#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>



void echo() {
    printf("Abyss is alive!\n");
}

void print_i32(int32_t v) {
    printf("%d\n", v);
}


// Forward Declarations
void sym_2(void);

// Implementations
void sym_2(void) {
    int32_t sym_3 = 0;
    sym_3;
    while ((sym_3 < 5)) {
        print_i32(sym_3);
        sym_3 = (sym_3 + 1);
    }
    return ;
}

int main(void) {
	sym_2();
	return 0;
}