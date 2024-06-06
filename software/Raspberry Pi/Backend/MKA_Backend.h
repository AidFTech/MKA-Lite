#include <stdint.h>
#include <stdbool.h>
#include <time.h>

#include "IBus_IO.h"
#include "MKA_Socket.h"
#include "ParameterList.h"

#ifndef mka_backend_h
#define mka_backend_h

typedef struct MKA {
    int ibus_port;
    MKA_Socket* mka_socket;
    ParameterList parameter_list;
} MKA;

int main(int argc, char* argv[]);

#endif
