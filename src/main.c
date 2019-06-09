#include <stdio.h>

extern void initialise_monitor_handles(void);

int main(void) {
    initialise_monitor_handles();
    while (1)
    {
        print_this_shit();
    }
}

void print_this_shit(void) {
    printf("hello daniel\n");
}
