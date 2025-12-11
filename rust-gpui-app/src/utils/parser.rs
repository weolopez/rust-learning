use crate::components::message_item::{ContentBlock, ExecutionStatus};
use gpui::SharedString;
use pulldown_cmark::{Event, Options, Parser, Tag, CodeBlockKind};

/// Parse an assistant raw string into structured ContentBlocks.
/// - Fenced code blocks: ```lang [exec] -> ContentBlock::Code
/// - Inline citations: [^n] -> ContentBlock::Citation
/// - File downloads: [file:name.ext|type|sizeBytes] -> ContentBlock::FileDownload
pub fn parse_assistant_response(raw: &str) -> Vec<ContentBlock> {
    let mut blocks: Vec<ContentBlock> = Vec::new();

    // Markdown parser options
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(raw, opts);

    // Accumulators for text segments between events
    let mut current_text: String = String::new();

    // Helper to flush accumulated plain text into a block
    fn flush_text(blocks: &mut Vec<ContentBlock>, current_text: &mut String) {
        if !current_text.trim().is_empty() {
            blocks.push(ContentBlock::Text(SharedString::from(current_text.clone())));
        }
        current_text.clear();
    }

    // State for code blocks
    let mut in_code_block = false;
    let mut code_lang: Option<String> = None;
    let mut code_flags: Vec<String> = Vec::new();
    let mut code_buf: String = String::new();

    for ev in parser {
        match ev {
            Event::Start(tag) => {
                if let Tag::CodeBlock(kind) = tag {
                    match kind {
                        CodeBlockKind::Fenced(info) => {
                            // Enter fenced code block
                            in_code_block = true;
                            code_buf.clear();
                            code_flags.clear();

                            // Info string may contain "lang" or "lang extra"
                            let info_str: &str = info.as_ref();
                            let mut parts = info_str.split_whitespace();
                            code_lang = parts.next().map(|s| s.to_string());
                            code_flags = parts.map(|s| s.to_string()).collect();

                            // Flush any preceding text
                            flush_text(&mut blocks, &mut current_text);
                        }
                        CodeBlockKind::Indented => {
                            // Indented code block without language; treat as text code
                            in_code_block = true;
                            code_buf.clear();
                            code_flags.clear();
                            code_lang = None;
                            flush_text(&mut blocks, &mut current_text);
                        }
                    }
                }
            }
            Event::End(tag) => {
                if let Tag::CodeBlock(_kind) = tag {
                    // Exit code block
                    let language = SharedString::from(code_lang.clone().unwrap_or_else(|| "text".to_string()));
                    let code = SharedString::from(code_buf.clone());

                    // Executable if "exec" flag present or language is commonly executable
                    let is_exec_flag = code_flags.iter().any(|f| f.eq_ignore_ascii_case("exec"));
                    let lang_lower = language.to_string().to_lowercase();
                    let is_executable = is_exec_flag || matches!(lang_lower.as_str(), "rust" | "python" | "bash" | "sh" | "javascript" | "node");

                    blocks.push(ContentBlock::Code {
                        language,
                        code,
                        is_executable,
                        execution_status: ExecutionStatus::Idle,
                    });

                    // Reset
                    in_code_block = false;
                    code_lang = None;
                    code_flags.clear();
                    code_buf.clear();
                }
            }
            Event::Text(text) => {
                if in_code_block {
                    code_buf.push_str(text.as_ref());
                } else {
                    // Simple inline extensions handling
                    let t = text.to_string();

                    // Citation pattern: [^n]
                    if let Some(num) = parse_citation(&t) {
                        flush_text(&mut blocks, &mut current_text);
                        blocks.push(ContentBlock::Citation {
                            number: num,
                            source: SharedString::from(format!("citation {}", num)),
                            url: None,
                        });
                    }
                    // File download pattern: [file:name.ext|type|sizeBytes]
                    else if let Some((filename, ftype, size)) = parse_file_download(&t) {
                        flush_text(&mut blocks, &mut current_text);
                        blocks.push(ContentBlock::FileDownload {
                            filename: SharedString::from(filename),
                            file_type: SharedString::from(ftype),
                            size_bytes: size,
                        });
                    } else {
                        current_text.push_str(&t);
                    }
                }
            }
            Event::Code(inline_code) => {
                // Inline code stays in text for now
                current_text.push_str(inline_code.as_ref());
            }
            Event::SoftBreak => {
                if in_code_block {
                    code_buf.push('\n');
                } else {
                    current_text.push('\n');
                }
            }
            Event::HardBreak => {
                if in_code_block {
                    code_buf.push('\n');
                } else {
                    current_text.push('\n');
                }
            }
            // Other events are ignored or appended as plain text
            _ => {}
        }
    }

    // Flush trailing text
    flush_text(&mut blocks, &mut current_text);

    blocks
}

fn parse_citation(text: &str) -> Option<u32> {
    // match strings like [^1]
    if text.starts_with("[^") && text.ends_with(']') {
        let inner = &text[2..text.len() - 1];
        if let Ok(n) = inner.parse::<u32>() {
            return Some(n);
        }
    }
    None
}

fn parse_file_download(text: &str) -> Option<(String, String, u64)> {
    // match strings like [file:name.ext|type|sizeBytes]
    if text.starts_with("[file:") && text.ends_with(']') {
        let inner = &text[6..text.len() - 1]; // after "file:"
        let parts: Vec<&str> = inner.split('|').collect();
        if parts.len() == 3 {
            let filename = parts[0].to_string();
            let file_type = parts[1].to_string();
            if let Ok(size) = parts[2].parse::<u64>() {
                return Some((filename, file_type, size));
            }
        }
    }
    None
}