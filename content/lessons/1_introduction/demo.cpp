#include <iostream>
#include <cstdio>

using namespace std;

int main() {
    //# Syntax error
    // cout < "hello world";

    // constexpr int length = 3;
    // string array[length] = {"hello", "old", "world!"};

    //# Array out-of-bounds with a statically known index
    // cout << array[length] << endl;
    
    //# Array out-of-bounds with a dynamically computed index
    // for (int i = 0; i <= length; i++) {
    //     cout << array[i] << endl;
    // }

    //# Doing God knows what
    // const char* format = "a very innocent hello %s";
    // printf(format);

    //# Division by zero
    // int joy_division = 0/0;

    // int joy = 0;
    // int joy_division = 0/joy;

    // int joy = false ? 1 : 0;
    // int joy_division = 0/joy;

    // printf("joy division equals %d", joy_division);

    return 0;
}

