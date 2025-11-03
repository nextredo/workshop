#include <stdlib.h>
#include <stdio.h>
#include <errno.h>

#define BUF_LEN 64

int main()
{
    errno = 0;
    FILE* fp = fopen("/tmp/hehe", "r");
    if (!fp)
        return EXIT_FAILURE;


    char buffer[BUF_LEN];
    char* ptr = fgets(buffer, BUF_LEN, fp);
    if (!ptr)
        return EXIT_FAILURE;

    long bing = 0;
    int ret = sscanf(buffer, "bing: %ld", &bing);
    if (ret != 1)
    {
        printf("what\n");
        return EXIT_FAILURE;
    }

    printf("bing: %ld\n", bing);

    return EXIT_SUCCESS;
}
