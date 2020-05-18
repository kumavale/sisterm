use regex::Regex;

pub struct Param {
    name: String,
    color: String,
    regex: Vec<Regex>,
}
