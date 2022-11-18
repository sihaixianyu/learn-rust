use chrono::Local;

pub fn datetime_fmt_demo() {
    let dt = Local::now();
    println!("{}", dt.format("%Y%m%d"));
}

pub fn exmaple() {
    let dt = Local::now();
    println!("{}", dt.format("%Y%m%d"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_fmt_demo() {
        datetime_fmt_demo();
        exmaple();
    }
}
