use std::fs::File;
use std::io::prelude::*;

use regex::Regex;
use serde::Deserialize;

pub struct Params {
    pub port:     Option<String>,
    pub speed:    Option<String>,
    pub syntaxes: Vec<SyntaxDefinition>,
}

impl Params {
    pub fn new(config_file: &str) -> Option<Self> {
        let mut f = match File::open(config_file) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                eprintln!("Press ENTER to continue of without color mode");
                let _ = std::io::stdin().read_line(&mut String::new());
                return None;
            },
        };
        let mut setting = String::new();
        f.read_to_string(&mut setting).expect("Somothing went wrong reading the file");
        let setting: Result<Setting, toml::de::Error> = toml::from_str(&setting);
        let setting = match setting {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };

        let mut syntaxes: Vec<SyntaxDefinition> = Vec::new();
        for coloring in &setting.colorings {
            let re = Regex::new(&coloring.regex).expect("Failed compile regex");
            syntaxes.push(SyntaxDefinition::new(&coloring.color, re));
        }

        Some( Self {
            port:  setting.port,
            speed: setting.speed,
            syntaxes: Vec::new(),
        })
    }
}

pub struct SyntaxDefinition {
    color: String,
    regex: Regex,
}

impl SyntaxDefinition {
    fn new(color: &str, regex: Regex) -> Self {
        Self {
            color: color.to_string(),
            regex,
        }
    }
}

#[derive(Deserialize)]
struct Setting {
    port:      Option<String>,
    speed:     Option<String>,
    //timestamp: Option<bool>,
    //nocolor:   Option<bool>,

    colorings: Vec<Coloring>,
}

#[derive(Deserialize)]
struct Coloring {
    color: String,
    regex: String,
}
