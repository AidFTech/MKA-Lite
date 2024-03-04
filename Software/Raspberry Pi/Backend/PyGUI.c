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
void handleIBus(PyObject* mka, const int ibus_port, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l) {
	if(l < 1)
		return;

	if(receiver == IBUS_DEVICE_CDC && data[0] == 0x1) { //Ping.
		sendPong(ibus_port, sender, 0);
	} else if(sender == IBUS_DEVICE_IKE) {
		if(data[0] == IBUS_CMD_IKE_IGN_STATUS_RESP) {
			if((data[1]&0x1) == 0)
				MKAsetRun(mka, 0);
			else
				MKAsetRun(mka, 1);
		} else if(data[0] == IBUS_CMD_IKE_RESP_VEHICLE_CONFIG) {
			PyObject_SetAttrString(PyObject_GetAttrString(mka, "parameter_list"), "ike_24h", PyBool_FromLong(!(data[2]&0x1)));
		} else if(data[0] == IBUS_CMD_IKE_OBC_TEXT) {
			if(data[1] == 0x1) { //Time.
				char time_string[l-2];
				for(uint8_t i=3;i<l;i+=1)
					time_string[i-3] = (char)(data[i]);
				time_string[l-3] = '\0';
				setTime(mka, time_string);
			} else if(data[1] == 0x2) { //Date.
				char date_string[l-2];
				for(uint8_t i=3;i<l;i+=1)
					date_string[i-3] = (char)(data[i]);
				date_string[l-3] = '\0';

				PyObject_SetAttrString(PyObject_GetAttrString(mka, "parameter_list"), "ike_datestring", Py_BuildValue("s", date_string));
			}
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
	} else if(sender == IBUS_DEVICE_RAD) {
		handleRadioIBus(mka, ibus_port, sender, receiver, data, l);
	} else if(sender == IBUS_DEVICE_LCM) {
		if(data[0] == IBUS_CMD_LCM_BULB_IND_RESP) {
			PyObject* parameter_list = PyObject_GetAttrString(mka, "parameter_list");
			const uint8_t last_headlights_on = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "headlights_on"));
			const uint8_t light_level = PyLong_AsLong(PyObject_GetAttrString(parameter_list, "light_level"));

			PyObject_SetAttrString(parameter_list, "headlights_on", PyBool_FromLong(data[1]&0x01));
			if(light_level <= 0 && last_headlights_on != data[1]&0x01) { //RLS is not connected. Switch to day or night mode.
				PyObject* set_night_mode = PyObject_GetAttrString(mka, "setNightMode");
				PyObject_CallObject(set_night_mode, NULL);
			}
		}
	} else if(sender == IBUS_DEVICE_RLS) {
		if(data[0] == IBUS_CMD_RLS_LIGHT_CONTROL) {
			const uint8_t light_level = data[1]>>4;
			PyObject* parameter_list = PyObject_GetAttrString(mka, "parameter_list");
			PyObject_SetAttrString(parameter_list, "light_level", PyLong_FromLong(light_level));

			PyObject* set_night_mode = PyObject_GetAttrString(mka, "setNightMode");
			PyObject_CallObject(set_night_mode, NULL);
		}
	}
	#ifndef RPI_UART
	if(data[0] != IBUS_CMD_IKE_IGN_STATUS_RESP)
		MKAloop(mka);
	#endif
}

//Send a pong message.
void sendPong(const int ibus_port, const uint8_t receiver, const int first_pong) {
	uint8_t data[] = {0x2, 0x0};
	const uint16_t l = sizeof(data);
	writeIBusData(ibus_port, IBUS_DEVICE_CDC, receiver, data, l);
	//TODO: Change data[1] for first pong.
}

//Set the displayed time.
void setTime(PyObject* mka, char* time_string) {
	const int colon_index = getCharacterIndex(time_string, ':');
	if(colon_index < 0)
		return;

	char hour_array[] = {time_string[colon_index - 2], time_string[colon_index - 1], '\0'};
	char min_array[] = {time_string[colon_index + 1], time_string[colon_index + 2], '\0'};

	int hour, min;
	sscanf(hour_array, "%d", &hour);
	sscanf(min_array, "%d", &min);

	PyObject* parameter_list = PyObject_GetAttrString(mka, "parameter_list");
	int meridian_index = getCharacterIndex(time_string, 'p');
	if(meridian_index < 0)
		meridian_index = getCharacterIndex(time_string, 'P');
	if(meridian_index < 0)
		meridian_index = getCharacterIndex(time_string, 'a');
	if(meridian_index < 0)
		meridian_index = getCharacterIndex(time_string, 'A');

	if(meridian_index < 0) {
		PyObject_SetAttrString(parameter_list, "ike_hour", PyLong_FromLong(hour));
	} else {
		int pm_index = getCharacterIndex(time_string, 'p');
		if(pm_index < 0)
			pm_index = getCharacterIndex(time_string, 'P');
		
		if(pm_index < 0) { //AM.
			if(hour >= 12)
				hour = 0;
			
			PyObject_SetAttrString(parameter_list, "ike_hour", PyLong_FromLong(hour));
		} else { //PM.
			PyObject_SetAttrString(parameter_list, "ike_hour", PyLong_FromLong(hour+12));
		}
	}
	PyObject_SetAttrString(parameter_list, "ike_minute", PyLong_FromLong(min));
}

//Returns the position of the first occurence of a character in a C string.
int getCharacterIndex(char* str, char desired) {
	int i=0;
	while(str[i] != '\0' && str[i] != desired)
		i+=1;
	
	if(str[i] == desired)
		return i;
	else
		return -1;
}
