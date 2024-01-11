#include "IBus_Handler.h"

#ifdef RPI_UART
//Open the serial port defined by string port. Returns the port number to be used throughout the program.
int ibusSerialInit(char* port) {
	gpioInitialise();
	return serOpen(port, IBUS_BAUD, 0); //TODO: Add even parity option.
}

//Close the serial port.
void ibusSerialClose(const int port) {
	serClose(port);
}
#endif

//Read IBus data. Return -1 if unsuccessful, otherwise return the number of bytes.
int readIBusData(const int port, uint8_t* sender, uint8_t* receiver, uint8_t* data) {
	#ifdef RPI_UART
	if(serDataAvailable(port) >= 2) {
		const uint8_t s = (uint8_t)(serReadByte(port));
		const uint8_t l = (uint8_t)(serReadByte(port));

		if(l<2 || serDataAvailable(port) != l)
			return -1;
		
		const uint8_t r = (uint8_t)(serReadByte(port));

		char d_c[l-1];
		serRead(port, d_c, l-1);

		uint8_t d[l-1];
		for(uint8_t i=0;i<l;i+=1)
			d[i] = (uint8_t)(d_c[i]);
		
		{
			uint8_t chex[l+2];
			chex[0] = s;
			chex[1] = l;
			chex[2] = r;

			for(uint8_t i=0;i<l-1;i+=1)
				chex[i+3] = d[i];
			
			if(getChecksum(s, r, d, l-1)
				return -1;
		}

		*sender = s;
		*receiver = r;

		for(uint8_t i=0;i<l-2;i+=1)
			data[i] = d[i];
		
		return l-2;
	} else
		return -1;
	#else
	return -1; //TODO: Use SCANF to read in a string.
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
	//TODO: Confirm that the port is free.
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
		checksum ^= data[3+i];

	return checksum;
}