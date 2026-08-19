# Button Component Architecture & Specs

---

## 1. Overview

Buttons are the primary interactive triggers in software. Tier-1 button components support variants (`primary`, `secondary`, `ghost`, `destructive`), sizes (`sm`, `md`, `lg`), loading spinner states, icon slots, and polymorphic `asChild` composition.

---

## 2. Polymorphic Button Pattern in TypeScript

```tsx
import React from 'react';

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost' | 'destructive';
  size?: 'sm' | 'md' | 'lg';
  isLoading?: boolean;
}

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ variant = 'primary', size = 'md', isLoading = false, children, className = '', disabled, ...props }, ref) => {
    return (
      <button
        ref={ref}
        disabled={disabled || isLoading}
        className={`btn btn-${variant} btn-${size} ${className}`}
        {...props}
      >
        {isLoading ? <span className="spinner-sm mr-2" /> : null}
        {children}
      </button>
    );
  }
);
```

---

## 3. Key References

- [Radix UI Slot Primitive Pattern](https://www.radix-ui.com/primitives/docs/utilities/slot)
- [Shadcn UI Button Component Architecture](https://ui.shadcn.com/docs/components/button)
