use std::env;
use tokio::process::Command;
use rust_gemini_llm_client::generate_content;

/// Parse arguments: if starts with "az", treat as az command; else, handle as gemini prompt.
fn parse_args() -> Result<(Option<String>, Option<String>, Option<Vec<String>>), String> {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.first().map(|s| s == "az").unwrap_or(false) {
        // AZ command mode: remove "az", rest are az args
        args.remove(0);
        return Ok((None, None, Some(args)));
    }

    // Gemini mode: parse API key and prompt
    let mut api_key: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-k" | "--key" => {
                if i + 1 < args.len() {
                    api_key = Some(args.remove(i + 1));
                    args.remove(i);
                    continue;
                } else {
                    return Err(format!("Missing value for {}", args[i]));
                }
            }
            s if s.starts_with("--key=") => {
                let val = s.splitn(2, '=').nth(1).map(|s| s.to_string());
                api_key = val;
                args.remove(i);
                continue;
            }
            _ => i += 1,
        }
    }

    let prompt = if !args.is_empty() { Some(args.join(" ")) } else { None };
    Ok((api_key, prompt, None))
}

/// Execute az command and return output
async fn execute_az_command(args: Vec<String>) -> Result<String, String> {
    if args.is_empty() {
        return Err("No az command provided".to_string());
    }

    let mut command = Command::new("az");
    command.args(&args);

    println!("Executing az command: {:?}", command);

    match command.output().await {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                eprintln!("AZ Command Stderr: {}", String::from_utf8_lossy(&output.stderr));
                eprintln!("AZ Command Stdout: {}", String::from_utf8_lossy(&output.stdout));
                eprintln!("AZ Command Status: {}", output.status);

                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                Err(format!("az command failed: {}", stderr))
            }
        }
        Err(e) => Err(format!("Failed to execute az command: {}", e)),
    }
}

#[tokio::main]
async fn main() {
    // 1. Handle the Result from parse_args()
    let args_result = parse_args();

    // 2. Match on the Result to handle success and error cases
    match args_result {
        Ok((_, Some(_), Some(_))) => {
            eprintln!("Error: Invalid state - both prompt and az command detected.");
        }
        Ok((api_key, Some(prompt), None)) => {
            // Gemini mode
            println!("Sending prompt: {}", prompt);
            match generate_content(&prompt, api_key).await {
                Ok(resp) => println!("Response:\n{}", resp),
                Err(e) => eprintln!("Error calling Gemini: {}", e),
            }
        }
        Ok((_, None, Some(az_args))) => {
            // Azure CLI mode
            match execute_az_command(az_args).await {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("Error executing az command: {}", e),
            }
        }
        Ok((_, None, None)) => {
            // No prompt or az command, print usage
            let exe = env::args().next().unwrap_or_else(|| "rust-cli-echo".into());
            eprintln!("Usage: {} [az <command> | [-k API_KEY] <prompt>]", exe);
        }
        Err(e) => eprintln!("Argument parsing error: {}", e),
    }
}