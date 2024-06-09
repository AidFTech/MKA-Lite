#include <fcntl.h>
#include <errno.h>
#include <termios.h>
#include <unistd.h>
#include <sys/ioctl.h>

#ifndef ibus_serial_h
#define ibus_serial_h

int iserialOpen(const char* port);
void iserialClose(int port);

int iserialRead(int port, char* buffer, int l);
void iserialWrite(int port, char* buffer, int l);
char iserialReadByte(int port);
void iserialWriteByte(int port, char byte);

int iserialBytesAvailable(int port);

#endif