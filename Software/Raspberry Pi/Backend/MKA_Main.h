#include <stdint.h>

#include "IBus_IO.h"
#include "PyGUI.h"

#ifndef mka_main_h
#define mka_main_h

int main(int argc, char* argv[]);
void readIBus(PyObject* mka, const int port);

#endif