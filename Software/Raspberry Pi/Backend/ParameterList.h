#include <stdint.h>
#include <stdbool.h>

#ifndef parameter_list_h
#define parameter_list_h

#define STRING_BUF_LEN 0x21

#define PARAM_NO_PHONE 0
#define PARAM_ANDROID 5
#define PARAM_CARPLAY 3

//A C parameter list to keep track of the states of the Python parameters.
typedef struct ParameterList {
	bool bmbt_connected;	//True if the BMBT has sent messages.

	bool phone_active;  //True if the phone screen is active.
	bool audio_selected;	//True if the MKA is the selected audio device.
	int8_t phone_type;  //The type of phone connected.

	char phone_name[STRING_BUF_LEN];	//The phone name.
	char song_title[STRING_BUF_LEN];	//The song title.
	char artist[STRING_BUF_LEN];	//The song artist.
	char album[STRING_BUF_LEN];	//The album name.
	char app_name[STRING_BUF_LEN];	//The name of the app that is currently playing music.
	bool playing;	//Whether the song is currently playing.
} ParameterList;

#endif
