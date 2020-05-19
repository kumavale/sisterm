use std::collections::HashMap;

use crate::setting;

use lazy_static::lazy_static;


//  Predefined colors
lazy_static! {
    static ref PREDEFINED_COLORS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("RESET",   "\x1b[0m");
        m.insert("BLACK",   "\x1b[30m");
        m.insert("RED",     "\x1b[31m");
        m.insert("GREEN",   "\x1b[32m");
        m.insert("YELLOW",  "\x1b[33m");
        m.insert("BLUE",    "\x1b[34m");
        m.insert("MAGENTA", "\x1b[35m");
        m.insert("CYAN",    "\x1b[36m");
        m.insert("WHITE",   "\x1b[37m");
        m
    };
}


pub fn coloring_from_file(text: String, params: Option<setting::Params>) {
    if let Some(params) = params {
        let tokens = split_whitespace(text);
        for token in &tokens {
            let mut matched = false;
            let mut index: usize = 0;

            for (i, syntax) in params.syntaxes.iter().enumerate() {
                if syntax.regex().captures(token).is_some() {
                    matched = true;
                    index = i;
                    break;
                }
            }

            if matched {
                let color = params.syntaxes[index].color();  // assert Some()
                print!("{}{}{}", color, token, PREDEFINED_COLORS["RESET"]);
            } else {
                print!("{}", token);
            }
        }
    } else {
        println!("{}", text);
    }
}

// Split by whitespace while leaving whitespace
fn split_whitespace(s: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars = s.chars().collect::<Vec<char>>();

    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' => {
                let mut token = String::new();
                while i < chars.len() && chars[i] == ' ' {
                    token += &" ".to_string();
                    i += 1;
                }
                tokens.push(token);
            },
            '\r' => {
                let mut token = String::new();
                while i < chars.len() && chars[i] == '\r' {
                    token += &"\r".to_string();
                    i += 1;
                }
                tokens.push(token);
            },
            '\n' => {
                let mut token = String::new();
                while i < chars.len() && chars[i] == '\n' {
                    token += &"\n".to_string();
                    i += 1;
                }
                tokens.push(token);
            },
            '\t' => {
                let mut token = String::new();
                while i < chars.len() && chars[i] == '\t' {
                    token += &"\t".to_string();
                    i += 1;
                }
                tokens.push(token);
            },
            _ => {
                let mut token = String::new();
                while i < chars.len() {
                    match chars[i] {
                        ' ' | '\r' | '\n' | '\t' => break,
                        _ => token += &chars[i].to_string(),
                    }
                    i += 1;
                }
                tokens.push(token);
            },
        }
    }

    tokens
}

/* Color example
    * RED
    * 001
    * FF0000
    * (255, 0, 0)
 */
pub fn valid_color_syntax(color: &str) -> Result<String, String> {
    if color.is_empty() {
        return Ok("".to_string());
    }
    if is_predefined_color(&color) {
        return Ok(PREDEFINED_COLORS[color].to_string());
    }
    if is_8bit_color(&color) {
        return Ok(to_8bit_color(&color));
    }
    if is_24bit_color(&color) {
        return Ok(to_24bit_color(&color));
    }
    if is_rgb_color(&color) {
        return Ok(to_rgb_color(&color));
    }

    Err(format!("invalid color syntax: \"{}\"", color))
}

fn is_predefined_color(color: &str) -> bool {
    PREDEFINED_COLORS.get(color).is_some()
}

fn is_8bit_color(color: &str) -> bool {
    color.parse::<u8>().is_ok()
}

fn is_24bit_color(color: &str) -> bool {
    if color.len() != 6 {
        return false;
    }

    i32::from_str_radix(color, 16).is_ok()
}

fn is_rgb_color(color: &str) -> bool {
    if !color.starts_with('(') || !color.ends_with(')') {
        return false;
    }
    let color = &color[1..color.len()-1].replace(',', " ");
    let rgb: Vec<&str> = color.split_whitespace().collect();
    if rgb.len() != 3 {
        return false;
    }
    for color in rgb {
        if color.parse::<u8>().is_err() {
            return false;
        }
    }

    true
}

fn to_8bit_color(color: &str) -> String {
    format!("\x1b[38;5;{}m", color)
}

fn to_24bit_color(color: &str) -> String {
    let r: u8 = u8::from_str_radix(&color[..2],  16).unwrap();
    let g: u8 = u8::from_str_radix(&color[2..4], 16).unwrap();
    let b: u8 = u8::from_str_radix(&color[4..],  16).unwrap();

    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

fn to_rgb_color(color: &str) -> String {
    let color = &color[1..color.len()-1].replace(',', " ");
    let rgb: Vec<&str> = color.split_whitespace().collect();

    format!("\x1b[38;2;{};{};{}m", rgb[0], rgb[1], rgb[2])
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_whitespace() {
        let input = r#"
aaa bbb  ccc   
    dddeee

"#;
        let expect = vec!["\n","aaa"," ","bbb","  ","ccc","   ","\n","    ","dddeee","\n\n"];
        let actual = split_whitespace(input.to_string());

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_is_predefined_color() {
        let tests = vec![
            (
                "BLACK",
                true,
            ),
            (
                "black",
                false,
            ),
            (
                "",
                false,
            ),
            (
                "shiro",
                false,
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(is_predefined_color(input), expect);
        }
    }

    #[test]
    fn test_is_8bit_color() {
        let tests = vec![
            (
                "000",
                true,
            ),
            (
                "001",
                true,
            ),
            (
                "255",
                true,
            ),
            (
                "0",
                true,
            ),
            (
                "10",
                true,
            ),
            (
                "  1",
                false,
            ),
            (
                "256",
                false,
            ),
            (
                "aaa",
                false,
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(is_8bit_color(input), expect);
        }
    }

    #[test]
    fn test_is_24bit_color() {
        let tests = vec![
            (
                "000000",
                true,
            ),
            (
                "FF0000",
                true,
            ),
            (
                "FFFFFF",
                true,
            ),
            (
                "abcdef",
                true,
            ),
            (
                "GGGGGG",
                false,
            ),
            (
                "ff000",
                false,
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(is_24bit_color(input), expect);
        }
    }

    #[test]
    fn test_is_rgb_color() {
        let tests = vec![
            (
                "(0, 0, 0)",
                true,
            ),
            (
                "(000, 000, 000)",
                true,
            ),
            (
                "(255, 0, 0)",
                true,
            ),
            (
                "(255, 255, 255)",
                true,
            ),
            (
                "(255 255 255)",
                true,
            ),
            (
                "(256, 255, 255)",
                false,
            ),
            (
                "(FF, FF, FF)",
                false,
            ),
            (
                "(0, 0, 0, 0)",
                false,
            ),
            (
                "[255, 255, 255]",
                false,
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(is_rgb_color(input), expect);
        }
    }

    #[test]
    fn test_to_8bit_color() {
        let tests = vec![
            (
                "000",
                "\x1b[38;5;000m",
            ),
            (
                "001",
                "\x1b[38;5;001m",
            ),
            (
                "255",
                "\x1b[38;5;255m",
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(to_8bit_color(input), expect);
        }
    }

    #[test]
    fn test_to_24bit_color() {
        let tests = vec![
            (
                "000000",
                "\x1b[38;2;0;0;0m",
            ),
            (
                "FF0000",
                "\x1b[38;2;255;0;0m",
            ),
            (
                "FFFFFF",
                "\x1b[38;2;255;255;255m",
            ),
            (
                "abcdef",
                "\x1b[38;2;171;205;239m",
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(to_24bit_color(input), expect);
        }
    }

    #[test]
    fn test_to_rgb_color() {
        let tests = vec![
            (
                "(0, 0, 0)",
                "\x1b[38;2;0;0;0m",
            ),
            (
                "(000, 000, 000)",
                "\x1b[38;2;000;000;000m",
            ),
            (
                "(255, 0, 0)",
                "\x1b[38;2;255;0;0m",
            ),
            (
                "(255, 255, 255)",
                "\x1b[38;2;255;255;255m",
            ),
            (
                "(255 255 255)",
                "\x1b[38;2;255;255;255m",
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(to_rgb_color(input), expect);
        }
    }
}
