# Physics Animations & Spring Curves

---

## 1. Overview

Spring physics animations simulate physical mass and stiffness, guaranteeing smooth transitions regardless of animation interrupts or mid-flight directional changes.

---

## 2. Spring Formula & Presets

$$\text{Damping Ratio } \zeta = \frac{c}{2\sqrt{k \cdot m}}$$

```typescript
export const SPRING_PRESETS = {
  // Snappy spring for popovers and contextual menus
  snappy: { stiffness: 400, damping: 30, mass: 0.8 },
  // Gentle spring for panel drawers and modal windows
  gentle: { stiffness: 250, damping: 25, mass: 1.0 },
  // Bouncy spring for badges and floating alerts
  bouncy: { stiffness: 500, damping: 15, mass: 0.5 },
};
```

---

## 3. Key References

- [Framer Motion Springs Documentation](https://www.framer.com/motion/animation/)
- [Apple HIG Motion Dynamics](https://developer.apple.com/design/human-interface-guidelines/motion)
