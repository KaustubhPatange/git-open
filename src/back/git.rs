use ini::{Ini};
use crate::NULL;

pub struct Git {
    pub url: String
}

impl Git {
    pub fn parse() -> Result<Git, String> {
        let conf = Ini::load_from_file(".git/config").unwrap();
        let mut origin = conf.section(Some("remote \"origin\""));
        if origin == None {
            origin = conf.section(Some("remote \"master\""))
        }
        if origin == None {
            origin = conf.section(Some("remote \"main\""))
        }

        return match origin {
            None => {
                Err("Git config doesn't contain definition for origin".to_owned())
            }
            Some(sec) => {
                let url = sec.get("url").unwrap_or_else(|| NULL);
                if url == NULL {
                    Err("Couldn't find github config url".to_owned())
                } else {
                    Ok(
                        Git { url: url.to_owned() }
                    )
                }
            }
        }
    }
}