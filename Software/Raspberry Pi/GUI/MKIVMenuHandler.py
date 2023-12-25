import IBusHandler

opt_lightsens = "Night Sens.: "
opt_autoconnect = "Autoplay: "
opt_source = "SRC: "
opt_main_menu = "MKA Main Menu"

MENU_HEADER = "MKA-Lite Settings"

def setOptions():
	options = [opt_autoconnect, opt_source, opt_main_menu]
	return options

class MKIVMMenu:
	options = setOptions()

	def __init__(self, parent, ibus_handler: IBusHandler.IBusHandler):
		self.parent = parent
		self.ibus_handler = ibus_handler
		self.menu_open = False
	
	#Send the menu creation message.
	def sendCreateMenuMessage(self):
		self.options = setOptions()

		if hasattr(self.parent, "RLS_connected") and self.parent.RLS_connected:
			str_lightsens = self.getLightSensitivityString()
			self.options.insert(0, str_lightsens)
		
		ind = self.options.index(opt_autoconnect)
		
		if self.parent.autoplay:
			self.options[ind] += "On"
		else:
			self.options[ind] += "Off"

		ind = self.options.index(opt_source)
		self.options[ind] += "MKA" #TODO: Expand this depending on selected source.

		self.ibus_handler.sendRadioButton(1, 9, MENU_HEADER, False)

		for i in range(0,len(self.options)):
			self.ibus_handler.sendRadioButton(1, i, self.options[i], i==len(self.options)-1)

		self.menu_open = True
		#TODO: Send a message to choose what is selected?
	
	#Make a selection. This is always called by BMirror after receiving a function call from the IBus handler.
	def makeSelection(self, index: int):
		if not self.menu_open:
			return
		
		if index < 0 or index >= len(self.options):
			return

		selected = self.options[index]
		
		if opt_lightsens in selected and self.parent.RLS_connected:
			if self.parent.light_thresh < 6:
				self.parent.light_thresh += 1
			else:
				self.parent.light_thresh = 1
			str_lightsens = self.getLightSensitivityString()
			self.options[index] = str_lightsens
			self.ibus_handler.sendRadioButton(1, index, str_lightsens, True)
		elif opt_autoconnect in selected:
			self.parent.autoplay = not self.parent.autoplay
			autoplay = self.parent.autoplay
			if autoplay:
				self.options[index] = opt_autoconnect + "On"
			else:
				self.options[index] = opt_autoconnect + "Off"
			self.ibus_handler.sendRadioButton(1, index, self.options[index], True)
		elif opt_main_menu in selected:
			if hasattr(self.parent, "mirror") and hasattr(self.parent.mirror, "decoder"):
				self.parent.mirror.decoder.setWindow(False)
				self.parent.openMainMenu()
				self.menu_open = False
				self.ibus_handler.sendAudioScreenClear()

	def getLightSensitivityString(self):
		str_lightsens = opt_lightsens
		str_lightsens += str(self.parent.light_thresh)
		return str_lightsens