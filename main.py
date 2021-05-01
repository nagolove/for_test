#!/usr/bin/env python3

def foo(n):
    if (n > 0):
        foo(n - 1)
    return 0

# ITER = 1000000
# ITER = 725
# ITER = 999
ITER = 1000

# if __main__ 
foo(ITER)
