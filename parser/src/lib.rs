pub struct SomethingNoncopyable {
    pub data: String,
}

pub struct SomethingElse {
    pub data: String,
}

pub struct Contents {
    pub something: SomethingNoncopyable,
    pub somethingelse: SomethingElse,
}

#[derive(Default)]
struct OptionalContents {
    something: Option<SomethingNoncopyable>,
    somethingelse: Option<SomethingElse>,
}
impl TryInto<Contents> for OptionalContents {
    type Error = String;

    fn try_into(self) -> Result<Contents, Self::Error> {
        Ok(Contents {
            something: self.something.ok_or_else(|| "Missing something")?,
            somethingelse: self.somethingelse.ok_or_else(|| "Missing somethingelse")?,
        })
    }
}
impl OptionalContents {
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

    let mut result = OptionalContents::default();

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

    let mut result = OptionalContents::default();

    for line in lines {
        let (key, value) = line.split_at(line.find(":").ok_or("Invalid line missing colon")?);
        _ = result.set_key_value(key, value); // throw away any error, it's ok
    }
    result.try_into()
}
