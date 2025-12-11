use rust_gpui_app::components::messages_area; // ensure crate exports needed modules via src/lib.rs
use rust_gpui_app::components::message_item::{ChatMessage, ContentBlock, ExecutionStatus};

#[test]
fn assistant_with_code_builds_executable_block() {
    let code = "let x = 1;\nlet y = x + 2;";
    let msg = ChatMessage::assistant_with_code("Intro", "rust", code, "Outro");

    // Find the code block
    let mut found = false;
    for b in &msg.blocks {
        if let ContentBlock::Code { language, code: c, is_executable, execution_status } = b {
            assert_eq!(language.as_str(), "rust");
            assert!(is_executable, "expected executable=true for assistant_with_code");
            assert_eq!(execution_status, &ExecutionStatus::Idle);
            assert_eq!(c.as_str(), code);
            found = true;
        }
    }
    assert!(found, "expected a code block in assistant_with_code");
}

#[test]
fn get_full_text_includes_code_and_text_blocks() {
    let code = "console.log('hi');";
    let msg = ChatMessage::assistant_with_code("Intro", "javascript", code, "Outro");
    let full = msg.get_full_text();

    assert!(full.contains("Intro"), "full_text should include intro text");
    assert!(full.contains(code), "full_text should include code content");
    assert!(full.contains("Outro"), "full_text should include outro text");
}

#[test]
fn non_executable_code_block_can_be_constructed() {
    let msg = ChatMessage {
        id: "test".to_string(),
        is_user: false,
        blocks: vec![
            ContentBlock::Text("Intro".into()),
            ContentBlock::Code {
                language: "html".into(),
                code: "<div>hi</div>\n<span>two</span>".into(),
                is_executable: false,
                execution_status: ExecutionStatus::Idle,
            },
        ],
        feedback: None,
        model_name: "Assistant".into(),
        timestamp: chrono::Local::now(),
        branch_index: 1,
        total_branches: 1,
        is_streaming: false,
        is_thinking: false,
        thought_process: None,
        error: None,
    };

    // Ensure structure is correct
    assert_eq!(msg.blocks.len(), 2);
    match &msg.blocks[1] {
        ContentBlock::Code { language, code, is_executable, execution_status } => {
            assert_eq!(language.as_str(), "html");
            assert_eq!(execution_status, &ExecutionStatus::Idle);
            assert!(!is_executable, "html preview-only code should not be executable in this test");
            // sanity: multi-line to enable line numbers rendering logic paths
            assert!(code.contains('\n'));
        }
        _ => panic!("expected second block to be Code"),
    }
}

// NOTE: render_code_block is a private method on ChatMessage.
// Unit testing private rendering is best done via an internal #[cfg(test)] module inside
// src/components/message_item.rs to directly call ChatMessage::render_code_block and inspect IDs.
// Integration tests here validate the data modeling and inputs used by render_code_block.
// For full coverage, add internal tests asserting:
// - presence of header action IDs: preview-btn, run-btn, copy-code-btn
// - code body container with gutter alignment and border styles
// - execution output panel rendering for Success and Error statuses