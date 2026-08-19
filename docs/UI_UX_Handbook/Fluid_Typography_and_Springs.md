# Deep Dive: Fluid Typography & Harmonic Spring Physics

---

## 1. Fluid Typography Math Engine

To prevent jarring responsive layout shifts, typography scales fluidly between viewport bounds ($VW_{\text{min}}$ and $VW_{\text{max}}$) using CSS `clamp()`:

$$S_{\text{preferred}} = S_{\text{min}} + (S_{\text{max}} - S_{\text{min}}) \times \frac{100\text{vw} - VW_{\text{min}}}{VW_{\text{max}} - VW_{\text{min}}}$$

```css
/* Fluid Header: 28px at 375px viewport to 40px at 1440px viewport */
h1 {
  font-size: clamp(1.75rem, 1.31rem + 1.17vw, 2.5rem);
  line-height: calc(4px * round(up, 1.2em, 4px)); /* Snapped to 4px grid */
}
```

---

## 2. Harmonic Spring Oscillator Physics

Spring dynamics eliminate static CSS durations by modeling physical mass ($m$), stiffness ($k$), and damping ($c$).

$$F = -k(x - x_0) - c v$$

$$\text{Damping Ratio } \zeta = \frac{c}{2\sqrt{k \cdot m}}$$

- **Underdamped ($\zeta < 1.0$)**: Energetic / Bouncy (drag release, popover badge).
- **Critically Damped ($\zeta = 1.0$)**: Rapid return without overshoot (drawers, dropdowns).

```typescript
export const TIER1_SPRING_CONFIGS = {
  snappy: { type: 'spring', stiffness: 400, damping: 30, mass: 0.8 },
  gentle: { type: 'spring', stiffness: 250, damping: 25, mass: 1.0 },
  bouncy: { type: 'spring', stiffness: 500, damping: 15, mass: 0.5 }
};
```

---

## 3. Official References

- [Apple HIG Motion Guidelines](https://developer.apple.com/design/human-interface-guidelines/motion)
- [Framer Motion Springs Documentation](https://www.framer.com/motion/)
- [MDN CSS clamp() Function](https://developer.mozilla.org/en-US/docs/Web/CSS/clamp)
