use std::fs::File;
use std::io::Read;

use crate::color;
use crate::default;

use regex::Regex;
use serde::Deserialize;

pub struct Params {
    pub port:                Option<String>,
    pub speed:               Option<String>,
    pub crlf:                bool,
    pub read_buf_size:       usize,
    pub tcp_connect_timeout: u64,
    pub timestamp_format:    String,
    pub debug:               bool,
    pub timestamp:           bool,
    pub auto_save_log:       bool,
    pub log_format:          String,
    pub log_destination:     String,
    pub terminal_type:       String,
    pub syntaxes:            Vec<SyntaxDefinition>,
}

impl Params {
    pub fn new(config_file: &str) -> Option<Self> {
        let mut f = match File::open(config_file) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("\"{}\": {}", config_file, e);
                eprintln!("Press ENTER to continue without color mode");
                let _ = std::io::stdin().read_line(&mut String::new());
                return None;
            },
        };
        let mut setting = String::new();
        f.read_to_string(&mut setting).expect("Somothing went wrong reading the file");
        let setting: Result<Setting, toml::de::Error> = toml::from_str(&setting);
        let setting = match setting {
            Ok(s) => s,
            Err(e) => panic!("Failed reading setting file: {}", e),
        };

        let mut syntaxes: Vec<SyntaxDefinition> = vec![
            SyntaxDefinition::new("\x1b[0m".to_string(), vec![Regex::new("0^").unwrap()], false),
        ];
        for coloring in &setting.colorings {
            let mut re_vec = Vec::new();
            for regex in &coloring.regex {
                re_vec.push(Regex::new(regex).expect("Failed compile regex"));
            }
            let color = color::valid_color_syntax(&coloring).unwrap();
            let ignore_whitespace = coloring.ignore_whitespace();
            syntaxes.push(SyntaxDefinition::new(color, re_vec, ignore_whitespace));
        }

        Some( Self {
            port:                setting.port,
            speed:               setting.speed,
            crlf:                setting.crlf,
            read_buf_size:       setting.read_buf_size.unwrap(),
            tcp_connect_timeout: setting.tcp_connect_timeout.unwrap(),
            timestamp_format:    setting.timestamp_format,
            debug:               setting.debug,
            timestamp:           setting.timestamp,
            auto_save_log:       setting.auto_save_log,
            log_format:          setting.log_format,
            log_destination:     setting.log_destination,
            terminal_type:       setting.terminal_type,
            syntaxes,
        })
    }
}

pub struct SyntaxDefinition {
    color:             String,
    regex:             Vec<Regex>,
    ignore_whitespace: bool,
}

impl SyntaxDefinition {
    fn new(color: String, regex: Vec<Regex>, ignore_whitespace: bool) -> Self {
        Self {
            color,
            regex,
            ignore_whitespace,
        }
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn regex(&self) -> &Vec<Regex> {
        &self.regex
    }

    pub fn ignore_whitespace(&self) -> bool {
        self.ignore_whitespace
    }
}

#[derive(Deserialize)]
struct Setting {
    port:  Option<String>,
    speed: Option<String>,

    #[serde(default)]
    crlf: bool,

    #[serde(default)]
    read_buf_size: ReadBufSize,

    #[serde(default)]
    tcp_connect_timeout: TcpConnectTimeout,

    #[serde(default = "default_timestamp_format")]
    timestamp_format: String,

    #[serde(default)]
    debug: bool,

    #[serde(default)]
    timestamp: bool,

    #[serde(default)]
    auto_save_log: bool,

    #[serde(default = "default_log_format")]
    log_format: String,

    #[serde(default = "default_log_destination")]
    log_destination: String,

    #[serde(default = "default_terminal_type")]
    terminal_type: String,

    //nocolor:    Option<bool>,

    colorings: Vec<Coloring>,
}

#[derive(Deserialize)]
struct ReadBufSize(usize);
impl Default for ReadBufSize {
    fn default() -> Self {
        ReadBufSize(default::READ_BUFFER_SIZE)
    }
}
impl ReadBufSize {
    fn unwrap(&self) -> usize {
        self.0
    }
}

#[derive(Deserialize)]
struct TcpConnectTimeout(u64);
impl Default for TcpConnectTimeout {
    fn default() -> Self {
        TcpConnectTimeout(default::TCP_CONNECT_TIMEOUT)
    }
}
impl TcpConnectTimeout {
    fn unwrap(&self) -> u64 {
        self.0
    }
}

fn default_timestamp_format() -> String {
    default::TIMESTAMP_FORMAT.to_string()
}

fn default_log_format() -> String {
    default::LOG_FORMAT.to_string()
}

fn default_log_destination() -> String {
    default::LOG_DESTINATION.to_string()
}

fn default_terminal_type() -> String {
    default::TERMINAL_TYPE.to_string()
}

#[derive(Deserialize)]
pub struct Coloring {
    color:             String,
    regex:             Vec<String>,
    underlined:        Option<bool>,
    ignore_whitespace: Option<bool>,
}

impl Coloring {
    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn underlined(&self) -> bool {
        self.underlined.unwrap_or(false)
    }

    pub fn ignore_whitespace(&self) -> bool {
        self.ignore_whitespace.unwrap_or(false)
    }
}
