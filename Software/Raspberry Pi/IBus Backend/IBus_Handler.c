#include "IBus_Handler.h"

#ifdef RPI_UART
int ibusSerialInit(char* port) {
	return serOpen(port, IBUS_BAUD, 0); //TODO: Add even parity option.
}
#endif