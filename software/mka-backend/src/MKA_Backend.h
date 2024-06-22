#include <stdint.h>
#include <stdbool.h>
#include <time.h>
#include <pthread.h>

#include "IBus_IO.h"
#include "MKA_Socket.h"
#include "ParameterList.h"
#include "Radio_Handler.h"

#ifndef mka_backend_h
#define mka_backend_h

typedef struct MKA {
    int ibus_port;
    MKA_Socket* mka_socket;
    ParameterList parameter_list;

    bool* running;
} MKA;

int main(int argc, char* argv[]);
void *socketThread(void* mka_v);

void handleIBus(MKA* mka, IBus_Message* ib_data);

#endif
