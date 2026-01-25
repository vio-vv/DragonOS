#define _GNU_SOURCE
#include <sys/syscall.h>
#include <unistd.h>
#include <stdio.h>

int main() {
    
    printf("Hello World!\n");

    int ret = syscall(2333);
    printf("syscall 2333 result: %d\n", ret);
    return 0;
}