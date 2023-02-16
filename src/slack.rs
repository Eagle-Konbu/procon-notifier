use crate::contest::{Contest, Host};
use reqwest::{Client, Error};
use std::env;
use serde_json::json;
use chrono_tz::Asia::Tokyo;

async fn send(contests: &[Contest]) -> Result<(), Error> {
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
        })
    ];

    for &host in vec![Host::AtCoder, Host::Codeforces, Host::Yukicoder, Host::Topcoder].iter() {
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
            let start_time_str = contest.start_time.with_timezone(&Tokyo).format("%m/%d (%a) %H:%M").to_string();
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
    }

    let body = json!({
        "blocks": blocks
    });

    let client = Client::new();
    client.post(&url).json(&body).send().await?;

    Ok(())
}

// Body Format
//{
//    "blocks": [
//        {
//            "type": "header",
//            "text": {
//                "type": "plain_text",
//                "text": ":deployparrot: 今週の競プロ :deployparrot:"
//            }
//        },
//        {
//            "type": "divider"
//        },
//        {
//            "type": "section",
//            "text": {
//                "type": "mrkdwn",
//                "text": "*AtCoder*"
//            }
//        },
//        {
//            "type": "section",
//            "text": {
//                "type": "mrkdwn",
//                "text": "<https://atcoder.jp/contests/agc061|AGC061>\n01/02 (日) 21:00 開始"
//            }
//        },
//        {
//            "type": "section",
//            "text": {
//                "type": "mrkdwn",
//                "text": "<https://atcoder.jp/contests/agc061|AGC061>\n01/02 (日) 21:00 開始"
//            }
//        },
//        {
//            "type": "divider"
//        },
//        {
//            "type": "section",
//            "text": {
//                "type": "mrkdwn",
//                "text": "*Codeforces*"
//            }
//        },
//        {
//            "type": "section",
//            "text": {
//                "type": "mrkdwn",
//                "text": "<https://codeforces.com/contestRegistrants/1793|Codeforces Round #852 (Div. 2)>\n01/02 (日) 21:00 開始"
//            }
//        },
//        {
//            "type": "section",
//            "text": {
//                "type": "mrkdwn",
//                "text": "Educational Codeforces Round 143 (Rated for Div. 2)\n01/02 (日) 21:00 開始"
//            }
//        }
//    ]
//}
