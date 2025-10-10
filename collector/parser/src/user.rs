use std::collections::HashSet;
use std::vec;

use chrono::{Months, Utc};
use visdom::Vis;
use visdom::types::Elements;

use super::common::parse_time;
use model::prelude::{Collections, InitUser, SubjectType, TypedCollection, Uid, UserState};

pub fn parse_collection(section: &Elements) -> Option<TypedCollection> {
    let list = section
        .find(".horizontalOptions ul li:not(.title) a")
        .map(|_, e| {
            let count = e.children().last().text().parse::<usize>().unwrap();
            let state = e
                .get_attribute("href")
                .unwrap()
                .to_string()
                .split("/")
                .last()
                .unwrap()
                .to_string();
            (state, count)
        });
    TypedCollection::build(list)
}

pub fn parse_userpage(html: &str) -> anyhow::Result<InitUser> {
    // tracing::debug!("Parsing user page HTML {html}");
    let document = Vis::load(html).map_err(|e| anyhow::anyhow!("Failed to load HTML: {}", e))?;
    let join_time = document
        .find("#user_home ul.network_service > li:first-child > span.tip")
        .text()
        .replace(" 加入", "");
    let join_time = parse_time(&join_time).ok();
    let name_single_element = document.find("#headerProfile .nameSingle");
    let name_element = name_single_element.find(".inner .name");
    let uid = name_element.find("small.grey").text()[1..].to_string();
    let uid = Uid::from_str(&uid);
    let (nid, sid) = if let Uid::Nid(nid) = uid {
        (Some(nid), None)
    } else {
        let pm_href = name_single_element
            .find(".inner .actions a.chiiBtn:last-of-type")
            .attr("href");
        let nid = pm_href
            .map_or(None, |href| {
                let href = href.to_string();
                if !href.starts_with("/pm/compose/") {
                    return None;
                }
                let nid = href
                    .trim_start_matches("/pm/compose/")
                    .trim_end_matches(".chii");
                let nid = Uid::from_str(nid);
                if let Uid::Nid(nid) = nid {
                    Some(nid)
                } else {
                    None
                }
            })
            .map_or_else(
                || {
                    let avatar_style = name_single_element
                        .find(".headerAvatar .avatar span.avatarNeue")
                        .attr("style");
                    if avatar_style.is_none() {
                        return None;
                    }
                    let avatar_style = avatar_style.unwrap().to_string();
                    if !avatar_style.starts_with("background-image:url('//lain.bgm.tv/pic/user/") {
                        return None;
                    }
                    let nid = avatar_style.split("/").last().unwrap();
                    let nid = nid.split(".").next().unwrap();
                    let nid = nid.split("_").next().unwrap();
                    let nid = Uid::from_str(nid);
                    if let Uid::Nid(nid) = nid {
                        Some(nid)
                    } else {
                        None
                    }
                },
                |x| Some(x),
            );
        let Uid::Sid(sid) = uid else {
            return Err(anyhow::anyhow!("Failed to parse user id"));
        };
        (nid, Some(sid))
    };

    let name = name_element.find("a").text();
    let timeline = document
        .find("#pinnedLayout ul.timeline > li small.time")
        .map(|_, element| element.text());
    let (state, collections) = if timeline.len() > 0 {
        let last = parse_time(&timeline[0])?;
        let list = vec!["anime", "game", "book", "music", "real"];
        let list = list.iter().map(|t| {
            let st = t.parse::<SubjectType>().unwrap();
            let collection = parse_collection(&document.find(&format!("#{t}.section")));
            (st, collection)
        });
        let collections = Collections::build(list.collect());
        if last + Months::new(12) < Utc::now() {
            (UserState::Abondon, collections)
        } else {
            (UserState::Active, collections)
        }
    } else if name.eq("[已封禁]") {
        (UserState::Banned, None)
    } else {
        (UserState::Dropped, None)
    };

    Ok(InitUser {
        name,
        nid,
        sid,
        join_time,
        state,
        collections,
        names_update: None,
    })
}

#[derive(Debug, Clone)]
pub struct NameHistoryWithKeyPoint {
    pub names: HashSet<String>,
    pub key_point: String,
}

pub fn parse_timeline_name_history(html: &str) -> anyhow::Result<Option<NameHistoryWithKeyPoint>> {
    let document = Vis::load(html).map_err(|e| anyhow::anyhow!("Failed to load HTML: {}", e))?;
    let timeline = document.find("#timeline");
    if timeline.length() < 1 {
        return Ok(None);
    }
    let key_point = timeline.find("h4.Header").first().text();
    let key_point = parse_time(&key_point)?;
    let key_point = key_point.format("%Y-%-m-%-d").to_string();
    let names = timeline
        .find("li.tml_item > span > p.status:has(strong) > strong")
        .map(|_, e| e.text())
        .into_iter()
        .collect::<HashSet<String>>();

    Ok(Some(NameHistoryWithKeyPoint { key_point, names }))
}

// #[cfg(test)]
// mod tests {
//     use std::fs;

//     use model::common::user::UserState;

//     #[test]
//     fn test_parse_user_page() {
//         let html = fs::read_to_string(".cache/vickscarlet.html").unwrap();

//         let result = super::parse_userpage(&html);
//         assert_eq!("神戸小鳥", result.name);
//         assert_eq!("vickscarlet", result.id);
//         assert_eq!("2016-4-14", result.join_time);
//         assert_eq!(UserState::Active, result.state);
//         println!("{:?}", result);

//         let html = fs::read_to_string(".cache/928410.html").unwrap();
//         let result = super::parse_userpage(&html);
//         assert_eq!("[已注销]", result.name);
//         assert_eq!("928410", result.id);
//         assert_eq!("2024-10-24", result.join_time);
//         assert_eq!(UserState::Dropped, result.state);
//         println!("{:?}", result);

//         let html = fs::read_to_string(".cache/1148920.html").unwrap();
//         let result = super::parse_userpage(&html);
//         assert_eq!("[已封禁]", result.name);
//         assert_eq!("1148920", result.id);
//         assert_eq!("2025-9-15", result.join_time);
//         assert_eq!(UserState::Banned, result.state);
//         println!("{:?}", result);

//         let html = fs::read_to_string(".cache/adachi9.html").unwrap();
//         let result = super::parse_userpage(&html);
//         assert_eq!("[已注销]", result.name);
//         assert_eq!("adachi9", result.id);
//         assert_eq!("2019-10-6", result.join_time);
//         assert_eq!(UserState::Abondon, result.state);
//         println!("{:?}", result);
//     }

//     #[test]
//     fn test_parse_timeline_name_history() {
//         let html = fs::read_to_string(".cache/sai_timeline_1.html").unwrap();
//         let result = super::parse_timeline_name_history(&html);
//         assert!(result.is_some());
//         let result = result.unwrap();
//         assert_eq!("2022-10-6", result.time);
//         assert!(result.names.contains("Sai🖖"));
//         assert!(result.names.contains("Sai 😊"));
//         assert!(result.names.contains("Sai😊"));
//         println!("{:?}", result);
//     }
// }
