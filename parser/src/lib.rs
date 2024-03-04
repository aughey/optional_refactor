use std::str::FromStr;

use macros::MakeOptional;

pub struct SomethingNoncopyable {
    pub data: String,
}
impl FromStr for SomethingNoncopyable {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SomethingNoncopyable {
            data: s.to_string(),
        })
    }
}

pub struct SomethingElse {
    pub data: String,
}
impl FromStr for SomethingElse {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SomethingElse {
            data: s.to_string(),
        })
    }
}

#[derive(MakeOptional)]
pub struct Contents {
    pub something: SomethingNoncopyable,
    pub somethingelse: SomethingElse,
}

#[derive(Default)]
struct ManualOptionalContents {
    something: Option<SomethingNoncopyable>,
    somethingelse: Option<SomethingElse>,
}
impl TryInto<Contents> for ManualOptionalContents {
    type Error = String;

    fn try_into(self) -> Result<Contents, Self::Error> {
        Ok(Contents {
            something: self.something.ok_or_else(|| "Missing something")?,
            somethingelse: self.somethingelse.ok_or_else(|| "Missing somethingelse")?,
        })
    }
}
impl ManualOptionalContents {
    fn set_key_value(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "something" => {
                self.something = Some(SomethingNoncopyable {
                    data: value.to_string(),
                });
            }
            "somethingelse" => {
                self.somethingelse = Some(SomethingElse {
                    data: value.to_string(),
                });
            }
            _ => return Err(format!("Invalid key: {}", key)),
        }
        Ok(())
    }
}

pub fn parse_content(content: &str) -> Result<Contents, String> {
    let lines = content.lines().filter(|line| !line.is_empty());

    let mut something: Option<SomethingNoncopyable> = None;
    let mut somethingelse: Option<SomethingElse> = None;

    for line in lines {
        let (key, value) = line.split_at(line.find(":").ok_or("Invalid line missing colon")?);
        match key {
            "something" => {
                something = Some(SomethingNoncopyable {
                    data: value.to_string(),
                });
            }
            "somethingelse" => {
                somethingelse = Some(SomethingElse {
                    data: value.to_string(),
                });
            }
            _ => return Err(format!("Invalid key: {}", key)),
        }

        // if let Some(something) = something {
        //     if let Some(somethingelse) = somethingelse {
        //         return Ok(Contents {
        //             something,
        //             somethingelse,
        //         });
        //     }
        // }

        // match (something, somethingelse) {
        //     (Some(something), Some(somethingelse)) => {
        //         return Ok(Contents {
        //             something: something,
        //             somethingelse: somethingelse,
        //         });
        //     }
        //     _ => {}
        // }
    }
    Ok(Contents {
        something: something.ok_or_else(|| "Missing something")?,
        somethingelse: somethingelse.ok_or_else(|| "Missing somethingelse")?,
    })
}

pub fn parse_content_better(content: &str) -> Result<Contents, String> {
    let lines = content.lines().filter(|line| !line.is_empty());

    let mut result = ManualOptionalContents::default();

    for line in lines {
        let (key, value) = line.split_at(line.find(":").ok_or("Invalid line missing colon")?);
        match key {
            "something" => {
                result.something = Some(SomethingNoncopyable {
                    data: value.to_string(),
                });
            }
            "somethingelse" => {
                result.somethingelse = Some(SomethingElse {
                    data: value.to_string(),
                });
            }
            _ => return Err(format!("Invalid key: {}", key)),
        }
    }
    result.try_into()
}

pub fn parse_content_event_better(content: &str) -> Result<Contents, String> {
    let lines = content.lines().filter(|line| !line.is_empty());

    let mut result = ManualOptionalContents::default();

    for line in lines {
        let (key, value) = key_value(line)?;
        _ = result.set_key_value(key, value); // throw away any error, it's ok
    }
    result.try_into()
}

fn key_value(line: &str) -> Result<(&str, &str), &str> {
    let mut parts = line.split(":");
    Ok((
        parts.next().ok_or("Invalid line missing key")?.trim(),
        parts.next().ok_or("Invalid line missing value")?.trim(),
    ))
}

pub fn parse_content_best(content: &str) -> Result<Contents, &str> {
    let lines = content.lines().filter(|line| !line.is_empty());

    let mut result = OptionalContents::default();

    for line in lines {
        let (key, value) = key_value(line)?;
        _ = result.set_key_value(key, value); // throw away any error, it's ok
    }
    result.try_into()
}

#[test]
fn test_best() {
    let content = r"something: data1
somethingelse: data2";

    let result = parse_content_best(content);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.something.data, "data1");
    assert_eq!(result.somethingelse.data, "data2");

    // override
    let content = r"something: data1
somethingelse: data2
somethingelse: data3";
    let result = parse_content_best(content);
    assert_eq!(result.unwrap().somethingelse.data, "data3");

    // missing something
    let content = r"somethingelse: data2";
    let result = parse_content_best(content);
    assert!(result.is_err());

    // something somethingelse
    let content = r"something: data1";
    let result = parse_content_best(content);
    assert!(result.is_err());
}
