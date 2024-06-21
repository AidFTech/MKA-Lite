#include "MKA_Socket.h"

//Create a new socket message.
Socket_Message* createSocketMessage(const uint8_t opcode, const uint16_t length) {
    Socket_Message* the_return;
    the_return = (Socket_Message*) malloc(sizeof(Socket_Message));

    the_return->opcode = opcode;
    the_return->l = length;

    the_return->data = (uint8_t*) malloc(length*sizeof(uint8_t));

    return the_return;
}

//Clear a socket message from memory.
void clearSocketMessage(Socket_Message* message) {
    free(message->data);
    free(message);
}

//Populate a socket message with data bytes.
void fillSocketMessageBytes(Socket_Message* message, uint8_t* data) {
    for(unsigned int i=0;i<message->l;i+=1)
        message->data[i] = data[i];
}

//Refresh a socket message.
void refreshSocketMessage(Socket_Message* message, const uint8_t opcode, const uint16_t length) {
    message->opcode = opcode;
    message->l = length;

    free(message->data);
    message->data = (uint8_t*)malloc(length*sizeof(uint8_t));
}

//Create a new socket with server and client ports.
MKA_Socket* createSocket() {
    remove(SOCKET_PATH);

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
    //TODO: Close the sockets?
    free(socket);
}

//Read binary data from the socket.
ssize_t readSocketBytes(MKA_Socket* socket, uint8_t* data, const int length) {
    return recv(socket->client_socket, data, length, 0);
}

//Write binary data to the socket.
void writeSocketBytes(MKA_Socket* socket, uint8_t* data, const int length) {
    send(socket->client_socket, data, length, 0);
}

//Read a message from the socket.
int readSocketMessage(MKA_Socket* socket, Socket_Message* message, const int length) {
    uint8_t read_data[length];
    const int message_size = readSocketBytes(socket, read_data, length);

    if(message_size <= 0)
        return message_size;

    if(message_size < sizeof(SOCKET_START) + 2)
        return -1;

    for(uint8_t i=0;i<sizeof(SOCKET_START)-1;i+=1) {
        if(read_data[i] != (uint8_t)SOCKET_START[i])
            return -1;
    }

    const uint8_t opcode = read_data[sizeof(SOCKET_START) - 1], msg_length = read_data[sizeof(SOCKET_START)]-1;

    refreshSocketMessage(message, opcode, msg_length);
    for(uint8_t i=0;i<msg_length;i+=1)
        message->data[i] = read_data[sizeof(SOCKET_START) + 1 + i];

    return msg_length;
}

//Write a message to the socket.
void writeSocketMessage(MKA_Socket* socket, Socket_Message* message) {
    if(message->l + 1 > 255)
        return;

    uint8_t write_data[message->l + 10];

    for(uint8_t i=0;i<sizeof(SOCKET_START)-1;i+=1)
        write_data[i] = SOCKET_START[i];

    write_data[7] = message->opcode;
    write_data[8] = (uint8_t)(message->l + 1);

    for(int i=0;i<message->l;i+=1)
        write_data[i+9] = message->data[i];

    uint8_t checksum = 0;
    for(uint8_t i=0;i<sizeof(write_data)-1;i+=1)
        checksum ^= write_data[i];

    write_data[sizeof(write_data)-1] = checksum;

    writeSocketBytes(socket, write_data, message->l + 10);
}

//Write an IBus message to the socket.
void writeIBusToSocket(MKA_Socket* socket, IBus_Message* message) {
    Socket_Message* sock_message = createSocketMessage(0x68, message->l + 4);

    uint8_t sock_bytes[message->l + 4];
    getBytes(message, sock_bytes);

    fillSocketMessageBytes(sock_message, sock_bytes);

    writeSocketMessage(socket, sock_message);
    clearSocketMessage(sock_message);
}
