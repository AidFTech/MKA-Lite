#include "Radio_Handler.h"

//Handle a radio-related IBus message.
void handleRadioIBus(ParameterList* parameter_list, IBus_Message* ib_data, const int ibus_port) {
    if(ib_data->data[0] == IBUS_COMMAND_CDC_REQUEST) { //CD changer request. Must reply.
        const bool selected = parameter_list->audio_selected;
        if(ib_data->data[1] == IBUS_CDC_CMD_GET_STATUS  //Request current CD and track status.
          || ib_data->data[1] == IBUS_CDC_CMD_CHANGE_TRACK) { //Change the song. The actual song change is carried out by the socket.
            if(selected)
                sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_PLAYING, ib_data->sender);
            else
                sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_STOP, ib_data->sender);
        } else if(ib_data->data[1] == IBUS_CDC_CMD_STOP_PLAYING) { //Stop the MKA.
            sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_STOP, ib_data->sender);
            parameter_list->audio_selected = false;
        } else if(ib_data->data[1] == IBUS_CDC_CMD_START_PLAYING || ib_data->data[1] == IBUS_CDC_CMD_PAUSE_PLAYING) { //Start the MKA.
            sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_PLAYING, ib_data->sender);
            parameter_list->audio_selected = true;
        } else { //N/A message- send the "wait" signal.
            if(parameter_list->audio_selected)
                sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_END, ib_data->sender);
            else
                sendCDStatusMessage(ibus_port, IBUS_CDC_STAT_STOP, ib_data->sender);
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
                        
    IBus_Message* status_msg = createMessage(sizeof(data), IBUS_DEVICE_CDC, receiver);
    writeIBusData(ibus_port, status_msg);
    
    clearMessage(status_msg);
}
