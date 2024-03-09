#include "MKA_Main.h"

//Main MKA code.
int main(int argc, char* argv[]) {
	#ifdef RPI_UART
	int ibus = ibusSerialInit("/dev/ttyAMA0");
	#else
	int ibus = ibusSerialInit("");
	#endif
	wchar_t* program = pyInit(argc, argv);

	PyObject* mka = startMKA("./GUI/MKA.py");

	int run = MKAgetRun(mka); //True if the MKA is running.
	ParameterList parameters;
	while(run > 0) {
		MKAloop(mka);
		run = MKAgetRun(mka);

		if(run <= 0)
			break;
		
		readIBus(mka, &ibus);
		checkParameterList(mka, &parameters, ibus);

		if(MKAgetRun(mka) <= 0)
			break;
	}

	pyFinalize(program);
}

//Read/handle any waiting IBus messages.
void readIBus(PyObject* mka, int* port) {
	uint8_t sender, receiver;
	uint8_t data[255];
	const int l = readIBusData(*port, &sender, &receiver, data, port);
	if(l > 0) {
		handleIBus(mka, *port, sender, receiver, data, l);
	}
}
