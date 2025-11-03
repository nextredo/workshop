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

    // WARN:
    // fgets will only ever produce a line of BUF_LEN
    // unit_str can only hold BUF_LEN chars
    // this means it'll never overflow, but, the sscanf could overflow it if it wanted to
    // since it has no idea of the size of unit_str, and will just shove shit it finds in there
    char unit_str[BUF_LEN];

    printf("reading %s...\n", FILEPATH);
    while (fgets(buffer, BUF_LEN, file)) {
        errno = 0;
        int ret = sscanf(buffer, "bing: %ld %s", &bing, unit_str);
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
    printf("unit_str: %s\n", unit_str);
    return EXIT_SUCCESS;
}
