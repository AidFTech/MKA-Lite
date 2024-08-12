use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::context::Context;
use crate::ibus::*;
use crate::mirror::handler::*;

pub struct MKAObj<'a> {
    context: &'a Arc<Mutex<Context>>,
    ibus_handler: &'a Arc<Mutex<IBusHandler>>,
    mirror_handler: &'a Arc<Mutex<MirrorHandler<'a>>>,
}

impl <'a> MKAObj<'a> {
    pub fn new(context: &'a Arc<Mutex<Context>>, ibus_handler: &'a Arc<Mutex<IBusHandler>>, mirror_handler: &'a Arc<Mutex<MirrorHandler<'a>>>) -> MKAObj<'a> {
        return MKAObj {
            context, ibus_handler, mirror_handler,
        }
    }

    pub fn check_ibus(&mut self) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("IBus handler locked.");
                return;
            }
        };

        if ibus_handler.bytes_available() >= 4 {
            let ibus_msg = match ibus_handler.read_ibus_message() {
                Some(ibus_msg) => ibus_msg,
                None => {
                    return;
                }
            };

            std::mem::drop(ibus_handler);
            println!("{:X?}", ibus_msg.get_bytes());
            self.handle_ibus_message(ibus_msg);
        }
    }

    fn handle_ibus_message(&mut self, ibus_msg: IBusMessage) {
        let context = match self.context.try_lock() {
            Ok(context) => context,
            Err(_) => {
                println!("IBus: Context locked.");
                return;
            }
        };

        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("IBus handler locked.");
                return;
            }
        };

        if ibus_msg.l() >= 1 && ibus_msg.receiver == IBUS_DEVICE_CDC && ibus_msg.data[0] == 0x1 {
            ibus_handler.write_ibus_message(IBusMessage {
                sender: IBUS_DEVICE_CDC,
                receiver: ibus_msg.sender,
                data: [0x2, 0x0].to_vec(),
            });
        } else if ibus_msg.l() >= 2 && ibus_msg.receiver == IBUS_DEVICE_CDC && ibus_msg.data[0] == 0x38 { //CDC request. Must reply.
            let selected = context.audio_selected;
            let sender = ibus_msg.sender;

            if ibus_msg.data[1] == IBUS_CDC_CMD_GET_STATUS {
                if selected {
                    let cd_msg = get_cd_status_message(IBUS_CDC_STAT_PLAYING, sender);
                    ibus_handler.write_ibus_message(cd_msg);
                } else {
                    let cd_msg = get_cd_status_message(IBUS_CDC_STAT_STOP, sender);
                    ibus_handler.write_ibus_message(cd_msg);
                }
            } else if ibus_msg.data[1] == IBUS_CDC_CMD_STOP_PLAYING { //Stop the MKA.
                let cd_msg = get_cd_status_message(IBUS_CDC_STAT_STOP, sender);
                ibus_handler.write_ibus_message(cd_msg);
                
                std::mem::drop(context);
                self.set_selected(false);
            } else if ibus_msg.data[1] == IBUS_CDC_CMD_START_PLAYING || ibus_msg.data[1] == IBUS_CDC_CMD_PAUSE_PLAYING { //Start the MKA.
                let cd_msg = get_cd_status_message(IBUS_CDC_STAT_PLAYING, sender);
                ibus_handler.write_ibus_message(cd_msg);

                std::mem::drop(context);
                self.set_selected(true);
            } else if ibus_msg.data[1] == IBUS_CDC_CMD_CHANGE_TRACK && ibus_msg.l() >= 3 {
                if selected {
                    let mut mirror_handler = match self.mirror_handler.try_lock() {
                        Ok(mirror_handler) => mirror_handler,
                        Err(_) => {
                            println!("Set Selected: Mirror handler locked.");
                            return;
                        }
                    };

                    if ibus_msg.data[2] == 0 {
                        mirror_handler.send_carplay_command(204);
                    } else if ibus_msg.data[2] == 1 {
                        mirror_handler.send_carplay_command(205);
                    }
                    let cd_msg = get_cd_status_message(IBUS_CDC_STAT_PLAYING, sender);
                    ibus_handler.write_ibus_message(cd_msg);
                } else {
                    let cd_msg = get_cd_status_message(IBUS_CDC_STAT_STOP, sender);
                    ibus_handler.write_ibus_message(cd_msg);
                }
            } else { //N/A message.
                if selected {
                    let cd_msg = get_cd_status_message(IBUS_CDC_STAT_END, sender);
                    ibus_handler.write_ibus_message(cd_msg);
                } else {
                    let cd_msg = get_cd_status_message(IBUS_CDC_STAT_STOP, sender);
                    ibus_handler.write_ibus_message(cd_msg);
                }
            }
        } else if ibus_msg.l() >= 2 && ibus_msg.data[0] == IBUS_CMD_RAD_SCREEN_MODE_UPDATE { //Audio screen changed.
            if context.phone_active && ((ibus_msg.data[1]&0x1) != 0 || (ibus_msg.data[1]&0x2) != 0) {
                let screen_msg = IBusMessage {
                    sender: IBUS_DEVICE_GT,
                    receiver: IBUS_DEVICE_RAD,
                    data: [IBUS_CMD_GT_SCREEN_MODE_SET, 0].to_vec(),
                };
                ibus_handler.write_ibus_message(screen_msg);
            }
        } else if ibus_msg.l() >= 1 && ibus_msg.data[0] == IBUS_CMD_GT_WRITE_TITLE { //Screen text. //TODO: Set "Selected" to false if this says an FM frequency, tape info, anything that is not a CD changer header.
            if context.audio_selected && ibus_msg.data[ibus_msg.l() - 1] != 0x8E {
                let start = Instant::now();

                let mut sent_22 = false;
                while !sent_22 && Instant::now() - start < Duration::from_millis(750) {
                    match ibus_handler.read_ibus_message() {
                        Some(ibus_msg) => {
                            if ibus_msg.sender == IBUS_DEVICE_GT && ibus_msg.l() >= 1 && ibus_msg.data[0] == IBUS_CMD_GT_WRITE_RESPONSE {
                                sent_22 = true;
                                break;
                            }
                        }
                        None => {
                            continue;
                        }
                    }
                }

                if sent_22 {
                    std::mem::drop(context);
                    std::mem::drop(ibus_handler);
                    self.send_radio_screen_update();
                }
            } else if !context.audio_selected {
                //TODO: Header overlay.
            }
        } else if ibus_msg.l() >= 2 && ibus_msg.sender == IBUS_DEVICE_BMBT && ibus_msg.data[0] == 0x48 && context.phone_active {
			if (ibus_msg.data[1]&0x3F) == 0x30 && context.phone_active { //Radio button. To make sure the screen stays active.
				let screen_msg = IBusMessage {
                    sender: IBUS_DEVICE_GT,
                    receiver: IBUS_DEVICE_RAD,
                    data: [IBUS_CMD_GT_SCREEN_MODE_SET, 0].to_vec(),
                };
                ibus_handler.write_ibus_message(screen_msg);
			}
        } else {
            std::mem::drop(context);
            std::mem::drop(ibus_handler);
            
            match self.mirror_handler.try_lock() {
                Ok(mut mirror_handler) => {
                    mirror_handler.handle_ibus_message(ibus_msg);
                }
                Err(_) => {
                    println!("Set Selected: Mirror handler locked.");
                    return;
                }
            };
        }
    }

    //Send a CD ping.
    pub fn send_cd_ping(&mut self) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("IBus handler locked.");
                return;
            }
        };

        ibus_handler.write_ibus_message(IBusMessage {
            sender: IBUS_DEVICE_CDC,
            receiver: IBUS_DEVICE_GLO,
            data: [0x2, 0x1].to_vec(),
        });
    }

    //Send all radio screen update messages.
    fn send_radio_screen_update(&mut self) {
        let context = match self.context.try_lock() {
            Ok(context) => context,
            Err(_) => {
                println!("Screen Update: Context locked.");
                return;
            }
        };

        let version = context.version;

        self.send_all_radio_center_text(version, true, context.song_title.clone(), context.artist.clone(), context.album.clone(), context.app.clone());
        let phone_type = context.phone_type;
        if phone_type == 3 {
            self.send_radio_main_text("CarPlay".to_string());
        } else if phone_type == 5 {
            self.send_radio_main_text("Android".to_string());
        } else {
            self.send_radio_main_text("MKA".to_string());
        }

        self.send_radio_subtitle_text(" ".to_string(), 1, false);
        
        if context.playing {
            self.send_radio_subtitle_text(">".to_string(), 2, false);
        } else {
            self.send_radio_subtitle_text("||".to_string(), 2, false);
        }

        let phone_name = context.phone_name.clone();
        self.send_radio_subtitle_text(phone_name, 6, true);
    }

    //Send a radio header change message.
    fn send_radio_main_text(&mut self, text: String) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("IBus handler locked.");
                return;
            }
        };

        let mut text_data = Vec::new();
        text_data.push(IBUS_CMD_GT_WRITE_TITLE);
        text_data.push(0x62);
        text_data.push(0x30);

        let text_bytes = text.as_bytes();

        if text.len() >= 1 {
            for i in 0..text.len() {
                text_data.push(text_bytes[i]);
            }
        } else {
            text_data.push(0x20);
        }

        text_data.push(0x8E);

        let text_msg = IBusMessage {
            sender: IBUS_DEVICE_RAD,
            receiver: IBUS_DEVICE_GT,
            data: text_data,
        };
        ibus_handler.write_ibus_message(text_msg);
    }

    //Send a radio subtitle change message. 
    fn send_radio_subtitle_text(&mut self, text: String, zone: u8, refresh: bool) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("IBus handler locked.");
                return;
            }
        };

        let mut text_data = Vec::new();
        text_data.push(IBUS_CMD_GT_WRITE_WITH_CURSOR);
        text_data.push(0x62);
        text_data.push(0x1);
        text_data.push(0x40 | (zone&0xF));

        let text_bytes = text.as_bytes();
        if text_bytes.len() >= 1 {
            for i in 0..text_bytes.len() {
                text_data.push(text_bytes[i]);
            }
        } else {
            text_data.push(0x20);
        }

        let text_msg = IBusMessage {
            sender: IBUS_DEVICE_RAD,
            receiver: IBUS_DEVICE_GT,
            data: text_data,
        };

        ibus_handler.write_ibus_message(text_msg);
        std::mem::drop(ibus_handler);

        if refresh {
            self.send_refresh(0x62);
        }
    }

    //Send a radio text change message.
    fn send_radio_center_text(&mut self, text: String, position: u8, version: i8) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("IBus handler locked.");
                return;
            }
        };
        
        let index: u8;
        if position == SONG_NAME {
            index = 0x41;
        } else if position == ARTIST {
            index = 0x42;
        } else if position == ALBUM {
            index = 0x43;
        } else if position == APP {
            index = 0x44;
        } else {
            return;
        }

        let mut text_data = Vec::new();
        text_data.push(IBUS_CMD_GT_WRITE_WITH_CURSOR);
        
        if version >= 5 {
            text_data.push(0x63);
        } else {
            text_data.push(0x60);
        }

        text_data.push(0x1);
        text_data.push(index);

        let text_bytes = text.as_bytes();
        for i in 0..text_bytes.len() {
            text_data.push(text_bytes[i]);
        }

        let text_change_message = IBusMessage {
            sender: IBUS_DEVICE_RAD,
            receiver: IBUS_DEVICE_GT,
            data: text_data,
        };

        ibus_handler.write_ibus_message(text_change_message);
    }

    //Send multiple radio text change messages.
    fn send_all_radio_center_text(&mut self, version: i8, refresh: bool, song_title: String, artist: String, album: String, app: String) {
        self.send_radio_center_text(song_title, SONG_NAME, version);
        self.send_radio_center_text(artist, ARTIST, version);
        self.send_radio_center_text(album, ALBUM, version);
        self.send_radio_center_text(app, APP, version);

        if refresh {
            let mut index = 0x63;
            if version < 5 {
                index = 0x60;
            }
            self.send_refresh(index)
        }
    }

    //Send a refresh message.
    fn send_refresh(&mut self, index: u8) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("IBus handler locked.");
                return;
            }
        };
        
        let refresh_data = [IBUS_CMD_GT_WRITE_WITH_CURSOR, index, 0x1, 0x0].to_vec();
        let refresh_msg = IBusMessage {
            sender: IBUS_DEVICE_RAD,
            receiver: IBUS_DEVICE_GT,
            data: refresh_data,
        };

        ibus_handler.write_ibus_message(refresh_msg);
    }

    //Set whether the MKA is the selected source.
    fn set_selected(&mut self, selected: bool) {
        let mut context = match self.context.try_lock() {
            Ok(context) => context,
            Err(_) => {
                println!("Set Selected: Context locked.");
                return;
            }
        };

        context.audio_selected = selected;
        std::mem::drop(context);

        let mut mirror_handler = match self.mirror_handler.try_lock() {
            Ok(mirror_handler) => mirror_handler,
            Err(_) => {
                println!("Set Selected: Mirror handler locked.");
                return;
            }
        };

        if selected {
            mirror_handler.send_carplay_command(201);
            //self.send_radio_screen_update();
        } else {
            mirror_handler.send_carplay_command(202);
        }
    }

    //Create the main settings menu.
    fn create_main_menu(&mut self) {
        let context = match self.context.try_lock() {
            Ok(context) => context,
            Err(_) => {
                println!("Set Selected: Context locked.");
                return;
            }
        };

        self.create_menu_option(9, "MKA Settings".to_string());

        //TODO: An option for auto light sensitivity if the RLS is present.
        self.create_menu_option(0, "Auto Connect".to_string());
        self.create_menu_option(1, "Auto Start Music".to_string());
        self.create_menu_option(2, "Audio Source".to_string());
        self.create_menu_option(3, "Audio HUD".to_string());

        if context.phone_type == 3 {
            self.create_menu_option(4, "Start CarPlay".to_string());
        } else if context.phone_type == 5 {
            self.create_menu_option(4, "Start Android".to_string());
        }

        self.send_refresh(0x61);
    }

    //Create a menu option.
    fn create_menu_option(&mut self, index: u8, text: String) {
        let mut ibus_handler = match self.ibus_handler.try_lock() {
            Ok(ibus_handler) => ibus_handler,
            Err(_) => {
                println!("IBus handler locked.");
                return;
            }
        };

        let mut menu_option_data = Vec::new();
        menu_option_data.push(IBUS_CMD_GT_WRITE_NO_CURSOR);
        menu_option_data.push(0x61);
        menu_option_data.push(0x1);
        menu_option_data.push(index&0x1F);

        let text_bytes = text.as_bytes();
        for b in text_bytes {
            menu_option_data.push(*b);
        }

        ibus_handler.write_ibus_message(IBusMessage {
            sender: IBUS_DEVICE_RAD,
            receiver: IBUS_DEVICE_GT,
            data: menu_option_data,
        });
    }
}

fn get_cd_status_message(status: u8, receiver: u8) -> IBusMessage {
    let mut pseudo_status = 0x89;
    if status == 0x0 {
        pseudo_status = 0x82;
    }

    let data = [0x39, status, pseudo_status, 0x00, 0x3f, 0x00, 0x1, 0x1, 0x0, 0x1, 0x1, 0x1];

    let status_msg = IBusMessage {
        sender: IBUS_DEVICE_CDC,
        receiver: receiver,
        data: data.to_vec(),
    };

    return status_msg;
}
