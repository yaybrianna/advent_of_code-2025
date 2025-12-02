#[path = "./utils/file.rs"]
mod file;

use std::u64;

use clap::Parser;
use fancy_regex::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}

#[derive(Debug)]
pub struct ProductIdRange {
    start: u64,
    end: u64,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    let raw_data = file::load_file(&file_path);
    let ranges = get_product_id_ranges_from_raw_file(&raw_data);
    let mut sum: u64 = 0;
    for range in ranges {
        for invalid_id in get_invalid_product_ids(&range){
            println!("invalid id: {}", invalid_id);
            sum += invalid_id;
        }
    }
    println!("total: {}", sum);
}

fn get_invalid_product_ids(range: &ProductIdRange)-> Vec<u64> {
    let mut result = Vec::new();
    for number in range.start..=range.end {
        if contains_repeated_pattern(number){
            result.push(number);
        }
    }
    return result;
}

fn contains_repeated_pattern(number: u64) -> bool {
    // let regex_pattern = r"\b(\d+)\1\b"; //part 1
    let regex_pattern = r"\b(\d+)\1+\b"; //part 2
    let regex: Regex = Regex::new(regex_pattern).unwrap();
    return regex.captures_iter(number.to_string().as_str()).count() > 0;
}

fn get_product_id_ranges_from_raw_file(raw_data: &String) -> Vec<ProductIdRange> {
    let mut result: Vec<ProductIdRange> = Vec::new();
    for line in raw_data.lines() {
        let range_strings = line.split(",");
        for range_string in range_strings {
            let range_parts = range_string.split("-");
            result.push(ProductIdRange {
                start: range_parts.clone().nth(0).unwrap().parse::<u64>().unwrap(),
                end: range_parts.clone().nth(1).unwrap().parse::<u64>().unwrap(),
            });
        }
    }
    return result;
}
