#!/bin/bash

# ------------------------------------------------------------------------------
# Minor tests for testing the "test" GNU core utility
# You can view more information with the following command
# man test
check_test()
{
    NOT_EMPTY="hello"
    EMPTY=""

    if [ -n "$NOT_EMPTY" ]; then
        echo "got not empty as non-zero str"
    fi

    if [ -n "$EMPTY" ]; then
        echo "got empty as non-zero str"
    fi
}

# ------------------------------------------------------------------------------
# Small program for testing ":" usage
# https://stackoverflow.com/questions/3224878/what-is-the-purpose-of-the-colon-gnu-bash-builtin
check_true()
{
    if [ $((1+1)) = 3 ]; then
        :
    else
        echo "failed"
    fi
}

# ------------------------------------------------------------------------------
# check_test
check_true

exit 0
