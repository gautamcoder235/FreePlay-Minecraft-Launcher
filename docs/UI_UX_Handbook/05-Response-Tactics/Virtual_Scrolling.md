# Virtual Scrolling & Viewport Windowing

---

## 1. Overview

When lists exceed **100+ items**, rendering all DOM nodes degrades scroll performance and increases memory footprint. Virtual scrolling instantiates only items within the visible viewport bounds.

---

## 2. Virtual Windowing Math Engine

Given total item count $N$, fixed item height $H$, container viewport height $C$, and scroll position $S$:

$$\text{StartIndex} = \max\left(0, \left\lfloor \frac{S}{H} \right\rfloor - \text{Overscan}\right)$$

$$\text{EndIndex} = \min\left(N - 1, \left\lceil \frac{S + C}{H} \right\rceil + \text{Overscan}\right)$$

$$\text{TotalContainerHeight} = N \times H$$

---

## 3. Production React Virtual List Component

```tsx
import React, { useState, useRef } from 'react';

export function SimpleVirtualList<T>({ items, itemHeight, containerHeight, renderRow }: {
  items: T[];
  itemHeight: number;
  containerHeight: number;
  renderRow: (item: T, index: number) => React.ReactNode;
}) {
  const [scrollTop, setScrollTop] = useState(0);

  const startIndex = Math.max(0, Math.floor(scrollTop / itemHeight) - 3);
  const endIndex = Math.min(items.length - 1, Math.ceil((scrollTop + containerHeight) / itemHeight) + 3);

  const visibleItems = items.slice(startIndex, endIndex + 1);
  const totalHeight = items.length * itemHeight;

  return (
    <div
      onScroll={(e) => setScrollTop(e.currentTarget.scrollTop)}
      style={{ height: containerHeight, overflowY: 'auto', position: 'relative' }}
    >
      <div style={{ height: totalHeight, width: '100%' }}>
        {visibleItems.map((item, idx) => {
          const actualIndex = startIndex + idx;
          return (
            <div
              key={actualIndex}
              style={{
                position: 'absolute',
                top: 0,
                left: 0,
                width: '100%',
                transform: `translateY(${actualIndex * itemHeight}px)`,
              }}
            >
              {renderRow(item, actualIndex)}
            </div>
          );
        })}
      </div>
    </div>
  );
}
```

---

## 4. Key References

- [TanStack Virtual Documentation](https://tanstack.com/virtual/v3)
- [MDN CSS Containment Specification](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_containment)
