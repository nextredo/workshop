#include <stdio.h>
#include <stdlib.h>

// https://en.cppreference.com/w/c/io/fclose

// Suppress specific clang-tidy warnings
// NOLINTBEGIN(readability-*, cppcoreguidelines-*, hicpp-*)
// https://clang.llvm.org/extra/clang-tidy/

int main()
{
    // TODO use tmpnam(NULL) instead
    const char* fname = "/tmp/unique_name.txt";

    FILE* fp = fopen(fname, "w+");
    if (!fp) {
        perror("File opening failed");
        return EXIT_FAILURE;
    }

    printf("File ptr before closure: %p\n", fp);
    int closed = fclose(fp);
    printf("Closed w/ exit code: %d\n", closed);
    printf("File ptr after closure: %p\n", fp);

    return EXIT_SUCCESS;
}

// NOLINTEND(readability-*, cppcoreguidelines-*, hicpp-*)
