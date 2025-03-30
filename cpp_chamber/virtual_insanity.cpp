#include <iostream>

/// @brief Simple base class
class Base
{
private:
public:
    /// @brief Standard constructor
    Base() { std::cout << "base ctor\n"; }

    /// @brief Copy/move constructors
    Base(const Base &)                = delete;
    Base &operator=(const Base &)     = delete;
    Base(Base &&) noexcept            = delete;
    Base &operator=(Base &&) noexcept = delete;

    /// @brief Standard destructor
    virtual ~Base() { std::cout << "base dtor\n"; }

    /// @brief Stub method
    [[nodiscard]] virtual int doSomething()
    {
        std::cout << "base vfunc\n";
        return 0;
    }
};


/// @brief Simple derived class
class Derived : public Base
{
private:
public:
    Derived()           { std::cout << "derived ctor\n"; }
    ~Derived() override { std::cout << "derived dtor\n"; }

    /// @brief Overidden method
    int doSomething() final
    {
        std::cout << "derived vfunc\n";
        return 1;
    }
};

int main()
{
    std::cout << "-------------------\n";
    {
        Derived der;
        der.doSomething();

        auto& alias = static_cast<Base&>(der);
        alias.doSomething();

        // class destructor runs here
    }
    std::cout << "-------------------\n";

    std::cout << "-------------------\n";
    {
        // WARN: NODISCARD DOESN'T PROPAGATE!!
        Derived deriv;
        deriv.doSomething();

        // NOTE: Discarded return here is picked up by clang-tidy and g++
        Base based;
        based.doSomething();

        // NOTE: Discarded return is NOT picked up here by g++
        // it is by clang-tidy though
        auto& alias = static_cast<Base&>(deriv);
        alias.doSomething();
    }
    std::cout << "-------------------\n";

    return 0;
}
