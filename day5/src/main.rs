#[path = "./utils/file.rs"]
mod file;

use std::{cmp, i64};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    lower_bound: i64,
    upper_bound: i64,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    let raw_data = file::load_file(&file_path);
    let mut valid_ranges = get_valid_ranges(&raw_data);
    let product_ids = get_product_ids(&raw_data);
    let valid_ids = get_valid_product_ids(&valid_ranges, &product_ids);
    let all_possible_valid_ids = get_possible_count(&mut valid_ranges);
    println!("No. Fresh: {}", valid_ids.len());
    println!("No. Possible: {}", all_possible_valid_ids);
}

fn get_possible_count(valid_ranges: &mut Vec<Range>) -> i64 {
    let merged_ranges = merge_overlapping_ranges(valid_ranges);
    return merged_ranges.iter().map(|r| r.upper_bound + 1).sum::<i64>()
        - merged_ranges.iter().map(|r| r.lower_bound).sum::<i64>();
}

fn merge_overlapping_ranges(ranges: &mut Vec<Range>) -> Vec<Range> {
    ranges.sort_by(|r1, r2| r1.lower_bound.cmp(&r2.lower_bound));

    let mut merged: Vec<Range> = Vec::new();
    merged.push(ranges[0].clone());

    for i in 1..ranges.len() {
        let cur: Range = ranges[i].clone();
        let last_merged_range_index: usize = merged.len() - 1;

        if cur.lower_bound >= merged[last_merged_range_index].lower_bound
            && cur.lower_bound <= merged[last_merged_range_index].upper_bound
        {
            merged[last_merged_range_index].upper_bound =
                cmp::max(cur.upper_bound, merged[last_merged_range_index].upper_bound);
        } else {
            merged.push(cur);
        }
    }
    return merged;
}

fn get_valid_product_ids(valid_ranges: &Vec<Range>, product_ids: &Vec<i64>) -> Vec<i64> {
    let mut valid_ids: Vec<i64> = Vec::new();
    'outer: for product_id in product_ids {
        for range in valid_ranges {
            if *product_id >= range.lower_bound && *product_id <= range.upper_bound {
                valid_ids.push(*product_id);
                continue 'outer;
            }
        }
    }

    return valid_ids;
}

fn get_valid_ranges(raw_data: &String) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();
    for line in raw_data.lines() {
        if line.contains("-") {
            let split = line.split("-").collect::<Vec<&str>>();
            let range = Range {
                lower_bound: split[0].parse::<i64>().expect("error parsing lower bound"),
                upper_bound: split[1].parse::<i64>().expect("error parsing upper bound"),
            };
            ranges.push(range);
        }
    }
    return ranges;
}

fn get_product_ids(raw_data: &String) -> Vec<i64> {
    let mut product_ids: Vec<i64> = Vec::new();
    for line in raw_data.lines() {
        if !line.contains("-") && !line.is_empty() {
            product_ids.push(line.parse::<i64>().expect("error parsing product_id"));
        }
    }
    return product_ids;
}
