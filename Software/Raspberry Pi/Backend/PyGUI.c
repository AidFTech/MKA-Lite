#include "PyGUI.h"

//Initialize the Python interface. Called from main at the start of the program.
wchar_t* pyInit(int argc, char *argv[]) {
	wchar_t* program = Py_DecodeLocale(argv[0], NULL);
	Py_SetProgramName(program);
	Py_Initialize();

	return program;
}

//Finalize the Python interface at the end of the program.
void pyFinalize(wchar_t* program) {
	PyMem_RawFree(program);
	Py_Finalize();
}

//Start the MKA.
PyObject* startMKA(const char* fname) {
	FILE* pyfile = fopen(fname, "r");
	PyRun_SimpleFile(pyfile, fname);

	PyObject* global_dict = PyModule_GetDict(PyImport_AddModule("__main__"));
	PyObject* mka_class = PyDict_GetItemString(global_dict, "MKA");

	PyObject* def_tuple = PyTuple_New(3);
	#ifdef RPI_UART
	PyTuple_SetItem(def_tuple, 0, PyBool_FromLong(1));
	#else
	PyTuple_SetItem(def_tuple, 0, PyBool_FromLong(0));
	#endif
	PyTuple_SetItem(def_tuple, 1, PyBool_FromLong(0));
	PyTuple_SetItem(def_tuple, 2, Py_BuildValue("s", fname));

	PyObject* mka = PyObject_CallObject(mka_class, def_tuple);
	fclose(pyfile);
		
	return mka;
}

//Call the MKA loop function.
void MKAloop(PyObject* mka) {
	PyObject* loop = PyObject_GetAttrString(mka, "loop");
	PyObject_CallObject(loop, NULL);
}

//Set the value of MKA boolean run, e.g. if the car is turned off.
void MKAsetRun(PyObject* mka, int run) {
	PyObject_SetAttrString(mka, "run", PyBool_FromLong(run));
}

//Get the value of MKA boolean run.
int MKAgetRun(PyObject* mka) {
	PyObject* run = PyObject_GetAttrString(mka, "run");
	return PyObject_IsTrue(run);
}

//Change the selected option on screen.
void MKAturnKnob(PyObject* mka, const uint8_t steps, const uint8_t clockwise) {
	PyObject* knob_turn = PyObject_GetAttrString(mka, "knobTurn");

	PyObject* tuple = PyTuple_New(2);
	PyTuple_SetItem(tuple, 0, PyBool_FromLong(clockwise));
	PyTuple_SetItem(tuple, 1, PyLong_FromLong(steps));

	PyObject_CallObject(knob_turn, tuple);
}

//Press the enter button.
void MKAenterButton(PyObject* mka) {
	PyObject* handle_enter_button = PyObject_GetAttrString(mka, "handleEnterButton");
	PyObject_CallObject(handle_enter_button, NULL);
}

//Handle an IBus message.
void handleIBus(PyObject* mka, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l) {
	if(l < 1)
		return;

	if(sender == IBUS_DEVICE_IKE) {
		if(data[0] == IBUS_CMD_IKE_IGN_STATUS_RESP) {
			if((data[1]&0x1) == 0)
				MKAsetRun(mka, 0);
			else
				MKAsetRun(mka, 1);
		}
	} else if(sender == IBUS_DEVICE_BMBT) {
		if(data[0] == IBUS_CMD_BMBT_KNOB) { //Knob turn.
			const uint8_t steps = data[1]&0x7F, clockwise = (data[1]&0x80)>>7;
			MKAturnKnob(mka, steps, clockwise);
		} else if(data[0] == IBUS_CMD_BMBT_BUTTON1) { //Button press.
			const uint8_t button = data[1]&0x3F, state = (data[1]&0xC0)>>6;
			if(button == 0x05 && state == 2) //Enter button.
				MKAenterButton(mka);
		}
	}
	#ifndef RPI_UART
	if(data[0] != IBUS_CMD_IKE_IGN_STATUS_RESP)
		MKAloop(mka);
	#endif
}