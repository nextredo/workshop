#include <stdlib.h>
#include <stdio.h>
#include <errno.h>

#define BUF_LEN 64
#define FILEPATH "/tmp/hehe"

/* Expecting a basic file that looks like this:
 * key:    value
 * key:    value
 * key:    value
 * key:    value
 */

int main() {
    errno = 0;
    FILE* file = fopen(FILEPATH, "r");
    if (!file) {
        perror("Bad fopen");
        return EXIT_FAILURE;
    }

    char buffer[BUF_LEN];
    char* ptr = fgets(buffer, BUF_LEN, file);
    if (!ptr) {
        perror("Bad fgets");
        return EXIT_FAILURE;
    }

    // NOTE: scanf version below. Bad since it doesn't report conversion errors
    // Alternatively, do strstr() and strtol() checking instead
    long bing = 0;
    errno = 0;
    int ret = sscanf(buffer, "bing: %ld", &bing);
    if (ret != 1) {
        perror("Bad sscanf");
        return EXIT_FAILURE;
    }

    printf("bing: %ld\n", bing);

    return EXIT_SUCCESS;
}
