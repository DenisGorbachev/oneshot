# Drafts

## Workaround for max_tokens limit

* `let mut response_body = String::new();`
* If the stop reason in the API response is equal to `max_tokens`, then send another request with the following body:
  * Append the response body to `response_body`
  * Set the new request body to a concatenation of initial request body and `response_body`.
* If the stop reason in the API response is not equal to `max_tokens`, return either an error (if the stop reason is an error reason) or `response_body` (if the stop reason is normal).
