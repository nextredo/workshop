#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#define NUMBER 5;

int main()
{
    srand(time(NULL));

    // const char* stuff = malloc(5);
    const char* stuff = NULL;
    {
        char* tmp = malloc(5);
        tmp[0] = 'A';
        tmp[1] = rand();
        tmp[3] = 'Z';
        tmp[4] = '\0';
        stuff = tmp;
    }

    // stuff += 1;
    // stuff -= 1;

    printf("%s\n", stuff);

    // free(stuff);             // WARN doesn't compile
    free((void*)stuff);         // NOTE compiles

    {
        char* fake = NULL;
        char* real = malloc(5);
        real[0] = '1';
        real[4] = '\0';

        strcpy(fake, real);
        // strcpy(real, fake);

        // strcat(real, fake);
        // strcat(fake, real);
        //
        // printf("%lu", strlen(fake));
        // printf("%lu", strlen(real));
    }

    return 0;
}
