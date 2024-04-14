#if __has_include(<python3.11/Python.h>) 
#include <python3.11/Python.h>
#elif __has_include(<python3.10/Python.h>)
#include <python3.10/Python.h>
#endif

#include <stdint.h>
#include <string.h>
#include <stdbool.h>
#include <time.h>

#include "IBus_IO.h"

#ifndef radio_handler_h
#define radio_handler_h

#define SONG_NAME 1
#define ARTIST 2
#define ALBUM 3
#define APP 4

void handleRadioIBus(PyObject* mka, const int ibus_port, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l);

void sendCDStatusMessage(const int ibus_port, const uint8_t status, const uint8_t receiver);
void setSelected(PyObject* mka, PyObject* parameter_list, const int selected);

void seekTrack(PyObject* mka, const uint8_t forward);

void sendRadioScreenUpdate(PyObject* parameter_list, const uint8_t version, const int port);

int sendRadioMainText(const char* text, const int8_t version, const int port);
int sendRadioSubtitleText(const char* text, const uint8_t zone, const int8_t version, const int port, const bool refresh);

int sendRadioCenterText(const char* text, const uint8_t position, const int8_t version, const int port);
void sendAllRadioCenterTextFromParameters(PyObject* parameter_list, const uint8_t version, const int port, const bool refresh);
void sendAllRadioCenterText(const char* song_title, const char* artist, const char* album, const char* app, const uint8_t version, const int port, const bool refresh);

void sendRefresh(const int port, const uint8_t index);

#endif
