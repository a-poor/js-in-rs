use clap::Parser;
use anyhow::{Result, Error};
use deno_core::v8;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Path to the file to be read
    file_path: String,
    
    /// JS matcher to be used
    js_matcher: String,
}

fn main() -> Result<()> {
    // Parse the arguments...
    let args = Args::parse();

    // Does the path exist?
    if !std::path::Path::new(&args.file_path).exists() {
        return Err(Error::msg("File does not exist!"));
    }

    // Validate the JS matcher...
    if args.js_matcher.trim().is_empty() {
        return Err(Error::msg("JS matcher cannot be empty!"));
    }
    
    // Read in the file...
    let raw = std::fs::read_to_string(args.file_path)?;
    
    // Parse the matcher...
    let mut runtime = JsRuntime::new(RuntimeOptions::default());

    for line in raw.lines() {
        // println!("Line: {}", line);

        // Serialize the line...
        let s = serde_json::to_string(&line)?;

        // Format the JS...
        let js_matcher = format!("!!((line) => {})({})", args.js_matcher.clone().trim(), s);
        // println!("JS: {}", js_matcher);

        // Run the JS...
        let result = runtime.execute_script("matcher.js", js_matcher.into());
        // println!("Result: {:?}", result);

        match result {
            Ok(global) => {
                let scope = &mut runtime.handle_scope();
                let local = v8::Local::new(scope, global);
                let deserialized_value = serde_v8::from_v8::<serde_json::Value>(scope, local);

                match deserialized_value {
                    Ok(value) => {
                        match value {
                            serde_json::Value::Bool(b) => {
                                if b {
                                    println!("{}", line);
                                }
                            },
                            _ => return Err(Error::msg(format!("JS matcher must return a boolean value!"))),
                        }
                    },
                    Err(err) => return Err(Error::msg(format!("Cannot deserialize value: {err:?}"))),
                }
            },
            Err(e) => {
                return Err(Error::msg(format!("Eval error: {}", e)));
            }
        }
    }

    Ok(())
}
