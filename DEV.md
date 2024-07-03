# Development

## Questions

* How to separate the code into multiple files?
  * The response may contain multiple files by itself
    * Examples
      * Writing Rust code
        * "Implement a Rust app for concatenating multiple PDF files"
        * LLM sends [a response](./README.md#response) that contains both `main.rs` and the `Cargo.toml` file.
    * Notes
      * Aider has solved this question.
      * Other tools that are competing at SWE bench must solve this question. 
* How to implement the plugin for adding a GitHub repo to LLM message?
  * Parameters
    * What to add?
      * Options
        * Full repo
          * Aider recommends against it
        * Repo map
          * Aider uses it
        * Repo docs
          * Maybe generalize to "Add a website subpath"?
    * How to add?
      * Append to the message
      * Pass the conf
    * Where to append?
      * In the main executable
      * In the API wrapper
