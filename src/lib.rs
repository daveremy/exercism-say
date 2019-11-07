use std::u64;

pub fn encode(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1..=9 => say_1_to_9(n),
        10..=99 => say_10_to_99(n),
        100..=999 => say(n, 100, "hundred"),
        1_000..=9_999 => say(n, 1_000, "thousand"),
        10_000..=99_999 => say(n, 10_000, "thousand"),
        100_000..=999_999 => say(n, 1_000, "thousand"),
        1_000_000..=999_999_999 => say(n, 1_000_000, "million"),
        1_000_000_000..=999_999_999_999 => say(n, 1_000_000_000, "billion"),
        1_000_000_000_000..=999_999_999_999_999 => say(n, 1_000_000_000_000, "trillion"),
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            say(n, 1_000_000_000_000_000, "quadrillion")
        }
        1_000_000_000_000_000_000..=u64::MAX => say(n, 1_000_000_000_000_000_000, "quintillion"),
    }
}

fn say_1_to_9(n: u64) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => panic!("invalid number"),
    }
}

fn say_10_to_19(n: u64) -> String {
    match n {
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteem".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => panic!("invalid number"),
    }
}

fn say_10_to_99(n: u64) -> String {
    format!(
        "{}{}",
        match n {
            10..=19 => say_10_to_19(n),
            20..=29 => "twenty".to_string(),
            30..=39 => "thirty".to_string(),
            40..=49 => "forty".to_string(),
            50..=59 => "fifty".to_string(),
            60..=69 => "sixty".to_string(),
            70..=79 => "seventy".to_string(),
            80..=89 => "eighty".to_string(),
            90..=99 => "ninety".to_string(),
            _ => panic!("invalid number"),
        },
        if n < 19 || (n % 10 == 0) {
            "".to_string()
        } else {
            format!("-{}", say_1_to_9(n % 10))
        }
    )
}

fn say(n: u64, base: u64, identifier: &str) -> String {
    format!(
        "{} {}{}",
        encode(n / base),
        identifier.to_string(),
        if n % base == 0 {
            "".to_string()
        } else {
            format!(" {}", encode(n % base))
        }
    )
}
