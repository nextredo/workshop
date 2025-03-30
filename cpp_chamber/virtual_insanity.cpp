#include <iostream>

/// @brief Simple base object
struct Base
{
public:
    int mInt = 4;
    virtual void doSomething() { std::cout << "base vfunc\n"; }

    Base()          { std::cout << "base ctor\n"; }
    virtual ~Base() { std::cout << "base dtor\n"; }
};

/// @brief Simple derived object
struct Derived : public Base
{
    int mInt = 4;
    void doSomething() override { std::cout << "derived vfunc\n"; }

    Derived()           { std::cout << "derived ctor\n"; }
    ~Derived() override { std::cout << "derived dtor\n"; }
};


int main()
{
    {
        Derived der;
        der.doSomething();

        auto& alias = static_cast<Base&>(der);
        alias.doSomething();

        // class destructor runs here
    }

    return 0;
}
