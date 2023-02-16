use std::fmt;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Contest {
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub url: Option<String>,
    pub host: Host,
}

impl Contest {
    pub fn new(name: String, start_time: DateTime<Utc>, url: Option<String>, host: Host) -> Self {
        Self {
            name,
            start_time,
            url,
            host,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Host {
    AtCoder,
    Codeforces,
    Yukicoder,
    Topcoder,
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Host::AtCoder => write!(f, "AtCoder"),
            Host::Codeforces => write!(f, "Codeforces"),
            Host::Yukicoder => write!(f, "Yukicoder"),
            Host::Topcoder => write!(f, "Topcoder"),
        }
    }
}
