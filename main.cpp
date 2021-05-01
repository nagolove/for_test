#include <stdio.h>

//#define ITER 1000000
  #define ITER 52000

int foo(int n) {
    if (n > 0) {
        foo(n - 1);
    }
    return 0;
}

int main(char *argv, int c) {
    foo(ITER);
    return 0;
}

