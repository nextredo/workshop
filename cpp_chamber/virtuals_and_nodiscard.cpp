#include <iostream>

/// @brief Simple base class
class Base
{
private:
public:
    /// @brief Standard constructor
    Base() = default;

    /// @brief Copy/move constructors
    Base(const Base &)            = default;
    Base &operator=(const Base &) = default;
    Base(Base &&) noexcept            = delete;
    Base &operator=(Base &&) noexcept = delete;

    /// @brief Standard destructor
    virtual ~Base() = default;

    /// @brief Stub method
    [[nodiscard]] virtual int doSomething()
    {
        std::cout << "doing base\n";
        return 0;
    }
};


/// @brief Simple derived class
class Derived : public Base
{
private:
public:
    /// @brief Overidden
    int doSomething() final
    {
        std::cout << "doing derived\n";
        return 1;
    }
};

int main()
{
    // WARN: NODISCARD DOESN'T PROPAGATE!!
    Derived deriv;
    deriv.doSomething();

    // NOTE: This one is picked up on by clang and the compiler
    Base based;
    based.doSomething();

    // NOTE: Access thru the base class is also picked up
    auto& alias = static_cast<Base&>(deriv);
    alias.doSomething();

    return 0;
}
