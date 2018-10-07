use std::process;
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> String {
    let mut file;
    let mut contents = String::new();

    file = File::open(path).unwrap_or_else(|e| {
        println!("error: {}", e);
        process::exit(1);
    });

    file.read_to_string(&mut contents).unwrap_or_else(|e| {
        println!("error: {}", e);
        process::exit(1);
    });

    contents
}

fn main() {
    let path = "./data/values";
    let contents = read_file(path);

    let mut theta0: f64 = 0.0;
    let mut theta1: f64 = 0.0;

    for raw in contents.lines() {
        let line = raw.split('#').next().unwrap_or_else(|| "");

        if line.len() == 0 {
            continue;
        }

        let mut split = raw.splitn(2, '=');
        let key = split.next().unwrap_or_else(|| "").trim();
        let value = split.next().unwrap_or_else(|| "").trim();

        match key {
            "theta0" => theta0 = value.parse().unwrap_or_else(|e| {
                println!("error: theta0: {}", e);
                process::exit(1);
            }),
            "theta1" => theta1 = value.parse().unwrap_or_else(|e| {
                println!("error: theta1: {}", e);
                process::exit(1);
            }),
            _ => {
                println!("error: invalid key in values file");
                process::exit(1);
            }
        }
    }

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap_or_else(|e| {
        println!("error: {}", e);
        process::exit(1);
    });

    let val: f64 = line.trim().parse().unwrap_or_else(|e| {
        println!("error: {}", e);
        process::exit(1);
    });

    println!("mileage: {}", val);

    let price = theta0 + theta1 * val;

    println!("price estimate: {}", price);

}
