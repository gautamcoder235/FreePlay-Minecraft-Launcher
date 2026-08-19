# AI Streaming Responses & SSE Tokens

---

## 1. Overview

AI response streaming presents generated tokens immediately as they are received over SSE or WebSockets. Tier-1 applications (ChatGPT, Cursor, Claude) batch token state updates using `requestAnimationFrame` to prevent main-thread layout thrashing.

---

## 2. Token Batching Engine with `requestAnimationFrame`

```typescript
import { useEffect, useRef, useState } from 'react';

export function useStreamBuffer(rawStream: AsyncIterable<string>) {
  const [displayedContent, setDisplayedContent] = useState('');
  const tokenBufferRef = useRef<string[]>([]);
  const frameIdRef = useRef<number | null>(null);

  useEffect(() => {
    let isMounted = true;

    const consumeStream = async () => {
      for await (const chunk of rawStream) {
        if (!isMounted) break;
        tokenBufferRef.current.push(chunk);
        scheduleFlush();
      }
    };

    const scheduleFlush = () => {
      if (frameIdRef.current !== null) return;
      frameIdRef.current = requestAnimationFrame(() => {
        frameIdRef.current = null;
        if (tokenBufferRef.current.length > 0) {
          const chunk = tokenBufferRef.current.join('');
          tokenBufferRef.current = [];
          setDisplayedContent((prev) => prev + chunk);
        }
      });
    };

    consumeStream();
    return () => {
      isMounted = false;
      if (frameIdRef.current !== null) cancelAnimationFrame(frameIdRef.current);
    };
  }, [rawStream]);

  return displayedContent;
}
```

---

## 3. Key References

- [OpenAI SSE API Reference](https://platform.openai.com/docs/api-reference/chat/streaming)
- [MDN requestAnimationFrame Specification](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame)
