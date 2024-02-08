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
