# ChatMessage Component Specification

## I. Core Content Rendering & Formatting
- **Markdown Support:** Parses and renders CommonMark (headers, lists, bold, italics, blockquotes, inline code).
- **Table Rendering:** Renders Markdown tables with horizontal scroll for overflow.
- **Mathematical Notation:** Recognizes and renders LaTeX syntax for inline ($E=mc^2$) and block ($$F(x) = \int_{-\infty}^{\infty} f(x) e^{-2\pi i \xi x} \,dx$$) math.
- **Streaming Typography:** Assistant messages use a typewriter effect with a blinking cursor for streaming tokens.
- **Inline Citation/Footnotes:** Renders clickable footnote numbers $[1]$ that reveal sources on hover/click.

## II. Code Block Capabilities
- **Syntax Highlighting:** Auto-detects language and applies syntax coloring.
- **Copy Code Button:** Each code block has a "Copy" button with visual feedback.
- **Language Labeling:** Displays language name in code block header.
- **Code Execution (Sandboxed):**
  - **Frontend Rendering:** HTML/CSS/SVG code can be previewed in a sandboxed iframe.
  - **Backend Execution:** Executable code (e.g., Python) can be run if backend supports, with output shown below.
  - **File Downloads:** If code generates files, UI shows a download card for artifacts.

## III. Message Interaction & Actions
- **Copy to Clipboard:** Global copy button for message text (plain or rich text).
- **Edit Message (User Side):** Users can edit previous prompts; edits create branches, not overwrite history. UI provides pagination for branches.
- **Regenerate Response (Assistant Side):** "Retry" button requests a new answer for the same prompt.
- **Text-to-Speech (TTS):** "Play" button reads message aloud.
- **Share/Deep Link:** Option to generate a unique URL for sharing messages/threads.

## IV. Feedback & Quality Assurance (RLHF)
- **Thumbs Up/Down:**
  - Positive feedback is a simple signal.
  - Negative feedback triggers a modal for details (e.g., style, accuracy, refusal).
- **Correction/Diffing:** Users can highlight text and suggest corrections for model training.

## V. State & Metadata
- **Loading/Thinking States:** Skeleton loader or "Thinking..." animation before first token. Collapsible "Thought Process" for chain-of-thought models.
- **Error Handling:** Network/content errors shown in bubble with retry option.
- **Timestamps:** Hover reveals exact date/time sent/received.
- **Model Attribution:** Indicates which model generated each message.

## VI. Accessibility & Responsive Design
- **Keyboard Navigation:** All buttons are focusable and actionable via keyboard.
- **Screen Readers:**
  - Code blocks announced as "Code block, [Language]".
  - Streaming text uses `aria-live="polite"`.
- **Mobile Adaptation:** Hover actions become permanent or accessible via long-press on mobile.

## Summary Table of Actions
| Feature           | User Message | Assistant Message |
|-------------------|:-----------:|:----------------:|
| Copy Text         |     ✅      |        ✅        |
| Edit Content      |     ✅      |        ❌        |
| Regenerate        |     ❌      |        ✅        |
| Feedback (+/-)    |     ❌      |        ✅        |
| Code Execution    |     ❌      |        ✅        |
| Read Aloud (TTS)  |     ❌      |        ✅        |
| View Branches     |     ✅      |        ✅        |

---

## Data Structures (Rust, gpui)

```
#[derive(Clone, Debug, PartialEq)]
pub enum ExecutionStatus {
    Idle,
    Running,
    Success(SharedString), // Standard Output
    Error(SharedString),   // Standard Error
}

#[derive(Clone, Debug)]
pub enum ContentBlock {
    Text(SharedString),
    Code {
        language: SharedString,
        code: SharedString,
        is_executable: bool,
        execution_status: ExecutionStatus,
    },
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub id: String,
    pub is_user: bool,
    pub blocks: Vec<ContentBlock>,
    pub feedback: Option<bool>,
    pub model_name: SharedString,
    pub timestamp: chrono::DateTime<chrono::Local>,
}
```

## Message Actions
```
#[derive(Clone, Debug)]
pub enum MessageAction {
    CopyText(SharedString),
    ExecuteCode { message_id: String, code: SharedString },
    RateMessage { message_id: String, is_positive: bool },
    Regenerate(String),
}
```

## Rendering & Interaction (gpui)
- Each `ChatMessage` renders its blocks (Markdown, code, etc.)
- Code blocks have headers for language, copy, and run actions
- Footer for assistant messages includes feedback and copy actions
- Actions are handled via event dispatch and async updates

## Error Handling & State
- Execution status is tracked per code block
- Feedback is stored per message
- Async code execution updates status and output

## Accessibility & Responsiveness
- All interactive elements are keyboard and screen reader accessible
- Mobile UI adapts hover actions for touch

---

