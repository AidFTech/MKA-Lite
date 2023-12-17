opt_lightsens = "Night Sens.: "
opt_autoconnect = "Auto Connect: "
opt_source = "SRC: MKA"
opt_main_menu = "MKA Main Menu"

class MKIVMMenu:
    options = [opt_autoconnect, opt_source, opt_main_menu]

    def __init__(self, parent, ibus_handler):
        self.parent = parent
        self.ibus_handler = ibus_handler