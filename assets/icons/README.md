# Icon Assets

This directory should contain the application icons in various formats.

## Required Files

- `icon.png` - 512x512 source icon (PNG with transparency)
- `icon.ico` - Windows icon (multi-resolution: 16, 32, 48, 256)
- `icon.icns` - macOS icon (multi-resolution)
- `icon-192.png` - Android launcher icon
- `icon-48.png` - Small icon for Linux

## Generating Icons

From a 512x512 source PNG:

```bash
# Windows .ico (requires ImageMagick)
convert icon.png -define icon:auto-resize=256,48,32,16 icon.ico

# macOS .icns (requires iconutil on macOS)
mkdir icon.iconset
sips -z 16 16 icon.png --out icon.iconset/icon_16x16.png
sips -z 32 32 icon.png --out icon.iconset/icon_16x16@2x.png
sips -z 32 32 icon.png --out icon.iconset/icon_32x32.png
sips -z 64 64 icon.png --out icon.iconset/icon_32x32@2x.png
sips -z 128 128 icon.png --out icon.iconset/icon_128x128.png
sips -z 256 256 icon.png --out icon.iconset/icon_128x128@2x.png
sips -z 256 256 icon.png --out icon.iconset/icon_256x256.png
sips -z 512 512 icon.png --out icon.iconset/icon_256x256@2x.png
sips -z 512 512 icon.png --out icon.iconset/icon_512x512.png
sips -z 1024 1024 icon.png --out icon.iconset/icon_512x512@2x.png
iconutil -c icns icon.iconset

# Android sizes
convert icon.png -resize 192x192 icon-192.png
convert icon.png -resize 48x48 icon-48.png
```

## Placeholder

Until custom icons are created, the build workflows will generate placeholder icons automatically.
