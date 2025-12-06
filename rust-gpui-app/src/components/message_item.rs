//! Message item component for rendering individual chat messages.
//!
//! This module contains the `ChatMessage` struct and related types for
//! rendering rich chat messages with support for:
//! - Markdown text blocks
//! - Code blocks with syntax highlighting, copy, and execution
//! - Citations and footnotes
//! - File download cards
//! - Streaming typography with typewriter effect
//! - Feedback (thumbs up/down) for RLHF
//! - Message branching for edit history
//! - Model attribution and timestamps

use gpui::{
    prelude::*,
    div, px, rgb, rgba, AnyElement,
    IntoElement, ParentElement, SharedString, Styled, Window,
    ClipboardItem,
};

// Helper color functions
fn white() -> gpui::Rgba { rgb(0xffffff).into() }
fn black() -> gpui::Rgba { rgb(0x000000).into() }

// --- Data Structures ---

/// Execution status for code blocks
#[derive(Clone, Debug, PartialEq)]
pub enum ExecutionStatus {
    /// Code has not been executed
    Idle,
    /// Code is currently running
    Running,
    /// Code executed successfully with output
    Success(SharedString),
    /// Code execution failed with error
    Error(SharedString),
}

/// Content block types within a message
#[derive(Clone, Debug)]
pub enum ContentBlock {
    /// Standard text (Markdown text would be parsed here)
    Text(SharedString),
    /// A code block with metadata
    Code {
        language: SharedString,
        code: SharedString,
        is_executable: bool,
        execution_status: ExecutionStatus,
    },
    /// A citation/footnote reference
    Citation {
        number: u32,
        source: SharedString,
        url: Option<SharedString>,
    },
    /// A file download card
    FileDownload {
        filename: SharedString,
        file_type: SharedString,
        size_bytes: u64,
    },
}

/// A single chat message with rich content support
#[derive(Clone, Debug)]
pub struct ChatMessage {
    /// Unique message identifier
    pub id: String,
    /// Whether this message is from the user
    pub is_user: bool,
    /// List of content blocks (text, code, etc.)
    pub blocks: Vec<ContentBlock>,
    /// User feedback (None = no feedback, true = up, false = down)
    pub feedback: Option<bool>,
    /// Model that generated this message
    pub model_name: SharedString,
    /// Timestamp when message was created
    pub timestamp: chrono::DateTime<chrono::Local>,
    /// Branch index for edit history (1-indexed)
    pub branch_index: u32,
    /// Total branches for this message position
    pub total_branches: u32,
    /// Whether the message is currently streaming
    pub is_streaming: bool,
    /// Loading/thinking state
    pub is_thinking: bool,
    /// Thought process (for chain-of-thought models)
    pub thought_process: Option<SharedString>,
    /// Error message if any
    pub error: Option<SharedString>,
}

impl ChatMessage {
    /// Create a new user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            is_user: true,
            blocks: vec![ContentBlock::Text(content.into().into())],
            feedback: None,
            model_name: "User".into(),
            timestamp: chrono::Local::now(),
            branch_index: 1,
            total_branches: 1,
            is_streaming: false,
            is_thinking: false,
            thought_process: None,
            error: None,
        }
    }

    /// Create a new assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            is_user: false,
            blocks: vec![ContentBlock::Text(content.into().into())],
            feedback: None,
            model_name: "Assistant".into(),
            timestamp: chrono::Local::now(),
            branch_index: 1,
            total_branches: 1,
            is_streaming: false,
            is_thinking: false,
            thought_process: None,
            error: None,
        }
    }

    /// Create a new assistant message with structured content blocks
    pub fn assistant_with_blocks(blocks: Vec<ContentBlock>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            is_user: false,
            blocks,
            feedback: None,
            model_name: "Assistant".into(),
            timestamp: chrono::Local::now(),
            branch_index: 1,
            total_branches: 1,
            is_streaming: false,
            is_thinking: false,
            thought_process: None,
            error: None,
        }
    }

    /// Create an assistant message with code block
    pub fn assistant_with_code(
        intro: impl Into<String>,
        language: impl Into<String>,
        code: impl Into<String>,
        outro: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            is_user: false,
            blocks: vec![
                ContentBlock::Text(intro.into().into()),
                ContentBlock::Code {
                    language: language.into().into(),
                    code: code.into().into(),
                    is_executable: true,
                    execution_status: ExecutionStatus::Idle,
                },
                ContentBlock::Text(outro.into().into()),
            ],
            feedback: None,
            model_name: "GPT-4".into(),
            timestamp: chrono::Local::now(),
            branch_index: 1,
            total_branches: 1,
            is_streaming: false,
            is_thinking: false,
            thought_process: None,
            error: None,
        }
    }

    /// Create a thinking/loading state message
    pub fn thinking() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            is_user: false,
            blocks: vec![],
            feedback: None,
            model_name: "Assistant".into(),
            timestamp: chrono::Local::now(),
            branch_index: 1,
            total_branches: 1,
            is_streaming: false,
            is_thinking: true,
            thought_process: None,
            error: None,
        }
    }

    /// Create an error message
    pub fn error(error_msg: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            is_user: false,
            blocks: vec![],
            feedback: None,
            model_name: "System".into(),
            timestamp: chrono::Local::now(),
            branch_index: 1,
            total_branches: 1,
            is_streaming: false,
            is_thinking: false,
            thought_process: None,
            error: Some(error_msg.into().into()),
        }
    }

    /// Get full text content for copying
    pub fn get_full_text(&self) -> String {
        self.blocks
            .iter()
            .map(|block| match block {
                ContentBlock::Text(text) => text.to_string(),
                ContentBlock::Code { code, .. } => code.to_string(),
                ContentBlock::Citation { number, source, .. } => {
                    format!("[{}] {}", number, source)
                }
                ContentBlock::FileDownload { filename, .. } => {
                    format!("[File: {}]", filename)
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Main render method for a single chat item
    pub fn render_message(&self, cx: &mut Window) -> impl IntoElement {
        let is_user = self.is_user;
        let bg_color = if is_user { rgb(0x3b82f6) } else { rgb(0x27272a) };
        let text_color = white();

        div()
            .id(SharedString::from(self.id.clone()))
            .flex()
            .flex_col()
            .gap_1()
            .p_2()
            .max_w(px(800.0))
            // Alignment based on sender
            .when(is_user, |d| d.ml_auto())
            .when(!is_user, |d| d.mr_auto())
            // Error state
            .when(self.error.is_some(), |d| {
                let error = self.error.clone().unwrap_or_default();
                d.child(
                    div()
                        .rounded_lg()
                        .bg(rgba(0xdc354580))
                        .border_1()
                        .border_color(rgb(0xdc3545))
                        .p_4()
                        .text_color(white())
                        .child(
                            div()
                                .flex()
                                .gap_2()
                                .items_center()
                                .child("⚠️")
                                .child(error)
                        )
                        .child(
                            div()
                                .mt_2()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(rgb(0xfbbf24))
                                .child("🔄 Retry")
                        )
                )
            })
            // Thinking state
            .when(self.is_thinking, |d| {
                d.child(
                    div()
                        .rounded_lg()
                        .bg(bg_color)
                        .p_4()
                        .text_color(text_color)
                        .child(
                            div()
                                .flex()
                                .gap_2()
                                .items_center()
                                .child(self.render_thinking_animation())
                        )
                )
            })
            // Normal message content
            .when(!self.is_thinking && self.error.is_none(), |d| {
                d.child(
                    div()
                        .rounded_lg()
                        .bg(bg_color)
                        .p_4()
                        .text_color(text_color)
                        // Thought process (collapsible)
                        .when(self.thought_process.is_some(), |d| {
                            let thought = self.thought_process.clone().unwrap_or_default();
                            d.child(
                                div()
                                    .mb_2()
                                    .p_2()
                                    .rounded_md()
                                    .bg(rgba(0x00000040))
                                    .text_xs()
                                    .text_color(rgb(0xa1a1aa))
                                    .child(
                                        div()
                                            .flex()
                                            .gap_1()
                                            .items_center()
                                            .child("💭 Thought Process")
                                    )
                                    .child(
                                        div().mt_1().child(thought)
                                    )
                            )
                        })
                        // Render content blocks
                        .children(self.blocks.iter().enumerate().map(|(idx, block)| {
                            self.render_block(idx, block, cx)
                        }))
                        // Streaming cursor
                        .when(self.is_streaming, |d| {
                            d.child(
                                div()
                                    .w(px(2.0))
                                    .h(px(16.0))
                                    .bg(white())
                                    .ml_1()
                            )
                        })
                )
            })
            // Model attribution and timestamp (hover to reveal)
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_xs()
                    .text_color(rgb(0x71717a))
                    .px_1()
                    .child(self.model_name.clone())
                    .child(self.timestamp.format("%H:%M").to_string())
            )
            // Branch navigation (if multiple branches)
            .when(self.total_branches > 1, |d| {
                d.child(self.render_branch_navigation())
            })
            // Footer actions
            .when(!is_user && !self.is_thinking && self.error.is_none(), |d| {
                d.child(self.render_assistant_footer(cx))
            })
            .when(is_user && self.total_branches > 1, |d| {
                d.child(self.render_user_footer())
            })
    }

    fn render_thinking_animation(&self) -> AnyElement {
        div()
            .flex()
            .gap_1()
            .items_center()
            .child("🤔")
            .child("Thinking...")
            .into_any_element()
    }

    fn render_block(&self, idx: usize, block: &ContentBlock, cx: &mut Window) -> AnyElement {
        match block {
            ContentBlock::Text(text) => {
                self.render_text_block(idx, text)
            }
            ContentBlock::Code { language, code, is_executable, execution_status } => {
                self.render_code_block(idx, language, code, *is_executable, execution_status, cx)
            }
            ContentBlock::Citation { number, source, url } => {
                self.render_citation(idx, *number, source, url)
            }
            ContentBlock::FileDownload { filename, file_type, size_bytes } => {
                self.render_file_download(idx, filename, file_type, *size_bytes)
            }
        }
    }

    fn render_text_block(&self, idx: usize, text: &SharedString) -> AnyElement {
        // Minimal Markdown rendering: headings (#, ##, ###), bullet lists (- ), and inline code `code`
        let content = text.to_string();
        let lines: Vec<&str> = content.split('\n').collect();

        let block = div()
            .id(SharedString::from(format!("text-{}", idx)))
            .mb_2()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(lines.into_iter().map(|line| {
                        // Headings
                        if let Some(stripped) = line.strip_prefix("### ") {
                            return div()
                                .text_sm()
                                .font_weight(gpui::FontWeight::MEDIUM)
                                .child(stripped.to_string())
                                .into_any_element();
                        } else if let Some(stripped) = line.strip_prefix("## ") {
                            return div()
                                .text_sm()
                                .font_weight(gpui::FontWeight::MEDIUM)
                                .child(stripped.to_string())
                                .into_any_element();
                        } else if let Some(stripped) = line.strip_prefix("# ") {
                            return div()
                                .text_lg()
                                .font_weight(gpui::FontWeight::BOLD)
                                .child(stripped.to_string())
                                .into_any_element();
                        }

                        // Bulleted list
                        if let Some(stripped) = line.strip_prefix("- ") {
                            return div()
                                .flex()
                                .gap_2()
                                .child(div().text_sm().child("•"))
                                .child(div().text_sm().child(stripped.to_string()))
                                .into_any_element();
                        }

                        // Inline code: split by backticks and alternate styles
                        let mut parts: Vec<&str> = Vec::new();
                        let mut buf = line;
                        while let Some(start) = buf.find('`') {
                            let (before, rest) = buf.split_at(start);
                            parts.push(before);
                            if let Some(end) = rest[1..].find('`') {
                                let (code_with_tick, after) = rest.split_at(end + 2);
                                // code_with_tick starts with ` and ends with `
                                parts.push(code_with_tick);
                                buf = after;
                            } else {
                                // unmatched backtick; push remainder and break
                                parts.push(rest);
                                buf = "";
                                break;
                            }
                        }
                        if !buf.is_empty() {
                            parts.push(buf);
                        }

                        // If we have inline code parts (contain backticks), render alternating segments
                        if parts.iter().any(|p| p.starts_with('`') && p.ends_with('`')) {
                            let row = div().flex().flex_wrap().gap_1();
                            let mut row = row;
                            for p in parts {
                                if p.starts_with('`') && p.ends_with('`') && p.len() >= 2 {
                                    let code_text = &p[1..p.len()-1];
                                    row = row.child(
                                        div()
                                            .rounded_sm()
                                            .bg(rgb(0x1f2937))
                                            .px_1()
                                            .child(
                                                div()
                                                    .font_family("monospace")
                                                    .text_sm()
                                                    .child(code_text.to_string())
                                            )
                                    );
                                } else if !p.is_empty() {
                                    row = row.child(div().text_sm().child(p.to_string()));
                                }
                            }
                            return row.into_any_element();
                        }

                        // Default paragraph
                        div().text_sm().child(line.to_string()).into_any_element()
                    }))
            );

        block.into_any_element()
    }

    fn render_code_block(
        &self,
        idx: usize,
        language: &SharedString,
        code: &SharedString,
        can_execute: bool,
        status: &ExecutionStatus,
        _cx: &mut Window,
    ) -> AnyElement {
        let code_content = code.clone();

        div()
            .id(SharedString::from(format!("code-{}", idx)))
            .bg(black())
            .rounded_md()
            .overflow_hidden()
            .mb_2()
            .border_1()
            .border_color(rgb(0x3f3f46))
            // Code header (language + actions)
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .bg(rgb(0x18181b))
                    .px_3()
                    .py_1()
                    // Language label
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xa1a1aa))
                            .child(language.clone())
                    )
                    // Action buttons
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            // Preview button (for HTML/CSS/SVG)
                            .when(
                                language.to_lowercase() == "html"
                                    || language.to_lowercase() == "svg"
                                    || language.to_lowercase() == "css",
                                |d| {
                                    d.child(
                                        div()
                                            .id("preview-btn")
                                            .cursor_pointer()
                                            .text_xs()
                                            .text_color(rgb(0x60a5fa))
                                            .child("👁 Preview")
                                    )
                                },
                            )
                            // Run button (for executable code)
                            .when(can_execute, |d| {
                                let run_text = match status {
                                    ExecutionStatus::Running => "⏳ Running...",
                                    _ => "▶ Run",
                                };
                                d.child(
                                    div()
                                        .id("run-btn")
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(rgb(0x4ade80))
                                        .child(run_text)
                                )
                            })
                            // Copy button
                            .child({
                                let code_for_copy = code_content.clone();
                                div()
                                    .id("copy-code-btn")
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(white())
                                    .child("📋 Copy")
                                    .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
                                        cx.write_to_clipboard(ClipboardItem::new_string(code_for_copy.to_string()));
                                    })
                            })
                    )
            )
            // Code body
            .child({
                // Render code as line-by-line rows to preserve newlines without relying on whitespace_pre
                let lines: Vec<&str> = code_content.split('\n').collect();
                let gutter_width = px(36.0);

                div()
                    .p_3()
                    .overflow_hidden()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_0()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0()
                                    .children(
                                        (0..lines.len()).map(|i| {
                                            let line_no = i + 1;
                                            let line_text = SharedString::from(lines[i].to_string());

                                            div()
                                                .flex()
                                                .items_start()
                                                .child(
                                                    div()
                                                        .min_w(gutter_width)
                                                        .pr_2()
                                                        .bg(rgb(0x111113))
                                                        .border_r_1()
                                                        .border_color(rgb(0x3f3f46))
                                                        .child(
                                                            div()
                                                                .font_family("monospace")
                                                                .text_xs()
                                                                .text_color(rgb(0x71717a))
                                                                .child(format!("{:>2}", line_no))
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .font_family("monospace")
                                                        .text_sm()
                                                        .child(line_text)
                                                )
                                                .into_any_element()
                                        })
                                    )
                            )
                    )
            })
            // Execution output panel
            .when(!matches!(status, ExecutionStatus::Idle), |d| {
                let (color, text) = match status {
                    ExecutionStatus::Running => (rgb(0xfacc15), SharedString::from("Running...")),
                    ExecutionStatus::Success(out) => (rgb(0x4ade80), out.clone()),
                    ExecutionStatus::Error(err) => (rgb(0xf87171), err.clone()),
                    _ => (rgb(0xffffff), SharedString::from("")),
                };

                d.child(
                    div()
                        .border_t_1()
                        .border_color(rgb(0x3f3f46))
                        .bg(rgba(0x00000080))
                        .p_2()
                        .overflow_hidden()
                        .child(
                            div()
                                .font_family("monospace")
                                .text_xs()
                                .text_color(color)
                                .whitespace_nowrap()
                                .child(text)
                        )
                )
            })
            .into_any_element()
    }

    fn render_citation(
        &self,
        idx: usize,
        number: u32,
        _source: &SharedString,
        _url: &Option<SharedString>,
    ) -> AnyElement {
        div()
            .id(SharedString::from(format!("citation-{}", idx)))
            .cursor_pointer()
            .text_xs()
            .text_color(rgb(0x60a5fa))
            .child(format!("[{}]", number))
            // Tooltip would show source on hover
            .into_any_element()
    }

    fn render_file_download(
        &self,
        idx: usize,
        filename: &SharedString,
        file_type: &SharedString,
        size_bytes: u64,
    ) -> AnyElement {
        let size_str = if size_bytes < 1024 {
            format!("{} B", size_bytes)
        } else if size_bytes < 1024 * 1024 {
            format!("{:.1} KB", size_bytes as f64 / 1024.0)
        } else {
            format!("{:.1} MB", size_bytes as f64 / (1024.0 * 1024.0))
        };

        div()
            .id(SharedString::from(format!("download-{}", idx)))
            .flex()
            .gap_2()
            .items_center()
            .p_3()
            .rounded_md()
            .bg(rgb(0x1f2937))
            .border_1()
            .border_color(rgb(0x374151))
            .cursor_pointer()
            .mb_2()
            .child(
                div()
                    .text_2xl()
                    .child("📄")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .child(filename.clone())
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x9ca3af))
                            .child(format!("{} • {}", file_type, size_str))
                    )
            )
            .child(
                div()
                    .ml_auto()
                    .text_color(rgb(0x60a5fa))
                    .child("⬇️")
            )
            .into_any_element()
    }

    fn render_branch_navigation(&self) -> AnyElement {
        div()
            .flex()
            .gap_2()
            .items_center()
            .justify_center()
            .text_xs()
            .text_color(rgb(0xa1a1aa))
            .mt_1()
            .child(
                div()
                    .cursor_pointer()
                    .when(self.branch_index > 1, |d| d.text_color(rgb(0x60a5fa)))
                    .when(self.branch_index <= 1, |d| d.text_color(rgb(0x52525b)))
                    .child("◀")
            )
            .child(format!("{} / {}", self.branch_index, self.total_branches))
            .child(
                div()
                    .cursor_pointer()
                    .when(self.branch_index < self.total_branches, |d| d.text_color(rgb(0x60a5fa)))
                    .when(self.branch_index >= self.total_branches, |d| d.text_color(rgb(0x52525b)))
                    .child("▶")
            )
            .into_any_element()
    }

    fn render_assistant_footer(&self, _cx: &mut Window) -> AnyElement {
        let full_text = self.get_full_text();

        div()
            .flex()
            .gap_3()
            .mt_1()
            .text_color(rgb(0xa1a1aa))
            .text_xs()
            // Thumbs up
            .child(
                div()
                    .id("thumbs-up")
                    .cursor_pointer()
                    .when(self.feedback == Some(true), |d| d.text_color(rgb(0x60a5fa)))
                    .child("👍")
            )
            // Thumbs down
            .child(
                div()
                    .id("thumbs-down")
                    .cursor_pointer()
                    .when(self.feedback == Some(false), |d| d.text_color(rgb(0xf87171)))
                    .child("👎")
            )
            // Copy full message
            .child({
                div()
                    .id("copy-message")
                    .cursor_pointer()
                    .child("📋 Copy")
                    .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(full_text.clone()));
                    })
            })
            // Regenerate
            .child(
                div()
                    .id("regenerate")
                    .cursor_pointer()
                    .child("🔄 Retry")
            )
            // Text-to-Speech
            .child(
                div()
                    .id("tts")
                    .cursor_pointer()
                    .child("🔊 Read")
            )
            // Share
            .child(
                div()
                    .id("share")
                    .cursor_pointer()
                    .child("🔗 Share")
            )
            .into_any_element()
    }

    fn render_user_footer(&self) -> AnyElement {
        div()
            .flex()
            .gap_3()
            .mt_1()
            .text_color(rgb(0xa1a1aa))
            .text_xs()
            .justify_end()
            // Edit
            .child(
                div()
                    .id("edit-message")
                    .cursor_pointer()
                    .child("✏️ Edit")
            )
            .into_any_element()
    }
}

// --- Message Actions ---

/// Actions that can be triggered from message UI
#[derive(Clone, Debug)]
pub enum MessageAction {
    /// Copy text to clipboard
    CopyText(SharedString),
    /// Execute code in a code block
    ExecuteCode { message_id: String, code: SharedString },
    /// Rate a message (positive/negative feedback)
    RateMessage { message_id: String, is_positive: bool },
    /// Regenerate an assistant response
    Regenerate(String),
    /// Edit a user message
    EditMessage { message_id: String, new_content: String },
    /// Navigate to a different branch
    NavigateBranch { message_id: String, direction: i32 },
    /// Read message aloud
    ReadAloud(String),
    /// Share message/conversation
    Share(String),
}
