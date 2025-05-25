use std::fmt;

pub mod notice;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    UnableConvert {
        source_event: String,
        target_event: String,
        because: String
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[rustfmt::skip]
            ErrorKind::UnableConvert { source_event, target_event, because } => {
                writeln!(f, "{source_event} cannot be converted to a {target_event}: {because}")
            }
        }
    }
}
