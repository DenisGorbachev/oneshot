use clust::messages::{MessagesError, MessagesRequestBody, MessagesResponseBody};
use derive_more::{Deref, DerefMut, Error, From, Into};
use derive_new::new;
use fmt_derive::Display;

/// This is experimental code, do not use it
#[derive(new, Deref, DerefMut, Into, Clone, Debug)]
pub struct Client<Writer>
where
    Writer: ConversationWriter,
{
    #[deref]
    #[deref_mut]
    inner: clust::Client,
    writer: Writer,
}

pub trait ConversationWriter {
    type Error;

    fn write_request_body(&mut self, messages_request_body: &MessagesRequestBody) -> Result<(), Self::Error>;

    fn write_response_body(&mut self, messages_response_body: &MessagesResponseBody) -> Result<(), Self::Error>;
}

impl<Writer, ConversationWriterError> Client<Writer>
where
    Writer: ConversationWriter<Error = ConversationWriterError>,
    ConversationWriterError: Error + 'static,
{
    pub async fn create_a_message(&mut self, request_body: MessagesRequestBody) -> Result<MessagesResponseBody, CreateMessageError> {
        self.writer
            .write_request_body(&request_body)
            .map_err(|err| CreateMessageError::TheBoxDynError(Box::new(err)))?;
        let response_body = self.inner.create_a_message(request_body).await?;
        self.writer
            .write_response_body(&response_body)
            .map_err(|err| CreateMessageError::TheBoxDynError(Box::new(err)))?;
        Ok(response_body)
    }
}

#[derive(Error, Display, From, Debug)]
pub enum CreateMessageError {
    TheMessagesError(MessagesError),
    TheBoxDynError(Box<dyn Error>),
}
