# Demo Assets

This directory should contain:

## panoramas/
- `demo.jpg` - A 4096x2048 (or 2:1 ratio) equirectangular panorama image

You can download free panoramas from:
- [Poly Haven HDRIs](https://polyhaven.com/hdris) - Convert HDR to JPG
- [360cities](https://www.360cities.net/) - Free panoramas
- [Flickr 360](https://www.flickr.com/groups/equirectangular/) - CC licensed

## models/
- `character.glb` - A humanoid 3D model

You can download free models from:
- [Mixamo](https://www.mixamo.com/) - Free with Adobe account
- [Sketchfab](https://sketchfab.com/features/free-3d-models) - CC licensed
- [Ready Player Me](https://readyplayer.me/) - Avatar generator

## audio/
- `dialogue.ogg` - Character dialogue audio

You can find free audio from:
- [Freesound](https://freesound.org/) - CC0 audio
- [OpenGameArt](https://opengameart.org/) - Game audio

## Quick Setup

```bash
# Download a sample panorama (example)
curl -o assets/panoramas/demo.jpg "https://example.com/panorama.jpg"

# Or use ImageMagick to create a test gradient
convert -size 4096x2048 gradient:blue-cyan assets/panoramas/demo.jpg
```
