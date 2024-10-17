use chrono::{Datelike, Duration, NaiveDate, Weekday};
use std::fs::File;
use std::io::prelude::*;
use std::{io::stdin, usize};
fn main() -> std::io::Result<()> {
    println!("Please enter a month and year in the format MM/YYYY to see the calendar:");
    let (year, month) = handle_input();
    let calendar = get_calendar_for(year, month);
    println!("{}", calendar);

    let mut file = File::create(format!("calendar_{}_{}.txt", year, month))?;
    file.write_all(calendar.as_bytes())?;

    println!("Saved to {:?}", file);
    Ok(())
}
fn get_calendar_for(year: i32, month: u32) -> String {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let mut calendar_text = " ".repeat(34)
        + months[month as usize - 1]
        + " "
        + &year.to_string()
        + "\n...Sunday.....Monday....Tuesday...Wednesday...Thursday....Friday....Saturday..\n";

    let week_separator = "+----------".repeat(7) + "\n";
    let blank = ("|".to_string() + &" ".repeat(10)).repeat(7) + "|\n";

    let mut day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

    while day_of_month.weekday() != Weekday::Sun {
        day_of_month -= Duration::days(1);
    }

    loop {
        calendar_text += &week_separator;

        let mut day_number_row = String::new();
        for _ in 0..7 {
            let day_number_label = (day_of_month.day0() + 1).to_string();
            let mut _day_spacer = String::new();
            if day_number_label.len() == 1 {
                _day_spacer = " ".repeat(9);
            } else {
                _day_spacer = " ".repeat(8);
            }
            day_number_row += &("|".to_string() + &day_number_label + &_day_spacer);
            day_of_month += Duration::days(1)
        }
        day_number_row += "|\n";
        calendar_text += &day_number_row;

        calendar_text += &blank.repeat(3);

        if day_of_month.month0() != month - 1 {
            break;
        }
    }
    calendar_text += &week_separator;
    return calendar_text;
}

fn handle_input() -> (i32, u32) {
    let mut month: u32;
    let year: i32;
    'input: loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let parts = input.trim().split("/").collect::<Vec<&str>>();
        if parts.len() != 2 {
            println!("Invalid date entered. Use format MM/YYYY. Exclude day.");
            continue;
        }

        match parts[0].parse::<u32>() {
            Ok(x) => {
                if x > 0 && x < 13 {
                    month = x;
                } else {
                    println!("Unable to parse month. Enter a positive integer from 1 - 12.");
                    continue;
                }
            }
            Err(_) => {
                println!("Unable to parse month. Enter a positive integer from 1 - 12.");
                continue;
            }
        }

        match parts[1].parse::<i32>() {
            Ok(x) => {
                if x > 0 {
                    year = x;
                    break 'input;
                } else {
                    println!("Unable to parse year. Enter a positive integer.");
                    continue;
                }
            }
            Err(_) => {
                println!("Unable to parse year. Enter a positive integer.");
                continue;
            }
        }
    }
    (year, month)
}
