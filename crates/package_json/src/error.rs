use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("npm alias parser error: {version}")]
    NpmAliasParserError { version: String },
}
