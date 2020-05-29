
#[allow(dead_code)]
mod commands {
    pub const SE   :u8 = 0xF0;  // 240
    pub const NOP  :u8 = 0xF1;  // 241
    pub const DM   :u8 = 0xF2;  // 242
    pub const BRK  :u8 = 0xF3;  // 243
    pub const IP   :u8 = 0xF4;  // 244
    pub const AO   :u8 = 0xF5;  // 245
    pub const AYT  :u8 = 0xF6;  // 246
    pub const EC   :u8 = 0xF7;  // 247
    pub const EL   :u8 = 0xF8;  // 248
    pub const GA   :u8 = 0xF9;  // 249
    pub const SB   :u8 = 0xFA;  // 250

    pub const WILL :u8 = 0xFB;  // 251
    pub const WONT :u8 = 0xFC;  // 252
    pub const DO   :u8 = 0xFD;  // 253
    pub const DONT :u8 = 0xFE;  // 254

    pub const IAC  :u8 = 0xFF;  // 255
}

#[allow(dead_code)]
mod options {
    pub const BINARY_TRANSMISSION   :u8 = 0x00;  //  0
    pub const ECHO                  :u8 = 0x01;  //  1
    //pub const                       :u8 = 0x02;  //  2
    pub const SUPPRESS_GO_AHEAD     :u8 = 0x03;  //  3
    //pub const                       :u8 = 0x04;  //  4
    pub const STATUS                :u8 = 0x05;  //  5
    pub const TIMING_MARK           :u8 = 0x06;  //  6
    //pub const                       :u8 = 0x07;  //  7
    //pub const                       :u8 = 0x08;  //  8
    //pub const                       :u8 = 0x09;  //  9
    //pub const                       :u8 = 0x0A;  // 10
    //pub const                       :u8 = 0x0B;  // 11
    //pub const                       :u8 = 0x0C;  // 12
    //pub const                       :u8 = 0x0D;  // 13
    //pub const                       :u8 = 0x0E;  // 14
    //pub const                       :u8 = 0x0F;  // 15
    //pub const                       :u8 = 0x10;  // 16
    //pub const                       :u8 = 0x11;  // 17
    //pub const                       :u8 = 0x12;  // 18
    //pub const                       :u8 = 0x13;  // 19
    //pub const                       :u8 = 0x14;  // 20
    //pub const                       :u8 = 0x15;  // 21
    //pub const                       :u8 = 0x16;  // 22
    //pub const                       :u8 = 0x17;  // 23
    pub const TERMINAL_TYPE         :u8 = 0x18;  // 24
    //pub const                       :u8 = 0x19;  // 25
    //pub const                       :u8 = 0x1A;  // 26
    //pub const                       :u8 = 0x1B;  // 27
    //pub const                       :u8 = 0x1C;  // 28
    //pub const                       :u8 = 0x1D;  // 29
    //pub const                       :u8 = 0x1E;  // 30
    pub const WINDOW_SIZE           :u8 = 0x1F;  // 31
    pub const TERMINAL_SPEED        :u8 = 0x20;  // 32
    pub const REMOTE_FLOW_CONTROL   :u8 = 0x21;  // 33
    pub const LINE_MODE             :u8 = 0x22;  // 34
    pub const X_DISPLAY_LOCATION    :u8 = 0x23;  // 35
    pub const ENVIRONMENT_VARIABLES :u8 = 0x24;  // 36
    //pub const                       :u8 = 0x25;  // 37
    //pub const                       :u8 = 0x26;  // 38
    pub const NEW_ENVIRONMENT       :u8 = 0x27;  // 39
    //pub const                       :u8 = 0x28;  // 40
    //pub const                       :u8 = 0x29;  // 41
    //pub const                       :u8 = 0x2A;  // 42
    //pub const                       :u8 = 0x2B;  // 43
    //pub const                       :u8 = 0x2C;  // 44
    //pub const                       :u8 = 0x2D;  // 45
    //pub const                       :u8 = 0x2E;  // 46
    //pub const                       :u8 = 0x2F;  // 47
    //pub const                       :u8 = 0x30;  // 48
    //pub const                       :u8 = 0x31;  // 49
    /* ------------   50-137 Unassigned  ------------ */
    //pub const                       :u8 = 0x8A;  // 138
    //pub const                       :u8 = 0x8B;  // 139
    //pub const                       :u8 = 0x8C;  // 140
    /* ------------  141-254 Unassigned  ------------ */
    //pub const                       :u8 = 0xFF;  // 255
}


pub fn parse_commands(t: usize, serial_buf: &[u8], send_neg: &mut Vec<u8>) -> usize {
    let mut i = 0;

    while i < t && serial_buf[i] == commands::IAC {
        i += 1;

        match serial_buf[i] {
            commands::WILL => {
                i += 1;
                match serial_buf[i] {
                    options::ECHO =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::DO,
                            options::ECHO,
                        ]),
                    options::SUPPRESS_GO_AHEAD =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::DO,
                            options::SUPPRESS_GO_AHEAD,
                        ]),
                    _ => (),
                }
            },
            commands::WONT => {},
            commands::DO   => {
                i += 1;
                match serial_buf[i] {
                    options::SUPPRESS_GO_AHEAD =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::WILL,
                            options::SUPPRESS_GO_AHEAD,
                        ]),
                    //options::WINDOW_SIZE =>
                    //    send_neg.extend_from_slice(&[
                    //        commands::IAC,
                    //        //commands::SB,
                    //        commands::WONT,
                    //        options::WINDOW_SIZE,
                    //        //0x00,
                    //        //0x50, // 80
                    //        //0x00,
                    //        //0x18, // 24
                    //    ]),
                    //options::TERMINAL_TYPE =>
                    //    send_neg.extend_from_slice(&[
                    //        commands::IAC,
                    //        commands::WONT,
                    //        options::TERMINAL_TYPE,
                    //    ]),
                    //_ => (),
                    _ => send_neg.extend_from_slice(&[
                        commands::IAC,
                        commands::WONT,
                        serial_buf[i],
                    ]),
                }
            },
            commands::DONT => {},
            commands::SB   => {},
            commands::SE   => {},
            _ => (),
        }

        i += 1;
    }

    i
}

