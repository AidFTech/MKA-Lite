#if __has_include(<python3.11/Python.h>) 
#include <python3.11/Python.h>
#elif __has_include(<python3.10/Python.h>)
#include <python3.10/Python.h>
#endif

#include <stdio.h>
#include <stdbool.h>
#include <string.h>

#include "IBus_IO.h"
#include "Radio_Handler.h"
#include "ParameterList.h"

#ifndef pygui_h
#define pygui_h

wchar_t* pyInit(int argc, char *argv[]);
void pyFinalize(wchar_t* program);

PyObject* startMKA(const char* fname);
void MKAloop(PyObject* mka);
int MKAgetRun(PyObject* mka);

void MKAturnKnob(PyObject* mka, const uint8_t steps, const uint8_t clockwise);
void MKAenterButton(PyObject* mka);
void MKAbackButton(PyObject* mka);
void MKAhomeButton(PyObject* mka);
void MKAdirectionButton(PyObject* mka);

void sendPong(const int ibus_port, const uint8_t receiver, const int first_pong);

void handleIBus(PyObject* mka, const int ibus_port, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l);
void setTime(PyObject* mka, char* time_string);
int getCharacterIndex(char* str, char desired);

void checkParameterList(PyObject* mka, ParameterList* current_parameters, const int ibus_port);

#endif
