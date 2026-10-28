#include <stdio.h>

enum shirt_size {
    small,
    medium,
    large,
    xlarge
};

void print_size(enum shirt_size size) {
    printf("my size is ");
    switch (size) {
        case small:
            printf("small");
            break;
        case medium:
            printf("medium");
            break;
        case large:
            printf("large");
            break;
        case xlarge:
            printf("xlarge");
            break;
        default:
            printf("unknown");
            break;
    }
    printf("\n");
}

int main() {
    enum shirt_size my_size = medium;
    print_size(my_size);
}
