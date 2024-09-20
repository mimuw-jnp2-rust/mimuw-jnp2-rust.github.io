#include <stdio.h>

#define LENGTH 3

int main() {
    //# Syntax error
    // printf("hello world";

    //char* array[LENGTH] = {"hello", "ancient", "world!"};

    //# Array out-of-bounds with a statically known index
    // printf("%s\n", array[LENGTH]);

    //# Array out-of-bounds with a dynamically computed index
    // for (int i = 0; i <= LENGTH; i++) {
    //     printf("%s\n", array[i]);
    // }

    //# Doing God knows what
    // char* format = "a very innocent hello %s";
    // printf(format);

    //# Division by zero
    // int joy_division = 1/0;

    // int joy = 0;
    // int joy_division = 1/joy;

    // int joy = 0 ? 1 : 0;
    // int joy_division = 1/joy;

    // printf("joy division equals %d", joy_division);

    return 0;
}
