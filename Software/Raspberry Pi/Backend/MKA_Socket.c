#include "MKA_Socket.h"

//Create a new socket with server and client ports.
MKA_Socket* createSocket() {
    MKA_Socket* the_return;
    the_return = (MKA_Socket*) malloc(sizeof(MKA_Socket));

    the_return->server_socket = socket(AF_UNIX, SOCK_STREAM, 0);
    
    struct sockaddr_un server_address;
    server_address.sun_family = AF_UNIX;
    strcpy(server_address.sun_path, SOCKET_PATH);

    bind(the_return->server_socket, (struct sockaddr*) &server_address, sizeof(server_address));
    listen(the_return->server_socket, 5);

    the_return->client_socket = accept(the_return->server_socket, NULL, NULL);

    return the_return;
}

//Clear the socket from memory.
void clearSocket(MKA_Socket* socket) {
    free(socket);
}

//Read data from the socket.
ssize_t readSocket(MKA_Socket* socket, uint8_t* data, const int length) {
    return recv(socket->client_socket, data, length, 0);
}
