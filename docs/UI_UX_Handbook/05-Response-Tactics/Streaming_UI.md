# Streaming UI & Progressive Hydration

---

## 1. Overview

Streaming UI enables modern web and AI applications to render text, components, and data layouts incrementally as data chunks arrive over network connections.

---

## 2. Server-Sent Events (SSE) vs WebSockets Architecture

| Protocol | Directionality | Connection Overhead | Best Use Case |
| :--- | :--- | :--- | :--- |
| **Server-Sent Events (SSE)** | Unidirectional (Server -> Client) | Low (Standard HTTP) | **AI Token Streaming, Progress Log** |
| **WebSockets** | Full Duplex (Bidirectional) | Medium (Connection Upgrade) | **Real-time Canvas, Multi-user Cursor** |

---

## 3. SSE Stream Parser Implementation with Buffer Split Protection

```typescript
export async function* parseSSEStream<T>(stream: ReadableStream<Uint8Array>): AsyncGenerator<T> {
  const reader = stream.getReader();
  const decoder = new TextDecoder('utf-8');
  let buffer = '';

  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;

      buffer += decoder.decode(value, { stream: true });
      const lines = buffer.split('\n');
      buffer = lines.pop() ?? ''; // Preserve partial chunk line in buffer

      for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith(':')) continue;
        if (trimmed === 'data: [DONE]') return;

        if (trimmed.startsWith('data: ')) {
          try {
            const parsed: T = JSON.parse(trimmed.slice(6));
            yield parsed;
          } catch (err) {
            console.error('Failed to parse SSE payload chunk:', trimmed, err);
          }
        }
      }
    }
  } finally {
    reader.releaseLock();
  }
}
```

---

## 4. Key References

- [OpenAI Production Best Practices - Streaming Protocol](https://platform.openai.com/docs/guides/production-best-practices)
- [MDN ReadableStream Protocol Docs](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream)
