#include <stdint.h>

#if __has_include(<pigpio.h>) //Including an "if has include" so we can test this on a desktop if need be.
#include <pigpio.h>
#define RPI_UART
#endif

#ifndef ibus_handler_h
#define ibus_handler_h

#define IBUS_BAUD 9600

#ifdef RPI_UART
int ibusSerialInit(char* port);
#endif

#endif