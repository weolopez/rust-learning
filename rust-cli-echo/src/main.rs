use std::env;
use rust_gemini_llm_client::generate_content;

/// Simple parsing: accept optional -k/--key API key and remaining args are the prompt.
fn parse_args() -> (Option<String>, Option<String>) {
    let mut args: Vec<String> = env::args().skip(1).collect();
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
                    eprintln!("Missing value for {}", args[i]);
                    break;
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
    (api_key, prompt)
}

#[tokio::main]
async fn main() {
    let (api_key, prompt_opt) = parse_args();

    match prompt_opt {
        Some(prompt) => {
            println!("Sending prompt: {}", prompt);
            match generate_content(&prompt, api_key).await {
                Ok(resp) => println!("Response:\n{}", resp),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        None => {
            let exe = env::args().next().unwrap_or_else(|| "rust-cli-echo".into());
            println!("Usage: {} [-k API_KEY] <message>", exe);
            println!("Example: {} -k sk_... Hello world", exe);
        }
    }
}