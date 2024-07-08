# OneShot Anthropic API

OneShot Anthropic API is a library that provides access to Anthropic API.

## Implementation details

* Use clap v4 derive to parse the command-line arguments
* Use `get_string_until_finished` from `oneshot_utils` package to work around the `max_tokens` limit in Anthropic API.
