#include <stdint.h>

#include "IBus_IO.h"
#include "PyGUI.h"
#include "ParameterList.h"

#ifndef mka_main_h
#define mka_main_h

int main(int argc, char* argv[]);
void readIBus(PyObject* mka, int* port);

#endif