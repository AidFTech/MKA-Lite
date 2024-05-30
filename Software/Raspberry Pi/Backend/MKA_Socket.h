#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>

#include "IBus_Message.h"

#ifndef mka_socket_h
#define mka_socket_h

#define SOCKET_PATH "/run/mka_to_backend.sock"
#define SOCKET_START "MKASock"

//Socket messages.
typedef struct Socket_Message {
    uint8_t opcode;
    uint16_t l;
    
    uint8_t* data;
} Socket_Message;

Socket_Message* createSocketMessage(const uint8_t opcode, const uint16_t length);
void clearSocketMessage(Socket_Message* message);
void fillSocketMessageBytes(Socket_Message* message, uint8_t* data);
void refreshSocketMessage(Socket_Message* message, const uint8_t opcode, const uint16_t length);

//Socket handler.
typedef struct MKA_Socket {
    int server_socket, client_socket;
} MKA_Socket;

MKA_Socket* createSocket();
void clearSocket(MKA_Socket* socket);

ssize_t readSocketBytes(MKA_Socket* socket, uint8_t* data, const int length);
void writeSocketBytes(MKA_Socket* socket, uint8_t* data, const int length);

int readSocketMessage(MKA_Socket* socket, Socket_Message* message, const int length);
void writeSocketMessage(MKA_Socket* socket, Socket_Message* message);

void writeIBusToSocket(MKA_Socket* socket, IBus_Message* message);

#endif
