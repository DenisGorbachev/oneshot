use std::io;
use std::io::Write;
use std::path::PathBuf;

use clust::messages::{MessagesRequestBody, MessagesResponseBody};
use derive_getters::{Dissolve, Getters};
use derive_more::{Error, From};
use derive_new::new;
use fmt_derive::Display;
use fs_err::{File, OpenOptions};

use crate::types::client::ConversationWriter;

#[derive(new, Getters, Dissolve, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RealConversationWriter {
    file: File,
}

impl RealConversationWriter {}

impl TryFrom<PathBuf> for RealConversationWriter {
    type Error = io::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let file = OpenOptions::new().append(true).create(true).open(value)?;
        Ok(Self::new(file))
    }
}

impl ConversationWriter for RealConversationWriter {
    type Error = RealConversationWriterWriteError;

    fn write_request_body(
        &mut self,
        messages_request_body: &MessagesRequestBody,
    ) -> Result<(), Self::Error> {
        let yaml = serde_yaml::to_string(&vec![messages_request_body])?;
        self.file.write_all(yaml.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }

    fn write_response_body(
        &mut self,
        messages_response_body: &MessagesResponseBody,
    ) -> Result<(), Self::Error> {
        let yaml = serde_yaml::to_string(&vec![messages_response_body])?;
        self.file.write_all(yaml.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }
}

#[derive(Error, Display, From, Debug)]
pub enum RealConversationWriterWriteError {
    TheIoError(io::Error),
    TheYamlError(serde_yaml::Error),
}
