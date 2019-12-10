use crate::opt::BuildConfig;


use chrono::prelude::*;
use serde::Serialize;

use std::convert::TryInto;

/// The front matter of every page
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(default)]
pub struct PageFrontMatter {
    /// <title> of the page
    pub title: Option<String>,
    /// Description in <meta> that appears when linked, e.g. on twitter
    pub description: Option<String>,
    /// Chrono converted datetime
    pub date: toml::value::Datetime,
    /// Whether this page is a draft
    pub draft: bool,
}

fn today() -> toml::value::Datetime {
    let n = Local::now().naive_local();
    let s = n.format("%Y-%m-%dT%X").to_string();
    s.parse().unwrap()
}

impl From<&BuildConfig> for PageFrontMatter {
    fn from(config: &BuildConfig) -> PageFrontMatter {
        let date = if let Some(ref given_date) = config.date {
            given_date.to_owned()
        } else {
            today()
        };

        PageFrontMatter {
            title: config.title.clone(),
            description: None,
            date,
            draft: config.draft,
        }
    }
}

pub struct Page {
    front_matter: PageFrontMatter,
    title: Option<String>,
    file_name: String,
}

impl Page {
    pub fn file_name(&self) -> String {
        [self.file_name.as_str() , ".md"].join("")
    }
}

impl TryInto<String> for Page {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<String, Self::Error> {
        let header = toml::to_string(&self.front_matter)?;
        if let Some(title) = self.title {
            Ok(format!("+++\n{}\n+++\n\n# {}", header, title))
        } else {
            Ok(format!("+++\n{}\n+++\n\n", header))
        }
    }
}

impl From<BuildConfig> for Page {
    fn from(config: BuildConfig) -> Page {
        let front_matter = PageFrontMatter::from(&config);
        let title = config.title.clone();
        let file_name = slug::slugify(config.title.unwrap_or_else(|| String::from("unnamed page")));

        Page {
            front_matter,
            title,
            file_name
        }
    }
}