#include "MKA_Main.h"

//Main MKA code.
int main(int argc, char* argv[]) {
	int ibus = ibusSerialInit("/dev/ttyAMA0");
	wchar_t* program = pyInit(argc, argv);

	PyObject* mka = startMKA("./GUI/MKA.py");

	int run = MKAgetRun(mka); //True if the MKA is running.
	while(run > 0) {
		MKAloop(mka);
		run = MKAgetRun(mka);

		if(run <= 0)
			break;
		
		readIBus(mka, ibus);
		//TODO: Check for differences in the Python parameters to see if anything needs to be updated (e.g. song title).

		if(MKAgetRun(mka) <= 0)
			break;
	}

	pyFinalize(program);
}

//Read/handle any waiting IBus messages.
void readIBus(PyObject* mka, const int port) {
	uint8_t sender, receiver;
	uint8_t data[255];
	const int l = readIBusData(0, &sender, &receiver, data);
	if(l > 0) {
		handleIBus(mka, sender, receiver, data, l);
	}
}