extern crate argparse;

use argparse::{ArgumentParser, Store, Collect};

fn parse_value(v : &String) -> u64 {
    if v.chars().nth(1) == Some('x') {
        let raw = v.trim_start_matches("0x");
        return u64::from_str_radix(&raw, 16).unwrap();
    } else if v.chars().nth(1) == Some('b'){
        let raw = v.trim_start_matches("0b");
        return u64::from_str_radix(&raw, 2).unwrap();
    } 
    return v.parse::<u64>().unwrap();
}

fn use_words_hex(ws: &usize, value: &u64) -> String {
    let word_size = ws/2; //One hex digit represents 2 bytes
    let size = format!("{:X}", value).chars().count();
    let res = format!("{:0>width$X}", value, width = word_size * (size as f64/word_size as f64).ceil() as usize);
    let res = res.chars()
                    .enumerate()    
                    .flat_map(|(i, c)| {
                        if i != 0 && i % word_size == 0 {
                            Some(' ')
                        } else {
                            None
                        }
                        .into_iter()
                        .chain(std::iter::once(c))
                    })
                    .collect::<String>();
    return res;

fn use_words_bin(ws: &usize, value: &u64) -> String { 
    let word_size = ws*8; //8 bin digitd represents 1 bytes
    let size = format!("{:b}", value).chars().count();
    let res = format!("{:0>width$b}", value, width = word_size * (size as f64/word_size as f64).ceil() as usize);
    let res = res.chars()
                    .enumerate()    
                    .flat_map(|(i, c)| {
                        if i != 0 && i % (word_size) == 0 {
                            Some(' ')
                        } else {
                            None
                        }
                        .into_iter()
                        .chain(std::iter::once(c))
                    })
                    .collect::<String>();
    return res;
}

fn main() {
    let mut word_size: usize = 0;
    let mut tp: String = String::new();
    let mut vec: Vec<String> = vec!();
    {  
        let mut ap = ArgumentParser::new();
        ap.set_description("Convert one number formarting into another. Supported types are binary, decimal, and hex.");
        ap.refer(&mut word_size)
            .add_option(&["-w", "--word"], Store,
            "Specify output wordsize (only eligible for binary and hex output)");
        ap.refer(&mut tp).add_argument("type", Store, "Type to convert to. Options are: 'bin', 'dec', and 'hex'").required();
        ap.refer(&mut vec).add_argument("value", Collect, "Value that should be converted. Hex values start with 0x, binary with 0b, spaces are allowed.").required();
        ap.parse_args_or_exit();
    }
    
    let v = vec.concat();
    let value = parse_value(&v);
    
    if word_size > 0 {
        if tp == "dec" {
            println!("Value {} in {} is: {}. Decimal value cannot be displayed in words.", v, tp, value);
        } else if tp == "hex" {
            let res = use_words_hex(&word_size, &value);
            println!("Value {} in {} is: {}", v, tp, res);
        } else if tp == "bin" {
            let res = use_words_bin(&word_size, &value);
            println!("Value {} in {} is: {}", v, tp, res);
        } else {
            println!("Type {} not supported.", tp);
        }
    } else {
        if tp == "dec" {
            println!("Value {} in {} is: {}", v, tp, value);
        } else if tp == "hex" {
            println!("Value {} in {} is: {:#X}", v, tp, value);
        } else if tp == "bin" {
            println!("Value {} in {} is: {:#b}", v, tp, value);
        } else {
            println!("Type {} not supported.", tp);
        }
    }

}

