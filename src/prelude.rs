use std::result as r;

#[derive(Debug)]
pub enum Error {
    General(&'static str),
}

pub type Result<Type> = r::Result<Type, Error>;
