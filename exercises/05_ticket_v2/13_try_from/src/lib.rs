use std::fmt::Debug;

#[derive(Debug)]
enum StatusError {
    StatusConversionError(String),
}

/* Alternative solution (complete):
#[derive(Debug, thiserror::Error)]
#[error("{invalid_status} is not a valid status")]
struct ParseStatusError {
    invalid_status: String,
}

And you use like:
Err(ParseStatusError {
    invalid_status: value.to_string(),
})
*/

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<&str> for Status {
    type Error = StatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized_value = value.to_ascii_lowercase();
        match normalized_value.as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(Self::Error::StatusConversionError(
                format!("'{value}' is not a valid Status")
            )),
        }
    }
}

impl TryFrom<String> for Status {
    type Error = StatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Status::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
