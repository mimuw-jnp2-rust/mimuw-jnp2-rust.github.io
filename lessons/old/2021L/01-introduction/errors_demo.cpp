#include <iostream>
#include <cstdio>
#include <array>

using std::cout;
using std::endl;

int main() {
    //# Syntax error
    // cout < "hello world";

    // constexpr int length = 3;
    // std::array<std::string, length> array = {"hello", "old", "world!"};

    //# Array access with an out of bounds index and bounds checking during compile time
    // cout << std::get<length>(array) << endl;

    //# Array access with an out of bounds index and bounds checking during runtime
    // cout << array.at(length) << endl;

    //# Most common access without any checks
    // cout << array[length] << endl;
    
    //# Array out-of-bounds with a dynamically computed index
    // for (int i = 0; i <= length; i++) {
    //     cout << array.at(i) << endl;
    // }

    //# This will be there in Clang 14 ...
    // std::string format = std::format("a very innocent hello {}");
    // cout << format << endl;

    //# ... but for now, this is doing God knows what
    // const char* format = "a very innocent hello %s";
    // printf(format);

    //# Division by zero
    // int joy_division = 1/0;

    // int joy = 0;
    // int joy_division = 1/joy;

    // int joy = false ? 1 : 0;
    // int joy_division = 1/joy;

    // cout << "joy division equals " << joy_division << endl;

    return 0;
}
