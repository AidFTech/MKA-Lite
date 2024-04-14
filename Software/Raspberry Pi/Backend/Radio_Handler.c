#include "Radio_Handler.h"

//Handle a radio-related IBus message.
void handleRadioIBus(PyObject* mka, const int ibus_port, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l) {
	PyObject* parameter_list = PyObject_GetAttrString(mka, "parameter_list");
	const bool selected = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "audio_selected")) != 0;
	if(receiver == IBUS_DEVICE_CDC && data[0] == IBUS_COMMAND_CDC_REQUEST) {
		const uint8_t selected = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "audio_selected"));
		
		if(data[1] == IBUS_CDC_CMD_GET_STATUS) { //Request current CD and track status.
			if(selected)
				sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_PLAYING, sender);
			else
				sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_STOP, sender);
		} else if(data[1] == IBUS_CDC_CMD_STOP_PLAYING) {
			sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_STOP, sender);
			setSelected(mka, parameter_list, 0);
		} else if(data[1] == IBUS_CDC_CMD_START_PLAYING || data[1] == IBUS_CDC_CMD_PAUSE_PLAYING) {
			sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_PLAYING, sender);
			setSelected(mka, parameter_list, 1);
		} else if(data[1] == IBUS_CDC_CMD_CHANGE_TRACK) {
			//int phone_active = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "phone_active"));
			if(selected) {
				if(data[2] == 0x0) {
					seekTrack(mka, 1);
				} else if(data[2] == 0x01) {
					seekTrack(mka, 0);
				}
				sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_PLAYING, sender);
			} else {
				sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_STOP, sender);
			}
		} else {
			if(selected)
				sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_END, sender);
			else
				sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_STOP, sender);
		}
	} else if(data[0] == IBUS_CMD_RAD_SCREEN_MODE_UPDATE) {
		if(PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "mka_active")) && (data[1]&0x1 != 0)) {
			//Audio screen was canceled. Force it back.
			uint8_t force_audio_screen_data[] = {IBUS_CMD_GT_SCREEN_MODE_SET, 0x0};
			writeIBusData(ibus_port, IBUS_DEVICE_GT, IBUS_DEVICE_RAD, force_audio_screen_data, sizeof(force_audio_screen_data));
		}
	} else if(data[0] == IBUS_CMD_GT_WRITE_TITLE) { //Screen text. //TODO: Set "Selected" to false if this says an FM frequency, tape info, anything that is not a CD changer header.
		if(selected && data[l-1] != 0x8E) {
			clock_t start_time = clock();

			bool sent_22 = false;
			uint8_t data[255];
			uint8_t l, sender, receiver;
			int p;
			while(!sent_22 && (clock() - start_time)/(CLOCKS_PER_SEC/1000) < 750) {
				l = readIBusData(ibus_port, &sender, &receiver, data, &p);
				if(l <= 0)
					continue;
				
				if(sender == IBUS_DEVICE_GT && data[0] == IBUS_CMD_GT_WRITE_RESPONSE) {
					sent_22 = true;
					break;
				}
			}

			if(sent_22) {
				const int8_t version = 5;	//TODO: Sync with GT. Get from Python?
				sendRadioScreenUpdate(parameter_list, version, ibus_port);
			}
		} else if(!selected) {
			char title_msg[l-3];
			for(int i=0;i<sizeof(title_msg)/sizeof(char);i+=1)
				title_msg[i] = (char)(data[i+3]);

			PyObject* title_msg_p = PyUnicode_FromString(title_msg);
			if(title_msg_p != NULL) {
				PyObject_SetAttrString(parameter_list, "main_radio_title", title_msg_p);
			}
		}
	}
}

//Send the 0x39 CD status reply.
void sendCDStatusMessage(const int ibus_port, const uint8_t status, const uint8_t receiver) {
	uint8_t pseudo_status = 0x89;
	if(status == IBUS_CDC_STAT_STOP)
		pseudo_status = 0x82;
	
	uint8_t data[] = {IBUS_COMMAND_CDC_RESPONSE,
						status,
						pseudo_status,
						0x00,
						0x3F,
						0x00,
						0x1,
						0x1,
						0x0,
						0x1,
						0x1,
						0x1};
						
	const uint16_t l = sizeof(data);
	
	writePriorityIBusData(ibus_port, IBUS_DEVICE_CDC, receiver, data, l);
}

//Press one of the seek buttons.
void seekTrack(PyObject* mka, const uint8_t forward) {
	PyObject* handle_seek = PyObject_GetAttrString(mka, "handleSeekButton");
	PyObject* tuple = PyTuple_New(1);
	PyTuple_SetItem(tuple, 0, PyBool_FromLong(forward));

	PyObject_CallObject(handle_seek, tuple);
}

//Set whether the MKA is selected.
void setSelected(PyObject* mka, PyObject* parameter_list, const int selected) {
	PyObject_SetAttrString(parameter_list, "audio_selected", PyBool_FromLong(selected));
	PyObject* set_selected = PyObject_GetAttrString(mka, "setSelected");
	PyObject_CallObject(set_selected, NULL);
}

//Send all radio screen update messages.
void sendRadioScreenUpdate(PyObject* parameter_list, const uint8_t version, const int port) {
	sendAllRadioCenterTextFromParameters(parameter_list, version, port, true);
	const int phone_type = PyLong_AsLong(PyObject_GetAttrString(parameter_list, "phone_type"));
	if(phone_type == 3) { //Apple.
		sendRadioMainText("CarPlay", version, port);
	} else if(phone_type == 5) { //Android.
		sendRadioMainText("Android", version, port);
	} else {
		sendRadioMainText("MKA", version, port);
	}

	sendRadioSubtitleText(" ", 1, version, port, false);

	const bool audio_play = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "playing"));
	if(audio_play)
		sendRadioSubtitleText(">", 2, version, port, false);
	else
		sendRadioSubtitleText("||", 2, version, port, !(phone_type == 3 || phone_type == 5));

	if(phone_type == 3 || phone_type == 5) {
		PyObject* phone_name_p = PyObject_GetAttrString(parameter_list, "phone_name");
		const char* phone_name = PyBytes_AsString(PyUnicode_AsEncodedString(phone_name_p, "utf-8", "strict"));

		sendRadioSubtitleText(phone_name, 6, version, port, true);
	}
}

//Send a radio header change message. Returns number of bytes if successful, -1 if not.
int sendRadioMainText(const char* text, const int8_t version, const int port) {
	int new_len = strlen(text);
	if(new_len <= 0)
		new_len = 1;
	
	uint8_t text_data[new_len + 4];

	text_data[0] = IBUS_CMD_GT_WRITE_TITLE;
	text_data[1] = 0x62;
	text_data[2] = 0x30;
	for(int i=0;i<strlen(text);i+=1)
		text_data[i+3] = text[i];

	if(strlen(text) < 1)
		text_data[3] = ' ';

	text_data[new_len + 3] = 0x8E; //To make sure we know not to respond to it.

	writeIBusData(port, IBUS_DEVICE_RAD, IBUS_DEVICE_GT, text_data, new_len + 4);
}

//Send a radio subtitle change message. Returns number of bytes if successful, -1 if not.
int sendRadioSubtitleText(const char* text, const uint8_t zone, const int8_t version, const int port, const bool refresh) {
	int new_len = strlen(text);
	if(new_len <= 0)
		new_len = 1;
	
	uint8_t text_data[new_len + 4];

	text_data[0] = IBUS_CMD_GT_WRITE_WITH_CURSOR;
	text_data[1] = 0x62;
	text_data[2] = 0x1;
	text_data[3] = 0x40|(zone&0xF);
	for(int i=0;i<strlen(text);i+=1)
		text_data[i+4] = text[i];

	if(strlen(text) < 1)
		text_data[4] = ' ';

	writeIBusData(port, IBUS_DEVICE_RAD, IBUS_DEVICE_GT, text_data, new_len + 4);

	if(refresh)
		sendRefresh(port, 0x62);
}

//Send a radio text change message. Returns number of bytes if successful, -1 if not.
int sendRadioCenterText(const char* text, const uint8_t position, const int8_t version, const int port) {
	uint8_t index = 0x40;
	if(position == SONG_NAME)
		index = 0x41;
	else if(position == ARTIST)
		index = 0x42;
	else if(position == ALBUM)
		index = 0x43;
	else if(position == APP)
		index = 0x44;
	else
		return -1;

	int new_len = strlen(text);
	if(new_len <= 0)
		new_len = 1;

	char text_message[4+new_len];
	text_message[0] = IBUS_CMD_GT_WRITE_WITH_CURSOR;
	if(version >= 5)
		text_message[1] = 0x63;
	else
		text_message[1] = 0x60;
	text_message[2] = 0x1;
	text_message[3] = index;
	for(int i=0;i<strlen(text);i+=1)
		text_message[i+4] = text[i];

	if(strlen(text) <= 0)
		text_message[4] = ' ';

	writeIBusData(port, IBUS_DEVICE_RAD, IBUS_DEVICE_GT, text_message, sizeof(text_message));
	return sizeof(text_message);
}

//Send multiple radio text change messages.
void sendAllRadioCenterTextFromParameters(PyObject* parameter_list, const uint8_t version, const int port, const bool refresh) {
	PyObject* song_title_p = PyObject_GetAttrString(parameter_list, "song_title");
	PyObject* artist_p = PyObject_GetAttrString(parameter_list, "artist");
	PyObject* album_p = PyObject_GetAttrString(parameter_list, "album");
	PyObject* app_p = PyObject_GetAttrString(parameter_list, "app");

	const char* song_title = PyBytes_AsString(PyUnicode_AsEncodedString(song_title_p, "utf-8", "strict"));
	const char* artist = PyBytes_AsString(PyUnicode_AsEncodedString(artist_p, "utf-8", "strict"));
	const char* album = PyBytes_AsString(PyUnicode_AsEncodedString(album_p, "utf-8", "strict"));
	const char* app = PyBytes_AsString(PyUnicode_AsEncodedString(app_p, "utf-8", "strict"));

	sendAllRadioCenterText(song_title, artist, album, app, version, port, refresh);
}

//Send multiple radio text change messages.
void sendAllRadioCenterText(const char* song_title, const char* artist, const char* album, const char* app, const uint8_t version, const int port, const bool refresh) {
	sendRadioCenterText(song_title, SONG_NAME, version, port);
	sendRadioCenterText(artist, ARTIST, version, port);
	sendRadioCenterText(album, ALBUM, version, port);
	sendRadioCenterText(app, APP, version, port);
	if(refresh) {
		uint8_t index = 0x63;
		if(version < 5)
			index = 0x60;
		sendRefresh(port, index);
	}
}

//Send a refresh message.
void sendRefresh(const int port, const uint8_t index) {
	char refresh_msg[] = {IBUS_CMD_GT_WRITE_WITH_CURSOR, index, 0x1, 0x0};
	writeIBusData(port, IBUS_DEVICE_RAD, IBUS_DEVICE_GT, refresh_msg, sizeof(refresh_msg));
}