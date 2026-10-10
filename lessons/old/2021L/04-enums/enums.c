#include <stdio.h>

enum shirt_size {
    small,
    medium,
    large,
    xlarge
};

void print_size(enum shirt_size size) {
    printf("my size is ");
    if (size == small) {
        printf("small");
    } else if (size == medium) {
        printf("medium");
    } else if (size == large) {
        printf("large");
    } else if (size == xlarge) {
        printf("xlarge");
    } else {
        printf("unknown");
    }
    printf("\n");
}

int main() {
    enum shirt_size my_size = medium;
    print_size(my_size);
}
