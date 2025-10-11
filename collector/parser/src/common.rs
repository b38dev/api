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
    let re = r"^\s*(?:(?P<year>\d+)y\s*)?(?:(?P<month>\d+)mo\s*)?(?:(?P<day>\d+)d\s*)?(?:(?P<hour>\d+)h\s*)?(?:(?P<minute>\d+)m\s*)?(?:(?P<second>\d+)s\s*)?ago\s*$";
    Regex::new(re).unwrap()
});
static CHS: LazyLock<Regex> = LazyLock::new(|| {
    let re = r"^\s*(?:(?P<year>\d+)年\s*)?(?:(?P<month>\d+)月\s*)?(?:(?P<day>\d+)天\s*)?(?:(?P<hour>\d+)小时\s*)?(?:(?P<minute>\d+)分钟\s*)?(?:(?P<second>\d+)秒\s*)?前\s*$";
    Regex::new(re).unwrap()
});

fn parse_capture<T>(caps: &regex::Captures, name: &str, default: T) -> T
where
    T: std::str::FromStr + Clone,
    T::Err: std::fmt::Debug,
{
    caps.name(name)
        .map_or(None, |m| m.as_str().parse().ok())
        .unwrap_or(default)
}

fn parse_ago_time(time: &str, lang: AgoLang) -> anyhow::Result<DateTime<Utc>> {
    let re = match lang {
        AgoLang::Eng => &ENG,
        AgoLang::Chs => &CHS,
    };
    let caps = re
        .captures(time)
        .ok_or(anyhow::anyhow!("Failed to parse time"))?;
    let years = parse_capture(&caps, "year", 0);
    let month = parse_capture(&caps, "month", 0);
    let days = parse_capture(&caps, "day", 0);
    let hours = parse_capture(&caps, "hour", 0);
    let minutes = parse_capture(&caps, "minute", 0);
    let seconds = parse_capture(&caps, "second", 0);
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

static SHANHAI_TZ: LazyLock<FixedOffset> =
    LazyLock::new(|| FixedOffset::east_opt(8 * 3600).unwrap());

pub fn tz() -> &'static FixedOffset {
    &SHANHAI_TZ
}

fn to_utc8(dt: &NaiveDateTime) -> DateTime<Utc> {
    let dt_with_tz = tz().from_local_datetime(&dt).unwrap();
    Utc.from_utc_datetime(&dt_with_tz.naive_utc())
}

pub fn today() -> DateTime<Utc> {
    let today_zero = Utc::now()
        .date_naive()
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    to_utc8(&today_zero)
}

pub fn parse_time(time: &str) -> anyhow::Result<DateTime<Utc>> {
    if time == "今天" {
        return Ok(today());
    }
    if time == "昨天" {
        return Ok(today() - Duration::days(1));
    }
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
    let now = Utc::now();
    println!("{:?} [NOW]", now);
    println!("{:?} [今天]", parse_time("今天").unwrap());
    println!("{:?} [昨天]", parse_time("昨天").unwrap());
    println!("{:?} [30m ago]", parse_time("30m ago").unwrap());
    println!("{:?} [1d ago]", parse_time("1d ago").unwrap());
    println!("{:?} [1月20天前]", parse_time("1月20天前").unwrap());
    println!("{:?} [5年9月前]", parse_time("5年9月前").unwrap());
    println!("{:?} [3年10月前]", parse_time("3年10月前").unwrap());
    let eq = |s: &str, format: &str| {
        let parsed = parse_time(s).unwrap();
        let parsed = parsed.with_timezone(tz());
        let formated = parsed.format(format).to_string();
        assert_eq!(formated, s);
    };
    eq("2025-09-12 17:00:05", "%Y-%m-%d %H:%M:%S");
    eq("2025-09-12 17:00", "%Y-%m-%d %H:%M");
    eq("2025-09-12", "%Y-%m-%d");
}
