#include <iostream>

int main()
{
    int mint = 4;
    const int* ptr = &mint;
    const void* voidptr = &mint;

    if (!voidptr)
        std::cout << "what\n";

    // ERROR won't compile
    // *ptr = 2;

    std::cout << "*ptr: " << *ptr << "\n";

    *const_cast<int*>(ptr) = 2;
    std::cout << "*ptr: " << *ptr << "\n";

    *(int*)(ptr) = 3;
    std::cout << "*ptr: " << *ptr << "\n";

    return 0;
}
