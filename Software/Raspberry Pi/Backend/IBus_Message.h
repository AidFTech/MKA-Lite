#include <stdint.h>
#include <stdlib.h>

#ifndef ibus_message_h
#define ibus_message_h

typedef struct IBus_Message {
    uint8_t* data;
    uint8_t sender, receiver;
    uint16_t l;

} IBus_Message;

struct IBus_Message* createMessage(const uint16_t l, const uint8_t sender, const uint8_t receiver);
void clearMessage(IBus_Message* msg);

void refreshIBusMessage(IBus_Message *msg, uint16_t newl, const uint8_t sender, const uint8_t receiver);
void fillIBusData(IBus_Message *msg, uint8_t* data);

#endif
