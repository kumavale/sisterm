use std::collections::HashMap;

use crate::setting;

use lazy_static::lazy_static;


//  Predefined colors
lazy_static! {
    //static ref VALID_KEYWORDS: HashMap<&'static str, Keyword> = {
    static ref PREDEFINED_COLORS: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert("RESET".to_string(),   "\x1b[0m");
        m.insert("BLACK".to_string(),   "\x1b[30m");
        m.insert("RED".to_string(),     "\x1b[31m");
        m.insert("GREEN".to_string(),   "\x1b[32m");
        m.insert("YELLOW".to_string(),  "\x1b[33m");
        m.insert("BLUE".to_string(),    "\x1b[34m");
        m.insert("MAGENTA".to_string(), "\x1b[35m");
        m.insert("CYAN".to_string(),    "\x1b[36m");
        m.insert("WHITE".to_string(),   "\x1b[37m");
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
                print!("{}{}\x1b[0m", PREDEFINED_COLORS[color], token);
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
}
