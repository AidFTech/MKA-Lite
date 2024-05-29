#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include <sys/types.h>
#include <sys/socket.h>
#include <sys/un.h>

#ifndef mka_socket_h
#define mka_socket_h

#define SOCKET_PATH "/run/mka_to_backend.sock"

typedef struct MKA_Socket {
    int server_socket, client_socket;
} MKA_Socket;

MKA_Socket* createSocket();
void clearSocket(MKA_Socket* socket);

ssize_t readSocket(MKA_Socket* socket, uint8_t* data, const int length);

#endif
