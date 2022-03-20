/* 
    Create a function that takes two dates and returns the number of days between the first and second date. - https://edabit.com/challenge/3hdXjfJozQySRC3gE
*/

use chrono::{Duration, Utc};
use chrono::prelude::*;
use std::cmp::Ordering;


fn get_days(x: Date<chrono::Utc>, y: Date<chrono::Utc>) -> Duration {
    match x.cmp(&y) {
        Ordering::Less => y - x,
        Ordering::Greater => y - x,
        Ordering::Equal => x - y,
    }
}

fn main() {

    assert_eq!(
        get_days(
            Utc.ymd(2019, 06, 14),
            Utc.ymd(2019, 06, 20),
        ),
        chrono::Duration::days(6)
    );
    
    assert_eq!(
        get_days(
            Utc.ymd(2018, 12, 29),
            Utc.ymd(2019, 01, 01),
        ),
        chrono::Duration::days(3)
    );

    assert_eq!(
        get_days(
            Utc.ymd(2019, 06, 20),
            Utc.ymd(2019, 06, 30),
        ),
        chrono::Duration::days(10)
    );

}
