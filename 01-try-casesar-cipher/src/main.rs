use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let in_text = &args[1];
    let shift: i16 = args[2].parse().unwrap();

    let encode_text = encode(in_text, shift);
    println!("{} -> {}", in_text, encode_text);
}

fn rotate(ch: char, code: i16, shift: i16, num: i16) -> u8 {
    return (((ch as i16) - code + shift + num) % num + code) as u8;
}

fn convert_u8_to_char(ch: u8) -> char {
    return ch as char;
}

fn encode(in_text: &str, shift: i16) -> String {
    let alphabet_num = 26;
    let upper_first_code = 'A' as i16;
    let lower_first_code = 'a' as i16;
    let mut result = String::new();

    for ch in in_text.chars() {
        let push_ch: char;
        if 'a' <= ch && ch <= 'z' {
            push_ch = convert_u8_to_char(rotate(ch, lower_first_code, shift, alphabet_num));
        } else if 'A' <= ch && ch <= 'Z' {
            push_ch = convert_u8_to_char(rotate(ch, upper_first_code, shift, alphabet_num));
        } else {
            push_ch = ch
        }
        result.push(push_ch);
    }

    return result;
}
