#include "MKA_Backend.h"

int main(int argc, char* argv[]) {
    #ifdef RPI_UART
    int ibus = ibusSerialInit("/dev/ttyAMA0");
    #else
    int ibus = ibusSerialInit("");
    #endif
    MKA mka;

    mka.mka_socket = createSocket();

    clock_t ping_start = clock();

    bool running = true;
    mka.running = &running;
    IBus_Message* ib_data = createMessage(0, 0x18, 0x68);

    pthread_t socket_thread;
    pthread_create(&socket_thread, NULL, socketThread, (void *)&mka);

    while(running) {
        if(readIBusData(ibus, ib_data, &ibus) > 0) {
            writeIBusToSocket(mka.mka_socket, ib_data);
        }
    }

    pthread_join(socket_thread, NULL);

    clearMessage(ib_data);
    clearSocket(mka.mka_socket);
}

void *socketThread(void* mka_v) {
    MKA* mka = (MKA*)mka_v;
    Socket_Message* recv_msg = createSocketMessage(0x68, 1024);
    while(*mka->running) {
        /*uint8_t sock_data[1024];
        const int size = readSocketBytes(mka->mka_socket, sock_data, sizeof(sock_data));
        if(size > 0) {
            //TODO: Handle socket input.
            printf("Received %d!\n",size);
        }*/
        if(readSocketMessage(mka->mka_socket, recv_msg, 1024) > 0) {
            if(recv_msg->l >= 1) {
                if(recv_msg->opcode == OPCODE_RECV_IBUS) { //Send an IBus message.
                    IBus_Message* ib_data = createMessage(recv_msg->l - 4, recv_msg->data[0], recv_msg->data[2]);
                    for(unsigned int i=3;i<recv_msg->l-1;i+=1)
                        ib_data->data[i-3] = recv_msg->data[i];

                    writeIBusData(mka->ibus_port, ib_data);

                    clearMessage(ib_data);
                } else if(recv_msg->opcode == OPCODE_PHONE_NAME) { //Set the phone name.
                    strncpy(mka->parameter_list.phone_name, recv_msg->data, sizeof(mka->parameter_list.phone_name)/sizeof(char) - 1);
                } else {
                    switch(recv_msg->opcode) {
                    case OPCODE_PHONE_ACTIVE:
                        mka->parameter_list.phone_active = recv_msg->data[0] != 0;
                        break;
                    case OPCODE_MKA_ACTIVE:
                        mka->parameter_list.mka_active = recv_msg->data[0] != 0;
                        break;
                    case OPCODE_AUDIO_SELECTED:
                        mka->parameter_list.audio_selected = recv_msg->data[0] != 0;
                        break;
                    case OPCODE_PHONE_TYPE:
                        mka->parameter_list.phone_type = recv_msg->data[0];
                        break;
                    case OPCODE_PLAYING:
                        mka->parameter_list.playing = recv_msg->data[0] != 0;
                        break;
                    case OPCODE_BMBT_CONNECTED:
                        mka->parameter_list.bmbt_connected = recv_msg->data[0] != 0;
                    }
                }
            }
            
        }
    }

    clearSocketMessage(recv_msg);
}