#Phone type:
NO_PHONE = 0
ANDROID = 1
CARPLAY = 2

#Next menu:
NEXTMENU_OEM_MENU = -1 #Return to the OEM nav menu.
NEXTMENU_NO_MENU = 0
NEXTMENU_MIRROR_MENU = 1
NEXTMENU_SETTINGS_MENU = 2
NEXTMENU_COLOR_MENU = 3
NEXTMENU_CUSTOM_COLOR_MENU = 4
NEXTMENU_NIGHT_SENS_MENU = 5

class ParameterList:
	"""A list of variables to be shared between the MKA object and its windows, as well as the C backend."""
	#General parameters.
	bmbt_connected = False	#True if the BMBT has sent any IBus messages.
	audio_selected = False	#True if the radio has sent the "Play" command to the MKA.
	mka_active = True	#True if the MKA screen is active. This should force the audio screen open.
	phone_active = False	#True if the phone screen is active.
	next_menu = NEXTMENU_NO_MENU	#The next menu to open.
	fullscreen = False	#True if the program is rendered in fullscreen, i.e. if tested on a Pi.
	
	#Phone parameters.
	phone_type = NO_PHONE	#The type of phone connected.
	phone_name = ""	#The name of the connected phone.
	dongle_connected = False	#Whether or not the dongle is connected.
	
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

	#Time and date parameters.
	ike_hour = -1
	ike_minute = -1
	ike_datestring = "--/--/----"
	ike_24h = True	#24hr format
