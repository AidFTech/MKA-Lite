#include <stdint.h>
#include <time.h>

#if __has_include(<pigpio.h>) //Including an "if has include" so we can test this on a desktop if need be.
#include <pigpio.h>
#define RPI_UART
#else
#include <stdio.h>
#endif

#ifndef ibus_handler_h
#define ibus_handler_h

#define IBUS_BAUD 9600
#define MAX_DELAY 500 //500ms. I believe this is longer than an IBus message will ever take. Adjust as needed.

#define IB_RX 4 //The GPIO input to use to determine whether the IBus RX is active.
#define IB_WAIT 20 //The amount of time to wait for the IBus RX to be free before sending any data.

#ifdef RPI_UART
int ibusSerialInit(char* port);
void ibusSerialClose(const int port);
#endif

int readIBusData(const int port, uint8_t* sender, uint8_t* receiver, uint8_t* data);
void writeIBusData(const int port, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l);

uint8_t getChecksum(const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l);
#endif