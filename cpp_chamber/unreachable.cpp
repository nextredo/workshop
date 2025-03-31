#include <iostream>
#include <cstdlib>

// NOLINTBEGIN(modernize-use-using)
typedef long  big;
typedef float biig;
// NOLINTEND(modernize-use-using)

using long_t  = long;
using loong_t = float;

// Anonymous namespace for translation unit locals
// (better version of static)
namespace
{
void func()
{
    std::cout << "func()\n";
}

// __attribute__((__noreturn__))
[[gnu::noreturn]] void trap()
{
    std::cout << "trap()\n";
    exit(EXIT_SUCCESS);
}
}

int main()
{
    std::cout << "start\n";

    func();
    // trap();

    // You're explicitly telling the compiler you'll never make it here
    // So if you do, bad things happen (undefined behaviour)
    // https://gcc.gnu.org/onlinedocs/gcc/Other-Builtins.html
    __builtin_unreachable();

    return 0;
}

