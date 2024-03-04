pub struct SomethingNoncopyable {
    pub data: String,
}

pub struct Contents {
    pub something: SomethingNoncopyable,
    pub somethingelse: SomethingNoncopyable,
}

pub fn parse_content(content: &str) -> Result<Contents, String> {
    let lines = content.lines().filter(|line| !line.is_empty());

    // The values of our struct that we will accumulate while parsing
    let mut something: Option<SomethingNoncopyable> = None;
    let mut somethingelse: Option<SomethingNoncopyable> = None;

    for line in lines {
        // Extract and match against our known keys
        let (key, value) = key_value(line)?;
        match key {
            "something" => {
                something = Some(SomethingNoncopyable {
                    data: value.to_string(),
                });
            }
            "somethingelse" => {
                somethingelse = Some(SomethingNoncopyable {
                    data: value.to_string(),
                });
            }
            _ => return Err(format!("Invalid key: {}", key)),
        }

        // Early stop, if we have both values
        match (something, somethingelse) {
            (Some(something), Some(somethingelse)) => {
                return Ok(Contents {
                    something: something,
                    somethingelse: somethingelse,
                });
            }
            _ => {}
        }
    }
    Err("Missing something or somethingelse".to_string())
}

fn key_value(line: &str) -> Result<(&str, &str), &str> {
    let mut parts = line.split(":");
    Ok((
        parts.next().ok_or("Invalid line missing key")?.trim(),
        parts.next().ok_or("Invalid line missing value")?.trim(),
    ))
}
