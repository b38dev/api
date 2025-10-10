use std::sync::LazyLock;

use chrono::{
    DateTime, Duration, FixedOffset, Months, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
};
use regex::Regex;

enum AgoLang {
    Eng,
    Chs,
}

static ENG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new( r"^\s*(?:(\d+)y\s*)?(?:(\d+)mo\s*)?(?:(\d+)d\s*)?(?:(\d+)h\s*)?(?:(\d+)m\s*)?(?:(\d+)s\s*)?ago\s*$").unwrap()
});
static CHS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new( r"^\s*(?:(\d+)年\s*)?(?:(\d+)月\s*)?(?:(\d+)天\s*)?(?:(\d+)小时\s*)?(?:(\d+)分钟\s*)?(?:(\d+)秒\s*)?前\s*$").unwrap()
});

fn parse_ago_time(time: &str, lang: AgoLang) -> anyhow::Result<DateTime<Utc>> {
    let re = match lang {
        AgoLang::Eng => &ENG,
        AgoLang::Chs => &CHS,
    };
    let caps = re
        .captures(time)
        .ok_or(anyhow::anyhow!("Failed to parse time"))?;
    let years = caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
    let month = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap());
    let days = caps.get(3).map_or(0, |m| m.as_str().parse().unwrap());
    let hours = caps.get(4).map_or(0, |m| m.as_str().parse().unwrap());
    let minutes = caps.get(5).map_or(0, |m| m.as_str().parse().unwrap());
    let seconds = caps.get(6).map_or(0, |m| m.as_str().parse().unwrap());
    let ago = Duration::days(days)
        + Duration::hours(hours)
        + Duration::minutes(minutes)
        + Duration::seconds(seconds);
    let time = Utc::now()
        .checked_sub_months(Months::new(years * 12 + month))
        .ok_or(anyhow::anyhow!("Time underflow"))?
        - ago;
    Ok(time)
}

fn to_utc8(dt: &NaiveDateTime) -> DateTime<Utc> {
    let tz = FixedOffset::east_opt(8 * 3600).unwrap();
    let dt_with_tz = tz.from_local_datetime(&dt).unwrap();
    Utc.from_utc_datetime(&dt_with_tz.naive_utc())
}

pub fn parse_time(time: &str) -> anyhow::Result<DateTime<Utc>> {
    if time.ends_with("ago") {
        return parse_ago_time(time, AgoLang::Eng);
    }
    if time.ends_with("前") {
        return parse_ago_time(time, AgoLang::Chs);
    }

    if let Ok(dt) = NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S") {
        return Ok(to_utc8(&dt));
    }

    if let Ok(dt) = NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M") {
        return Ok(to_utc8(&dt));
    }

    let d = NaiveDate::parse_from_str(time, "%Y-%m-%d")?;
    let dt = d.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    Ok(to_utc8(&dt))
}

#[cfg(test)]
#[test]
fn test_parse_time() {
    println!("{:?}", parse_time("1d ago"));
    println!("{:?}", parse_time("5年9月前"));
    println!("{:?}", parse_time("3年10月前"));
    println!("{:?}", parse_time("2025-9-12 17:00:01"));
    println!("{:?}", parse_time("2025-9-12 17:00"));
    println!("{:?}", parse_time("2025-9-12"));
}
