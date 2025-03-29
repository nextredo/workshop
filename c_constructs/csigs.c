// GNU extension time :)
#define _GNU_SOURCE

#include <stdlib.h>
#include <stdio.h>
#include <signal.h>
#include <string.h>
#include <sys/cdefs.h>


// ------------------------------ Macronutrients -------------------------------
#define RETPLS 47
typedef void(*handler_t)(int);

#define PRINTSIG() printf("%d sigs received. Last sig: %d.\n", g_num_sigs, g_last_sig);

// Practice preprocessor stuff, could just use the old syntax instead
#if __STDC_VERSION__ >= 202410L
    // Use C23 attributes where possible
    #define UNUSED [[maybe_unused]]
#else
    // Use normal attributes where not available
    #define UNUSED __attribute_maybe_unused__
#endif

// ------------------------------- Vars & funcs --------------------------------
// Suppress specific clang-tidy warnings
// NOLINTBEGIN(cppcoreguidelines-*)

sig_atomic_t g_last_sig = 0;
sig_atomic_t g_num_sigs = 0;

void sig_hdlr(int sig)
{
    // FIXME printing in signal handler is a bad idea :)
    printf("got signal %d\n", sig);
    printf("%s\n", strsignal(sig));
    printf("SIG %s\n", sigabbrev_np(sig));

    g_last_sig = sig;
    ++g_num_sigs;
}

void inc(int* num)
{
    ++num;
}

// ----------------------------------- main ------------------------------------

int main()
{
    UNUSED handler_t prev_handler = signal(SIGSEGV, sig_hdlr);
    PRINTSIG()

    UNUSED int raised = raise(SIGSEGV);

    inc(NULL);

    printf("%d\n", g_last_sig);

    exit(EXIT_SUCCESS);
}

// NOLINTEND(cppcoreguidelines-*)
