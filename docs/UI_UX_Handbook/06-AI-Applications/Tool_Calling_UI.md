# Tool Calling UI & State Indicators

---

## 1. Overview

When AI agents invoke tools (e.g., executing code search, querying a database, invoking a linter), the UI must present deterministic execution cards displaying tool inputs, real-time logs, and success/failure indicators.

---

## 2. Tool Call Lifecycle Sequence Diagram

```mermaid
sequenceDiagram
    participant User
    participant ChatUI as AI Chat Component
    participant Agent as SSE Stream Controller
    participant Tool as Tool Execution Engine

    Agent-->>ChatUI: SSE Event: tool_call (name: "search_repo", args: {...})
    ChatUI->>ChatUI: Render Tool Card (State: Executing Spinner)
    Agent->>Tool: Execute Function Call
    Tool-->>Agent: Return Execution Result JSON
    Agent-->>ChatUI: SSE Event: tool_result (status: "success")
    ChatUI->>ChatUI: Transition Tool Card (State: Green Checkmark + Expand Output)
```

---

## 3. Key References

- [OpenAI Function Calling Guide](https://platform.openai.com/docs/guides/function-calling)
- [Claude Tool Use Documentation](https://docs.anthropic.com/en/docs/tool-use)
