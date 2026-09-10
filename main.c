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
    bool _tmp0 = false;
    if (!(_tmp0)) {
        bool _tmp1 = true;
        if (_tmp1) {
            _tmp1 = true;
        }
        _tmp0 = _tmp1;
    }
    if (_tmp0) {
        print_i32(10);
        ;
    } else {
    }
    ;
    return ;
}

int main(void) {
	sym_1();
	return 0;
}