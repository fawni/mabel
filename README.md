# mabel 🍁

Declarative pixel art. Mabel lets you create tiny pixel art images by writing pixels as characters in a text file!

Here’s a little example that outputs the infamous CS Source missing texture pattern:

``` eno
size: 64
palette:
c = #000000
s = #ff00dc

-- pixels
cscscscs
scscscsc
cscscscs
scscscsc
cscscscs
scscscsc
cscscscs
scscscsc
-- pixels
```

for an in-depth explanation checkout the [Format](#format) section.

## Installation

### crates.io

    cargo install mabel

## Usage

    mabel <input.eno> [-o output.png]

`mabel -h` for more information.

### Format

mabel uses the [Eno](https://eno-lang.org) data language, you can learn more about the Eno format
[here](https://eno-lang.org/guide).

| Name | Type | Required | Notes |
|----|----|----|----|
| size | u8 | No | The size of each pixel in the image. |
| width | u32 | No | The amount of pixels in the x-axis. |
| height | u32 | No | The amount of pixels in the y-axis. |
| palette | [Fieldset](https://eno-lang.org/guide/elements/fieldsets) | No | The color palette. Keys are the characters and values are the colors. Keys must be one character long. Colors can be anything that [color-art](https://color-art.netlify.app/guide/usage.html) supports. |
| pixels | [Multiline Field](https://eno-lang.org/guide/elements/multiline-fields) | Yes | The image data. Spaces and empty lines are transparent. Characters must be defined in `palette`. |

To see some examples, check out the [examples](examples) directory.

### Aseprite

mabel can convert an aseprite file (`.ase`/`.aseprite`) into Eno; allowing you to edit aseprite files with mabel.

    mabel aseprite <input.ase> [-o output.eno]

This feature is gated behind an `aseprite` feature flag. Enabled by default.

## Acknowledgements

- [tep](https://github.com/gennyble/tep)
- [enolib-rs](https://codeberg.org/simonrepp/enolib-rs)
- [asefile](https://github.com/alpine-alpaca/asefile)

## License

[Apache 2.0](LICENSE)
