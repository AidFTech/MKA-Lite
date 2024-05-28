#include "IBus_Message.h"

//Create a new IBus message.
struct IBus_Message* createMessage(const uint16_t l, const uint8_t sender, const uint8_t receiver) {
    IBus_Message* the_return;
    the_return = (IBus_Message*) malloc(sizeof(IBus_Message));

    the_return->l = l;
    the_return->sender = sender;
    the_return->receiver = receiver;

    the_return->data = (uint8_t*) malloc(l);

    return the_return;
}

//Clear an IBus message.
void clearMessage(IBus_Message* msg) {
    free(msg->data);
    free(msg);
}

//Refresh the length, sender, and receiver of a message.
void refreshIBusMessage(IBus_Message *msg, uint16_t newl, const uint8_t sender, const uint8_t receiver) {
    msg->l = newl;
    msg->sender = sender;
    msg->receiver = receiver;

    free(msg->data);
    msg->data = (uint8_t*) malloc(newl);
}

//Fill an IBus data array with a populated array.
void fillIBusData(IBus_Message *msg, uint8_t* data) {
    for(unsigned int i=0;i<msg->l;i+=1)
        msg->data[i] = data[i];
}
