# Aurora Design Language Specification

## Overview

Aurora's design language is grounded in five pillars:

1. **Intentional Simplicity** — Remove everything unnecessary. Every pixel serves purpose.
2. **Dignified Motion** — Animation clarifies interaction, never decorates. Spring physics make movement feel alive.
3. **Typography-First** — Text is the primary interface. Exceptional typography enables clarity.
4. **Semantic Color** — Color conveys meaning through consistent patterns, not aesthetics.
5. **Accessibility Inherent** — Design for everyone from the start. Constraints breed elegance.

## Visual Identity

### Grid & Spacing

Aurora uses an 8px baseline grid. All spacing derives from this:
- `spacing.xxs` = 2px (micro adjustments)
- `spacing.xs` = 4px (tight spacing)
- `spacing.sm` = 8px (grid unit)
- `spacing.md` = 12px (standard padding)
- `spacing.lg` = 16px (section spacing)
- `spacing.xl` = 24px (large spacing)
- `spacing.xxl` = 32px (layout spacing)
- `spacing.xxxl` = 48px (screen margins)

Grid adherence: Every layout element aligns to 8px grid. Exceptions only for typography baseline alignment.

### Border Radius

Rounded corners follow a scale:
- `radius.xs` = 4px (subtle softness, desktop app dialogs)
- `radius.sm` = 8px (buttons, cards, input fields)
- `radius.md` = 12px (larger containers, modals)
- `radius.lg` = 16px (prominent surfaces)
- `radius.xl` = 24px (large cards, full-width containers)

**Philosophy**: Radius creates visual hierarchy. Smaller components use tighter radius; larger containers use generous radius. No fully circular elements except affordances (e.g., profile photos, badges).

### Elevation & Shadows

Aurora uses **semantic elevation**, not arbitrary shadows. Five elevation levels:

```
level1: 0 1px 2px rgba(0,0,0,0.08)
level2: 0 3px 8px rgba(0,0,0,0.12)
level3: 0 8px 16px rgba(0,0,0,0.16)
level4: 0 12px 24px rgba(0,0,0,0.20)
level5: 0 16px 32px rgba(0,0,0,0.24)
```

Usage:
- **level1**: Subtle hover states, tooltips
- **level2**: Popovers, dropdowns, cards
- **level3**: Modals, floating windows
- **level4**: Prominent surfaces, app windows
- **level5**: Full-screen overlays, system alerts

**Dark theme adjustment**: Shadows use lighter opacity + reduced blur (contrast preserved in dark mode):
```
level1: 0 1px 2px rgba(255,255,255,0.04)
level2: 0 3px 8px rgba(255,255,255,0.06)
...
```

## Typography System

### Font Family

**Primary Font**: Inter (or equivalent open-source variable font)
- Universal support across scripts (CJK, Arabic, Devanagari)
- Optical sizing support
- Variable weight and width
- Exceptional rendering at all sizes

**Fallback**: IBM Plex Sans (if variable fonts unavailable)

**Mono Font**: IBM Plex Mono (terminal, code blocks)

### Type Scales

#### Display
- **Size**: 48px / 60px (responsive)
- **Weight**: 600 or 700
- **Line Height**: 1.2
- **Letter Spacing**: -0.02em
- **Use**: Page titles, hero sections, prominent headings

#### Headline
- **Size**: 32px / 40px (responsive)
- **Weight**: 600
- **Line Height**: 1.25
- **Letter Spacing**: -0.01em
- **Use**: Section headings, major titles

#### Title
- **Size**: 20px / 24px (responsive)
- **Weight**: 600
- **Line Height**: 1.3
- **Letter Spacing**: 0
- **Use**: Card titles, dialog titles, subsection headings

#### Body
- **Size**: 14px / 16px (responsive)
- **Weight**: 400
- **Line Height**: 1.5
- **Letter Spacing**: 0.01em
- **Use**: Primary reading content, descriptions, form inputs

#### Caption
- **Size**: 12px / 13px (responsive)
- **Weight**: 500
- **Line Height**: 1.4
- **Letter Spacing**: 0.02em
- **Use**: Secondary text, metadata, hints, form labels

#### Micro
- **Size**: 11px / 12px (fixed, no responsive scaling)
- **Weight**: 500
- **Line Height**: 1.3
- **Letter Spacing**: 0.03em
- **Use**: Badges, tags, timestamps, small notifications

### Responsive Typography

Type scales respond to viewport width:

| Breakpoint | Display | Headline | Title | Body |
|-----------|---------|----------|-------|------|
| Mobile (11") | 36px | 24px | 18px | 14px |
| Tablet (14") | 44px | 32px | 22px | 15px |
| Desktop (24"–27") | 60px | 40px | 24px | 16px |
| Ultrawide (>30") | 72px | 48px | 28px | 18px |

**Implementation**: CSS `clamp()` for fluid typography. No jarring jumps between breakpoints.

### Text Hierarchy

Text hierarchy is enforced through:
1. **Size** (primary signal)
2. **Weight** (secondary signal: 400 vs 600)
3. **Color** (tertiary signal: foreground vs surfaceVariant)
4. **Opacity** (quaternary signal: reduced opacity for hints)

**Never use italics for hierarchy.** Italics reserved for emphasis (quotes, citations, warnings).

### Internationalization (i18n)

Aurora typography must support:
- **Latin scripts** (Western European languages)
- **CJK** (Chinese, Japanese, Korean)
- **RTL scripts** (Arabic, Hebrew)
- **Complex scripts** (Devanagari, Thai)
- **Variable-width scripts** (different optimal line lengths)

**Line height adjustment by script**:
- Latin: 1.5x (tight, efficient)
- CJK: 1.6x (additional breathing room)
- RTL: 1.5x (same as Latin)

**Optimal line length by script**:
- Latin: 65–75 characters
- CJK: 45–55 characters (due to character width)

## Color System

### Theme Foundation

Aurora supports four distinct themes:

#### Light Theme (Default)
Primary use: Daytime, well-lit environments. Optimized for readability in bright light.

#### Dark Theme
Primary use: Low-light environments, OLED displays. Reduced eye strain in dark conditions.

#### OLED Theme
Primary use: OLED displays with true blacks. Maximizes battery life on OLED hardware.

#### HDR Theme (Future)
Primary use: HDR-capable displays with extended color gamut. Vibrant, rich color expression.

### Semantic Color Tokens

Applications never reference raw colors. Only semantic tokens:

#### Surface Tokens (Interactive Elements)
- `surface` — Default background for buttons, cards, input fields
- `surfaceVariant` — Secondary surface, lower priority (sidebars, secondary cards)
- `surfaceInverse` — Inverted surface for high-contrast modes

#### Background Tokens (Large Areas)
- `background` — Canvas, primary screen background
- `backgroundSecondary` — Secondary backgrounds (alternate sections)

#### Foreground Tokens (Text)
- `foreground` — Primary text color
- `foregroundSecondary` — Secondary text (captions, hints)
- `foregroundTertiary` — Tertiary text (disabled, very low priority)
- `foregroundInverse` — High contrast text on colored backgrounds

#### Semantic Color Tokens (Meaning)
- `primary` — Brand color, primary actions, selected states
- `secondary` — Secondary actions, alternative interactions
- `accent` — Highlights, hover states, focus indicators
- `success` — Affirmative actions, positive outcomes (green-ish)
- `warning` — Caution, non-blocking alerts (amber-ish)
- `error` — Destructive actions, critical alerts (red-ish)
- `info` — Informational content (blue-ish)

#### Outline Token
- `outline` — Borders, dividers, separators

### Light Theme Color Values

```yaml
# Surfaces
surface: "#f5f5f5"
surfaceVariant: "#efefef"
surfaceInverse: "#1a1a1a"

# Background
background: "#ffffff"
backgroundSecondary: "#f9f9f9"

# Foreground
foreground: "#1a1a1a"
foregroundSecondary: "#616161"
foregroundTertiary: "#9e9e9e"
foregroundInverse: "#ffffff"

# Semantic
primary: "#0066cc"
secondary: "#6200ee"
accent: "#ff4081"
success: "#4caf50"
warning: "#ffc107"
error: "#f44336"
info: "#2196f3"

# Outline
outline: "#e0e0e0"
```

### Dark Theme Color Values

```yaml
surface: "#1e1e1e"
surfaceVariant: "#2a2a2a"
surfaceInverse: "#f5f5f5"

background: "#121212"
backgroundSecondary: "#1a1a1a"

foreground: "#f5f5f5"
foregroundSecondary: "#b3b3b3"
foregroundTertiary: "#757575"
foregroundInverse: "#1a1a1a"

primary: "#6eb7ff"
secondary: "#c5b3ff"
accent: "#ff80ab"
success: "#81c784"
warning: "#ffca28"
error: "#ef5350"
info: "#64b5f6"

outline: "#424242"
```

### OLED Theme Color Values

```yaml
surface: "#0d0d0d"  # Nearly black (OLED true black is #000000)
surfaceVariant: "#1a1a1a"
surfaceInverse: "#f5f5f5"

background: "#000000"  # True black
backgroundSecondary: "#0d0d0d"

foreground: "#f5f5f5"
foregroundSecondary: "#b3b3b3"
foregroundTertiary: "#757575"
foregroundInverse: "#000000"

primary: "#6eb7ff"
secondary: "#c5b3ff"
accent: "#ff80ab"
success: "#81c784"
warning: "#ffca28"
error: "#ef5350"
info: "#64b5f6"

outline: "#333333"
```

### Contrast Ratios

All colors meet or exceed WCAG AAA contrast ratios:
- Primary text on backgrounds: 7:1 minimum
- Secondary text on backgrounds: 4.5:1 minimum
- Interactive elements: 3:1 minimum

## Motion Language

### Animation Timing

Four animation durations:
- `instant` = 80ms (micro-interactions, tooltips)
- `fast` = 120ms (quick feedback, hover states)
- `normal` = 220ms (standard transitions, state changes)
- `slow` = 350ms (complex animations, entrance animations)
- `dramatic` = 500ms (page transitions, full-screen changes)

### Easing Functions

Aurora uses **spring physics** as the primary easing model, not predefined curves.

Spring configuration:
```
mass: 1
tension: 280  // Stiffness
friction: 60  // Damping
```

This produces:
- Quick settlement (feels responsive)
- Slight overshoot (feels alive)
- No oscillation (feels stable)

**Alternative easing** (for non-spring animations):
- Entrance: `cubic-bezier(0.34, 1.56, 0.64, 1)`  (bounce-in)
- Exit: `cubic-bezier(0.16, 1, 0.3, 1)`  (ease-out)
- Hover: `cubic-bezier(0.4, 0, 0.2, 1)`  (standard material curve)

### Animation Principles

1. **Purpose First** — Every animation clarifies interaction or provides feedback
2. **Velocity Aware** — Inherit momentum from gestures; don't reset to static start
3. **GPU Acceleration** — Animate `transform` and `opacity` only; avoid layout thrashing
4. **Consistent Durations** — Similar interactions use similar durations across the system
5. **Respect Preferences** — Honor `prefers-reduced-motion` system preference

### Motion Scenarios

#### Window Open
- Duration: `slow` (350ms)
- Motion: Fade in + scale from center
- Easing: Bounce-in spring
- GPU: `opacity` + `transform: scale()`

#### Window Close
- Duration: `fast` (120ms)
- Motion: Fade out + scale to center
- Easing: Quick spring
- GPU: `opacity` + `transform: scale()`

#### Button Hover
- Duration: `instant` (80ms)
- Motion: Elevation increase, color shift
- Easing: Spring
- GPU: `transform: translateZ()` (elevation shadow), `color`

#### Menu Appearance
- Duration: `fast` (120ms)
- Motion: Fade in + slide up (24px)
- Easing: Ease-out
- GPU: `opacity` + `transform: translateY()`

#### Focus Indicator
- Duration: `instant` (80ms)
- Motion: Glow animation (color pulse)
- Easing: Spring
- GPU: `box-shadow`, `color`

#### Data Loading
- Duration: Continuous
- Motion: Smooth progress bar animation (not spinning)
- Easing: Linear
- GPU: `width`, `scaleX`

## Iconography

### Grid & Sizing

Icons align to a 24×24px grid (standard desktop size).
- **Grid unit**: 2px
- **Stroke width**: 1.5px
- **Stroke cap**: Round (for end terminals)
- **Stroke join**: Round (for corners)

### Proportions & Geometry

All icons maintain consistent visual weight:
- Solid shapes fill 60–70% of the 24px canvas
- Outline icons use stroke only
- Spacing between mark elements: 2–4px minimum

### Icon Sets

**System Icons**: File, folder, close, menu, search, settings, etc. (300+ icons)
**Semantic Icons**: Success (checkmark), error (X), warning (triangle), info (i) (50+ icons)
**Application Icons**: Browser, terminal, editor, chat, notes (200+ icons)
**Action Icons**: Play, pause, forward, backward, volume, brightness (100+ icons)

### Design Specifications

- **Stroke width**: 1.5px (consistent across all icons)
- **Minimum stroke width**: 1px (for very tight elements)
- **Radius on corners**: 2px (subtle softness)
- **Pixel alignment**: Icons snap to 2px grid (no 0.5px offset)
- **Optical adjustment**: Optical weight equalization (diagonal elements appear thinner, need compensation)

## Component Architecture

### Button

**States**: Default, hover, active, disabled, loading

**Sizing**:
- **Small**: 32px height, 8px vertical padding, 12px horizontal padding
- **Medium**: 40px height, 10px vertical padding, 16px horizontal padding
- **Large**: 48px height, 12px vertical padding, 20px horizontal padding

**Variants**:
- **Filled** (primary): `background: primary`, `foreground: white`
- **Tinted** (secondary): `background: primaryLight`, `foreground: primary`
- **Outlined**: `border: 1px primary`, `foreground: primary`
- **Ghost** (tertiary): No background, `foreground: primary`

### Card

**Structure**: Container with elevation, padding, border radius

**Specifications**:
- **Padding**: 16px (md spacing)
- **Radius**: 12px (md radius)
- **Elevation**: level2 (default), level3 (hover)
- **Background**: `surface`

**Variants**:
- **Filled**: `background: surface`
- **Outlined**: `border: 1px outline`, `background: background`

### Input Field

**States**: Default, focus, filled, error, disabled

**Specifications**:
- **Height**: 40px
- **Padding**: 12px horizontal, 10px vertical
- **Border radius**: 8px
- **Border**: 1px `outline` (default), 2px `primary` (focus)
- **Font**: Body (14px)

### Dialog

**Specifications**:
- **Min width**: 320px, max width: 520px
- **Padding**: 24px (xl spacing)
- **Radius**: 16px (lg radius)
- **Elevation**: level5
- **Backdrop**: 30% black overlay

### Tooltip

**Specifications**:
- **Padding**: 8px 12px
- **Radius**: 6px
- **Font**: Caption (12px, 500 weight)
- **Elevation**: level1
- **Delay**: 200ms before appearance
- **Duration**: 80ms entrance, instant exit

## Accessibility Considerations

### Color Alone

Never convey information through color alone. Always pair color with:
- Icon (success = checkmark + green)
- Text label (error = "Error" text + red)
- Pattern (disabled = color + reduced opacity)

### Contrast

- Text on background: 7:1 minimum (WCAG AAA)
- Interactive elements: 3:1 minimum
- Graphics: 3:1 minimum

### Focus Indicators

Every interactive element must have a visible focus indicator:
- Focus color: `accent` (typically contrasting with background)
- Minimum thickness: 2px
- Offset from element: 2px
- Shape: Follows element shape (rounded if element is rounded)

### Reduced Motion

Respect `prefers-reduced-motion: reduce`:
- Spring animations → instant state change
- Entrance animations → no animation
- Scroll animations → instant scroll
- Auto-play animations → paused

### Screen Reader Support

- All interactive elements: Semantic HTML + ARIA labels
- Icons: `aria-label` or hidden labels
- Form fields: Associated `<label>` elements
- Status updates: `aria-live` regions
- Navigation: Landmark roles (`<nav>`, `<main>`, `<aside>`)

---

This design language is **living documentation**. As Aurora evolves, this spec evolves with it.
