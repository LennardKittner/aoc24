use std::{fmt::Write, fs::{self, File}, time::Instant, io::Write as IOWrite};
use std::cmp::max;
use std::io::Read;
use std::os::unix::fs::FileExt;
use std::str::FromStr;
use std::time::Duration;

use indoc::formatdoc;
use itertools::Itertools;
use regex::Regex;

mod days;

#[allow(dead_code)]
fn exec<F1: Fn(&str) -> String, F2: Fn(&str) -> String>(day: i32, part1: F1, part2: F2, input: &str) {
    let start_time = Instant::now();
    let result1 = part1(input);
    let end_time = Instant::now();
    let duration1 = end_time - start_time;
    let start_time = Instant::now();
    let result2 = part2(input);
    let end_time = Instant::now();
    let duration2 = end_time - start_time;

    println!("part1 result: {result1} time: {:?}", duration1);
    println!("part2 result: {result2} time: {:?}", duration2);

    add_day_to_readme_table(day, duration1, duration2);
}

fn add_day_to_readme_table(day: i32, time_part_1: Duration, time_part_2: Duration) {
    let entry = generate_readme_table_entry(day, time_part_1, time_part_2);

    let mut readme = File::options().read(true).write(true).open("README.md").expect("Failed to open README");
    let mut content = String::new();
    let _ = readme.read_to_string(&mut content).expect("Failed to read README");
    let table_match = content.match_indices(
        "| Day | Part 1 | Part 2 |\n| :---: | :---: | :---:  |"
    ).collect_vec().first().copied();
    if table_match.is_none() {
        println!("This template can automatically recorde the execution time of your solution if the README.md contains: \n| Day | Part 1 | Part 2 |\n| :---: | :---: | :---:  |\n\nTotal time: `0s`");
        return;
    }
    let table_match = table_match.unwrap();
    let first_entry_index = table_match.0 + table_match.1.len();
    let table_header = table_match.1;
    let match_decimal = r"(?:-?\d+)(?:,?\d+)*(?:\.\d+(?:e\d+)?)?";

    let table_rows = &mut content[first_entry_index..].lines().filter_map(|row| {
        if !row.starts_with('|') {
            None
        } else {
            let re = Regex::new(&format!(r"\| \[Day (\d+).* \| `({0})(..)` \| `({0})(..)` \|", match_decimal)).unwrap();
            let result: (&str, [&str; 5]) = re.captures(row).unwrap().extract();
            let time1 = (f64::from_str(result.1[1]).unwrap() * if result.1[2] == "ms" { 1000000f64 } else { 1000f64 }) as u64;
            let time2 = (f64::from_str(result.1[3]).unwrap() * if result.1[4] == "ms" { 1000000f64 } else { 1000f64 }) as u64;
            Some((result.0, i32::from_str(result.1[0]).unwrap(), time1, time2))
        }
    }).collect_vec();
    let mut old_table = format!("{}\n{}", table_header, table_rows.iter().map(|r| r.0).join("\n"));
    if table_rows.is_empty() {
        old_table.pop(); // remove last new line if table is empty
    }
    let mut insert_at = 0;
    let mut overwrite = false;
    for row in table_rows.iter() {
        if row.1 <= day {
            insert_at += 1;
        }
        if row.1 == day {
            overwrite = true;
            break;
        }
    }
    if overwrite {
        table_rows[insert_at-1 /* zero based */] = (&entry, day, time_part_1.as_nanos() as u64, time_part_2.as_nanos() as u64);
    } else {
        table_rows.insert(insert_at, (&entry, day, time_part_1.as_nanos() as u64, time_part_2.as_nanos() as u64));
    }
    let new_table = format!("{}\n{}", table_header, table_rows.iter().map(|r| r.0).join("\n"));

    let total_time = format!("Total time: `{:?}`", Duration::from_nanos(table_rows.iter().map(|e| e.2 + e.3).sum()));
    let total_time_regex = Regex::new(&format!("Total time: `{match_decimal}.*`")).unwrap();
    let old_total_time = total_time_regex.captures(&content);
    let mut output = content.replace(&old_table, &new_table);
    if old_total_time.is_none() {
        println!("This template can automatically recorded the total execution time of your solution if the README.md contains: \nTotal time: `0s`");
    } else {
        let old_total_time: (&str, [&str; 0]) = old_total_time.unwrap().extract();
        output = output.replace(old_total_time.0, &total_time);
    }
    readme.write_at(output.as_ref(), 0).expect("Failed to write README");
}

fn generate_readme_table_entry(day: i32, time_part_1: Duration, time_part_2: Duration) -> String {
    if max(time_part_1.as_millis(), time_part_2.as_millis()) < 2 {
        format!("| [Day {day}](./src/days/day{day}.rs) | `{:.2}µs` | `{:.2}µs` |", time_part_1.as_secs_f64() * 1000f64 * 1000f64, time_part_2.as_secs_f64() * 1000f64 * 1000f64)
    } else {
        format!("| [Day {day}](./src/days/day{day}.rs) | `{:.2}ms` | `{:.2}ms` |", time_part_1.as_secs_f64() * 1000f64, time_part_2.as_secs_f64() * 1000f64)
    }
}

fn generate_mod_string(days: &Vec<i32>) -> String {
    let mut result: String = String::new();
    for i in days {
        result += format!("mod day{i};\n").as_str();
    }
    result
}

fn generate_match_branches(days: &[i32]) -> String {
    days.iter()
        .fold(String::new(), |mut result, d| { write!(&mut result, r#"
        {d} => {{
            exec({d}, day{d}::exec_day{d}_part1, day{d}::exec_day{d}_part2, &input);
        }},"#).unwrap();
            result
        })
}

pub fn generate(day: i32) {
    let _ = File::create(format!("./input/day{day}.txt")).expect("Failed to generate input");
    let _ = File::create(format!("./example/day{day}.txt")).expect("Failed to generate example");

    let src_file_path = format!("./src/days/day{day}.rs");
    let mod_path = "./src/days/mod.rs";
    let mut days = get_days();
    if days.contains(&day) {
        println!("skipping code generation of {src_file_path} already exists");
    } else {
        days.push(day);
        let mut new_day = File::create(&src_file_path).expect("Failed to generate source.");
        new_day.write_all(formatdoc!{
            r#"
            #[cfg(test)]
            use std::fs;


            pub fn exec_day{day}_part1(input: &str) -> String {{
                //TODO
                "Not implemented!".to_string()
            }}

            pub fn exec_day{day}_part2(input: &str) -> String {{
                //TODO
                "Not implemented!".to_string()
            }}

            #[test]
            fn test_day{day}_part1() {{
                let input = match fs::read_to_string("./example/day{day}.txt".to_string()) {{
                    Ok(s) => s,
                    Err(_) => panic!(),
                }};
                assert_eq!(exec_day{day}_part1(&input), "TODO")
            }}

            #[test]
            fn test_day{day}_part2() {{
                let input = match fs::read_to_string("./example/day{day}.txt".to_string()) {{
                    Ok(s) => s,
                    Err(_) => panic!(),
                }};
                assert_eq!(exec_day{day}_part2(&input), "TODO")
            }}
            "#
        }.as_bytes()).unwrap_or_else(|_| panic!("Failed to write {src_file_path}."));
    }
    let mut new_mod = File::create(mod_path).unwrap_or_else(|_| panic!("Failed to generate {mod_path}."));
    new_mod.write_all( formatdoc!{
        r#"
        use std::fs;

        use crate::exec;

        {}

        pub fn run(day: i32) {{
            let input = match fs::read_to_string(format!("./input/day{{}}.txt", day)) {{
                Ok(s) => s,
                Err(_) => return,
            }};
            match day {{
                t if t < 0 => (),{}
                _ => (),
            }}
        }}
        "#, generate_mod_string(&days), generate_match_branches(&days)
    }.as_bytes()).unwrap_or_else(|_| panic!("Failed to write {mod_path}."));
    println!("Put your input in ./input/day{day}.txt, examples in ./example/day{day}.txt, and your code in ./src/days/day{day}.txt")
}

fn get_days() -> Vec<i32> {
    let days_dir_path = "./src/days";
    let days: Vec<i32> = fs::read_dir(days_dir_path).unwrap().filter_map(|entry|
        entry.unwrap().file_name().to_string_lossy().chars()
            .filter(|c|
                c.is_numeric()
            ).collect::<String>().parse::<i32>().ok()
    ).collect();
    days
}

pub fn run(day: i32) {
    days::run(day);
}