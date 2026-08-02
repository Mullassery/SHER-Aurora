# Aurora Package Control File Examples

Reference examples for all Aurora sub-packages.

---

## aurora-themes/debian/control

```
Package: aurora-themes
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}
Recommends: aurora-icons, aurora-cursors
Suggests: aurora-fonts, aurora-accessibility

Description: Aurora Linux Design System — GTK/Qt/Plasma themes
 Aurora themes provide a cohesive visual experience across GTK, Qt, and
 KDE Plasma environments.
 .
 Includes:
  • GTK 3 and GTK 4 themes
  • Qt 5 and Qt 6 platform styles
  • KDE Plasma color schemes
 .
 Variants:
  • Aurora (Auto light/dark)
  • Aurora Light (Always light)
  • Aurora Dark (Always dark)
```

---

## aurora-icons/debian/control

```
Package: aurora-icons
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}
Recommends: aurora-themes, aurora-cursors
Suggests: aurora-accessibility

Description: Aurora Linux Design System — Icon theme
 High-quality icon theme set for Aurora, compatible with GNOME, KDE Plasma,
 Xfce, and other desktop environments.
 .
 Features:
  • 2000+ icons in multiple sizes (16px–512px)
  • Light and dark variants
  • Scalable (SVG) and raster formats
  • Symbolic icon set for system indicators
  • Fallback chains for missing icons
```

---

## aurora-fonts/debian/control

```
Package: aurora-fonts
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}, fontconfig

Recommends: aurora-themes

Description: Aurora Linux Design System — Typography
 Carefully curated font collection for Aurora, including:
  • Aurora Display (headings, UI)
  • Aurora Text (body text, long-form reading)
  • Aurora Mono (code editors, terminals)
  • Aurora Icons (font-based icons, fallback)
 .
 All fonts are licensed under open-source licenses (OFL-1.1, Apache 2.0).
```

---

## aurora-kde-themes/debian/control

```
Package: aurora-kde-themes
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}
Recommends: aurora-icons, aurora-cursors, aurora-fonts
Suggests: aurora-sddm, aurora-kde-integration

Description: Aurora Linux Design System — Full KDE Plasma theme
 Complete Aurora theme for KDE Plasma including:
  • Workspace theme (window decorations, panels)
  • Color scheme (UI colors, syntax highlighting)
  • Cursors
  • Icons
  • Splash screen
  • SDDM login theme (optional)
 .
 Provides seamless Aurora experience across entire KDE Plasma desktop.
```

---

## aurora-sddm/debian/control

```
Package: aurora-sddm
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}, sddm

Recommends: aurora-kde-themes

Description: Aurora Linux Design System — KDE SDDM login theme
 Aurora theme for KDE's Simple Desktop Display Manager (SDDM),
 used on KDE Plasma login screen.
 .
 Features:
  • Aurora color scheme integrated into login screen
  • Consistent user experience from login to desktop
  • Dark and light variants
```

---

## aurora-gdm/debian/control

```
Package: aurora-gdm
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}, gdm3 | gdm

Recommends: aurora-themes

Description: Aurora Linux Design System — GNOME login theme
 Aurora theme for GNOME Display Manager (GDM), used on GNOME login screen.
 .
 Features:
  • Aurora colors integrated into GNOME login experience
  • Seamless transition from login to GNOME desktop
  • Respects system dark/light mode preference
```

---

## aurora-vscode/debian/control

```
Package: aurora-vscode
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}

Suggests: code | code-oss, aurora-fonts

Description: Aurora Linux Design System — VS Code color theme
 Aurora color theme for Visual Studio Code and VS Code derivatives (Code OSS).
 .
 Features:
  • Carefully balanced colors for extended coding sessions
  • Syntax highlighting for 50+ languages
  • Light and dark variants
  • Consistent with Aurora desktop theme
```

---

## aurora-jetbrains/debian/control

```
Package: aurora-jetbrains
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}

Suggests: intellij-idea | pycharm | webstorm | clion, aurora-fonts

Description: Aurora Linux Design System — JetBrains IDE theme
 Aurora color scheme for all JetBrains IDEs:
  • IntelliJ IDEA (Java, Kotlin)
  • PyCharm (Python)
  • WebStorm (JavaScript, TypeScript)
  • CLion (C, C++)
  • GoLand (Go)
  • RubyMine (Ruby)
  • And all others in the JetBrains suite
 .
 Unified color scheme across all IDEs with Aurora design language.
```

---

## aurora-terminal-themes/debian/control

```
Package: aurora-terminal-themes
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}

Suggests: gnome-terminal | xfce4-terminal | konsole | kitty | alacritty

Description: Aurora Linux Design System — Terminal color schemes
 Aurora color schemes for popular terminal emulators:
  • GNOME Terminal
  • Xfce Terminal
  • KDE Konsole
  • Kitty
  • Alacritty
  • iTerm2
  • ZSH/Bash/Fish shell prompt themes
 .
 Installation files ready to import into each terminal's settings.
```

---

## aurora-wallpapers/debian/control

```
Package: aurora-wallpapers
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}

Recommends: aurora-themes

Description: Aurora Linux Design System — Wallpapers
 Collection of high-quality wallpapers for Aurora, available in:
  • 4K resolution (3840×2160)
  • Full HD (1920×1080)
  • Ultrawide (3440×1440)
  • Mobile (various sizes)
 .
 Includes light, dark, and auto-switching variants.
```

---

## aurora-colors/debian/control

```
Package: aurora-colors
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}

Description: Aurora Linux Design System — Design tokens & palettes
 Color definitions and design tokens for Aurora, provided in multiple formats:
  • CSS variables (`:root { --aurora-primary: ... }`)
  • SCSS/LESS variables
  • JSON/YAML configuration
  • Tailwind CSS preset
  • Color palette images
 .
 Enables consistent color usage across web and desktop applications.
```

---

## aurora-branding/debian/control

```
Package: aurora-branding
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}

Recommends: aurora-fonts

Description: Aurora Linux Design System — Brand assets
 Official Aurora brand materials including:
  • Logo files (SVG, PNG)
  • Brand guidelines
  • Media kit
  • Typography specifications
  • Color palette reference
  • Usage guidelines
 .
 For use in community projects, distributions, and integrations.
```

---

## aurora-accessibility/debian/control

```
Package: aurora-accessibility
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}

Recommends: aurora-themes, aurora-fonts

Description: Aurora Linux Design System — Accessibility variants
 Specialized Aurora variants for users with accessibility needs:
  • High contrast (≥7:1 WCAG AA compliance)
  • Dyslexia-friendly (sans-serif, spacing, colors)
  • Large print (increased UI sizes, text)
  • Colorblind-safe (deuteranopia, protanopia compatible)
 .
 All variants maintain Aurora design principles while improving readability
 and usability for users with visual processing differences.
```

---

## aurora-gnome-integration/debian/control

```
Package: aurora-gnome-integration
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}, gnome-shell (>= 40)

Recommends: aurora-themes, aurora-icons, aurora-fonts, aurora-wallpapers

Suggests: gnome-tweaks, dconf-editor

Description: Aurora Linux Design System — GNOME Shell integration
 Deep integration with GNOME Shell including:
  • GSettings schemas for Aurora-specific options
  • GNOME Shell theme components
  • Activities overview styling
  • Notification center theming
  • Desktop icons styling
 .
 Provides seamless Aurora experience across entire GNOME desktop.
```

---

## aurora-kde-integration/debian/control

```
Package: aurora-kde-integration
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}, plasma-framework (>= 5.80)

Recommends: aurora-kde-themes, aurora-icons, aurora-sddm

Suggests: kde-cli-tools, systemsettings

Description: Aurora Linux Design System — KDE Plasma integration
 Deep integration with KDE Plasma including:
  • Look-and-feel package (complete workspace theme)
  • KDE Service menus for right-click actions
  • Plasma customization presets
  • Activity defaults
  • Application defaults
 .
 Provides Aurora as a complete KDE Plasma look-and-feel package.
```

---

## aurora-plymouth/debian/control

```
Package: aurora-plymouth
Version: 1.0.0
Architecture: all
Maintainer: Aurora Team <aurora@example.com>
Homepage: https://github.com/aurora-linux/aurora
Vcs-Git: https://github.com/aurora-linux/aurora.git
Built-Using: debhelper (= 13.11.6ubuntu1)
Standards-Version: 4.6.2
Priority: optional

Depends: ${misc:Depends}, plymouth (>= 0.9.4)

Description: Aurora Linux Design System — Plymouth boot splash
 Aurora-themed Plymouth boot splash screen shown during system boot.
 .
 Features:
  • Smooth animated boot sequence
  • Aurora color scheme
  • Responsive to boot events (mounting, starting services)
  • Works with encrypted drives and password prompts
```

---

## How to Use These Templates

1. **For each package**, create a directory:
   ```
   aurora-themes/
   ├── debian/
   │   ├── control          (use template above)
   │   ├── postinst
   │   ├── postrm
   │   ├── copyright
   │   ├── changelog
   │   └── rules
   └── [package files]
   ```

2. **Modify version/maintainer** as needed

3. **Adjust dependencies** based on actual requirements

4. **Build package**:
   ```bash
   cd aurora-themes
   dpkg-buildpackage -us -uc
   ```

5. **Test installation**:
   ```bash
   sudo dpkg -i ../aurora-themes_1.0.0_all.deb
   dpkg -L aurora-themes  # Verify files
   dpkg -r aurora-themes  # Test removal
   ```

---

**Use these templates as the foundation for Aurora's complete package ecosystem.**
