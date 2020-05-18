use regex::Regex;

pub fn coloring_from_file(text: String) {
    let re_dbg = Regex::new("ip").unwrap();
    let tokens = split_whitespace(text);

    for token in &tokens {
        match re_dbg.captures(token) {
            Some(_) => print!("\x1b[31m{}\x1b[0m", token),
            None => print!("{}", token),
        }
    }
    //println!("{:?}", tokens);
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
