use crate::error::{BotifactoryError, Result};

pub enum Identifier {
    Name(String),
    Id(i64),
}
