#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

// NOTE References
// https://www.reddit.com/r/cpp_questions/comments/nsfb5e/address_sanitizer_seems_to_have_problems_on/
// https://stackoverflow.com/questions/51963570/how-get-output-of-address-sanitizer-when-emiting-sigint-to-halt-a-loop

// NOTE Compile me with:
// gcc --std=c17 -Wall -Wextra -pedantic -g -fno-omit-frame-pointer -fsanitize=address -O0 leaky.c -o leaky && ./leaky hehe

int main(int argc, const char** argv) {
    if (argc == 1) {
        puts("NOT testing leak");
    } else {
        puts("Testing leak");
        printf("%s", argv[1]);
    }

    void* p = malloc(25);
    if (argc == 1) {
        free(p);
    }

    return 1;
}
