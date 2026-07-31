# Aurora Component Specifications

## Overview

Aurora components are the building blocks of applications. Each component is defined by:

1. **Structure** — HTML/semantic hierarchy
2. **Spacing** — Internal padding, margin
3. **Typography** — Font size, weight, line height
4. **Color** — Background, text, border (from semantic tokens)
5. **States** — Default, hover, active, disabled, loading, error
6. **Animation** — Transitions, motion language
7. **Accessibility** — ARIA labels, keyboard support, screen reader text

All components use **semantic design tokens** only. No hardcoded colors.

---

## Button

Clickable action element.

### Sizes

| Size | Height | Vertical Padding | Horizontal Padding | Use Case |
|------|--------|------------------|-------------------|----------|
| Small | 32px | 8px | 12px | Compact, inline actions |
| Medium | 40px | 10px | 16px | Primary actions, default |
| Large | 48px | 12px | 20px | Prominent, call-to-action |

### Variants

#### Filled (Primary)
- **Background**: `primary` semantic token
- **Text Color**: `foregroundInverse`
- **Border**: None
- **Use**: Primary, high-priority actions

**Default State**:
```
background: primary
foreground: foregroundInverse
elevation: none
```

**Hover State**:
```
background: primary (slightly lighter, +5% brightness)
elevation: level1 (subtle lift)
animation: 80ms spring
```

**Active State**:
```
background: primary (5% darker)
elevation: none
animation: instant
```

**Disabled State**:
```
background: surface
foreground: foregroundTertiary (reduced opacity)
cursor: not-allowed
```

#### Tinted (Secondary)
- **Background**: `primaryLight` (10% primary opacity on surface)
- **Text Color**: `primary`
- **Border**: None
- **Use**: Secondary actions, less important than filled

#### Outlined
- **Background**: Transparent
- **Text Color**: `primary`
- **Border**: 1px `primary`
- **Use**: Tertiary actions, alternative paths

**Hover State**:
```
background: primary (5% opacity)
border: 1px primary
```

#### Ghost (Text-Only)
- **Background**: Transparent
- **Text Color**: `primary`
- **Border**: None
- **Underline**: None (no decoration)
- **Use**: Links, lightweight actions

**Hover State**:
```
background: transparent
foreground: primary (darker)
underline: 1px primary
```

### Typography

- **Font Size**: Body (14px)
- **Font Weight**: Medium (500)
- **Text Transform**: None (preserve case)
- **Letter Spacing**: Normal (+1%)

### Interaction

- **Click feedback**: Immediate color/elevation change, 80ms animation
- **Focus indicator**: 2px `accent` glow around button, 2px offset
- **Keyboard support**: Enter/Space activates button
- **Touch target**: Minimum 44×44px (iOS guideline)

### Accessibility

```html
<button 
  class="button button--primary"
  aria-label="Submit form"
  aria-pressed="false"
>
  Submit
</button>
```

- **Semantic**: `<button>` element (not `<div onclick>`)
- **ARIA**: `aria-label` for icon-only buttons, `aria-pressed` for toggles
- **Focus**: Visible focus indicator (2px accent glow)
- **Disabled**: `disabled` attribute, visual indication, `cursor: not-allowed`

### Motion

**Press animation**:
```
scale: 0.95 → 1.0
duration: 80ms
easing: spring (snappy)
transform: scale(0.95)
opacity: 1.0
```

**Hover lift**:
```
elevation: none → level1
duration: 120ms
easing: spring
```

---

## Card

Container for content grouping.

### Structure

```
┌─ Card ─────────────────────┐
│                            │
│ ┌─ Header (optional) ──┐   │
│ │ Title                │   │
│ └────────────────────┘   │
│                            │
│ ┌─ Content ──────────────┐│
│ │ Main content area      ││
│ └────────────────────┘  │
│                            │
│ ┌─ Actions (optional) ──┐  │
│ │ Button  Button        │  │
│ └────────────────────┘  │
└─────────────────────────┘
```

### Specifications

| Property | Value |
|----------|-------|
| Padding | 16px (md) |
| Border Radius | 12px (md) |
| Background | `surface` |
| Border | None |
| Elevation | level2 (default), level3 (hover) |
| Min Height | 64px |
| Max Width | 520px (common), no limit |

### Variants

#### Filled
- **Background**: `surface`
- **Border**: None
- **Elevation**: level2

#### Outlined
- **Background**: `background`
- **Border**: 1px `outline`
- **Elevation**: None

#### Elevated (Premium)
- **Background**: `surface`
- **Border**: None
- **Elevation**: level4 (prominent)

### Hover State

- **Elevation**: level3 (increase from default)
- **Duration**: 120ms spring animation
- **Cursor**: pointer (if clickable)

### Interactions

- **Clickable Card**: Entire card is interactive (link)
- **Ripple Effect**: Optional: 200ms ripple from click point
- **Focus**: Blue outline (2px `accent`), 2px offset

### Accessibility

```html
<article 
  class="card"
  role="article"
  aria-label="Article title"
>
  <header class="card__header">
    <h3 class="card__title">Title</h3>
  </header>
  <div class="card__content">
    Content...
  </div>
  <footer class="card__actions">
    <button>Action</button>
  </footer>
</article>
```

---

## Input Field

Text input, select, textarea.

### Structure

```
┌─ Input ─────────────────────┐
│ Label                       │
│ ┌─────────────────────────┐ │
│ │ Input content      ✗ ┐  │ Focus border: 2px primary
│ └─────────────────────────┘ │
│ Hint text (optional)        │
└─────────────────────────────┘
```

### Specifications

| Property | Value |
|----------|-------|
| Height | 40px |
| Vertical Padding | 10px |
| Horizontal Padding | 12px |
| Border Radius | 8px (sm) |
| Font Size | Body (14px) |
| Border | 1px `outline` |
| Background | `surface` |

### States

**Default**:
- Border: 1px `outline`
- Background: `surface`
- Text: `foreground`

**Focus**:
- Border: 2px `primary` (thicker, more visible)
- Background: `surface`
- Outline: None (border replaces outline)
- Duration: 80ms spring

**Filled**:
- Background: `surface` (unchanged)
- Text: `foreground`
- Cursor: text

**Disabled**:
- Border: 1px `outline` (same)
- Background: `surfaceVariant` (muted)
- Text: `foregroundTertiary` (reduced opacity)
- Cursor: not-allowed

**Error**:
- Border: 2px `error` (indicates problem)
- Background: `surface`
- Error Text: `error` color below field
- Icon: Warning icon (optional)

**Success**:
- Border: 1px `success`
- Icon: Checkmark icon
- Text: `success` (below field)

### Typography

- **Font**: Body (14px)
- **Weight**: Regular (400)
- **Line Height**: 1.5x
- **Placeholder**: `foregroundSecondary` color, italic

### Label

- **Typography**: Caption (12px, 500 weight)
- **Margin**: 8px below label
- **Color**: `foreground`
- **Required Indicator**: `*` in `error` color (if required)

### Hint Text

- **Typography**: Micro (11px, 500 weight)
- **Color**: `foregroundSecondary`
- **Margin**: 4px above hint
- **Max Width**: 100% (wraps)

### Clear Button (X Icon)

- **Size**: 24×24px
- **Color**: `foregroundSecondary`
- **Hover**: `foreground`
- **Position**: Right side, 8px from edge
- **Visible**: Only when field has content
- **Animation**: Fade in/out, 80ms

### Keyboard Support

- **Focus**: Tab navigation
- **Submit**: Enter key (form submission)
- **Clear**: Escape key clears field and resets focus

### Accessibility

```html
<div class="input-group">
  <label for="email" class="input-label">
    Email Address
    <span class="required">*</span>
  </label>
  <input 
    id="email"
    type="email"
    placeholder="you@example.com"
    class="input"
    aria-label="Email address"
    aria-describedby="email-hint"
    required
  />
  <p id="email-hint" class="input-hint">
    We'll never share your email.
  </p>
</div>
```

---

## Dialog (Modal)

Full-screen overlay with centered content.

### Structure

```
┌─ Backdrop (dark overlay) ─────────────┐
│                                       │
│  ┌─ Dialog ──────────────────────┐   │
│  │ ┌─ Header ──────────────────┐ │   │
│  │ │ Title            [Close] │ │   │
│  │ └───────────────────────────┘ │   │
│  │ ┌─ Content ─────────────────┐ │   │
│  │ │ Main content              │ │   │
│  │ └───────────────────────────┘ │   │
│  │ ┌─ Actions ─────────────────┐ │   │
│  │ │ [Cancel] [Confirm]        │ │   │
│  │ └───────────────────────────┘ │   │
│  └───────────────────────────────┘   │
│                                       │
└───────────────────────────────────────┘
```

### Specifications

| Property | Value |
|----------|-------|
| Min Width | 320px |
| Max Width | 520px |
| Max Height | 90vh (viewport) |
| Padding | 24px (xl) |
| Border Radius | 16px (lg) |
| Background | `surface` |
| Elevation | level5 (full-screen overlay) |
| Backdrop | `#000000` 30% opacity |

### Header

- **Title**: Headline (32px, 600 weight)
- **Close Button**: Icon-only, top-right corner
- **Position**: Fixed at top of dialog
- **Padding**: 24px bottom (xl spacing)

### Content

- **Typography**: Body (14px)
- **Color**: `foreground`
- **Max Height**: `90vh - 200px` (accounting for header/actions)
- **Overflow**: Scrollable if content exceeds height
- **Padding**: 0 (inherited from dialog)

### Actions

- **Position**: Fixed at bottom
- **Layout**: Right-aligned buttons (Confirm on right)
- **Button Order**: [Cancel] [Confirm]
- **Spacing**: 12px between buttons
- **Padding**: 24px top (xl)
- **Border Top**: 1px `outline`

### Animations

**Entrance**:
```
opacity: 0 → 1
scale: 0.95 → 1.0
duration: 220ms
easing: spring
backdrop: 0% → 30%
```

**Exit**:
```
opacity: 1 → 0
scale: 1.0 → 0.95
duration: 120ms
easing: spring
```

### Keyboard Support

- **Focus Trap**: Tab cycles through buttons in dialog
- **Escape**: Dismisses dialog (close button behavior)
- **Enter**: Submits form (if form inside)

### Accessibility

```html
<div 
  class="dialog-backdrop"
  role="presentation"
  aria-hidden="true"
></div>

<div 
  class="dialog"
  role="dialog"
  aria-labelledby="dialog-title"
  aria-modal="true"
  focus-trap
>
  <header class="dialog__header">
    <h2 id="dialog-title" class="dialog__title">
      Confirm Action
    </h2>
    <button 
      class="dialog__close"
      aria-label="Close dialog"
    >
      ✕
    </button>
  </header>
  
  <div class="dialog__content">
    Content...
  </div>
  
  <footer class="dialog__actions">
    <button>Cancel</button>
    <button class="button--primary">Confirm</button>
  </footer>
</div>
```

---

## Tooltip

Small popup hint.

### Specifications

| Property | Value |
|----------|-------|
| Padding | 8px horizontal, 6px vertical |
| Border Radius | 6px |
| Font | Micro (12px, 500 weight) |
| Background | `foreground` |
| Text Color | `foregroundInverse` |
| Elevation | level1 |
| Max Width | 200px |
| Delay | 200ms before appearance |
| Duration | 80ms entrance, instant exit |

### Arrow

- **Size**: 6×6px triangle
- **Color**: `foreground`
- **Position**: Points to trigger element

### Animation

**Entrance**:
```
opacity: 0 → 1
scale: 0.9 → 1.0
duration: 80ms
easing: ease-out
```

**Exit**: Instant (no animation)

### Keyboard Support

- **Hover**: Shows tooltip
- **Focus**: Shows tooltip (on focusable elements)
- **Escape**: Hides tooltip

### Accessibility

- **ARIA**: `aria-describedby` links trigger to tooltip ID
- **No Information Critical**: Tooltip must not contain essential information
- **Redundant**: Use `<title>` attribute as fallback

---

## Checkbox & Radio Button

Binary choice input.

### Checkbox

- **Size**: 18×18px
- **Border**: 2px `outline`
- **Border Radius**: 4px
- **Checked**: Background `primary`, checkmark `foregroundInverse`
- **Focus**: 2px `accent` outline
- **Label**: Caption (12px) to the right

### Radio Button

- **Size**: 18×18px
- **Border**: 2px `outline`
- **Border Radius**: 9px (circle)
- **Checked**: Border `primary`, inner dot `primary` (6px)
- **Focus**: 2px `accent` outline
- **Label**: Caption (12px) to the right

### Animation

**Check/Uncheck**:
```
duration: 80ms
easing: spring
scale: 1.0 → 1.05 → 1.0 (minor pulse)
```

---

## List

Vertical sequence of related items.

### Specifications

- **Spacing**: 8px (xs) between items
- **Background**: `background` (default)
- **Item Background**: Hover: `surfaceVariant`
- **Border**: None (items separated by whitespace)
- **Padding**: 8px per item (xs)

### Dividers

- **Use**: Separate groups of items
- **Height**: 1px
- **Color**: `outline`
- **Spacing**: 8px above/below

### Accessibility

```html
<ul class="list" role="list">
  <li class="list-item" role="listitem">Item 1</li>
  <li class="list-item" role="listitem">Item 2</li>
</ul>
```

---

## Badge & Chip

Inline label/tag.

### Badge

- **Padding**: 2px horizontal (xxs), 4px vertical
- **Font**: Micro (11px, 500 weight)
- **Border Radius**: 4px
- **Background**: `accent`
- **Text**: `foregroundInverse`
- **Use**: Labels, tags, counts, status indicators

### Chip

- **Padding**: 6px horizontal (xs), 8px vertical (xs)
- **Font**: Caption (12px, 500 weight)
- **Border Radius**: 16px (fully rounded)
- **Background**: `surfaceVariant`
- **Text**: `foreground`
- **Close Icon**: Optional, right side
- **Use**: Filters, selected tags, removable items

---

## Complete Example: Form

```html
<form class="form" role="form">
  <fieldset class="form-group">
    <legend class="form-legend">Personal Information</legend>
    
    <div class="input-group">
      <label for="name" class="input-label">
        Full Name
        <span class="required">*</span>
      </label>
      <input 
        id="name"
        type="text"
        class="input"
        placeholder="John Doe"
        required
      />
    </div>
    
    <div class="input-group">
      <label for="email" class="input-label">
        Email
        <span class="required">*</span>
      </label>
      <input 
        id="email"
        type="email"
        class="input"
        placeholder="john@example.com"
        required
      />
      <p class="input-hint">We'll never share your email.</p>
    </div>
    
    <div class="checkbox-group">
      <input 
        id="agree"
        type="checkbox"
        class="checkbox"
        required
      />
      <label for="agree" class="checkbox-label">
        I agree to the terms and conditions
      </label>
    </div>
  </fieldset>
  
  <div class="form-actions">
    <button type="reset" class="button button--outlined">
      Clear
    </button>
    <button type="submit" class="button button--primary">
      Submit
    </button>
  </div>
</form>
```

---

## Accessibility Principles

All components follow:

1. **Semantic HTML** — Use proper elements (`<button>`, `<label>`, `<input>`, etc.)
2. **Keyboard Navigation** — Tab, Enter, Arrow keys, Escape all functional
3. **Focus Visible** — Clear, high-contrast focus indicators (2px `accent` glow)
4. **ARIA Labels** — `aria-label`, `aria-describedby`, `aria-pressed`, etc.
5. **Color + Icon** — Never convey info through color alone (pair with icon or text)
6. **High Contrast** — All text meets WCAG AAA (7:1)
7. **Reduced Motion** — Respect `prefers-reduced-motion`, instant animations
8. **Screen Readers** — Semantic landmarks, live regions for status updates

---

## CSS Architecture

All component styles derive from design tokens:

```css
/* Spacing */
.button {
  padding: var(--spacing-md) var(--spacing-lg);
}

/* Colors */
.button--primary {
  background-color: var(--color-primary);
  color: var(--color-foreground-inverse);
}

/* Motion */
.button {
  transition: all var(--motion-fast) cubic-bezier(...);
}

/* Typography */
.button {
  font-size: var(--font-body-size);
  font-weight: var(--font-medium);
}
```

No hardcoded values. All tokens used.

---

## Next Steps

1. **Implement Components** — Build Button, Card, Dialog, Input, etc. in GTK4 (Phase 2)
2. **Qt6 Variants** — Replicate in Qt6 with native styling
3. **Web Components** — React/Vue/Svelte component library
4. **Electron Integration** — Electron + Aurora styling
5. **Figma Library** — Design tokens in Figma, sync with code
6. **Storybook** — Interactive component gallery for designers & developers
