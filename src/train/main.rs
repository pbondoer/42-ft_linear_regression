use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process;

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

    let mut min = std::i64::MAX;
    let mut max = std::i64::MIN;

    // Parse data
    for line in contents.lines() {
        if line.contains("km,price") || line.len() == 0 {
            continue;
        }

        let mut split = line.split(',');
        let km: i64 = split
            .next()
            .unwrap_or_else(|| "")
            .trim()
            .parse()
            .unwrap_or_else(|e| {
                println!("error: {}", e);
                process::exit(1);
            });
        let price: i64 = split
            .next()
            .unwrap_or_else(|| "")
            .trim()
            .parse()
            .unwrap_or_else(|e| {
                println!("error: {}", e);
                process::exit(1);
            });

        // Collect minimum and maximum as we parse
        if km < min {
            min = km;
        }
        if km > max {
            max = km;
        }

        map.insert(km, price);
    }

    let scale = (max - min) as f64;

    // Sort for display
    let len = map.len() as i64;
    let mut sorted: Vec<_> = map.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    for i in &sorted {
        let km = (i.0 - min) as f64 / scale;
        println!("km {}, price {} - {}", i.0, i.1, km);
    }

    let rate = 0.1;
    let mut theta: (f64, f64) = (0.0, 0.0);
    let mut temp: (f64, f64) = (1.0, 1.0);

    println!("len {}", len);

    let mut i = 0;

    while temp.0.abs() > 0.001 && temp.1.abs() > 0.001 {
        let sum: (f64, f64) = (
            sorted.iter().fold(0.0, |acc, &val| {
                let scaled = (val.0 - min) as f64 / scale;
                acc + (theta.0 + theta.1 * scaled) - val.1 as f64
            }),
            sorted.iter().fold(0.0, |acc, &val| {
                let scaled = (val.0 - min) as f64 / scale;
                acc + ((theta.0 + theta.1 * scaled) - val.1 as f64) * scaled
            }),
        );

        // calculate slopes
        temp.0 = rate * sum.0 / len as f64;
        temp.1 = rate * sum.1 / len as f64;

        theta.0 -= temp.0;
        theta.1 -= temp.1;

        // show some stats
        if i == 0 {
            i = 100;
            println!("current: {:?}", theta);
            println!("slope:   {:?}", temp);
        }

        i -= 1;
    }

    // scale back theta1
    theta.1 = theta.1 / scale;

    println!("------------ FINAL ----------------");
    println!("theta: {:?}", theta);

    // write to file
    let mut f = File::create("./data/values").expect("Unable to create file");
    write!(f, "theta0 = {}\ntheta1 = {}\n", theta.0, theta.1).unwrap();

    // rust will close the file once we get out of scope
}
