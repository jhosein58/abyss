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

    // case1:   false and true        وقتی اولی فالس شد. دومی نباید چک بشه
    // درست کار میکنه 


    // case2: true and false          اولی ترو میشه. دومی هم باید چک بشه

    // درست کار میکنه!

    


    bool cond = LG; // false

    if (cond) { // false
        cond = RG; // true
    }

    if (cond) { // true
        // ......
    }



    if (false) {
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