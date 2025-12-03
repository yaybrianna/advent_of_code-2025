#[path = "./utils/file.rs"]
mod file;

use std::ops::Index;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,

    #[arg(short, long, value_name = "FILE", required = true)]
    battery_count: usize,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    let raw_data = file::load_file(&file_path);
    let banks = get_banks_of_joltages(&raw_data);
    let mut sum: u64 = 0;
    for (i, bank) in banks.iter().enumerate() {
        let max = get_max_joltage(&bank, cli.battery_count);
        println!("-Bank[{}] Max: {} ⚡", i + 1, max);
        sum += max;
    }
    println!("Total Jolts ⚡: {}", sum);
}


fn get_max_joltage(bank: &Vec<u32>, length: usize) -> u64 {
    return get_max_string(&bank, length)
        .parse::<u64>()
        .expect("Error converting max string to u64");
}

fn get_max_string(bank: &Vec<u32>, length: usize) -> String {
    if length == 0 {
        return "".to_string();
    }
    //println!("bank: {:?}", bank);
    let mut max = 0;
    let mut max_index = 0;

    for i in 0..=bank.len() - length {
        if *bank.index(i) > max {
            max = bank.index(i).clone();
            max_index = i
        }
    }
    let (_first, second) = bank.split_at(max_index + 1);
    return bank.index(max_index).to_string() + &get_max_string(&second.to_vec(), length - 1);
}

fn get_banks_of_joltages(raw_data: &String) -> Vec<Vec<u32>> {
    let mut banks: Vec<Vec<u32>> = Vec::new();
    for line in raw_data.lines() {
        let mut bank = Vec::new();
        for c in line.chars() {
            let joltage = c.to_digit(10).expect("Error parsing number") as u32;
            bank.push(joltage);
        }
        banks.push(bank);
    }
    return banks;
}
