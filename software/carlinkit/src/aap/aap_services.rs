pub enum ServiceChannels {
	ControlChannel = 0,
	TouchChannel,
	SensorChannel,
	VideoChannel,
	MediaAudioChannel,
	Audio1Channel,
	Audio2Channel,
	MicrophoneChannel,
	BluetoothChannel,
	PhoneStatusChannel,
	NotificationChannel,
	NavigationChannel,
	MaximumChannel=256,
}

pub enum SensorType {
	SensorTypeLocation = 1,
	SensorTypeCompass,
	SensorTypeSpeed,
	SensorTypeRpm,
	SensorTypeOdometer,
	SensorTypeFuel,
	SensorTypeParkingBrake,
	SensorTypeGear,
	SensorTypeDiagnostics,
	SensorTypeNightData,
	SensorTypeEnvironment,
	SensorTypeHvac,
	SensorTypeDrivingStatus,
	SensorTypeDeadReconing,
	SensorTypePassenger,
	SensorTypeDoor,
	SensorTypeTire,
	SensorTypeAccel,
	SensorTypeGyro,
	SensorTypeGps,
}

pub enum ControlMessage {
	ControlMessageVersionRequest = 1,
	ControlMessageVersionResponse,
	ControlMessageSSLHandshake,
	ControlMessageAuthComplete,
	ControlMessageServiceDiscoveryRequest,
	ControlMessageServiceDiscoveryResponse,
}

pub enum ProtocolMessage {
	ProtocolMessageMediaDataTime = 0,
	ProtocolMessageMediaData,
	ProtocolMessageChannelOpenRequest = 7,
	ProtocolMessageChannelOpenResponse,
	ProtocolMessagePingRequest = 0xB,
	ProtocolMessagePingResponse,
	ProtocolMessageNavigationFocusRequest,
	ProtocolMessageNavigationFocusResponse,
	ProtocolMessageShutdownRequest,
	ProtocolMessageShutdownResponse,
	ProtocolMessageVoiceSessionRequest,
	ProtocolMessageAudioFocusRequest,
	ProtocolMessageAudioFocusResponse,
}

pub enum InputChannelMessage {
	InputChannelMessageInputEvent = 0x8001,
	InputChannelMessageBindingRequest,
	InputChannelMessageBindingResponse,
}

pub enum SensorChannelMessage {
	SensorChannelMessageStartRequest = 0x8001,
	SensorChannelMessageStartResponse,
	SensorChannelMessageEvent,
}

pub enum MediaChannelMessage {
	MediaChannelMessageSetupRequest = 0x8000,
	MediaChannelMessageStartRequest,
	MediaChannelMessageStopRequest,
	MediaChannelMessageSetupResponse,
	MediaChannelMessageAck,
	MediaChannelMessageMicRequest,
	MediaChannelMessageMicResponse,
	MediaChannelMessageVideoFocusRequest,
	MediaChannelMessageVideoFocus,
}

pub enum MediaInfoMessage {
	MediaInfoMessagePlayback = 0x8001,
	MediaInfoMessageMeta = 0x8003,
}

pub enum InputButton {
	ButtonMic1 = 1,
	ButtonMenu,
	ButtonHome,
	ButtonBack,
	ButtonPhone,
	ButtonCallend,
	ButtonUp = 0x13,
	ButtonDown,
	ButtonLeft,
	ButtonRight,
	ButtonEnter,
	ButtonMic = 0x54,
	ButtonPlayPause,
	ButtonNext = 0x57,
	ButtonPrev,
	ButtonStart = 0x7E,
	ButtonStop,
	ButtonMusic = 0xD1,
	ButtonScroll = 0x10000,
	ButtonMedia,
	ButtonNavigation,
	ButtonRadio,
	ButtonTel,
	Button1,
	Button2,
	Button3,
}