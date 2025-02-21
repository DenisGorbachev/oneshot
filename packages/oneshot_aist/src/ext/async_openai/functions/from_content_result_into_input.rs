use crate::TryIntoContentError;
use derive_more::{Display, Error};
use std::error::Error;

/// This function uses `TryFrom<String>` instead of `FromStr` because it allows to save on allocation when `Input = String`
pub fn from_content_result_into_input<InputTryFromStringError: Error, Input: TryFrom<String, Error = InputTryFromStringError>>(result: Result<String, TryIntoContentError>) -> Result<Input, FromContentResultIntoInputError<InputTryFromStringError>> {
    use FromContentResultIntoInputError::*;
    match result {
        Ok(string) => Input::try_from(string).map_err(TryFromString),
        Err(error) => Err(TryIntoContent(error)),
    }
}

#[derive(Error, Display, Clone, Debug)]
pub enum FromContentResultIntoInputError<TryFromStringError: Error> {
    TryFromString(TryFromStringError),
    TryIntoContent(TryIntoContentError),
}
