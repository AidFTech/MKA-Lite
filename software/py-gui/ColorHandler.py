from AttributeGroup import AttributeGroup

OEMColor1 = AttributeGroup()

OEMColor1.br = (40, 32, 95)
OEMColor1.text_color = (191, 191, 239)
OEMColor1.header_color = (103, 95, 143)
OEMColor1.rect_color = (239, 96, 32)
OEMColor1.border_color = (215, 215, 239)
OEMColor1.border_outline = (0, 0, 0)

OEMColor2 = AttributeGroup()

OEMColor2.br = (8, 19, 91)
OEMColor2.text_color = (0, 171, 200)
OEMColor2.header_color = (8, 29, 222)
OEMColor2.rect_color = (0, 170, 177)
OEMColor2.border_color = (0, 151, 225)
OEMColor2.border_outline = (0, 0, 0)

MColor = AttributeGroup()

MColor.br = (55, 52, 133)
MColor.text_color = (230, 230, 231)
MColor.header_color = (51, 160, 209)
MColor.rect_color = (197, 43, 48)
MColor.border_color = (22, 72, 142)
MColor.border_outline = (0, 0, 0)

NightColor = AttributeGroup()

NightColor.br = (0, 0, 132)
NightColor.text_color = (255, 170, 0)
NightColor.header_color = (0, 0, 255)
NightColor.rect_color = (247, 32, 0)
NightColor.border_color = (82, 93, 255)
NightColor.border_outline = (0, 0, 0)

def setColors(paste: AttributeGroup, copy: AttributeGroup):
	"""Set the color scheme."""
	paste.br = copy.br
	paste.text_color = copy.text_color
	paste.header_color = copy.header_color
	paste.rect_color = copy.rect_color
	paste.border_color = copy.border_color
	paste.border_outline = copy.border_outline