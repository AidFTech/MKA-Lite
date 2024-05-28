#include "MKA_Backend.h"

int main(int argc, char* argv[]) {
    #ifdef RPI_UART
    int ibus = ibusSerialInit("/dev/ttyAMA0");
    #else
    int ibus = ibusSerialInit("");
    #endif
    clock_t ping_start = clock();

    bool running = true;
    IBus_Message* ib_data = createMessage(0, 0x18, 0x68);

    while(running) {
        if(readIBusData(ibus, ib_data, &ibus) > 0) {
            printIBusData(ib_data);
        }
    }

    clearMessage(ib_data);
}
