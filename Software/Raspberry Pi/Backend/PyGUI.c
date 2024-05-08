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
	if(run <= 0) 
		MKAloop(mka);
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

//Press the back/phone button.
void MKAbackButton(PyObject* mka) {
	PyObject* handle_back = PyObject_GetAttrString(mka, "handleBackButton");
	PyObject_CallObject(handle_back, NULL);
}

//Press the home button. (Hold the phone button)
void MKAhomeButton(PyObject* mka) {
	PyObject* handle_home = PyObject_GetAttrString(mka, "handleHomeButton");
	PyObject_CallObject(handle_home, NULL);
}

//Press the direction button for play/pause.
void MKAdirectionButton(PyObject* mka) {
	PyObject* handle_dir = PyObject_GetAttrString(mka, "handleDirectionButton");
	PyObject_CallObject(handle_dir, NULL);
}

//Set the state of the phone LEDs on the BMBT.
void setPhoneLight(const int ibus_port, const uint8_t state) {
	uint8_t light_message[] = {0x2B, 0x00};
	if(state == PHONE_LED_GREEN)
		light_message[1] = 0x10;
	else if(state == PHONE_LED_RED)
		light_message[1] = 0x1;
		
	writeIBusData(ibus_port, IBUS_DEVICE_TEL, IBUS_DEVICE_ANZV, light_message, sizeof(light_message));
}

//Handle an IBus message.
void handlePythonIBus(PyObject* mka, const int ibus_port, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l) {
	if(l < 1)
		return;

	PyObject* parameter_list = PyObject_GetAttrString(mka, "parameter_list");

	if(receiver == IBUS_DEVICE_CDC && data[0] == 0x1) { //Ping.
		sendPong(ibus_port, sender, 0);
	} else if(sender == IBUS_DEVICE_IKE) {
		if(data[0] == IBUS_CMD_IKE_IGN_STATUS_RESP) { //Change the ignition status.
			if((data[1]&0x1) == 0)
				MKAsetRun(mka, 0);
			else
				MKAsetRun(mka, 1);
		} else if(data[0] == IBUS_CMD_IKE_RESP_VEHICLE_CONFIG) { //Set the 24h time mode.
			PyObject_SetAttrString(parameter_list, "ike_24h", PyBool_FromLong(!(data[2]&0x1)));
		} else if(data[0] == IBUS_CMD_IKE_OBC_TEXT) { //Set the time or date.
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

				PyObject_SetAttrString(parameter_list, "ike_datestring", Py_BuildValue("s", date_string));
			}
		}
	} else if(sender == IBUS_DEVICE_BMBT) {
		PyObject_SetAttrString(parameter_list, "bmbt_connected", PyBool_FromLong(1)); //Make sure Python knows the BMBT is connected.

		if(PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "mka_active"))) { //Respond to these events only if the MKA is actively displaying video.
			if(data[0] == IBUS_CMD_BMBT_KNOB) { //Knob turn.
				const uint8_t steps = data[1]&0x7F, clockwise = (data[1]&0x80)>>7;
				MKAturnKnob(mka, steps, clockwise);
			} else if(data[0] == IBUS_CMD_BMBT_BUTTON1) { //Button press.
				const uint8_t button = data[1]&0x3F, state = (data[1]&0xC0)>>6;
				if(button == 0x05 && state == 2) //Enter button.
					MKAenterButton(mka);
				else if(button == 0x8 && state == 2) //Phone button, released.
					MKAbackButton(mka);
				else if(button == 0x8 && state == 1) //Phone button, held.
					MKAhomeButton(mka);
				else if(button == 0x14 && state == 2) //Tape direction button.
					MKAdirectionButton(mka);
			}
		} else {
			if(data[0] == IBUS_CMD_BMBT_BUTTON1) { //Button press. If the MKA screen is not active, only listen for the direction button.
				const uint8_t button = data[1]&0x3F, state = (data[1]&0xC0)>>6;
				if(button == 0x14 && state == 2) //Tape direction button.
					MKAdirectionButton(mka);
			}
		}
	} else if(sender == IBUS_DEVICE_RAD) { //If the message is from the radio, handle functions in the Radio_Handler "object."
		handleRadioIBus(mka, ibus_port, sender, receiver, data, l);
	} else if(sender == IBUS_DEVICE_LCM) { //LCM. Headlights on/off.
		if(data[0] == IBUS_CMD_LCM_BULB_IND_RESP) {
			const uint8_t last_headlights_on = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "headlights_on"));
			const uint8_t light_level = PyLong_AsLong(PyObject_GetAttrString(parameter_list, "light_level"));

			PyObject_SetAttrString(parameter_list, "headlights_on", PyBool_FromLong(data[1]&0x01));
			if(light_level <= 0 && last_headlights_on != data[1]&0x01) { //RLS is not connected. Switch to day or night mode.
				PyObject* set_night_mode = PyObject_GetAttrString(mka, "setNightMode");
				PyObject_CallObject(set_night_mode, NULL);
			}
		}
	} else if(sender == IBUS_DEVICE_RLS) { //RLS. Level of brightness.
		if(data[0] == IBUS_CMD_RLS_LIGHT_CONTROL) {
			const uint8_t light_level = data[1]>>4;
			PyObject_SetAttrString(parameter_list, "light_level", PyLong_FromLong(light_level));

			PyObject* set_night_mode = PyObject_GetAttrString(mka, "setNightMode");
			PyObject_CallObject(set_night_mode, NULL);
		}
	} else if(sender == IBUS_DEVICE_GT) { //Version message from the GT.
		bool request_version = true; //Request the version if it is not set. Set to false if we get the version message.

		if(data[0] == 0xA0) { //Version message.
			const uint8_t version_data0 = data[12], version_data1 = data[13];
			PyObject* tuple = PyTuple_New(2);
			PyTuple_SetItem(tuple, 0, PyLong_FromLong(version_data0));
			PyTuple_SetItem(tuple, 1, PyLong_FromLong(version_data1));

			PyObject* set_version = PyObject_GetAttrString(mka, "setVersion");
			PyObject_CallObject(set_version, tuple);
		}

		int version = PyLong_AsLong(PyObject_GetAttrString(parameter_list, "version"));
		if(version <= 0 && request_version) 
			sendVersionQuery(ibus_port, IBUS_DEVICE_GT);
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

//Send the CD ping message to the radio.
void sendCDPing(const int ibus_port) {
	uint8_t data[] = {0x2, 0x1};
	const uint16_t l = sizeof(data);
	writeIBusData(ibus_port, IBUS_DEVICE_CDC, IBUS_DEVICE_GLO, data, l);
}

//Send a diagnostic query.
void sendVersionQuery(const int ibus_port, const uint8_t receiver) {
	uint8_t data[] = {0x0};
	const uint16_t l = sizeof(data);
	writeIBusData(ibus_port, IBUS_DEVICE_DIA, receiver, data, l);
}

//Set the displayed time.
void setTime(PyObject* mka, char* time_string) {
	const int colon_index = getCharacterIndex(time_string, ':'); //The time string should contain a colon.
	if(colon_index < 0)
		return;
	
	//Separate the hour and minute.
	char hour_array[] = {time_string[colon_index - 2], time_string[colon_index - 1], '\0'};
	char min_array[] = {time_string[colon_index + 1], time_string[colon_index + 2], '\0'};

	//Convert the hour and minute to integers.
	int hour, min;
	sscanf(hour_array, "%d", &hour);
	sscanf(min_array, "%d", &min);

	//Find "AM" or "PM."
	PyObject* parameter_list = PyObject_GetAttrString(mka, "parameter_list");
	int meridian_index = getCharacterIndex(time_string, 'p');
	if(meridian_index < 0)
		meridian_index = getCharacterIndex(time_string, 'P');
	if(meridian_index < 0)
		meridian_index = getCharacterIndex(time_string, 'a');
	if(meridian_index < 0)
		meridian_index = getCharacterIndex(time_string, 'A');

	if(meridian_index < 0) { //24h mode.
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

//Check the parameter list and send IBus data as needed.
void checkParameterList(PyObject* mka, ParameterList* current_parameters, const int ibus_port) {
	PyObject* parameter_list = PyObject_GetAttrString(mka, "parameter_list");

	//Compare the C and Python parameter lists.
	const int8_t phone_type = PyLong_AsLong(PyObject_GetAttrString(parameter_list, "phone_type"))&0xFF;
	const int8_t version = PyLong_AsLong(PyObject_GetAttrString(parameter_list, "version"));

	const bool phone_active = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "phone_active"));
	const bool playing = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "playing"));
	const bool bmbt_connected = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "bmbt_connected"));
	const bool audio_selected = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "audio_selected"));

	char* song_title = PyBytes_AsString(PyUnicode_AsEncodedString(PyObject_GetAttrString(parameter_list, "song_title"), "utf-8", "strict"));
	if(strlen(song_title) >= STRING_BUF_LEN)
		song_title[STRING_BUF_LEN - 1] = '\0';
	
	char* artist = PyBytes_AsString(PyUnicode_AsEncodedString(PyObject_GetAttrString(parameter_list, "artist"), "utf-8", "strict"));
	if(strlen(artist) >= STRING_BUF_LEN)
		artist[STRING_BUF_LEN - 1] = '\0';

	char* album = PyBytes_AsString(PyUnicode_AsEncodedString(PyObject_GetAttrString(parameter_list, "album"), "utf-8", "strict"));
	if(strlen(album) >= STRING_BUF_LEN)
		album[STRING_BUF_LEN - 1] = '\0';

	char* app = PyBytes_AsString(PyUnicode_AsEncodedString(PyObject_GetAttrString(parameter_list, "app"), "utf-8", "strict"));
	if(strlen(app) >= STRING_BUF_LEN)
		app[STRING_BUF_LEN - 1] = '\0';

	char* phone_name = PyBytes_AsString(PyUnicode_AsEncodedString(PyObject_GetAttrString(parameter_list, "phone_name"), "utf-8", "strict"));
	if(strlen(phone_name) >= STRING_BUF_LEN)
		phone_name[STRING_BUF_LEN - 1] = '\0';
	
	bool refresh = false; //True if a refresh message is required.

	//Check if a phone was connected or disconnected.
	if(phone_type != current_parameters->phone_type) { //TODO: Should the phone light message be sent if the BMBT is not connected?
		current_parameters->phone_type = phone_type;
		if(bmbt_connected) {
			if(phone_type == PARAM_NO_PHONE)
				setPhoneLight(ibus_port, PHONE_LED_RED);
			else
				setPhoneLight(ibus_port, PHONE_LED_GREEN);
			if(audio_selected) {
				sendAllRadioCenterText(song_title, artist, album, app, version, ibus_port, false);
				if(phone_type == PARAM_ANDROID){
					sendRadioMainText("Android", version, ibus_port);
				} else if(phone_type == PARAM_CARPLAY) {
					sendRadioMainText("CarPlay", version, ibus_port);
				} else {
					sendRadioMainText("MKA", version, ibus_port);
				}
				refresh = true;
			}
		}
	}

	//Check if the song title changed.
	if(strcmp(song_title, current_parameters->song_title) != 0) {
		strcpy(current_parameters->song_title, song_title);
		if(audio_selected) {
			sendRadioCenterText(song_title, SONG_NAME, version, ibus_port);
			refresh = true;
		}
	}

	//Check if the artist changed.
	if(strcmp(artist, current_parameters->artist) != 0) {
		strcpy(current_parameters->artist, artist);
		if(audio_selected) {
			sendRadioCenterText(artist, ARTIST, version, ibus_port);
			refresh = true;
		}
	}

	//Check if the album changed.
	if(strcmp(album, current_parameters->album) != 0) {
		strcpy(current_parameters->album, album);
		if(audio_selected) {
			sendRadioCenterText(album, ALBUM, version, ibus_port);
			refresh = true;
		}
	}

	//Check if the app name changed.
	if(strcmp(app, current_parameters->app_name) != 0) {
		strcpy(current_parameters->app_name, app);
		if(audio_selected) {
			sendRadioCenterText(app, APP, version, ibus_port);
			refresh = true;
		}
	}

	//Send the refresh message.
	if(refresh && version >= 5) //TODO: Do earlier versions require a refresh?
		sendRefresh(ibus_port, 0x63);

	//Activate phone lights.
	if(bmbt_connected != current_parameters->bmbt_connected) {
		current_parameters->bmbt_connected = bmbt_connected;
		if(bmbt_connected) {
			if(phone_type == PARAM_NO_PHONE)
				setPhoneLight(ibus_port, PHONE_LED_RED);
			else {
				setPhoneLight(ibus_port, PHONE_LED_GREEN);
			}
		}
	}

	//Determine whether the MKA was selected/deselected as the source.
	if(audio_selected != current_parameters->audio_selected) {
		current_parameters->audio_selected = audio_selected;
		if(audio_selected ) {
			//TODO: Anything?
		} else {
			PyObject* title_msg_p = PyObject_GetAttrString(parameter_list, "main_radio_title");

			PyObject* refresh_tuple = PyTuple_New(2);
			PyTuple_SetItem(refresh_tuple, 0, title_msg_p);
			PyTuple_SetItem(refresh_tuple, 1, PyBool_FromLong(1));

			PyObject* set_overlay_text = PyObject_GetAttrString(mka, "setOverlayText");
			PyObject_CallObject(set_overlay_text, refresh_tuple);
		}
	}

	//Determine whether the phone name was sent.
	if(strcmp(phone_name, current_parameters->phone_name) != 0) {
		strcpy(current_parameters->phone_name, phone_name);
		if(audio_selected)
			sendRadioSubtitleText(phone_name, 6, version, ibus_port, true);
	}

	//Determine whether the phone is playing music.
	if(playing != current_parameters->playing) {
		current_parameters->playing = playing;

		if(audio_selected) {
			if(playing)
				sendRadioSubtitleText(">", 2, version, ibus_port, true);
			else
				sendRadioSubtitleText("||", 2, version, ibus_port, true);
		}
	}
}
