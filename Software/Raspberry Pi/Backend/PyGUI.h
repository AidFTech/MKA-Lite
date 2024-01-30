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

#endif