# js-in-rs

_created by Austin Poor_

A demo of using JavaScript in a Rust program, via the [deno_core](https://docs.rs/deno_core/) crate.

## Usage

`js-in-rs` is a sample `grep`-like application, written in Rust, where a JavaScript filter is applied to each
line of an input file to determine if it should be printed to `stdout`.

The CLI is written using [clap](https://docs.rs/clap/). Here's what the (very minimal) help output looks like:

```sh
$ js-in-rs --help
Usage: js-in-rs <FILE_PATH> <JS_MATCHER>

Arguments:
  <FILE_PATH>   Path to the file to be read
  <JS_MATCHER>  JS matcher to be used

Options:
  -h, --help  Print help
```

The filter code is used to create a JS function that will be applied to each line in the input file.

For example, if you wanted to only print lines with more than 20 characters, you could use the filter
`line.length > 20` which would be formatted as a JS function `(line) => line.length > 20` and then
applied to the line as `!!(line) => line.length > 20)("...")`.

Here's an example of running the app to only display lines with between 20 and 50 characters, excluding
leading and trailing whitespace.

```sh
$ js-in-rs src/main.rs "line.trim().length > 20 && line.trim().length < 50"
```

<details>
<summary>Output</summary>

```
use anyhow::{Result, Error};
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
#[derive(Parser, Debug)]
    /// Path to the file to be read
    /// JS matcher to be used
fn main() -> Result<()> {
    // Parse the arguments...
    let args = Args::parse();
    // Does the path exist?
        return Err(Error::msg("File does not exist!"));
    // Validate the JS matcher...
    if args.js_matcher.trim().is_empty() {
    // Read in the file...
    // Parse the matcher...
    let mut runtime = JsRuntime::new(
        RuntimeOptions::default(),
    for line in raw.lines() {
        // Serialize the line...
        let s = serde_json::to_string(&line)?;
        let js_matcher = format!(
            "!!((line) => {})({})",
            args.js_matcher.clone().trim(),
        let result = runtime.execute_script(
                let scope = &mut runtime.handle_scope();
                let local = v8::Local::new(scope, global);
                match deserialized_value {
                            serde_json::Value::Bool(b) => {
                                    println!("{}", line);
                            _ => return Err(Error::msg(format!(
                                "JS matcher must return a boolean value!",
                    Err(err) => return Err(Error::msg(
                        format!("Cannot deserialize value: {err:?}"),
```
</details>


Here's another example where I've added a `;` in the middle of the `&&` operator, to show how errors are reported:

```sh
$ js-in-rs src/main.rs 'line.trim().length > 20 &;& line.trim().length < 50'
```

<details>
<summary>Output</summary>

```
Error: Eval error: Uncaught SyntaxError: Unexpected token ';'
    at matcher.js:1:39
```
</details>

