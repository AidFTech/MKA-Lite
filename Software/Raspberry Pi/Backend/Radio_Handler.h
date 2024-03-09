#if __has_include(<python3.11/Python.h>) 
#include <python3.11/Python.h>
#elif __has_include(<python3.10/Python.h>)
#include <python3.10/Python.h>
#endif

#include <stdint.h>
#include <string.h>

#include "IBus_IO.h"

#ifndef radio_handler_h
#define radio_handler_h

#define SONG_NAME 1
#define ARTIST 2
#define ALBUM 3
#define APP 4

void handleRadioIBus(PyObject* mka, const int ibus_port, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l);

void sendCDStatusMessage(const int ibus_port, const uint8_t status, const uint8_t receiver);
void setSelected(PyObject* parameter_list, const int selected);

int sendRadioText(const char* text, const uint8_t position, const int8_t version, const int port);
void sendRefresh(const int port);

#endif
