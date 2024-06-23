#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>

#include "IBus_Message.h"

#ifndef mka_socket_h
#define mka_socket_h

#define SOCKET_PATH "/run/mka_to_backend.sock"
#define SOCKET_START "MKASock"
#define SOCKET_MAX_CLIENTS 16
#define SOCKET_MAX_DATA_SIZE 255
#define SOCKET_FRAME_SIZE 10

#define OPCODE_SEND_IBUS 0x18
#define OPCODE_RECV_IBUS 0x68

#define OPCODE_PHONE_ACTIVE 0x21
#define OPCODE_MKA_ACTIVE 0x22
#define OPCODE_AUDIO_SELECTED 0x23
#define OPCODE_PHONE_TYPE 0x2B
#define OPCODE_PHONE_NAME 0x2C
#define OPCODE_PLAYING 0x39
#define OPCODE_BMBT_CONNECTED 0xF0

// Socket handler.
typedef struct MKASocket {
    int32_t server;
    int32_t clients[SOCKET_MAX_CLIENTS];
} MKASocket;

// Socket messages.
typedef struct SocketMessage {
    uint8_t opcode;
    uint16_t len;
    uint8_t *data;
} SocketMessage;

SocketMessage *SocketCreateMessage(const uint8_t opcode, const uint16_t length);
void SocketClearMessage(SocketMessage * message);
void SocketFillMessageBytes(SocketMessage * message, uint8_t* data);
void SocketRefreshMessage(SocketMessage * message, const uint8_t opcode, const uint16_t length);

MKASocket *SocketCreate();
void SocketClear(MKASocket *socket);
ssize_t SocketReadBytes(MKASocket *socket, uint8_t *data, const int length);
void SocketWriteBytes(MKASocket *socket, uint8_t *data, const int length);
int SocketReadMessage(MKASocket *socket, SocketMessage *message, const int length);
void SocketWriteMessage(MKASocket *socket, SocketMessage *message);
void writeIBusToSocket(MKASocket *socket, IBus_Message *message);

#endif
