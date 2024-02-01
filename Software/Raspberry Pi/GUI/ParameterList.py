#Phone type:
NO_PHONE = 0
ANDROID = 1
CARPLAY = 2

#Next menu:
NO_MENU = 0
MIRROR_MENU = 1
SETTINGS_MENU = 2

'''A list of variables to be shared between the MKA object and its windows, as well as the C backend.'''
class ParameterList:
	#General parameters.
	selected = False	#True if the radio has sent the "Play" command to the MKA.
	phone_active = False	#True if the phone screen is active.
	next_menu = NO_MENU	#The next menu to open.
	
	#Phone parameters.
	phone_type = NO_PHONE	#The type of phone connected.
	phone_name = ""	#The name of the connected phone.
	
	#Music parameters.
	song_title = ""	#The current song title.
	artist = ""	#The current artist name.
	album = ""	#The current album name.
	app = ""	#The name of the app that is currently playing music.
	playing = False	#True if the phone is playing music.
	
	#Setting parameters.
	night_level = 4	#The configured light level at which to turn night mode on (for RLS vehicles).
	light_level = 0	#The current light level for day/night mode. A value of less than 1 indicates no RLS is connected.
	headlights_on = False	#Whether headlights are on or off, for non-RLS vehicles.
	audio_hud = True	#Whether to display the heads-up display for other audio sources.
	autoconnect = True	#Whether to automatically start the MKA and mirrored phone when the car is turned on.
