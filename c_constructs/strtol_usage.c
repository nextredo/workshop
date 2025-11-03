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

// NOTE: sscanf version.
// Bad since sscanf doesn't report conversion errors
// Alternatively, do strstr() and strtol() checking instead

int main() {
    errno = 0;
    FILE* file = fopen(FILEPATH, "r");
    if (!file) {
        perror("Bad fopen");
        return EXIT_FAILURE;
    }

    long bing = 0;
    char buffer[BUF_LEN];

    printf("reading %s...\n", FILEPATH);
    while (fgets(buffer, BUF_LEN, file)) {
        errno = 0;
        int ret = sscanf(buffer, "bing: %ld", &bing);
        if (ret != 1) {
            errno = 0;
            int ret = fprintf(stderr, ":( sscanf unhappy\n");
            if (ret < 0) {
                perror("Bad fprintf");
                return EXIT_FAILURE;
            } else {
                printf(":) sscanf happy\n");
            }

        }
    }
    printf("read it\n");

    printf("bing: %ld\n", bing);

    return EXIT_SUCCESS;
}
