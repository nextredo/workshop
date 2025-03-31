#include <iostream>
#include <cstdlib>

int main()
{
    srand(time(nullptr));
    printf("rand: %d", rand());

    system("mkdir -p /run/overlay/root");

    return 0;
}
