#[path = "./utils/file.rs"]
mod file;

use std::i32;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", required = true)]
    file: String,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file;
    let raw_data = file::load_file(&file_path);
    let instructions = get_vector_instructions_from_raw_file(&raw_data);

    let result = run_instructions(&instructions);
    println!("Zero Count A: {:?}\n", result);

    let result_b = run_instructions_0x434_c49434_b(&instructions);
    println!("Zero Count B: {:?}", result_b);
}

fn run_instructions(instructions: &Vec<i32>) -> i32 {
    let mut zero_count = 0;
    let mut current_dial_position = 50;
    let num_positions = 100;
    for move_distance in instructions.clone() {
        current_dial_position = current_dial_position + move_distance;
        current_dial_position = current_dial_position.rem_euclid(num_positions);
        if current_dial_position == 0 {
            zero_count += 1;
        }
        println!("Current Pos: {:?}", current_dial_position);
    }

    return zero_count;
}

fn run_instructions_0x434_c49434_b(instructions: &Vec<i32>) -> i32 {
    let mut zero_count = 0;
    let mut current_dial_position = 50;
    let num_positions = 100;
    let col_width = 8;
    println!("Start Pos: {:>col_width$}", current_dial_position);
    for move_distance in instructions.clone() {
        let start_pos = current_dial_position;

        let full_revolutions = move_distance / num_positions;
        zero_count += full_revolutions.abs();

        current_dial_position = current_dial_position + (move_distance - (full_revolutions * num_positions));
        if current_dial_position.rem_euclid(num_positions) != 0  && start_pos != 0 && current_dial_position != current_dial_position.rem_euclid(num_positions) {
            zero_count += 1;
            println!("Zero Counted!");
        }
        current_dial_position = current_dial_position.rem_euclid(num_positions);
        if current_dial_position == 0 {
            zero_count += 1;
        }

        println!(
            "Start Pos: {:>col_width$}, Move Distance: {:>col_width$}, New Pos: {:>col_width$}",
            start_pos, move_distance, current_dial_position
        );
    }

    return zero_count;
}

fn get_vector_instructions_from_raw_file(raw_data: &String) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for line in raw_data.lines() {
        let (direction, magnitude) = line.split_at(1);
        let mut value = magnitude.parse::<i32>().unwrap();
        if direction == "L" {
            value *= -1;
        }
        result.push(value);
    }
    return result;
}
