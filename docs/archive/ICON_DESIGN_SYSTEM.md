# Aurora Icon Design System

**Version**: v1.0.0  
**Status**: Specification (v1.1 implementation)  
**Target**: 1000+ icons with personality and charm

---

## Vision

**Icons that bring joy to every interaction.**

Aurora icons are not just symbols—they're **micro-experiences** that add charm, clarity, and delight to GNOME applications. Each icon is crafted with:

- ✨ **Visual warmth** (personality, not corporate coldness)
- 🎨 **Color intelligence** (semantic meaning, theme-aware)
- 💎 **Refined craftsmanship** (geometric precision with human touch)
- ♿ **Accessibility** (clear at all sizes, high contrast)
- 🌈 **Motion-ready** (designed for animation and transitions)

---

## Design Philosophy

### 1. Personality, Not Perfection

Aurora icons have **character**. They're:
- ✅ Warm and approachable (not sterile)
- ✅ Crafted with intention (not automated)
- ✅ Playful where appropriate (not boring)
- ✅ Professional in context (not cartoonish)

**Example:** A "download" icon isn't just an arrow—it's a confident, flowing gesture.

### 2. Color as Meaning

Icons use **semantic colors**, not arbitrary choices:
- 🟦 **Primary** (action, navigation, primary functions)
- 🟩 **Success** (completed, saved, confirmed)
- 🔴 **Error** (warnings, failed operations, destructive actions)
- 🟨 **Warning** (caution, attention needed)
- 🔵 **Info** (information, help, documentation)
- ⚫ **Neutral** (secondary actions, disabled states)

### 3. Geometric Grace

Icons are built on:
- 📐 **2px grid** (perfect alignment at any size)
- 🟦 **Rounded corners** (uses Aurora's radius scale)
- 🎯 **Optical centering** (perfectly centered, visually balanced)
- 📏 **Consistent stroke weight** (1.5px for 24x24)

### 4. Scalability

Icons work beautifully at **any size**:
- 🔹 **16px** — Toolbar icons (tight detail, bold strokes)
- 🔹 **24px** — Standard everywhere (primary size)
- 🔹 **32px** — Dialog icons (more detail visible)
- 🔹 **48px** — Badges, status (even more refined)
- 🔹 **64px+** — App icons (can include subtle effects)

---

## Icon Categories

### 1. **Navigation Icons** (50+ icons)
- ↪️ Back, Forward, Up, Down
- 🏠 Home, Dashboard
- ⚙️ Settings, Preferences
- 🔍 Search, Find
- 📂 Folders, Files
- ☰ Menu, Sidebar Toggle
- ❌ Close, Dismiss
- ✓ Confirm, Accept

**Color**: Primary (semantic action)  
**Charm**: Confident, flowing gestures  
**Animation**: Slide, rotate, fade

---

### 2. **Action Icons** (80+ icons)
- 💾 Save, Export
- 📋 Copy, Duplicate
- 🔗 Link, Share
- 🗑️ Delete, Remove
- 🔄 Refresh, Reload, Sync
- ⏹️ Play, Pause, Stop
- 📥 Download, Upload
- 🖨️ Print
- 🎨 Edit, Draw
- 📝 Compose, Write
- 🔐 Lock, Unlock

**Color**: Primary or semantic (Success for save, Error for delete)  
**Charm**: Dynamic, purposeful movement  
**Animation**: Scale, pulse, or rotate

---

### 3. **Status Icons** (60+ icons)
- ✓ Success, Completed
- ✕ Error, Failed
- ⚠️ Warning, Caution
- ℹ️ Info, Help
- ⏳ Loading, Processing
- 🔔 Notification, Alert
- 📡 Connected, Disconnected
- 🔋 Battery, Power
- 📶 Signal, Strength
- 🔊 Volume, Sound

**Color**: Semantic (Success, Error, Warning, Info)  
**Charm**: Trustworthy, clear at a glance  
**Animation**: Pulse, animate on status change

---

### 4. **Media Icons** (120+ icons)
- 🎵 Music, Audio
- 📹 Video, Camera
- 🖼️ Image, Picture
- 📊 Chart, Graph
- 📅 Calendar, Date
- ⏰ Clock, Timer
- 📱 Phone, Mobile
- 💻 Computer, Laptop
- 🖥️ Monitor, Screen
- 📡 Radio, Broadcast

**Color**: Primary or themed  
**Charm**: Instantly recognizable  
**Animation**: Contextual (play button → play circle)

---

### 5. **Application Icons** (200+ icons)
- 📧 Email, Mail
- 📞 Phone, Contacts
- 📚 Books, Library
- 🎓 Learning, Education
- 💼 Work, Business
- 🏦 Finance, Banking
- 🛒 Shopping, Commerce
- 🎮 Games, Entertainment
- 🎨 Creative, Design
- 📊 Data, Analytics

**Color**: Brand-aware with semantic overlay  
**Charm**: App personality while keeping GNOME consistency  
**Animation**: Subtle entrance effects

---

### 6. **System Icons** (200+ icons)
- 🔔 Notifications
- 🌙 Dark Mode
- ☀️ Light Mode
- 🌐 Internet, Network
- 🔐 Security, Privacy
- 👤 User, Account
- 🗂️ Folder, Directory
- 🔍 Magnifying Glass
- 📌 Pin, Bookmark
- ⭐ Favorite, Star

**Color**: Semantic or neutral  
**Charm**: Clear, intuitive, universally understood  
**Animation**: Simple, non-distracting

---

## Color Enhancement Techniques

### Technique 1: Semantic Color Base

```
✅ Success Icon (Checkmark)
├─ Fill: Success color (#81C784)
├─ Stroke: Success dark (#4CAF50)
└─ Background: Success light (#C8E6C9)
```

**Rules:**
- Use semantic colors as primary
- Add darker variant for stroke/outline
- Add lighter variant for background/container
- Never use arbitrary colors

---

### Technique 2: Gradient Charm (Subtle)

For larger icons (48px+), subtle gradients add depth:

```
📥 Download Icon (Gradient)
├─ Top: Primary color (solid)
├─ Bottom: Primary darker (1-15% darker)
└─ Direction: Top-to-bottom (vertical)
```

**Rules:**
- Gradients maximum 15% value shift
- Only for 48px+ icons
- Always vertical (top-to-bottom)
- Gradients must be imperceptible at 24px
- Never rainbow gradients (unprofessional)

---

### Technique 3: Depth Through Shadows

Shadows add **dimension** without complexity:

```
🎨 Art Icon with Shadow
├─ Icon shape: Solid or stroked
├─ Drop shadow: rgba(0,0,0,0.1) 0px 2px 4px
├─ Inset shadow: None (too complex)
└─ Blur radius: 4px
```

**Rules:**
- Only on 48px+ icons
- Subtle shadows (0.1 opacity max)
- 2-4px blur radius
- Never harsh or prominent
- Ensure shadow doesn't obscure details

---

### Technique 4: Color Layering

Add **visual interest** through color layers:

```
💼 Work Icon (Layered)
├─ Primary shape: Primary color (#003D99)
├─ Secondary accent: Accent color (#AA0044)
└─ Ratio: 70% primary, 30% accent
```

**Rules:**
- Maximum 2 semantic colors per icon
- Primary color dominates (70%+)
- Accent color highlights/accents only
- Both colors must pass WCAG AAA on white/dark
- Clear visual hierarchy

---

### Technique 5: Stroke Enhancement

Strategic stroke weight creates **visual balance**:

```
🔐 Lock Icon
├─ Main shape: 1.5px stroke (Aurora default)
├─ Detail lines: 1px stroke (finer)
├─ Keyhole: 0.75px stroke (finest)
└─ Fill: Semantic color or none
```

**Rules:**
- Primary stroke: 1.5px (standard)
- Detail strokes: 1px (secondary)
- Fine details: 0.75px (minimal)
- Maintain optical balance
- Never mix (0.75px + 2px = jarring)

---

### Technique 6: Animation Enhancement

Icons become **alive** with thoughtful animation:

#### Save Icon Animation
```
💾 Save → Saved Transition
├─ Icon A: Floppy disk (save action)
├─ Transition: 200ms fade + scale
├─ Icon B: Checkmark circle (confirmation)
└─ Color: Primary → Success (semantic)
```

#### Download Icon Animation
```
📥 Download → Complete
├─ Icon A: Arrow down (downloading)
├─ Transition: 400ms spring animation
├─ Icon B: Checkmark (complete)
└─ Effect: Slight bounce on completion
```

#### Loading Icons
```
⏳ Spinner (3 variations)
├─ Rotating circle (smooth spin)
├─ Rotating dots (bouncing)
└─ Pulsing indicator (fade in/out)
```

---

## Design Specifications

### Grid & Alignment

**24x24 Canvas** (primary size):
```
┌─────────────────────────────┐
│ 2px margin (safe zone)      │
│  ┌─────────────────────────┐│
│  │                         ││ 20x20 working area
│  │     Icon Design         ││
│  │     (centered)          ││
│  │                         ││
│  └─────────────────────────┘│
│ 2px margin (safe zone)      │
└─────────────────────────────┘
```

**Rules:**
- ✅ 2px margin on all sides
- ✅ 20x20px working area
- ✅ Optically centered (may be 1px off for balance)
- ✅ Snap to 2px grid
- ✅ No pixels at half-positions

### Stroke Weight by Size

| Size | Stroke | Scale Ratio |
|------|--------|------------|
| 16px | 1.25px | -17% |
| 24px | 1.5px | Standard |
| 32px | 1.75px | +17% |
| 48px | 2px | +33% |
| 64px | 2.5px | +67% |

**Formula**: `stroke = 1.5px * (size / 24)`

### Corner Radius

All rounded corners use **Aurora's radius scale**:

| Icon Size | Radius | Style |
|-----------|--------|-------|
| 16px | 2px | Xs |
| 24px | 3px | Sm |
| 32px | 4px | Md |
| 48px | 6px | Lg |
| 64px | 8px | Xl |

**Rules:**
- Never sharp corners (except geometric badges)
- Use Aurora's radius scale consistently
- Radius = size / 8 (approximately)

---

## Accessibility Requirements

### Contrast & Visibility

**Minimum Requirements:**
- ✅ WCAG AAA on white backgrounds (7:1 ratio)
- ✅ WCAG AAA on dark backgrounds (7:1 ratio)
- ✅ Visible at 16px (smallest supported size)
- ✅ Distinguishable from background
- ✅ Not color-only differentiation

**Testing:**
```bash
# Test icon contrast
1. Place icon on white (#FFFFFF)
2. Place icon on dark (#121212)
3. Check contrast ratio ≥ 7:1
4. Verify at 16px zoom
5. Test with colorblind simulator
```

### Animation Accessibility

**Respect User Preferences:**
- ✅ `prefers-reduced-motion` support
- ✅ Animations optional, not required
- ✅ No auto-playing animations
- ✅ Animation speed ≤ 200ms (max)
- ✅ No flashing (>3 times/second)

```css
/* Animation respects user preference */
@media (prefers-reduced-motion: no-preference) {
  .icon {
    animation: pulse 1s ease-in-out;
  }
}

@media (prefers-reduced-motion: reduce) {
  .icon {
    animation: none; /* Disable animations */
  }
}
```

### Semantic Meaning

**Icons must be clear without text:**
- ✅ Use established conventions (✓ = success)
- ✅ Pair with text in UI (label below icon)
- ✅ Provide `aria-label` for screen readers
- ✅ Use semantic colors where meaning is important
- ✅ Never rely on color alone (add shape variety)

```html
<!-- Accessible icon usage -->
<button aria-label="Save file">
  💾 <!-- Icon -->
</button>

<!-- Better: Icon + Text -->
<button>
  💾 Save
</button>
```

---

## Icon Format & Delivery

### SVG Specification

**Why SVG?**
- ✅ Infinitely scalable (16px to 512px)
- ✅ Small file size (200-500 bytes per icon)
- ✅ Easy to animate (CSS + JS)
- ✅ Theme-aware (can use CSS variables)
- ✅ Accessible (semantic tags)

**SVG Template:**
```xml
<svg
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 24 24"
  width="24"
  height="24"
  fill="currentColor"
  stroke="currentColor"
  stroke-width="1.5"
  stroke-linecap="round"
  stroke-linejoin="round"
>
  <!-- Icon path -->
  <path d="M12 2v20M2 12h20" />
</svg>
```

**Attributes:**
- `viewBox="0 0 24 24"` — 24x24 canvas
- `fill="currentColor"` — Inherits text color
- `stroke-width="1.5"` — Aurora standard
- `stroke-linecap="round"` — Soft edges
- `stroke-linejoin="round"` — Smooth corners

### Icon Font (Alternative)

**Fallback for compatibility:**
- ✅ TTF font format (subset of 1000+ icons)
- ✅ Named glyphs (e.g., `icon-save`)
- ✅ Same visual as SVG
- ✅ Lighter than images
- ✅ Works offline

---

## Color Palette for Icons

### Semantic Colors (Theme-Aware)

**Light Theme:**
```
Primary:    #003D99 (confident blue)
Secondary:  #440099 (mysterious purple)
Accent:     #AA0044 (vibrant magenta)
Success:    #004400 (trusty green)
Warning:    #994400 (cautious orange)
Error:      #990000 (alert red)
Info:       #0066CC (informative blue)
Neutral:    #1A1A1A (dark gray)
```

**Dark Theme:**
```
Primary:    #6EB7FF (bright blue)
Secondary:  #C5B3FF (light purple)
Accent:     #FF80AB (bright magenta)
Success:    #81C784 (bright green)
Warning:    #FFD54F (bright yellow)
Error:      #F8A29A (bright red)
Info:       #64B5F6 (bright blue)
Neutral:    #F5F5F5 (light gray)
```

**OLED Theme:**
```
Same as Dark but with deeper blacks
Background: #000000
Surface: #0D0D0D
```

### Icon Color Rules

1. **Navigation Icons** → Primary color
2. **Action Icons** → Primary or semantic
3. **Status Icons** → Always semantic (Success/Error/Warning/Info)
4. **Media Icons** → Primary or themed
5. **System Icons** → Neutral or semantic
6. **Disabled Icons** → 50% opacity neutral

---

## Icon Library Structure

```
aurora-icons/
├── src/
│   ├── svg/                  # SVG originals
│   │   ├── navigation/       # Back, Home, Settings, etc.
│   │   ├── actions/          # Save, Copy, Delete, etc.
│   │   ├── status/           # Success, Error, Warning, etc.
│   │   ├── media/            # Music, Video, Image, etc.
│   │   ├── application/      # App-specific icons
│   │   └── system/           # System icons
│   ├── fonts/
│   │   ├── aurora-icons.ttf  # Icon font
│   │   └── aurora-icons.woff2# Web font
│   └── css/
│       ├── icons.css         # Icon classes
│       └── themes.css        # Theme variables
├── figma/
│   └── aurora-icons.fig      # Figma library (editable)
├── docs/
│   ├── CONTRIBUTING.md       # How to add icons
│   ├── SHOWCASE.md          # Visual gallery
│   └── SPECIFICATIONS.md    # Technical specs
└── tests/
    ├── contrast.test.ts     # Contrast validation
    ├── accessibility.test.ts # A11y checks
    └── rendering.test.ts    # Size rendering tests
```

---

## Icon Creation Process

### Step 1: Design (Figma)

```
1. Create 24x24 artboard
2. Apply 2px grid + guides
3. Design icon with rounded corners
4. Export as SVG (clean code)
5. Test at multiple sizes
```

### Step 2: Specification

```
- Name: `icon-save`
- Category: `actions`
- Color: `Primary`
- Accessibility: `Save file`
- Animation: `None` (or specify)
```

### Step 3: Implementation

```
1. Add to SVG folder
2. Create CSS class (.icon-save)
3. Add to font (if using icon font)
4. Test contrast ratio
5. Add accessibility label
6. Document in showcase
```

### Step 4: Quality Assurance

```
✅ Contrast ≥ 7:1 (WCAG AAA)
✅ Visible at 16px
✅ Aligns to 2px grid
✅ Consistent stroke weight
✅ Proper corner radius
✅ No floating pixels
✅ Accessible labels
✅ Passes colorblind test
```

---

## Icon Animation Patterns

### Pattern 1: State Transition

```
Save Button Interaction:
1. Idle: Save icon (disk)
2. On Click: Rotate 90° (100ms)
3. Saving: Pulse (200ms)
4. Complete: Fade to checkmark (150ms)
5. Hold: Checkmark for 2s
6. Reset: Fade back to save icon (150ms)
```

### Pattern 2: Loading Indicator

```
Spinner Animation (3 variations):
1. Rotating circle (continuous 1s spin)
2. Rotating dots (3 dots, offset rotation)
3. Pulsing indicator (fade in/out 1s)
```

### Pattern 3: Toggle State

```
Volume Icon Toggle:
1. Muted: Speaker with X
2. Low: Speaker with 1 wave
3. Medium: Speaker with 2 waves
4. High: Speaker with 3 waves

Animation: Cross-fade (150ms) between states
```

---

## Charm Techniques (Personality)

### Technique 1: Micro-Details

Add small details that delight:
- 📌 Slight perspective tilt
- 🌟 Subtle highlight on success
- 💫 Slight glow on interactive elements
- 🎯 Perfectly centered but optically balanced

### Technique 2: Gesture Language

Icons use recognizable **gestures**:
- ➡️ Arrow gestures (back, forward, down)
- 👆 Pointer/hand for interactive
- 🌀 Circular for refresh/rotate
- ✋ Stop hand for important actions

### Technique 3: Warmth Through Curves

Replace hard edges with **rounded grace**:
- ✅ Rounded corners (Aurora radius scale)
- ✅ Curved strokes (no sharp angles)
- ✅ Smooth transitions (rounded linecaps)
- ✅ Flowing lines (natural curves)

### Technique 4: Visual Hierarchy

Guide user attention through **emphasis**:
- Primary element: Bolder, more saturated
- Secondary element: Lighter, less saturated
- Background: Minimal, supporting

---

## Testing & Quality

### Contrast Testing

```bash
# Use WCAG Contrast Checker
1. Export icon as PNG
2. Place on white background
3. Check contrast ratio ≥ 7:1
4. Place on dark background
5. Verify contrast ≥ 7:1
```

### Size Testing

```bash
# Visual audit at multiple sizes
16px  → Small toolbar icons (tight detail)
24px  → Standard (primary use)
32px  → Dialog headers
48px  → Badges, badges
64px+ → App icons (can show more detail)
```

### Colorblind Testing

```bash
# Simulate colorblind vision
- Deuteranopia (red-green, 1% males)
- Protanopia (red-green, 1% males)
- Tritanopia (blue-yellow, 0.001%)
- Monochromacy (complete colorblindness, rare)

Tools: Color Blindness Simulator, Coblis
```

---

## Icon Showcase & Documentation

### Live Gallery

**Interactive showcase showing:**
- ✅ All 1000+ icons at multiple sizes
- ✅ Each icon with semantic color
- ✅ Hover effects and animations
- ✅ Copy SVG code
- ✅ Copy icon font class
- ✅ Accessibility labels
- ✅ Contrast ratio verification
- ✅ Download options

### Figma Library

**Open Figma file with:**
- ✅ All icons organized by category
- ✅ Variants for each size (16, 24, 32, 48, 64)
- ✅ Color variants (light/dark/semantic)
- ✅ Animation specifications
- ✅ Accessibility notes
- ✅ Usage guidelines
- ✅ Drag-and-drop into designs

---

## Roadmap

### v1.1 (Jan-Mar 2027)
- ✅ 300 core icons (most essential)
- ✅ SVG library + icon font
- ✅ Live showcase website
- ✅ Figma library
- ✅ Animation specs for 50 icons

### v1.2 (Apr-Jun 2027)
- ✅ 1000+ total icons
- ✅ All animations implemented
- ✅ Icon editor (web-based)
- ✅ API for icon lookup
- ✅ Community contributions accepted

### v1.3+ (Jul 2027+)
- ✅ Custom icon generator (AI-assisted)
- ✅ Icon packs (domain-specific)
- ✅ Brand color customization
- ✅ Animation builder (no-code)

---

## Contributing Icons

### To add a new icon:

1. **Design** in Figma (24x24 grid)
2. **Export** as clean SVG
3. **Create PR** with SVG + metadata
4. **Test** contrast and accessibility
5. **Showcase** in gallery
6. **Celebrate** your contribution! 🎉

---

**Aurora Icons: Bringing joy, clarity, and charm to GNOME. One pixel at a time.** ✨

---

**Last Updated**: August 1, 2026  
**Aurora Version**: v1.0.0  
**Status**: Icon System Specification (v1.1 Implementation Ready)
