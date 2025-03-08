# Dev notes

* The `test` command must be represented as a single string because the user wants to specify it as a single string
  * Examples
    * `mise run test`
    * `deno test`
    * `forge test`
* The `test` command may output either a validation error that needs to be fixed or an internal issue that can't be fixed and must be reported to the user
  * Examples
    * Internal issue: "not enough space left on device"
    * Internal issue: "can't parse configuration file"
