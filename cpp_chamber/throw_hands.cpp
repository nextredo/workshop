#include <cstdlib>
#include <ctime>
#include <iostream>

#include <stdexcept>
// #include <optional>

// NOTE handling nullptrs is not a C++ exception job
// It's a signal.h job (catch SIGSEGV)

namespace
{

/// @brief Small function to throw errors
int mathematical(int aaa, int bbb)
{
    if ((aaa == 0) && (bbb == 0))
    {
        throw;
    }

    return aaa * bbb;
}


/// @brief Small function to throw and catch errors
int baseball(int num)
{
    try
    {
        if (num == 0)
        {
            throw std::runtime_error("zero num received");
            // throw std::bad_optional_access();
        }

        ++num;
        return 0;
    }
    catch (std::exception& ex)  { std::cout << "exception: " << ex.what() << "\n"; }
    catch (int errcode)         { std::cout << "err code: " << errcode << "\n"; }
    catch (const char* strcode) { std::cout << "err str: " << (strcode ? strcode : "") << "\n"; }
    catch (...)                 { std::cout << "default err\n"; }

    return -1;
}

}

int main()
{
    std::cout << "starting\n";
    mathematical(1, 2);

    baseball(2);
    baseball(0);

    std::cout << "exiting\n";
    exit(EXIT_SUCCESS);
}
