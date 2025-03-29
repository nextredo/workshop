#include <cstring>
#include <cstdint>
#include <iostream>
#include <iterator>
#include <array>

int main()
{
    {
        std::cout << "C-style arrays\n";

        // NOLINTBEGIN(cppcoreguidelines-*, hicpp-*, modernize-*)
        uint8_t arr_1[] {0b000, 0b000, 0b000};
        uint8_t arr_2[] {0b000, 0b001, 0b000};

        std::cout << memcmp(arr_1, arr_2, std::size(arr_1)) << "\n";
        // NOLINTEND(cppcoreguidelines-*, hicpp-*, modernize-*)
    }

    {
        std::cout << "std arrays\n";

        // Define arrays using Class Template Argument Deduction (CTAD)
        // Allows specifying std arrays without an explicit length
        std::array arr_1{0b000, 0b000, 0b000};
        std::array arr_2{0b000, 0b001, 0b000};

        // Memcmp, using std arrays instead
        std::cout << memcmp(arr_1.data(), arr_2.data(), std::size(arr_1)) << "\n";
    }

    return 0;
}
