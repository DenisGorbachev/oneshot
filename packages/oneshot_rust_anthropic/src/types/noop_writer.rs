use clust::messages::{MessagesRequestBody, MessagesResponseBody};
use derive_getters::{Dissolve, Getters};
use derive_new::new;

use crate::types::client::ConversationWriter;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NoopConversationWriter {}

impl NoopConversationWriter {}

impl ConversationWriter for NoopConversationWriter {
    type Error = ();

    fn write_request_body(&mut self, _messages_request_body: &MessagesRequestBody) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write_response_body(&mut self, _messages_response_body: &MessagesResponseBody) -> Result<(), Self::Error> {
        Ok(())
    }
}
