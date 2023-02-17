use crate::contest::{Contest, Host};
use chrono_tz::Asia::Tokyo;
use reqwest::{Client, Error};
use serde_json::{json, Value};
use std::env;

pub async fn send(contests: &[Contest]) -> Result<(), Error> {
    let url = env::var("SLACK_URL").unwrap();

    let body = message_body(contests);

    let client = Client::new();
    client.post(&url).json(&body).send().await?;

    Ok(())
}

fn message_body(contests: &[Contest]) -> Value {
    let mut blocks = vec![
        json!({
            "type": "header",
            "text": {
                "type": "plain_text",
                "text": ":deployparrot: 今週の競プロ :deployparrot:"
            }
        }),
        json!({
            "type": "divider"
        }),
    ];

    for &host in vec![
        Host::AtCoder,
        Host::Codeforces,
        Host::Yukicoder,
        Host::Topcoder,
    ]
    .iter()
    {
        let filtered_contests = contests.iter().filter(|contest| contest.host == host);
        if filtered_contests.clone().count() == 0 {
            continue;
        }

        blocks.push(json!({
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": format!("*{}*", host)
            }
        }));

        for contest in filtered_contests {
            let mut start_time_str = contest
                .start_time
                .with_timezone(&Tokyo)
                .format("%m/%d (%a) %H:%M")
                .to_string();

            for &(from, to) in vec![
                ("Mon", "月"),
                ("Tue", "火"),
                ("Wed", "水"),
                ("Thu", "木"),
                ("Fri", "金"),
                ("Sat", "土"),
                ("Sun", "日"),
            ]
            .iter()
            {
                start_time_str = start_time_str.replace(from, to);
            }

            blocks.push(json!({
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": match &contest.url {
                        Some(url) => format!("<{}|{}>\n{} 開始", url, contest.name, start_time_str),
                        None => format!("{} {} 開始", contest.name, start_time_str)
                    }
                }
            }));
        }
        blocks.push(json!({
            "type": "divider"
        }));
    }

    json!({ "blocks": blocks, "text": "今週の競プロ" })
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;

    #[test]
    fn bench_message_body() {
        let contests = (1..100)
            .map(|i| {
                Contest::new(
                    format!("Contest {}", i),
                    Utc::now() + Duration::days(i),
                    Some(format!("https://atcoder.jp/contests/abc{}", i)),
                    if i % 2 == 0 {
                        Host::AtCoder
                    } else {
                        Host::Codeforces
                    },
                )
            })
            .collect::<Vec<_>>();

        let body = message_body(&contests);
        assert!(body["text"].as_str().unwrap().contains("今週の競プロ"));
    }
}
