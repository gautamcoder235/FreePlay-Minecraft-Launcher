# Thinking & Reasoning Indicators

---

## 1. Overview

Models with reasoning steps (OpenAI o1/o3, Claude 3.7 Sonnet) emit intermediate thinking tokens. The UI renders these tokens inside collapsible shimmer accordions to maintain clean main conversation flow.

---

## 2. Thinking Accordion State Engine

```tsx
import React, { useState } from 'react';

export function ThinkingAccordion({ reasoningContent, isFinished }: {
  reasoningContent: string;
  isFinished: boolean;
}) {
  const [isOpen, setIsOpen] = useState(!isFinished);

  return (
    <div className="border border-subtle rounded-lg mb-3 bg-surface-elevated overflow-hidden">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full px-3 py-2 flex items-center justify-between text-xs text-secondary hover:bg-surface-hover"
      >
        <span className="flex items-center gap-2">
          {!isFinished && <span className="w-2 h-2 rounded-full bg-accent animate-pulse" />}
          {isFinished ? 'Thought for 4 seconds' : 'Thinking...'}
        </span>
        <span>{isOpen ? '▲' : '▼'}</span>
      </button>
      {isOpen && (
        <div className="p-3 text-xs font-mono text-tertiary border-t border-subtle max-h-48 overflow-y-auto whitespace-pre-wrap">
          {reasoningContent}
        </div>
      )}
    </div>
  );
}
```

---

## 3. Key References

- [OpenAI Reasoning Models Guide](https://platform.openai.com/docs/guides/reasoning)
- [Anthropic Claude Thinking Models Documentation](https://docs.anthropic.com/)
