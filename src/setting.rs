use std::fs::File;
use std::io::prelude::*;

use crate::color;

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

        let mut syntaxes: Vec<SyntaxDefinition> = Vec::new();
        for coloring in &setting.colorings {
            let re = Regex::new(&coloring.regex).expect("Failed compile regex");
            let color = color::valid_color_syntax(&coloring).unwrap();
            syntaxes.push(SyntaxDefinition::new(color, re));
        }

        Some( Self {
            port:  setting.port,
            speed: setting.speed,
            syntaxes,
        })
    }
}

pub struct SyntaxDefinition {
    color: String,
    regex: Regex,
}

impl SyntaxDefinition {
    fn new(color: String, regex: Regex) -> Self {
        Self {
            color,
            regex,
        }
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn regex(&self) -> &Regex {
        &self.regex
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
pub struct Coloring {
    color:      String,
    regex:      String,
    underlined: Option<bool>,
}

impl Coloring {
    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn underlined(&self) -> bool {
        if let Some(true) = self.underlined {
            true
        } else {
            false
        }
    }
}
