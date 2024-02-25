#include <stdint.h>
#include <time.h>
#include <string.h>

#include "IBus_Serial.h"

#if __has_include(<pigpio.h>) //Including an "if has include" so we can test this on a desktop if need be.
#include <pigpio.h>
#define RPI_UART
#else
#include <stdio.h>
#endif

#ifndef ibus_handler_h
#define ibus_handler_h

//From BlueBus:
// Devices
#define IBUS_DEVICE_GM 0x00 /* Body module */
#define IBUS_DEVICE_CDC 0x18 /* CD Changer */
#define IBUS_DEVICE_FUH 0x28 /* Radio controlled clock */
#define IBUS_DEVICE_CCM 0x30 /* Check control module */
#define IBUS_DEVICE_GT 0x3B /* Graphics driver (in navigation system) */
#define IBUS_DEVICE_DIA 0x3F /* Diagnostic */
#define IBUS_DEVICE_GTF 0x43 /* Graphics driver for rear screen (in navigation system) */
#define IBUS_DEVICE_EWS 0x44 /* EWS (Immobileiser) */
#define IBUS_DEVICE_CID 0x46 /* Central information display (flip-up LCD screen) */
#define IBUS_DEVICE_MFL 0x50 /* Multi function steering wheel */
#define IBUS_DEVICE_IHK 0x5B /* HVAC */
#define IBUS_DEVICE_PDC 0x60 /* Park Distance Control */
#define IBUS_DEVICE_RAD 0x68 /* Radio */
#define IBUS_DEVICE_DSP 0x6A /* DSP */
#define IBUS_DEVICE_SM0 0x72 /* Seat memory - 0 */
#define IBUS_DEVICE_SDRS 0x73 /* Sirius Radio */
#define IBUS_DEVICE_CDCD 0x76 /* CD changer, DIN size. */
#define IBUS_DEVICE_NAVE 0x7F /* Navigation (Europe) */
#define IBUS_DEVICE_IKE 0x80 /* Instrument cluster electronics */
#define IBUS_DEVICE_GLO 0xBF /* Global, broadcast address */
#define IBUS_DEVICE_MID 0xC0 /* Multi-info display */
#define IBUS_DEVICE_TEL 0xC8 /* Telephone */
#define IBUS_DEVICE_TCU 0xCA /* BMW Assist */
#define IBUS_DEVICE_LCM 0xD0 /* Light control module */
#define IBUS_DEVICE_IRIS 0xE0 /* Integrated radio information system */
#define IBUS_DEVICE_ANZV 0xE7 /* Front display */
#define IBUS_DEVICE_RLS 0xE8 /* Rain Light Sensor */
#define IBUS_DEVICE_VM 0xED /* Video Module */
#define IBUS_DEVICE_BMBT 0xF0 /* On-board monitor */
#define IBUS_DEVICE_LOC 0xFF /* Local */

#define IBUS_CMD_BMBT_BUTTON0 0x47
#define IBUS_CMD_BMBT_BUTTON1 0x48
#define IBUS_CMD_BMBT_KNOB 0x49

#define IBUS_CMD_DIA_JOB_REQUEST 0x0C
#define IBUS_CMD_DIA_DIAG_RESPONSE 0xA0

#define IBUS_CMD_EWS_IMMOBILISER_STATUS 0x74

#define IBUS_CMD_GM_DOORS_FLAPS_STATUS_RESP 0x7A
#define IBUS_CMD_ZKE3_GM4_JOB_CENTRAL_LOCK 0x0B
#define IBUS_CMD_ZKE3_GM4_JOB_LOCK_HIGH 0x40
#define IBUS_CMD_ZKE3_GM4_JOB_LOCK_LOW 0x41
#define IBUS_CMD_ZKE3_GM4_JOB_LOCK_ALL 0x88
#define IBUS_CMD_ZKE3_GM6_JOB_LOCK_ALL 0x90
#define IBUS_CMD_ZKE3_GM4_JOB_UNLOCK_HIGH 0x42
#define IBUS_CMD_ZKE3_GM4_JOB_UNLOCK_LOW 0x43
#define IBUS_CMD_ZKE5_JOB_CENTRAL_LOCK 0x03
#define IBUS_CMD_ZKE5_JOB_LOCK_ALL 0x34
#define IBUS_CMD_ZKE5_JOB_UNLOCK_LOW 0x37
#define IBUS_CMD_ZKE5_JOB_UNLOCK_ALL 0x45

#define IBUS_CMD_VOLUME_SET 0x32

#define IBUS_CMD_GT_WRITE_NO_CURSOR 0x21

#define IBUS_CMD_GT_CHANGE_UI_REQ 0x20
#define IBUS_CMD_GT_CHANGE_UI_RESP 0x21
#define IBUS_CMD_GT_WRITE_RESPONSE 0x22
#define IBUS_CMD_GT_WRITE_TITLE 0x23
#define IBUS_CMD_GT_MENU_SELECT 0x31
#define IBUS_CMD_GT_DISPLAY_RADIO_MENU 0x37
#define IBUS_CMD_GT_SCREEN_MODE_SET 0x45
#define IBUS_CMD_GT_RAD_TV_STATUS 0x4E
#define IBUS_CMD_GT_MONITOR_CONTROL 0x4F
#define IBUS_CMD_GT_WRITE_INDEX 0x60
#define IBUS_CMD_GT_WRITE_INDEX_TMC 0x61
#define IBUS_CMD_GT_WRITE_ZONE 0x62
#define IBUS_CMD_GT_WRITE_STATIC 0x63
#define IBUS_CMD_GT_TELEMATICS_COORDINATES 0xA2
#define IBUS_CMD_GT_TELEMATICS_LOCATION 0xA4
#define IBUS_CMD_GT_WRITE_WITH_CURSOR 0xA5

#define IBUS_CMD_IKE_IGN_STATUS_REQ 0x10
#define IBUS_CMD_IKE_IGN_STATUS_RESP 0x11
#define IBUS_CMD_IKE_SENSOR_REQ 0x12
#define IBUS_CMD_IKE_SENSOR_RESP 0x13
#define IBUS_CMD_IKE_REQ_VEHICLE_TYPE 0x14
#define IBUS_CMD_IKE_RESP_VEHICLE_CONFIG 0x15
#define IBUS_CMD_IKE_SPEED_RPM_UPDATE 0x18
#define IBUS_CMD_IKE_TEMP_UPDATE 0x19
#define IBUS_CMD_IKE_OBC_TEXT 0x24
#define IBUS_CMD_IKE_SET_REQUEST 0x40
#define IBUS_CMD_IKE_SET_REQUEST_TIME 0x01
#define IBUS_CMD_IKE_SET_REQUEST_DATE 0x02

#define IBUS_CMD_LCM_REQ_REDUNDANT_DATA 0x53
#define IBUS_CMD_LCM_RESP_REDUNDANT_DATA 0x54
#define IBUS_CMD_LCM_BULB_IND_REQ 0x5A
#define IBUS_CMD_LCM_BULB_IND_RESP 0x5B

#define IBUS_CMD_RLS_LIGHT_CONTROL 0x59

#define IBUS_CMD_MOD_STATUS_REQ 0x01
#define IBUS_CMD_MOD_STATUS_RESP 0x02

#define IBUS_CMD_PDC_STATUS 0x07

#define IBUS_CMD_RAD_LED_TAPE_CTRL 0x4A

#define IBUS_CMD_RAD_SCREEN_MODE_UPDATE 0x46
#define IBUS_CMD_RAD_UPDATE_MAIN_AREA 0x23
#define IBUS_CMD_RAD_C43_SCREEN_UPDATE 0x21
#define IBUS_CMD_RAD_C43_SET_MENU_MODE 0xC0
#define IBUS_CMD_RAD_WRITE_MID_DISPLAY 0x23
#define IBUS_CMD_RAD_WRITE_MID_MENU 0x21

#define IBUS_CMD_VOL_CTRL 0x32

#define IBUS_COMMAND_CDC_REQUEST 0x38
#define IBUS_COMMAND_CDC_RESPONSE 0x39

// CDC Commands
#define IBUS_CDC_CMD_GET_STATUS 0x00
#define IBUS_CDC_CMD_STOP_PLAYING 0x01
#define IBUS_CDC_CMD_PAUSE_PLAYING 0x02
#define IBUS_CDC_CMD_START_PLAYING 0x03
#define IBUS_CDC_CMD_CHANGE_TRACK 0x0A
#define IBUS_CDC_CMD_SEEK 0x04
#define IBUS_CDC_CMD_CHANGE_TRACK_BLAUPUNKT 0x05
#define IBUS_CDC_CMD_CD_CHANGE 0x06
#define IBUS_CDC_CMD_SCAN 0x07
#define IBUS_CDC_CMD_RANDOM_MODE 0x08
// CDC Status
#define IBUS_CDC_STAT_STOP 0x00
#define IBUS_CDC_STAT_PAUSE 0x01
#define IBUS_CDC_STAT_PLAYING 0x02
#define IBUS_CDC_STAT_FAST_FWD 0x03
#define IBUS_CDC_STAT_FAST_REV 0x04
#define IBUS_CDC_STAT_END 0x07
#define IBUS_CDC_STAT_LOADING 0x08
// CDC Function
#define IBUS_CDC_FUNC_NOT_PLAYING 0x02
#define IBUS_CDC_FUNC_PLAYING 0x09
#define IBUS_CDC_FUNC_PAUSE 0x0C
#define IBUS_CDC_FUNC_SCAN_MODE 0x19
#define IBUS_CDC_FUNC_RANDOM_MODE 0x29

#define IBUS_BAUD 9600
#define MAX_DELAY 500 //500ms. I believe this is longer than an IBus message will ever take. Adjust as needed.

#define IB_RX 4 //The GPIO input to use to determine whether the IBus RX is active.
#define IB_WAIT 20 //The amount of time to wait for the IBus RX to be free before sending any data.

int ibusSerialInit(char* port);
void ibusSerialClose(const int port);

int readIBusData(const int port, uint8_t* sender, uint8_t* receiver, uint8_t* data, int* new_port);
void writeIBusData(const int port, const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l);

uint8_t getChecksum(const uint8_t sender, const uint8_t receiver, uint8_t* data, const unsigned int l);

#ifndef RPI_UART
uint16_t charToNumber(char c);
#endif
#endif
