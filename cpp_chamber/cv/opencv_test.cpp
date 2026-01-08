/*
Optionally run this through `bear` to get a `compile_commands.json` that `clangd` can use
g++ --std=c++11 -I/usr/include/opencv4 -Wall -Wextra -pedantic $(pkg-config --cflags --libs opencv4) opencv_test.cpp
*/

#include <opencv4/opencv2/opencv.hpp>

#include <cstdlib>
#include <iostream>

int main(int argc, char** argv)
{
    if (argc < 2)
        return EXIT_FAILURE;

    std::cout << "hello world\n";

    cv::Mat img = cv::imread(argv[1]);
    cv::imshow("hehe", img);
    cv::waitKey();

    return EXIT_SUCCESS;
}
