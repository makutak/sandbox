#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <signal.h>

typedef void (*signalhandler_t)(int);

signalhandler_t trap_signal(int sig, signalhandler_t handler) {
  struct sigaction act, old;

  act.sa_handler = handler;
  sigemptyset(&act.sa_mask);
  act.sa_flags = SA_RESTART;
  if (sigaction(sig, &act, &old) < 0)
    return NULL;

  return old.sa_handler;
}

void print_exit(int sig) {
  printf("Got signal %d\n", sig);
  exit(0);
}

int main (int argc, char **argv) {
  trap_signal(SIGINT, print_exit);
  pause();

  exit(0);
}
