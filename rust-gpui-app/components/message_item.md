# Message Item Rendering and Parsing Plan

This plan updates the design to parse assistant responses for fenced code blocks like ```rust and render them with line numbers and action buttons.

## Objectives

- Parse assistant responses into structured blocks using Markdown + extensions.
- Detect fenced code blocks and map to [Rust.enum ContentBlock::Code](src/components/message_item.rs:46) with language, code, and executability.
- Render code blocks with an always-visible line number gutter and conditional actions: copy, run, preview.
- Integrate parsing in [Rust.struct GeminiService](src/services/gemini_service.rs:24) before emitting AssistantMessage.

## Implementation Steps

1. Dependencies
   - Add pulldown-cmark to Cargo.toml:
     - `pulldown-cmark = "0.9"`

2. Parser Module
   - Create `src/utils/parser.rs` exposing:
     - `Rust.fn parse_assistant_response(raw: &str) -> Vec<ContentBlock>`(src/utils/parser.rs:1)
   - Responsibilities:
     - Iterate pulldown-cmark events
     - Text -> [Rust.enum ContentBlock::Text](src/components/message_item.rs:41)
     - Fenced code blocks -> [Rust.enum ContentBlock::Code](src/components/message_item.rs:46)
       - Language from info string (e.g., rust, toml)
       - Executability via tag in info string (e.g., `exec`)
     - Inline extensions:
       - Citations `[^n]` -> [Rust.enum ContentBlock::Citation](src/components/message_item.rs:53)
       - Downloads `[file:name.ext|type|sizeBytes]` -> [Rust.enum ContentBlock::FileDownload](src/components/message_item.rs:59)

3. Service Integration
   - In [Rust.fn GeminiService::process_message](src/services/gemini_service.rs:64):
     - After receiving response_text (line ~116), call `parse_assistant_response(&response_text)`
     - Change event payload from `AssistantMessage(String)` to `AssistantMessage(Vec<ContentBlock>)`
     - Update conversation history to store parsed blocks or a serialized representation
   - Update emitters/subscribers in components to consume parsed blocks

4. Rendering
   - [Rust.fn ChatMessage::render_message](src/components/message_item.rs:219):
     - Iterate the parsed blocks and call [Rust.fn ChatMessage::render_block](src/components/message_item.rs:361)
   - [Rust.fn ChatMessage::render_code_block](src/components/message_item.rs:387):
     - Maintain header with language and actions:
       - Copy: always
       - Run: when is_executable
       - Preview: html/css/svg
     - Body: render line-by-line rows with a gutter showing line numbers aligned to code content
     - Output panel: show [Rust.enum ExecutionStatus](src/components/message_item.rs:27) transitions

5. Events and Actions
   - Define or reuse [Rust.enum MessageAction](src/components/message_item.rs:697):
     - ExecuteCode { message_id, code }
     - CopyText(code)
   - Hook actions to UI buttons via mouse handlers already present in code header.

6. Tests
   - Parser unit tests:
     - `tests/parser_fences_test.rs` to assert mapping of ```rust, ```toml fences
     - Executability via `exec` tag
     - Text + citations + downloads mapping
   - Rendering tests (existing):
     - Extend [tests/message_item_render_test.rs](tests/message_item_render_test.rs) with cases for code blocks with multiple lines and language labels

## Data Flow

```mermaid
flowchart TD
    A[Gemini API raw response] --> B[pulldown-cmark]
    B --> C{Event}
    C -->|Text| D[ContentBlock::Text]
    C -->|Fenced Code| E[ContentBlock::Code(language, code, exec)]
    C -->|Custom Inline| F[ContentBlock::Citation / FileDownload]
    D --> G[Vec<ContentBlock>]
    E --> G
    F --> G
    G --> H[GeminiService emits AssistantMessage(parsed)]
    H --> I[ChatMessage::render_message() dispatches blocks]
    I --> J[ChatMessage::render_code_block() with gutter + actions]
```

## Notes

- Backward compatibility: if parsing is not enabled, fallback keeps [Rust.enum ContentBlock::Text](src/components/message_item.rs:41).
- Performance: parsing on the service side avoids heavy UI-time conversions.
- Future: optional syntax highlighting layer can be added atop code content.

## Open Changes Required

- Event signature change:
  - [Rust.enum GeminiServiceEvent](src/services/gemini_service.rs:13) from `AssistantMessage(String)` to `AssistantMessage(Vec<ContentBlock>)`
- Introduce new module:
  - [src/utils/parser.rs](src/utils/parser.rs)
- Add dependency:
  - Cargo.toml: pulldown-cmark
