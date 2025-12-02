# LLM Chat Component Design

## Overview
This document outlines the design and implementation plan for a full-featured LLM chat component integrated with Google's Gemini models. The component will be built as a reusable GPUI component following the application's modular architecture.

## Requirements
- **API Integration**: Google Gemini models
- **Core Features**:
  - Message input field with send button
  - Scrollable chat history (user/AI messages)
  - Loading indicators during API calls
  - Error handling and retry options
  - Streaming responses
- **API Key Management**: User input via settings UI, stored in localStorage

## Architecture

### Component Structure
```
ChatComponent
├── ChatHistory (Scrollable list of messages)
├── MessageInput (Text field + send button)
├── LoadingIndicator
└── ErrorDisplay
```

### Data Models
```rust
struct ChatMessage {
    id: u64,
    role: MessageRole, // User | Assistant
    content: SharedString,
    timestamp: DateTime<Utc>,
}

struct ChatState {
    messages: Vec<ChatMessage>,
    is_loading: bool,
    error: Option<String>,
    api_key: Option<String>,
}
```

### State Integration
The chat state will be added to `AppState` to persist across view changes and provide global access.

## Implementation Plan

### Phase 1: Core Infrastructure
1. Update Cargo.toml with Gemini API dependencies (reqwest for HTTP, serde for JSON)
2. Add chat-related fields to AppState
3. Create ChatMessage and ChatState structs
4. Implement Gemini API client module

### Phase 2: UI Components
5. Create ChatComponent struct with Render implementation
6. Implement ChatHistory with scrollable message list
7. Implement MessageInput with text field and send button
8. Add loading indicators and error displays

### Phase 3: Functionality
9. Implement message sending logic with API calls
10. Add streaming response handling
11. Implement error handling and retry mechanisms
12. Add API key input/settings UI

### Phase 4: Integration & Polish
13. Add ChatComponent to components/mod.rs exports
14. Integrate into HomeView for demo
15. Add theme styling for chat elements
16. Test full chat flow

## TODO List

- [ ] Update Cargo.toml with Gemini API dependencies
- [ ] Add chat state fields to AppState
- [ ] Create ChatMessage and ChatState data models
- [ ] Implement Gemini API client module
- [ ] Create ChatComponent struct and Render impl
- [ ] Implement ChatHistory component
- [ ] Implement MessageInput component
- [ ] Add loading indicators
- [ ] Add error handling UI
- [ ] Implement message sending with API calls
- [ ] Add streaming response support
- [ ] Implement retry mechanisms
- [ ] Create API key settings UI
- [ ] Export ChatComponent in mod.rs
- [ ] Integrate ChatComponent into HomeView
- [ ] Add chat-specific theme styling
- [ ] Test complete chat functionality