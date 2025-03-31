#include <cstdlib>
#include <ctime>
#include <iterator>
#include <iostream>
#include <array>

// Pretty self-explanatory really
constexpr int ARR_LEN = 15;

int main()
{
    // NOLINTBEGIN(cert-*, modernize-*, readability-*, hicpp-*, concurrency-*, cppcoreguidelines-*)
    srand(time(nullptr));

    // Define C-style array
    int nums[ARR_LEN] { };

    // Fill random element with a random value
    // (Get length rather than using the constant)
    size_t rand_idx = rand() % std::size(nums);
    nums[rand_idx] = rand();

    std::cout << "hehe\n";
    std::cout << nums << "\n";
    std::cout << std::begin(nums) << "\n";
    std::cout << std::end(nums) << "\n";

    for (const auto& ele : nums)
        std::cout << "ele: " << ele << "\n";

    return 0;
    // NOLINTEND(cert-*, modernize-*, readability-*, hicpp-*, concurrency-*, cppcoreguidelines-*)
}
