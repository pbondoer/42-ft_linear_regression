use std::process;
use std::fs::File;
use std::io::{self, Read, BufRead};

use std::collections::HashMap;

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
    let path = "./data/data.csv";
    let contents = read_file(path);

    let mut map: HashMap<i64, i64> = HashMap::new();

    for line in contents.lines() {
        if line.contains("km,price") || line.len() == 0 {
            continue;
        }

        let mut split = line.split(',');
        let km: i64 = split.next().unwrap_or_else(|| "").trim().parse().unwrap_or_else(|e| {
            println!("error: {}", e);
            process::exit(1);
        });
        let price: i64 = split.next().unwrap_or_else(|| "").trim().parse().unwrap_or_else(|e| {
            println!("error: {}", e);
            process::exit(1);
        });

        map.insert(km, price);

    }

    let len = map.len() as i64;
    let mut sorted: Vec<_> = map.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    for i in &sorted {
        println!("km {}, price {}", i.0, i.1);
    }

    let rate = 0.1;
    let mut theta: (i64, i64) = (0, 0);
    let mut temp: (i64, i64) = (0, 0);

    println!("len {}", len);

    while true {
        println!("current: {:?}", theta);
        let sum: (i64, i64) = (
            sorted.iter().fold(0, |s, &val| {
                s + (theta.0 + theta.1 * val.0 as i64) - val.1 as i64
            }),
            sorted.iter().fold(0, |s, &val| {
                s + ((theta.0 + theta.1 * val.0 as i64) - val.1 as i64) * val.0 as i64
            })
        );


        temp.0 = (rate * sum.0 as f64 / len as f64) as i64;
        temp.1 = (rate * sum.1 as f64 / len as f64) as i64;

        println!("slope: {:?}", temp);

        theta.0 -= temp.0;
        theta.1 -= temp.1;
    }

    /*
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap_or_else(|e| {
        println!("error: {}", e);
        process::exit(1);
    });

    let val: i64 = line.trim().parse().unwrap_or_else(|e| {
        println!("error: {}", e);
        process::exit(1);
    });

    println!("mileage: {}", val);

    let price = theta0 + theta1 * val;

    println!("price estimate: {}", price);
*/
}
