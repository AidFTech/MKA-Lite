#if __has_include(<python3.11/Python.h>) 
#include <python3.11/Python.h>
#elif __has_include(<python3.10/Python.h>)
#include <python3.10/Python.h>
#endif

#include "IBus_IO.h"

#ifndef pygui_h
#define pygui_h

wchar_t* pyInit(int argc, char *argv[]);
void pyFinalize(wchar_t* program);

PyObject* startMKA(const char* fname);
void MKAloop(PyObject* mka);
int MKAgetRun(PyObject* mka);
void MKAturnKnob(PyObject* mka, const uint8_t steps, const uint8_t clockwise);
void MKAenterButton(PyObject* mka);

void handleIBus(PyObject* mka, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l);

#endif
