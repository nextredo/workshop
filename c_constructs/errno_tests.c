// Enables non-portable GNU extensions :)
#define _GNU_SOURCE

#include <stdlib.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>

int main()
{
    FILE* file = fopen("not_real", "r");
    if (file == NULL)
    {
        printf("%s\n", strerror(errno));
        perror("bad thing happen");

        // GNU extension time
        printf("%s\n", strerrorname_np(errno));

        // memfrob(NULL, 2);
        // printf("%s\n", strfry(strerror(errno)));
    }

    printf("errno is 0? Prints: %s\n", strerror(0));

    errno = 0;
    system("/usr/bin/this_file_doesnt_exist");
    perror(NULL);

    // crash time hehe
    char* no_touchy = strerror(0);
    no_touchy[2]++;

    return 0;
}
