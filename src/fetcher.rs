use chrono::{TimeZone, Utc};
use chrono_tz::Asia::Tokyo;
use scraper::{Html, Selector};

use crate::contest::{Contest, Host};

pub async fn fetch_upcoming_contests() -> Vec<Contest> {
    let mut contests = Vec::new();

    contests.append(&mut fetch_atcoder_contests().await);
    contests.append(&mut fetch_cf_contests().await);
    contests.retain(|contest| {
        contest.start_time > Utc::now()
            && contest.start_time < Utc::now() + chrono::Duration::weeks(1)
    });

    contests
}

async fn fetch_atcoder_contests() -> Vec<Contest> {
    let mut contests = Vec::new();

    let url = "https://atcoder.jp/home?lang=ja";

    let contest_table_selector = "#contest-table-upcoming > div > table > tbody > tr";
    let date_selector = "td:nth-child(1) > small > a > time";
    let name_selector = "td:nth-child(2) > small > a";

    let response = reqwest::get(url).await.unwrap().text().await.unwrap();

    let document = Html::parse_document(&response);
    let selector = Selector::parse(contest_table_selector).unwrap();

    for element in document.select(&selector) {
        let name = element
            .select(&Selector::parse(name_selector).unwrap())
            .next()
            .unwrap()
            .inner_html();
        let start_time_str = element
            .select(&Selector::parse(date_selector).unwrap())
            .next()
            .unwrap()
            .inner_html();
        let path = element
            .select(&Selector::parse(name_selector).unwrap())
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .to_string();

        let start_time = Tokyo
            .datetime_from_str(
                &start_time_str
                    .chars()
                    .take(start_time_str.len() - 5)
                    .collect::<String>(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap();

        contests.push(Contest::new(
            name,
            start_time.with_timezone(&Utc),
            Some(format!("https://atcoder.jp{}", path)),
            Host::AtCoder,
        ));
    }

    contests.sort_by_key(|contest| contest.start_time);
    contests
}

async fn fetch_cf_contests() -> Vec<Contest> {
    let mut contests = Vec::new();

    let url = "https://codeforces.com/api/contest.list?gym=false";
    let response = reqwest::get(url).await.unwrap().text().await.unwrap();

    let json: serde_json::Value = serde_json::from_str(&response).unwrap();
    let contests_json = json["result"].as_array().unwrap();

    for contest_json in contests_json {
        let name = contest_json["name"].as_str().unwrap().to_string();
        let start_time_seconds = contest_json["startTimeSeconds"].as_i64().unwrap();

        let start_time = Utc.timestamp_opt(start_time_seconds, 0).unwrap();

        contests.push(Contest::new(name, start_time, None, Host::Codeforces));
    }

    contests.sort_by_key(|contest| contest.start_time);
    contests
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_atcoder_contests() {
        let contests = fetch_atcoder_contests().await;
        assert!(contests.iter().all(|contest| contest.host == Host::AtCoder));
    }

    #[tokio::test]
    async fn test_fetch_cf_contests() {
        let contests = fetch_cf_contests().await;
        assert!(contests
            .iter()
            .all(|contest| contest.host == Host::Codeforces));
    }

    #[tokio::test]
    async fn test_fetch_upcoming_contests() {
        let contests = fetch_upcoming_contests().await;
        assert!(contests
            .iter()
            .all(|contest| contest.start_time > Utc::now()
                && contest.start_time < Utc::now() + chrono::Duration::weeks(1)));
    }
}
