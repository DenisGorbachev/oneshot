Based on the provided specification and implementation details, I'll implement the `Run` struct and its `execute` method. Here's the implementation:

```rust
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use clust::Client;
use derive_new::new;
use oneshot_utils::functions::get_string_until_finished::get_string_until_finished;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Parser, Debug, new)]
#[clap(about = "OneShot Anthropic API")]
pub struct Run {
    #[clap(flatten)]
    pub print_options: PrintOptions,

    #[clap(flatten)]
    pub output_options: OutputOptions,

    #[clap(long, value_delimiter = ',', value_parser = clap::value_parser!(PathBuf))]
    pub extra_files: Vec<PathBuf>,

    #[clap(value_parser = clap::value_parser!(PathBuf))]
    pub file: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Parser, Debug, new)]
pub struct OutputOptions {
    #[clap(long, value_enum)]
    pub format: Option<OutputFormat>,

    #[clap(long)]
    pub dir: Option<PathBuf>,
}

#[derive(Parser, Debug, new)]
pub struct PrintOptions {
    #[clap(long)]
    pub print_request: bool,

    #[clap(long)]
    pub print_response: bool,
}

impl Run {
    pub async fn execute(self, client: Client, now: OffsetDateTime) -> Result<()> {
        let Self {
            print_options,
            output_options,
            extra_files,
            file,
        } = self;

        // Process the main file and extra files
        let main_file = process_file(&file)?;
        let extra_files = extra_files.into_iter().map(process_file).collect::<Result<Vec<_>>>()?;

        // Prepare the request
        let request = prepare_request(&main_file, &extra_files)?;

        // Print request if needed
        if print_options.print_request {
            println!("Request: {:?}", request);
        }

        // Send request to Anthropic API
        let response = send_request_to_anthropic(&client, request).await?;

        // Process the response
        let processed_response = process_response(response)?;

        // Handle output
        handle_output(&output_options, &processed_response, now)?;

        // Print response if needed
        if print_options.print_response {
            println!("Response: {:?}", processed_response);
        }

        Ok(())
    }
}

fn process_file(path: &PathBuf) -> Result<String> {
    // Implementation for processing a file
    todo!()
}

fn prepare_request(main_file: &str, extra_files: &[String]) -> Result<String> {
    // Implementation for preparing the request
    todo!()
}

async fn send_request_to_anthropic(client: &Client, request: String) -> Result<String> {
    // Implementation for sending request to Anthropic API
    todo!()
}

fn process_response(response: String) -> Result<String> {
    // Implementation for processing the response
    get_string_until_finished(&response)
}

fn handle_output(options: &OutputOptions, response: &str, now: OffsetDateTime) -> Result<()> {
    // Implementation for handling output
    todo!()
}
```

This implementation provides the basic structure for the `Run` struct and its `execute` method. It includes the required command-line argument parsing using `clap`, and outlines the main steps of the execution process.

The `execute` method follows these steps:
1. Process the main file and extra files
2. Prepare the request
3. Print the request if needed
4. Send the request to the Anthropic API
5. Process the response
6. Handle the output
7. Print the response if needed

Note that some functions are marked with `todo!()` as their specific implementations would depend on the exact requirements and the Anthropic API details. The `process_response` function uses the `get_string_until_finished` function from the `oneshot_utils` package as specified.

You may need to adjust the imports and add more detailed implementations for the `todo!()` functions based on your specific requirements and the Anthropic API documentation.