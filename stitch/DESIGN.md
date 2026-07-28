---
name: Rust Industrial Functional
colors:
  surface: '#131313'
  surface-dim: '#131313'
  surface-bright: '#3a3939'
  surface-container-lowest: '#0e0e0e'
  surface-container-low: '#1c1b1b'
  surface-container: '#201f1f'
  surface-container-high: '#2a2a2a'
  surface-container-highest: '#353534'
  on-surface: '#e5e2e1'
  on-surface-variant: '#e2bfb8'
  inverse-surface: '#e5e2e1'
  inverse-on-surface: '#313030'
  outline: '#a98a84'
  outline-variant: '#5a413c'
  surface-tint: '#ffb4a6'
  primary: '#ffb4a6'
  on-primary: '#660700'
  primary-container: '#ce412b'
  on-primary-container: '#fff8f7'
  inverse-primary: '#b12c19'
  secondary: '#c8c6c5'
  on-secondary: '#303030'
  secondary-container: '#474746'
  on-secondary-container: '#b7b5b4'
  tertiary: '#81cfff'
  on-tertiary: '#00344b'
  tertiary-container: '#007baa'
  on-tertiary-container: '#f6faff'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#ffdad4'
  primary-fixed-dim: '#ffb4a6'
  on-primary-fixed: '#3f0300'
  on-primary-fixed-variant: '#8f1202'
  secondary-fixed: '#e5e2e1'
  secondary-fixed-dim: '#c8c6c5'
  on-secondary-fixed: '#1b1b1c'
  on-secondary-fixed-variant: '#474746'
  tertiary-fixed: '#c6e7ff'
  tertiary-fixed-dim: '#81cfff'
  on-tertiary-fixed: '#001e2d'
  on-tertiary-fixed-variant: '#004c6b'
  background: '#131313'
  on-background: '#e5e2e1'
  surface-variant: '#353534'
typography:
  headline-lg:
    fontFamily: Geist
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
  headline-md:
    fontFamily: Geist
    fontSize: 18px
    fontWeight: '600'
    lineHeight: 24px
  body-base:
    fontFamily: Inter
    fontSize: 13px
    fontWeight: '400'
    lineHeight: 18px
  body-sm:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '400'
    lineHeight: 16px
  label-caps:
    fontFamily: Inter
    fontSize: 11px
    fontWeight: '700'
    lineHeight: 14px
    letterSpacing: 0.05em
  mono-data:
    fontFamily: JetBrains Mono
    fontSize: 12px
    fontWeight: '400'
    lineHeight: 16px
spacing:
  rail_width: 56px
  menu_width: 200px
  top_bar_height: 48px
  gutter: 8px
  pane_padding: 16px
  row_height_dense: 28px
---

## Brand & Style

The design system is engineered for high-utility industrial applications, reflecting the safety and performance of the Rust programming language. It prioritizes information density and structural clarity over decorative flair, making it ideal for ERP systems where users manage complex data for extended periods.

The aesthetic follows a **Modern Corporate** style with a **Minimalist / Industrial** edge. It utilizes a predominantly monochromatic foundation to reduce cognitive load, punctuated by a singular, high-visibility primary color for intent and action. Key characteristics include:
- **Calm & Focused:** A dark-mode default provides a stable, low-strain environment for professional data entry.
- **Dense & Systematic:** Compact vertical rhythm and precise alignment ensure maximum data visibility.
- **Rectilinear Precision:** Sharp corners and hairline borders create a sense of architectural stability and "egui" compatibility.

## Colors

The color palette is grounded in a deep charcoal and black foundation to maintain a "snug" and professional feel. 

- **Primary (Rust Orange):** Used sparingly for primary actions, active state indicators (rail selection), and critical alerts.
- **Neutral Stack:** A range of grays defines the hierarchy of the interface. The deepest black is reserved for the background, while slightly lighter tones (`#1F1F1F`) define the Icon Rail and Domain Menu.
- **Semantic Colors:** Success, warning, and error states follow standard utility conventions but use slightly desaturated tones to match the professional atmosphere.

## Typography

Typography in this design system is optimized for legibility at small sizes. **Inter** provides the backbone for general UI, while **Geist** is used for structural headers to add a subtle technical character.

- **Scale:** Small base font sizes (13px) allow for high data density in tables and forms.
- **Data Display:** For financial figures or SKU codes, a monospaced font (JetBrains Mono) is used to ensure column alignment in tables.
- **Hierarchy:** Use all-caps labels for section headers within menus to differentiate navigation tiers from content.

## Layout & Spacing

The layout is a rigid, multi-panel frame designed for complex workflows. It utilizes a three-tier navigation architecture:

1.  **Icon Rail (56px):** Global domains (e.g., Inventory, Finance, CRM). Persistent on the left.
2.  **Domain Menu (200px):** Contextual sub-navigation. Can be collapsed to an icon-only view if needed.
3.  **Top Bar (48px):** Reserved for global search, white-label logo, system messages, and user profiles.

**Spacing Rhythm:**
- A strict **4px/8px grid** is enforced. 
- Content density is high; list rows should not exceed 32px in height to maximize visibility of table data.
- Margins between panels are minimal (1px borders or subtle 4px insets) to keep the UI "snug."

## Elevation & Depth

This design system avoids traditional drop shadows to maintain a flat, industrial aesthetic compatible with immediate-mode rendering.

- **Tonal Layering:** Depth is communicated through color. The further "back" a surface is, the darker it is. 
  - *Background:* #0F0F0F
  - *Panels/Rail:* #1F1F1F
  - *Inputs/Active Elements:* #2D2D2D
- **Inset Borders:** To create separation between the Icon Rail and the Domain Menu, a 1px vertical border in a lighter gray (#2D2D2D) is used.
- **Focus States:** Instead of a shadow, use a 1px solid Rust Orange outline for active input focus or selected rail icons.

## Shapes

The design system is strictly **rectilinear**. 

- **Sharp Edges:** All buttons, inputs, and panels use 0px border-radius. This aligns with the "egui" visual language and reinforces the industrial, functional persona.
- **Separators:** Horizontal and vertical dividers should be 1px solid lines. Avoid thick gutters; use hair-line borders to maximize screen real estate.

## Components

### Buttons
- **Primary:** Solid Rust Orange (#CE412B) with white text. Sharp corners.
- **Secondary:** Ghost style with a 1px #2D2D2D border. 
- **Sizes:** Compact height (28px for standard, 24px for small/inline).

### Inputs & Fields
- **Background:** #151515.
- **Border:** 1px #2D2D2D. On focus, border changes to Rust Orange.
- **Labels:** Positioned above the field in `label-caps` typography to save horizontal space.

### Data Tables
- **Header:** Background #1F1F1F, bold text, 1px bottom border.
- **Rows:** Alternate background colors (zebra striping) are optional; 1px subtle borders are preferred.
- **Selection:** A 2px vertical Rust Orange bar on the far left of the row indicates selection.

### Navigation Rail
- **Icons:** Minimalist line icons. 20px size.
- **Active State:** The background of the icon slot remains #1F1F1F, but a 3px Rust Orange vertical indicator appears on the left edge.

### Tabs
- Horizontal tabs appear below the page header. 
- Active tab is indicated by a bottom-border in Rust Orange, no rounded corners.