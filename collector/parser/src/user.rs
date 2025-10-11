use std::collections::HashSet;
use std::vec;

use chrono::{Months, Utc};
use visdom::Vis;
use visdom::types::Elements;

use super::common;
use model::{
    common::user::NamesUpdate,
    prelude::{Collections, InitUser, SubjectType, TypedCollection, Uid, UserState},
};

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

pub fn parse_userpage(html: &str, init: Option<InitUser>) -> anyhow::Result<InitUser> {
    let document = Vis::load(html).map_err(|e| anyhow::anyhow!("Failed to load HTML: {}", e))?;
    let message = document.find(".message>h2").text();
    if message.eq("呜咕，出错了") {
        tracing::warn!("User not found");
        return Err(anyhow::anyhow!("User not found"));
    }
    let mut init = init.unwrap_or_else(InitUser::default);
    let join_time = document
        .find("#user_home ul.network_service > li:first-child > span.tip")
        .text()
        .replace(" 加入", "");
    init.set_join_time(common::parse_time(&join_time).ok());
    let name_single_element = document.find("#headerProfile .nameSingle");
    let name_element = name_single_element.find(".inner .name");
    if init.sid.is_none() {
        let uid = name_element.find("small.grey").text();
        if !uid.starts_with("@") {
            return Err(anyhow::anyhow!("Failed to parse user id"));
        }
        init.update_uid(Uid::from_str(&uid[1..]));
    }
    if init.nid.is_none() {
        let pm_href = name_single_element
            .find(".inner .actions a.chiiBtn:last-of-type")
            .attr("href");
        if let Some(href) = pm_href {
            let href = href.to_string();
            if href.starts_with("/pm/compose/") {
                let nid = href
                    .trim_start_matches("/pm/compose/")
                    .trim_end_matches(".chii");
                init.set_nid(nid.parse().ok());
            }
        }
    }
    if init.nid.is_none() {
        let avatar_element = name_single_element
            .find(".headerAvatar .avatar span.avatarNeue")
            .attr("style");
        if let Some(avatar_style) = avatar_element {
            let avatar_style = avatar_style.to_string();
            if avatar_style.starts_with("background-image:url('//lain.bgm.tv/pic/user/") {
                let nid = avatar_style.split("/").last().unwrap();
                let nid = nid.split(".").next().unwrap();
                let nid = nid.split("_").next().unwrap();
                init.set_nid(nid.parse().ok());
            }
        }
    }

    let name = name_element.find("a").text();
    init.update_name(name.clone());
    let timeline = document
        .find("#pinnedLayout ul.timeline > li small.time")
        .map(|_, element| element.text());
    if timeline.len() > 0 {
        init.set_last_active(common::parse_time(&timeline[0]).ok());
        let list = vec!["anime", "game", "book", "music", "real"];
        let list = list.iter().map(|t| {
            let st = t.parse::<SubjectType>().unwrap();
            let collection = parse_collection(&document.find(&format!("#{t}")));
            (st, collection)
        });

        init.set_collections(Collections::build(list.collect()));
    }

    let tip_intro = document.find("#main .tipIntro .inner");
    let is_banned = tip_intro.find("h3").text().eq("用户已封禁");
    if is_banned && !tip_intro.find(".tip").text().contains("解封") {
        init.update_state(UserState::Banned);
        return Ok(init.to_owned());
    }

    let Some(la) = init.last_active else {
        init.update_state(if name.eq("[已封禁]") {
            UserState::Banned
        } else {
            UserState::Dropped
        });
        return Ok(init.to_owned());
    };

    init.update_state(
        if la > Utc::now() - Months::new(config::get().collector.user.active_month) {
            UserState::Active
        } else {
            UserState::Abondon
        },
    );

    Ok(init.to_owned())
}

pub fn parse_timeline_name_history(html: &str) -> anyhow::Result<Option<NamesUpdate>> {
    let document = Vis::load(html).map_err(|e| anyhow::anyhow!("Failed to load HTML: {}", e))?;
    let timeline = document.find("#timeline");
    if timeline.length() < 1 {
        return Ok(None);
    }
    let key_point = timeline.find("h4.Header").first().text();
    let key_point = common::parse_time(&key_point)?;
    let names = timeline
        .find("li.tml_item > span > p.status:has(strong) > strong")
        .map(|_, e| e.text())
        .into_iter()
        .collect::<HashSet<String>>();

    Ok(Some(NamesUpdate { key_point, names }))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use model::common::user::UserState;

    #[test]
    fn test_parse_user_page() {
        let html = fs::read_to_string(".cache/vickscarlet.html").unwrap();

        let Ok(init) = super::parse_userpage(&html, None) else {
            panic!("Failed to parse user page");
        };
        assert_eq!("神戸小鳥", init.name);
        assert_eq!("vickscarlet", init.sid.unwrap());
        assert_eq!(
            "2016-4-14",
            init.join_time
                .unwrap()
                .with_timezone(crate::common::tz())
                .format("%Y-%-m-%-d")
                .to_string()
        );
        assert_eq!(UserState::Active, init.state);

        // let html = fs::read_to_string(".cache/928410.html").unwrap();
        // let result = super::parse_userpage(&html);
        // assert_eq!("[已注销]", result.name);
        // assert_eq!("928410", result.id);
        // assert_eq!("2024-10-24", result.join_time);
        // assert_eq!(UserState::Dropped, result.state);
        // println!("{:?}", result);

        // let html = fs::read_to_string(".cache/1148920.html").unwrap();
        // let result = super::parse_userpage(&html);
        // assert_eq!("[已封禁]", result.name);
        // assert_eq!("1148920", result.id);
        // assert_eq!("2025-9-15", result.join_time);
        // assert_eq!(UserState::Banned, result.state);
        // println!("{:?}", result);

        // let html = fs::read_to_string(".cache/adachi9.html").unwrap();
        // let result = super::parse_userpage(&html);
        // assert_eq!("[已注销]", result.name);
        // assert_eq!("adachi9", result.id);
        // assert_eq!("2019-10-6", result.join_time);
        // assert_eq!(UserState::Abondon, result.state);
        // println!("{:?}", result);
    }

    // #[test]
    // fn test_parse_timeline_name_history() {
    //     let html = fs::read_to_string(".cache/sai_timeline_1.html").unwrap();
    //     let result = super::parse_timeline_name_history(&html);
    //     assert!(result.is_some());
    //     let result = result.unwrap();
    //     assert_eq!("2022-10-6", result.time);
    //     assert!(result.names.contains("Sai🖖"));
    //     assert!(result.names.contains("Sai 😊"));
    //     assert!(result.names.contains("Sai😊"));
    //     println!("{:?}", result);
    // }
}
