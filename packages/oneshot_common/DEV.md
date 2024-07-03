# Development notes

## Questions

* How to provide the function signatures that need to be implemented to the LLM?
  * Options
    * Provide whole file templates
    * Provide `pub type AppendFromFile = fn(path: impl AsRef<Path>, string: &mut String)`
