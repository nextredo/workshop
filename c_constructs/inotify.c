#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <errno.h>
#include <sys/inotify.h>
#include <unistd.h>
#include <time.h>

// TODO address sanitizer doesn't work on CTRL+C signal?
// TODO signal handlers to close inotify fd if applicable
// TODO macro-ise (or function-ify) error handling func
    // change to `< 0` instead of `== -1`
    // custom string to print if desired

//

#define FILE_ARG_INDEX 1
#define INOTIFY_MASK IN_ACCESS | IN_ATTRIB | IN_CLOSE_WRITE | IN_CLOSE_NOWRITE | IN_CREATE | IN_DELETE | IN_DELETE_SELF | IN_MODIFY | IN_MOVE_SELF | IN_MOVED_FROM | IN_MOVED_TO | IN_OPEN
#define BUF_LEN 255

int main(int argc, const char** argv) {
    // Argument checks
    if (argc != 2) {
        (void)fprintf(stderr, "No filepath arg\n");
        return EXIT_FAILURE;
    }

    const char* filepath = argv[FILE_ARG_INDEX];

    // Spawn an inotify instance
    errno = 0;
    int inotify_fd = inotify_init();
    if (inotify_fd == -1) {
        perror(NULL);
        return EXIT_FAILURE;
    }

    // Watch the given file
    errno = 0;
    int watch_fd = inotify_add_watch(inotify_fd, filepath, INOTIFY_MASK);
    if (watch_fd == -1) {
        perror(NULL);
        return EXIT_FAILURE;
    }

    // TODO remove this crap
    srand(time(NULL));
    char* lol = malloc(45);
    printf("mem waster: %p\n", lol);
    lol[0] = 55;
    lol[4] = 20;
    lol[30] = rand();
    printf("mem waster: %d\n", lol[30]);

    // Infini-loop
    uint8_t buffer[BUF_LEN];
    ssize_t n_bytes = 0;
    size_t counter = 0;
    int printf_ret = 0;
    while (1) {
        // Wait on all watches in the given inotify instance
        // (this is a blocking call)
        errno = 0;
        n_bytes = read(inotify_fd, buffer, BUF_LEN);
        if (n_bytes == -1) {
            perror(NULL);
            return EXIT_FAILURE;
        }

        // Whinge to the user
        errno = 0;
        printf_ret = printf("Something happened - count: %zu\n", counter++);
        if (printf_ret < 0) {
            perror(NULL);
            return EXIT_FAILURE;
        }
    }

    // Cleanup
    errno = 0;
    int close_ret = close(inotify_fd);
    if (close_ret == -1) {
        perror(NULL);
        return EXIT_FAILURE;
    }

    // Thanks lol
    return EXIT_SUCCESS;
}
