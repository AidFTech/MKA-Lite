#include <stdint.h>
#include <stdbool.h>

#include "IBus_IO.h"
#include "IBus_Message.h"
#include "ParameterList.h"

#ifndef radio_handler_h
#define radio_handler_h

bool handleRadioIBus(ParameterList* parameter_list, IBus_Message* ib_data, const int ibus_port);
void sendCDStatusMessage(const int ibus_port, const uint8_t status, const uint8_t receiver);

#endif