#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include <iostream>
using namespace std;

// Modified code from https://github.com/doctest/doctest/blob/master/doc/markdown/tutorial.md
TEST_CASE("lots of nested subcases")
{
    cout << endl << "root" << endl;
    SUBCASE("")
    {
        cout << "1" << endl;
        SUBCASE("") { cout << "1.1leaf" << endl; }
        SUBCASE("") { cout << "1.2leaf" << endl; }
        SUBCASE("")
        {
            cout << "1.3" << endl;
            SUBCASE("") { cout << "1.3.1leaf" << endl; }
            cout << "1.3end" << endl;
        }
        cout << "1end" << endl;
    }

    SUBCASE("")
    {
        cout << "2" << endl;
        SUBCASE("") { cout << "2.1leaf" << endl; }
        SUBCASE("") { cout << "2.2leaf (WITH RETURN)" << endl; return; }

// -------------------------------------------------------------------------------------------------
// WARN: Subcases past this point are never invoked (if 2.2leaf (WITH RETURN) runs)
// Not subcases within this current subcase
// Not subcases outside of this subcase either

        SUBCASE("")
        {
            cout << "2.3" << endl;
            SUBCASE("") { cout << "2.3.1leaf" << endl; }
            cout << "2.3end" << endl;
        }
        cout << "2end" << endl;
    }

    SUBCASE("")
    {
        cout << "3" << endl;
        SUBCASE("") { cout << "3.1leaf" << endl; }
        SUBCASE("") { cout << "3.2leaf" << endl; }
        cout << "3end" << endl;
    }

    cout << "rootend" << endl;
}
