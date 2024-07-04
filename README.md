# OneShot

OneShot generates whole apps in one shot from a `README.md` and [a config](#configuration).

**[Rationale](#rationale)** | **[Benefits](#benefits)** | **[Features](#features)** | **[How it works](#how-it-works)** | **[FAQ](#faq)**

## Rationale

LLMs are better at writing new code rather than editing existing code. It makes sense to adopt a new workflow:

* Write a specification in the `README.md`
* Ask the LLM to write the code according to this specification.
* If an LLM makes a mistake: **update the specification** and ask it to rewrite the code from scratch.

Don't edit the code written by LLM - edit the spec.

## Benefits

* Faster development.
* Faster approval by clients / stakeholders (you can edit the spec & rewrite the app right on the meeting).
* Faster onboarding of new teammates (they can read the spec).

## Features

* Fully open-source.
* Works with local models.
* Supports plugins.

The goal is to help you generate the code in one command: `oneshot`. If you need to perform any extra actions, you can implement them as a plugin. There are hooks for all stages of the rewrite cycle.

## FAQ

<details>
<summary>How to test?</summary>

Split your code into two packages: the app itself & the tests. This way, the LLM would not overwrite your tests when regenerating the app.

</details>

<details>
<summary>How to maintain backward compatibility?</summary>

You can specify the API of your code in the README. This way, you'll steer the LLM into maintaining the same API across different code versions.

</details>

## How it works

OneShot works according to the following algorithm:

* [Parse the configuration](#parsing-the-configuration) (see also: [the specification](#configuration)).
* Read the `README.md`
* Pass the request to the API wrapper.
* Apply auto-fixes and other transformations to code.
* Lint the code.
* Test the code.

The API wrapper is a separate program that sends an API request and processes the response. The API wrapper is responsible for actually writing the code into files.

## Configuration

* Format: [TOML](https://toml.io/en/).
* Location: at the root of the [package](#package).
* Examples:
  * [Example 1](./examples/config/example1.toml)

At minimum, the configuration must specify the API wrapper for communicating with an LLM.

An API wrapper is a binary program:

* The API program accepts TOML on standard input.
* The API program outputs the text received from the LLM to standard output.
* The API program outputs code by writing directly into files within a specified directory.

You can use an existing API wrapper or write your own.

## Algorithms

### Parsing the configuration

Every configuration option may be specified as a flag to `oneshot` program.

Configuration cascade (the next level overrides the previous level):

* Configuration file in the home directory (`~/oneshot.toml`).
* Environment variables.
* Configuration file in the current directory (`oneshot.toml`) (location may be specified with `--config` option).
* Command-line options.

Note that the local configuration file options override the environment variables. This is because the rewrites must be reproducible by other developers.

## API wrappers

### Use an existing API wrapper

We have plans for developing the following wrappers:

* OpenAI API
* [Anthropic API](./packages/oneshot_rust_anthropic)
* Ollama API (local models)

### Write your own API wrapper

TODO

Every plugin runs in the same working directory that was passed to `oneshot` (or current directory if none was passed).

## Definitions

### Package

Package is a unit of code generated in one shot.

Package defines its own dependencies.

Package can be published and installed via package manager.

Notes:

* A single repository may contain multiple packages.
* Most languages use the same word ("package") to define a shareable unit of code, but some languages use different words. For example, Ruby uses the word "gem".

### Response

Response is a message returned as a result of LLM request.

## Notes

* You can pass API-specific parameters to API wrappers. OneShot provides sensible defaults for parameters that aren't provided.

## Code guidelines

* Always use full English words. Only use abbreviations if they are very common, for example: `Cmd`, `Args`, `Config`.  
