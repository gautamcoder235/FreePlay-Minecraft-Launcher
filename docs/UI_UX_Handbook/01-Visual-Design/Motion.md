# Motion Design & Physics-Based Spring Animations

---

## 1. Overview

Motion in Tier-1 software communicates state changes, maintains physical spatial continuity, and directs user attention. Linear CSS transitions (`transition: all 0.3s ease`) feel robotic and unresponsive. Tier-1 applications use **physics-based spring dynamics** (stiffness, damping, mass) and strict duration targets.

---

## 2. Spring Physics Equation & Timing Profiles

Harmonic Oscillator Spring Equation:

$$F = -k(x - x_0) - c v$$

Where:
- $k$: Stiffness (Resistance to displacement)
- $c$: Damping coefficient (Energy dissipation rate)
- $m$: Mass (Inertia of element)

```css
:root {
  /* Motion Easing Curves (Cubic Bezier Targets) */
  --ease-spring-snappy: cubic-bezier(0.175, 0.885, 0.32, 1.275);
  --ease-out-emphasized: cubic-bezier(0.2, 0.0, 0.0, 1.0);
  --ease-in-out-smooth: cubic-bezier(0.4, 0.0, 0.2, 1.0);

  /* Duration Budget Targets */
  --duration-instant: 100ms; /* Hover, press micro-feedback */
  --duration-fast:    150ms; /* Dropdown popovers, tooltips */
  --duration-normal:  250ms; /* Modal open, drawer slide */
  --duration-slow:    350ms; /* Major full-screen route transitions */
}
```

---

## 3. Motion Decision Matrix

| UI Trigger / Motion Event | Target Duration | Easing / Spring Configuration | Rationale |
| :--- | :--- | :--- | :--- |
| **Button Active / Press** | **80ms - 100ms** | `stiffness: 500, damping: 30` | Needs sub-100ms tactile acknowledgement |
| **Dropdown Popover** | **150ms** | `stiffness: 400, damping: 28` | Quick entrance without disrupting workflow |
| **Modal / Dialog Enter** | **200ms - 250ms** | `stiffness: 300, damping: 25` | Smooth scale up + opacity fade |
| **Drawer Panel Slide** | **250ms - 300ms** | `stiffness: 280, damping: 26` | Matches natural dragging physics |
| **Theme Shift Toggle** | **150ms** | `linear` color transition | Prevents strobe effect during color interpolation |

---

## 4. Key References

- [Apple HIG - Motion Design](https://developer.apple.com/design/human-interface-guidelines/motion)
- [Framer Motion Physics Guide](https://www.framer.com/motion/animation/)
- [MDN CSS Cubic Bezier Easing](https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function)
