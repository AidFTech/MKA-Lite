#include "MKA_Backend.h"

int main(int argc, char* argv[]) {
    #ifdef RPI_UART
    int ibus = ibusSerialInit("/dev/ttyAMA0");
    #else
    int ibus = ibusSerialInit("");
    #endif
    MKA mka;

    mka.mka_socket = createSocket();

    clock_t ping_start = clock();

    bool running = true;
    IBus_Message* ib_data = createMessage(0, 0x18, 0x68);

    while(running) {
        if(readIBusData(ibus, ib_data, &ibus) > 0) {
            //TODO: Handle IBus.
        }

        uint8_t sock_data[1024];
        const int size = readSocket(mka.mka_socket, sock_data, sizeof(sock_data));
        if(size > 0) {
        	//TODO: Handle socket input.
        }
    }

    clearMessage(ib_data);
    clearSocket(mka.mka_socket);
}
