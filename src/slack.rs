use crate::contest::{Contest, Host};
use chrono_tz::Asia::Tokyo;
use reqwest::{Client, Error};
use serde_json::json;
use std::env;

pub async fn send(contests: &[Contest]) -> Result<(), Error> {
    let url = env::var("SLACK_URL").unwrap();
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
            let start_time_str = contest
                .start_time
                .with_timezone(&Tokyo)
                .format("%m/%d (%a) %H:%M")
                .to_string();
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

    let body = json!({ "blocks": blocks, "text": "今週の競プロ" });

    let client = Client::new();
    client.post(&url).json(&body).send().await?;

    Ok(())
}