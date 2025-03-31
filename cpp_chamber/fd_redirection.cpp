#include <cstdio>
#include <cstdlib>
#include <sys/stat.h>
#include <sys/wait.h>

#include <unistd.h>
#include <fcntl.h>

#include <iostream>

int main()
{
    std::cout << "start\n";

    int backup  = 0;
    int newfile = 0;

    fflush(stdout);
    backup = dup(STDOUT_FILENO);
    newfile = open("/dev/null", O_WRONLY);
    dup2(newfile, STDOUT_FILENO);
    close(newfile);

    std::cout << "middle\n";

    // Put silenced code here
    // (code that uses stdout, which is now rebound to /dev/null)

    fflush(stdout);
    dup2(backup, STDOUT_FILENO);
    close(backup);

    std::cout << "end\n";
}

// int main()
// {
//     std::cout << "start\n";
//
//     // int stdout_saved = dup(STDOUT_FILENO);
//
//     int the_void = open("/dev/null", O_WRONLY);
//     close(STDOUT_FILENO);
//     dup(the_void);
//
//     std::cout << "middle\n";
//
//     close(the_void);
//     wait(nullptr);
//
//     std::cout << "end\n";
//
//     return 0;
// }

// int main()
// {
//     // FILE* original_stdout = stdout;
//     // FILE* original_stderr = stderr;
//
//     std::cout << "start\n";
//
//     // Redirect stdout and stderr to /dev/null
//         // TODO make sure it works correctly (it returns to normal outside of this test case)
//     freopen("/dev/null", "w", stdout); // Redirect stdout to /dev/null
//     freopen("/dev/null", "w", stderr); // Redirect stderr to /dev/null
//
//     std::cout << "middle\n";
//
//     // Close the redirected streams
//     // fclose(stdout);
//     // fclose(stderr);
//
//     // TODO connect it back to whatever it was before, not this
//     freopen("/dev/tty", "w", stdout);
//     freopen("/dev/tty", "w", stderr);
//
//     std::cout << "end\n";
//
//     return 0;
// }
