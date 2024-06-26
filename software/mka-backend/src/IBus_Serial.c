#include "IBus_Serial.h"

//Open the TERMIOS serial port.
int iserialOpen(const char* port) {
	int serial = open(port, O_RDWR);
	
	struct termios tty;
	tcgetattr(serial, &tty);

	tty.c_cflag |= PARENB;
	tty.c_cflag &= ~CSTOPB;

	tty.c_cflag &= ~CSIZE;
	tty.c_cflag |= CS8;

	tty.c_cflag |= CREAD | CLOCAL;

	tty.c_lflag &= ~ICANON;
	tty.c_lflag &= ~ECHO;
	tty.c_lflag &= ~ECHOE;
	tty.c_lflag &= ~ECHONL;
	tty.c_lflag &= ~ISIG;
	tty.c_iflag &= ~(IXON |IXOFF |IXANY);
	tty.c_iflag &= ~(IGNBRK|BRKINT|PARMRK|ISTRIP|INLCR|IGNCR|ICRNL);

	tty.c_oflag &= ~OPOST;
	tty.c_oflag &= ~ONLCR;
	tty.c_cc[VTIME] = 5;

	cfsetispeed(&tty, B9600);
	cfsetospeed(&tty, B9600);

	tcsetattr(serial, TCSANOW, &tty);

	return serial;
}

//Close the TERMIOS serial port.
void iserialClose(int port) {
	close(port);
}

//Read from the TERMIOS serial port.
int iserialRead(int port, char* buffer, int l) {
	return read(port, buffer, l);
}

//Write to the TERMIOS serial port.
void iserialWrite(int port, char* buffer, int l) {
	write(port, buffer, l);
}

//Read a single byte from the TERMIOS serial port.
char iserialReadByte(int port) {
	char bytes[1];
	iserialRead(port, bytes, 1);
	return bytes[0];
}

//Write a single byte to the TERMIOS serial port.
void iserialWriteByte(int port, char byte) {
	char bytes[1] = {byte};
	iserialWrite(port, bytes, 1);
}

//Get the number of bytes available on the TERMIOS serial port.
int iserialBytesAvailable(int port) {
	int bytes = 0;
	ioctl(port, FIONREAD, &bytes);
	return bytes;
}