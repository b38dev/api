use scraper::{Html, Selector};

struct Selectors {
    join_time: Selector,
    id: Selector,
    name: Selector,
}

impl Selectors {
    fn new() -> Self {
        Selectors {
            join_time: Selector::parse("#user_home ul.network_service > li:first-child > span.tip")
                .unwrap(),
            id: Selector::parse("#headerProfile .nameSingle .inner .name small.grey").unwrap(),
            name: Selector::parse("#headerProfile .nameSingle .inner .name a").unwrap(),
        }
    }
}

pub struct UserParser {
    // selectors
    selectors: Selectors,
}
impl UserParser {
    pub fn new() -> Self {
        UserParser {
            selectors: Selectors::new(),
        }
    }

    pub fn parse_userpage(&self, html: &str) -> (String, String, String) {
        let document = Html::parse_document(html);
        let join_time = document
            .select(&self.selectors.join_time)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()[0]
            .replace(" 加入", "");
        let id = document
            .select(&self.selectors.id)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()[0][1..]
            .to_string();
        let name = document
            .select(&self.selectors.name)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()[0]
            .to_string();
        (join_time, id, name)
    }
}

#[cfg(test)]
mod tests {
    use super::UserParser;
    use std::fs;

    #[test]
    fn parse_test() {
        let html = fs::read_to_string("test/vickscarlet.html").unwrap();
        let parser = UserParser::new();
        let (join_time, id, name) = parser.parse_userpage(&html);
        assert_eq!("神戸小鳥", name);
        assert_eq!("vickscarlet", id);
        assert_eq!("2016-4-14", join_time);
    }
}
