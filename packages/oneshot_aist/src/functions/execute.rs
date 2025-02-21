use crate::{to_chat_completion_request_message_chat_completion_response_message, try_into_content_iter, try_into_content_iter_from_messages, user_text, ConfigLike, Outcome, TryIntoContentError, ValidateV1};
use async_openai::error::OpenAIError;
use async_openai::types::{ChatChoice, ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse};
use async_openai::Client;
use derive_more::{Display, Error, From, Into};
use derive_new::new;
use itertools::Itertools;
use std::error::Error;
use std::ops::{ControlFlow, Deref};
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use syn::File;
use syn_more::SynFrom;
use tokio::task::JoinSet;

pub fn execute_v1(path: &Path, _command: Command) -> Outcome {
    let _file = File::syn_from(path)?;
    todo!()
}

/// Parallel execution is implemented separately
/// Use `args.n()` to receive multiple completions in a single response
///
/// TODO: Implement forking for each ChatChoice
pub async fn execute_v2<Conf: ConfigLike, Err: Error, Validator: ValidateV1<String, Error = Err>, Test: FnMut(&str) -> Vec<Err>>(input: String, validator: &Validator, client: &Client<Conf>, args: &CreateChatCompletionRequestArgs, mut gas: u32) -> Result<String, ExecuteV2Error> {
    use ExecuteV2Error::*;
    let errors = validator.validate_v1(&input);
    if errors.is_empty() {
        return Ok(input);
    }
    if gas == 0 {
        return Err(OutOfGas);
    }
    gas -= 1;
    // TODO: This is only to silence the warning
    let _gas = gas;
    let mut messages = vec![user_text(input)];
    messages.extend(errors.iter().map(|e| user_text(e.to_string())));
    let request = args.clone().messages(messages).build().map_err(Client)?;
    let iter = try_into_content_iter(client, request)
        .await
        .map_err(Client)?;
    let _results = iter.collect_vec();
    todo!()
}

/// Note: this type doesn't implement many useful traits because [`TryIntoSolutionIterWithGasError`] doesn't implement them
pub type Out<Input> = Result<Vec<Result<Input, TryIntoContentError>>, TryIntoSolutionIterWithGasError>;

/// Note: this type doesn't implement many useful traits because [`Out`] doesn't implement them
#[derive(new, From, Into, Debug)]
pub struct Trace<Input, Problem> {
    pub input: Input,
    pub problems: Vec<Problem>,
    pub out: Option<Out<Input>>,
}

// impl<Input, Problem> Trace<Input, Problem> {
//     pub async fn from_input_problem(input: Input, problem: Problem) -> Self {}
// }

pub type Traces<Input, Problem> = Vec<Trace<Input, Problem>>;

pub async fn execute_v3<Config, Candidate, Problem, Choices, Validate>(choices: Choices, mut validate: Validate, client: Arc<Client<Config>>, args: Arc<CreateChatCompletionRequestArgs>) -> Result<ChatChoice, JoinSet<Result<CreateChatCompletionResponse, OpenAIError>>>
where
    Config: ConfigLike + Send + Sync + 'static,
    Choices: Iterator<Item = ChatChoice>,
    Validate: FnMut(&ChatChoice) -> ControlFlow<(), Vec<ChatCompletionRequestMessage>>,
{
    // TODO: Currently this function continues checking trace one-by-one, so if a solution_iter contains a single successful trace at the end, this function will continue trying unsuccessful traces
    // TODO: Currently this function doesn't check traces in parallel
    use ControlFlow::*;

    let mut branches = vec![];
    for choice in choices {
        match validate(&choice) {
            Continue(mut messages) => {
                messages.insert(0, to_chat_completion_request_message_chat_completion_response_message(choice.message));
                branches.push(messages)
            }
            Break(()) => {
                return Ok(choice);
            }
        }
    }
    let join_set = branches
        .into_iter()
        .map(move |messages| {
            // TODO: Optimize with async closures?
            let client_clone = client.clone();
            let args_clone = args.clone();
            async move {
                let request = args_clone.deref().clone().messages(messages).build()?;
                client_clone.chat().create(request).await
            }
        })
        .collect();
    Err(join_set)
    // let mut traces: Traces<StringInput, Problem> = vec![];
    // let mut traces_index = 0;
    // while gas != 0 {
    //     gas -= 1;
    //     match candidates_batches.get(candidates_batches_index) {
    //         None => return Err(ExecuteV3Error::new(OutOfCandidates, traces)),
    //         Some(candidates) => {

    //             match traces.get_mut(traces_index) {
    //                 None => {
    //                     todo!()
    //                 }
    //                 Some(trace) => {
    //                     let result = try_into_content_iter_from_input_and_problems_with_gas(trace.input.clone(), trace.problems.iter(), gas, client, args)
    //                         .await
    //                         .map(|iter| {
    //                             iter.map(|res| {
    //                                 // TODO: Rewrite to use a generic Input instead of String
    //                                 match res {
    //                                     Ok(solution) => {
    //                                         let output = StringInput::new(solution);
    //                                         let problems = validator.validate_v1(&output);
    //                                         traces.push(Trace::new(output.clone(), problems, None));
    //                                         Ok(output)
    //                                     }
    //                                     Err(err) => Err(err),
    //                                 }
    //                             })
    //                             .collect_vec()
    //                         });
    //                     trace.out = Some(result);
    //                     traces_index += 1;
    //                 }
    //             }
    //         }
    //     }
    //     candidates_batches_index += 1;
    // }
    // Err(ExecuteV3Error::new(OutOfGas, traces))
}

pub fn check_candidates<Candidate, Problem, Validator>(candidates: impl Iterator<Item = Candidate>, mut validate: impl FnMut(&Candidate) -> Vec<Problem> + 'static) -> impl Iterator<Item = Result<Candidate, (Candidate, Vec<Problem>)>> {
    candidates.map(move |candidate| {
        let problems = validate(&candidate);
        if problems.is_empty() {
            Ok(candidate)
        } else {
            Err((candidate, problems))
        }
    })
}

pub async fn try_into_content_iter_from_input_and_problems_with_gas<Config, Input, Problem>(input: Input, problems: impl Iterator<Item = Problem>, gas: u32, client: &Client<Config>, args: &CreateChatCompletionRequestArgs) -> Result<impl Iterator<Item = Result<String, TryIntoContentError>>, TryIntoSolutionIterWithGasError>
where
    Config: ConfigLike,
    Input: Into<ChatCompletionRequestMessage>,
    Problem: Into<ChatCompletionRequestMessage>,
{
    use TryIntoSolutionIterWithGasError::*;
    if gas == 0 {
        Err(OutOfGas)
    } else {
        let messages = messages_from_input_problems(input, problems);
        let solutions = try_into_content_iter_from_messages(messages, client, args)
            .await
            .map_err(Client)?;
        Ok(solutions)
    }
}

// NOTE: it might be better to append the candidate as the assistant message, and the problems as user message
pub fn messages_from_input_problems<Input, Problem>(input: Input, problems: impl Iterator<Item = Problem>) -> Vec<ChatCompletionRequestMessage>
where
    Input: Into<ChatCompletionRequestMessage>,
    Problem: Into<ChatCompletionRequestMessage>,
{
    let mut messages = vec![input.into()];
    messages.extend(problems.map(Into::into));
    messages
}

pub enum ExecuteV2Error {
    Client(OpenAIError),
    /// The function has reached the maximum count of steps
    OutOfGas,
}

#[derive(new, Into, Debug)]
pub struct ExecuteV3Error<Input, Problem> {
    pub reason: ExecuteV3ErrorReason,
    pub traces: Traces<Input, Problem>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ExecuteV3ErrorReason {
    OutOfGas,
    OutOfCandidates,
}

/// Note: this type doesn't implement many useful traits because [`OpenAIError`] doesn't implement them
#[derive(Error, Display, From, Debug)]
pub enum TryIntoSolutionIterWithGasError {
    OutOfGas,
    Client(OpenAIError),
}
