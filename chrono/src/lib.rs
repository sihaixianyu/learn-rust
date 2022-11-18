pub mod time_fmt;

#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveTime};

    #[test]
    fn test_case1() {
        let time = "14:21:50.761".parse::<NaiveTime>().unwrap();
        let today = Local::today();
        let dt = today.and_time(time).unwrap();

        println!("{}", dt);
        println!("{}", dt.timestamp_millis());
    }
}
