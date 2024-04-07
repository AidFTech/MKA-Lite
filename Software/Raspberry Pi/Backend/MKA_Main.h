#include <stdint.h>
#include <time.h>

#include "IBus_IO.h"
#include "PyGUI.h"
#include "ParameterList.h"

#ifndef mka_main_h
#define mka_main_h

#define CD_PING 20000

int main(int argc, char* argv[]);
void readIBus(PyObject* mka, int* port);

#endif