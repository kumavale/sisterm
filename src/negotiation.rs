
#[allow(dead_code)]
mod commands {
    pub const SE   :u8 = 0xF0;
    pub const NOP  :u8 = 0xF1;
    pub const DM   :u8 = 0xF2;
    pub const BRK  :u8 = 0xF3;
    pub const IP   :u8 = 0xF4;
    pub const AO   :u8 = 0xF5;
    pub const AYT  :u8 = 0xF6;
    pub const EC   :u8 = 0xF7;
    pub const EL   :u8 = 0xF8;
    pub const GA   :u8 = 0xF9;
    pub const SB   :u8 = 0xFA;

    pub const WILL :u8 = 0xFB;
    pub const WONT :u8 = 0xFC;
    pub const DO   :u8 = 0xFD;
    pub const DONT :u8 = 0xFE;
    pub const IAC  :u8 = 0xFF;
}

#[allow(dead_code)]
mod options {
    pub const SUPPRESS_GO_AHEAD      :u8 = 0x03;
    pub const STATUS                 :u8 = 0x05;
    pub const ECHO                   :u8 = 0x01;
    pub const TIMING_MARK            :u8 = 0x06;
    pub const TERMINAL_TYPE          :u8 = 0x18;
    pub const WINDOW_SIZE            :u8 = 0x1F;
    pub const TERMINAL_SPEED         :u8 = 0x20;
    pub const REMOTE_FLOW_CONTROL    :u8 = 0x21;
    pub const LINE_MODE              :u8 = 0x22;
    pub const ENVIRONMENT_VARIABLES  :u8 = 0x24;
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
                    options::WINDOW_SIZE =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::SB,
                            options::WINDOW_SIZE,
                            0x00,
                            0x50, // 80
                            0x00,
                            0x18, // 24
                        ]),
                    options::TERMINAL_TYPE =>
                        send_neg.extend_from_slice(&[
                            commands::IAC,
                            commands::WONT,
                            options::TERMINAL_TYPE,
                        ]),
                    _ => (),
                }
            },
            commands::DONT => {},
            commands::SB   => {},
            commands::SE   => {},
            _ => (),
        }
    }

    i
}

