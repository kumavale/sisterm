use std::collections::HashMap;
use std::io::Write;
use std::fmt::Write as _;

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
    if params.is_none() {
        println!("{}", text);
        return;
    }

    let params = params.unwrap();
    let mut all_string = String::new();

    for line in text.lines() {
        let mut line_str       = String::new();
        let mut increasing_str = String::new();
        let mut prev_matched   = false;
        let mut substring_len  = 0;
        let mut comment_now    = false;

        'outer: for c in line.chars() {

            if c == ' ' && !comment_now {
                if !increasing_str.is_empty() {
                    if prev_matched {
                        line_str.push_str(PREDEFINED_COLORS["RESET"]);
                        prev_matched = false;
                    } else {
                        line_str.push_str(&increasing_str[..increasing_str.len()]);
                    }
                    increasing_str.clear();
                }
                line_str.push(' ');
                continue;
            }

            increasing_str.push(c);
            for (index, syntax) in params.syntaxes.iter().enumerate() {
                for regex in syntax.regex() {
                    if let Some(cap) = regex.captures(&increasing_str) {
                        if prev_matched {
                            let len = cap.get(0).unwrap().as_str().len();
                            if substring_len == len {
                                prev_matched = false;
                                line_str.push_str(PREDEFINED_COLORS["RESET"]);
                                substring_len = 0;
                                increasing_str.clear();
                                increasing_str.push(c);
                            } else {
                                substring_len = len;
                                line_str.push(c);
                            }
                        } else {
                            let substr = cap.get(0).unwrap().as_str();
                            let len = substr.len();
                            let color = params.syntaxes[index].color();
                            comment_now = params.syntaxes[index].ignore_whitespace();
                            line_str.push_str(&increasing_str[..increasing_str.len()-len]);
                            line_str.push_str(color);
                            line_str.push_str(substr);
                            increasing_str = substr.to_string();
                            substring_len = len;
                            prev_matched = true;
                        }
                        continue 'outer;
                    }
                }
            }

            if prev_matched {
                prev_matched = false;
                line_str.push_str(PREDEFINED_COLORS["RESET"]);
                line_str.push_str(&increasing_str);
                increasing_str.clear();
            }
        }

        if !prev_matched {
            line_str.push_str(&increasing_str);
        }
        line_str.push_str(PREDEFINED_COLORS["RESET"]);
        all_string.push_str(&line_str);
        all_string.push('\n');
    }

    println!("{}", all_string);
}

pub fn coloring_words(serial_buf: &str,
                     (increasing_str, prev_matched, comment_now): &mut (String, bool, bool),
                      params: &Option<setting::Params>)
{
    let params = params.as_ref().expect("assert Some");
    let mut substring_len = increasing_str.len();
    let mut line_str = String::new();

    'outer: for c in serial_buf.chars() {

        if c == ' ' && !*comment_now {
            increasing_str.clear();
            if *prev_matched {
                line_str.push_str(PREDEFINED_COLORS["RESET"]);
            }
            *prev_matched = false;
            line_str.push(' ');
            continue;
        }

        if c == '\r' || c == '\n' {
            increasing_str.clear();
            if *prev_matched {
                line_str.push_str(PREDEFINED_COLORS["RESET"]);
            }
            *prev_matched = false;
            line_str.push(c);
            continue;
        }

        if c == '\x08' { // BS
            increasing_str.pop();
            line_str.push(c);
            continue;
        }

        increasing_str.push(c);

        for (index, syntax) in params.syntaxes.iter().enumerate() {
            for regex in syntax.regex() {
                if let Some(cap) = regex.captures(increasing_str) {
                    if *prev_matched {
                        let len = cap.get(0).unwrap().as_str().len();
                        if substring_len == len {
                            *prev_matched = false;
                            line_str.push_str(PREDEFINED_COLORS["RESET"]);
                            substring_len = 0;
                            increasing_str.clear();
                            increasing_str.push(c);
                        } else {
                            substring_len = len;
                        }
                        line_str.push(c);
                    } else {
                        let substr = cap.get(0).unwrap().as_str();
                        let len = substr.len();
                        let color = params.syntaxes[index].color();
                        *comment_now = params.syntaxes[index].ignore_whitespace();
                        let _ = write!(line_str, "{:\x08<1$}", "", len-1);
                        line_str.push_str(color);
                        line_str.push_str(substr);
                        *increasing_str = substr.to_string();
                        substring_len = len;
                        *prev_matched = true;
                    }
                    continue 'outer;
                }
            }
        }

        if *prev_matched {
            *prev_matched = false;
            line_str.push_str(PREDEFINED_COLORS["RESET"]);
            increasing_str.clear();
            increasing_str.push(c);
        }

        line_str.push(c);
    }

    std::io::stdout().write_all(line_str.as_bytes()).unwrap();
}

/* Color example
    * RED
    * 001
    * FF0000
    * #FF0000
    * (255, 0, 0)
 */
pub fn valid_color_syntax(coloring: &setting::Coloring) -> Result<String, String> {
    let color      = coloring.color();
    let underlined = coloring.underlined();

    if color.is_empty() {
        if underlined {
            return Ok("\x1b[4m".to_string());
        } else {
            return Ok("\x1b[0m".to_string());
        }
    }
    if is_predefined_color(color) {
        if underlined {
            return Ok(format!("\x1b[4m{}", PREDEFINED_COLORS[color]));
        } else {
            return Ok(PREDEFINED_COLORS[color].to_string());
        }
    }
    if is_8bit_color(color) {
        return Ok(to_8bit_color(color, underlined));
    }
    if is_24bit_color(color) {
        return Ok(to_24bit_color(color, underlined));
    }
    if is_24bit_color_hash(color) {
        return Ok(to_24bit_color(&color[1..], underlined));
    }
    if is_rgb_color(color) {
        return Ok(to_rgb_color(color, underlined));
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
    color.len() == 6 && i32::from_str_radix(color, 16).is_ok()
}

fn is_24bit_color_hash(color: &str) -> bool {
    color.len() == 7 && color.starts_with('#') && i32::from_str_radix(&color[1..], 16).is_ok()
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

fn to_8bit_color(color: &str, underlined: bool) -> String {
    if underlined {
        format!("\x1b[4;38;5;{}m", color)
    } else {
        format!("\x1b[38;5;{}m", color)
    }
}

fn to_24bit_color(color: &str, underlined: bool) -> String {
    let r: u8 = u8::from_str_radix(&color[..2],  16).unwrap();
    let g: u8 = u8::from_str_radix(&color[2..4], 16).unwrap();
    let b: u8 = u8::from_str_radix(&color[4..],  16).unwrap();

    if underlined {
        format!("\x1b[4;38;2;{};{};{}m", r, g, b)
    } else {
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }
}

fn to_rgb_color(color: &str, underlined: bool) -> String {
    let color = &color[1..color.len()-1].replace(',', " ");
    let rgb: Vec<&str> = color.split_whitespace().collect();

    if underlined {
        format!("\x1b[4;38;2;{};{};{}m", rgb[0], rgb[1], rgb[2])
    } else {
        format!("\x1b[38;2;{};{};{}m", rgb[0], rgb[1], rgb[2])
    }
}



#[cfg(test)]
mod tests {
    use super::*;

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
            (
                "#ff000",
                false,
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(is_24bit_color(input), expect);
        }
    }

    #[test]
    fn test_is_24bit_color_hash() {
        let tests = vec![
            (
                "#000000",
                true,
            ),
            (
                "#FF0000",
                true,
            ),
            (
                "#FFFFFF",
                true,
            ),
            (
                "#abcdef",
                true,
            ),
            (
                "#GGGGGG",
                false,
            ),
            (
                "#ff000",
                false,
            ),
            (
                "ff000",
                false,
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(is_24bit_color_hash(input), expect);
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
            assert_eq!(to_8bit_color(input, false), expect);
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
            assert_eq!(to_24bit_color(input, false), expect);
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
            assert_eq!(to_rgb_color(input, false), expect);
        }
    }

    #[test]
    fn test_underlined() {
        let tests = vec![
            (
                to_8bit_color("123", true),
                "\x1b[4;38;5;123m",
            ),
            (
                to_24bit_color("000000", true),
                "\x1b[4;38;2;0;0;0m",
            ),
            (
                to_rgb_color("(0, 0, 0)", true),
                "\x1b[4;38;2;0;0;0m",
            ),
        ];

        for (actual, expect) in tests {
            assert_eq!(actual, expect);
        }
    }
}
