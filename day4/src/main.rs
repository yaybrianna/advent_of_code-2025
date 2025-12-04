#[path = "./utils/file.rs"]
mod file;

use std::usize;

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
    let mut grid = get_grid(&raw_data);
    let mut _master_count = 0;
    let mut _paper_count = 0;
    loop {
        _paper_count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '@' {
                    let mut inner_paper_count = 0;
                    for i in -1..=1 {
                        let cur_y = y as i32 + i;
                        if cur_y < 0 || cur_y >= grid.len() as i32 {
                            continue;
                        }
                        for j in -1..=1 {
                            let cur_x = x as i32 + j;
                            if cur_x < 0 || cur_x >= grid.len() as i32 {
                                continue;
                            }
                            if cur_x as usize == x && cur_y as usize == y {
                                continue;
                            }
                            if grid[cur_y as usize][cur_x as usize] == '@' {
                                inner_paper_count += 1;
                            }
                        }
                    }
                    if inner_paper_count < 4 {
                        _paper_count += 1;
                        grid[y][x] = '.'
                    }
                }
            }
        }
        _master_count += _paper_count;

        if _paper_count == 0 {
            break;
        }
    }

    println!("No. reachable rolls: {}", _master_count);
}

fn get_grid(raw_data: &String) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in raw_data.lines() {
        let mut columns = Vec::new();
        for c in line.chars() {
            columns.push(c);
        }
        rows.push(columns);
    }
    return rows;
}
