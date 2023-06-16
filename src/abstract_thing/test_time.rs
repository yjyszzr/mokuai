use chrono::{DateTime, Duration, FixedOffset, Local, Utc};

#[test]
fn test_time0() {
    let now_utc: DateTime<Utc> = Utc::now();
    println!("Current UTC time: {}", now_utc);

    let now_local: DateTime<Local> = Local::now();
    println!("Current local time: {}", now_local);
}


use chrono::{NaiveDateTime};

#[test]
fn test_time1() {
    let now: DateTime<Utc> = Utc::now();
    let duration = Duration::days(7);
    let one_week_later = now + duration;
    println!("One week later: {}", one_week_later);

    let duration_since_unix_epoch = now.timestamp();
    println!("Duration since Unix Epoch: {} seconds", duration_since_unix_epoch);
}



#[test]
fn test_time2() {
    let timestamp = 1_620_000_000;
    let date_time = DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(timestamp, 0), Utc);
    println!("Date and time from timestamp: {}", date_time);

    let new_timestamp = date_time.timestamp();
    println!("Timestamp from date and time: {}", new_timestamp);
}

