#include "IBus_IO.h"

//Open the serial port defined by string port. Returns the port number to be used throughout the program.
int ibusSerialInit(char* port) {
	#ifdef RPI_UART
	gpioInitialise();
	gpioSetMode(IB_RX, PI_INPUT);
	gpioSetPullUpDown(IB_RX, PI_PUD_UP);
	return serOpen(port, IBUS_BAUD, 0); //TODO: Add even parity option.
	#else
	printf("Ready!\nEnter the sender, receiver, and data. Separate all characters with a space. Do not include the checksum.\n");
	#endif
}

//Close the serial port.
void ibusSerialClose(const int port) {
	#ifdef RPI_UART
	serClose(port);
	gpioTerminate();
	#endif
}

//Read IBus data. Return -1 if unsuccessful, otherwise return the number of bytes.
int readIBusData(const int port, uint8_t* sender, uint8_t* receiver, uint8_t* data) {
	#ifdef RPI_UART
	if(serDataAvailable(port) >= 2) {
		const uint8_t s = (uint8_t)(serReadByte(port));
		const uint8_t l = (uint8_t)(serReadByte(port));

		if(l<2)
			return -1;
		
		clock_t start = clock();
		while(serDataAvailable(port) < l) {
			if((clock() - start)/(CLOCKS_PER_SEC/1000) >= MAX_DELAY) {
				return -1;
			}
		}
		
		const uint8_t r = (uint8_t)(serReadByte(port));

		char d_c[l-1];
		serRead(port, d_c, l-1);

		uint8_t d[l-1];
		for(uint8_t i=0;i<l-1;i+=1)
			d[i] = (uint8_t)(d_c[i]);
		
		if(getChecksum(s, r, d, l-2) != d[l-2])
			return -1;

		*sender = s;
		*receiver = r;

		for(uint8_t i=0;i<l-2;i+=1)
			data[i] = d[i];
		
		return l-2;
	} else
		return -1;
	#else
	char c;
	int num = 0;
	uint8_t l=0;
	uint8_t data_in[255];

	do {
		scanf("%c", &c);
		if(c == '\n')
			break;
		else if(c == ' ') { //New character.
			data_in[l] = (uint8_t)(num);
			l += 1;
			num = 0;
		} else if((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')) {
			num <<= 4;
			num |= charToNumber(c);
		}
	} while(c != '\n' && l < 255);
	data_in[l] = num;
	l+=1;
	
	if(l >= 2) {
		*sender = data_in[0];
		*receiver = data_in[1];

		for(uint8_t i=0;i<l-2;i+=1)
			data[i] = data_in[i+2];

		return l-2;
	} else
		return -1;
	#endif
}

//Write IBus data to the serial port.
void writeIBusData(const int port, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l) {
	const unsigned int full_length = l+4;
	uint8_t msg_data[full_length];

	msg_data[0] = sender;
	msg_data[1] = l + 2;
	msg_data[2] = receiver;

	for(uint8_t i=0;i<l;i+=1)
		msg_data[3+i] = data[i];

	msg_data[full_length-1] = getChecksum(sender, receiver, data, l);
	
	#ifdef RPI_UART
	clock_t start = clock();
	while((clock() - start)/(CLOCKS_PER_SEC/1000) < IB_WAIT) {
		if(gpioRead(IB_RX) == 0)
			start = clock();
	}
	for(uint8_t i=0;i<full_length;i+=1)
		serWriteByte(port, msg_data[i]);
	#else
	for(uint8_t i=0;i<full_length;i+=1)
		printf("%X ", msg_data[i]);
	printf("\n");
	#endif
}

//Get an IBus checksum.
uint8_t getChecksum(const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l) {
	uint8_t checksum = 0;
	checksum ^= sender;
	checksum ^= (uint8_t)(l+2);
	checksum ^= receiver;
	for(uint8_t i=0;i<l;i+=1)
		checksum ^= data[i];

	return checksum;
}

#ifndef RPI_UART
uint16_t charToNumber(char c) {
	uint16_t the_return = 0;

	switch(c) {
		case '0':
			the_return |= 0x0;
			break;
		case '1':
			the_return |= 0x1;
			break;
		case '2':
			the_return |= 0x2;
			break;
		case '3':
			the_return |= 0x3;
			break;
		case '4':
			the_return |= 0x4;
			break;
		case '5':
			the_return |= 0x5;
			break;
		case '6':
			the_return |= 0x6;
			break;
		case '7':
			the_return |= 0x7;
			break;
		case '8':
			the_return |= 0x8;
			break;
		case '9':
			the_return |= 0x9;
			break;
		case 'A':
		case 'a':
			the_return |= 0xA;
			break;
		case 'B':
		case 'b':
			the_return |= 0xB;
			break;
		case 'C':
		case 'c':
			the_return |= 0xC;
			break;
		case 'D':
		case 'd':
			the_return |= 0xD;
			break;
		case 'E':
		case 'e':
			the_return |= 0xE;
			break;
		case 'F':
		case 'f':
			the_return |= 0xF;
			break;
	}

	return the_return;
}
#endif
