#include "Radio_Handler.h"

//Handle a radio-related IBus message.
void handleRadioIBus(PyObject* mka, const int ibus_port, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l) {
	if(receiver == IBUS_DEVICE_CDC && data[0] == IBUS_COMMAND_CDC_REQUEST) {
		PyObject* parameter_list = PyObject_GetAttrString(mka, "parameter_list");
		const uint8_t selected = PyObject_IsTrue(PyObject_GetAttrString(parameter_list, "audio_selected"));
		
		if(data[1] == IBUS_CDC_CMD_GET_STATUS) { //Request current CD and track status.
			if(selected)
				sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_PLAYING, sender);
			else
				sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_STOP, sender);
		} else if(data[1] == IBUS_CDC_CMD_STOP_PLAYING) {
			sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_STOP, sender);
			setSelected(parameter_list, 0);
		} else if(data[1] == IBUS_CDC_CMD_START_PLAYING || data[1] == IBUS_CDC_CMD_PAUSE_PLAYING) {
			sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_PLAYING, sender);
			setSelected(parameter_list, 1);
		} else if(data[1] == IBUS_CDC_CMD_CHANGE_TRACK) {
			if(data[2] == 0x0) {
				//TODO: Next track.
			} else if(data[2] == 0x01) {
				//TODO: Previous track.
			}
			sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_PLAYING, sender);
		} else {
			if(selected)
				sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_END, sender);
			else
				sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_STOP, sender);
		}
	}
}

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
	
	writeIBusData(ibus_port, IBUS_DEVICE_CDC, receiver, data, l);
}

void setSelected(PyObject* parameter_list, const int selected) {
	PyObject_SetAttrString(parameter_list, "audio_selected", PyBool_FromLong(selected));
	//TODO: Start/stop phone music playback.
}

//Send a radio text change message. Returns number of bytes if successful, -1 if not.
int sendRadioText(const char* text, const uint8_t position, const int8_t version, const int port) {
	if(version >= 5) { //Newer GT.
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
		text_message[1] = 0x63;
		text_message[2] = 0x1;
		text_message[3] = index;
		for(int i=0;i<strlen(text);i+=1)
			text_message[i+4] = text[i];

		if(strlen(text) <= 0)
			text_message[4] = ' ';

		writeIBusData(port, IBUS_DEVICE_RAD, IBUS_DEVICE_GT, text_message, sizeof(text_message));
		return sizeof(text_message);
	} else return -1;
}

void sendRefresh(const int port) {
	char refresh_msg[] = {IBUS_CMD_GT_WRITE_WITH_CURSOR, 0x63, 0x1, 0x0};
	writeIBusData(port, IBUS_DEVICE_RAD, IBUS_DEVICE_GT, refresh_msg, sizeof(refresh_msg));
}