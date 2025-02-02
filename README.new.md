# Oneshot

Oneshot tries to fix all errors in a repo.

## How it works

* Collect errors
* For error in errors:
  * Build context
  * Send a request to the LLM
  * Apply the response from the LLM
  * If the error has been fixed:
    * Then: commit and push
    * Else: revert edits

It is possible to run Oneshot multiple times for the same errors. Since LLM responses are non-deterministic, it may fix an error in the next iteration.

It is recommended to run Oneshot with different LLMs (progressing from fast to slow). Some errors are simple enough to be fixed by fast & cheap LLMs, while others are only fixable by slow & expensive LLMs. 
