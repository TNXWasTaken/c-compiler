#include <stdio.h>

#define MACRO 0
#define MACRO1 MARCO
#define MACRO2 MACRO1 MACRO

int main() {
    printf("Hello, World!");
    return MACRO;
}  // this is a comment

/* this is a long
comment*/