# mabel

Declarative pixel art.

## Installation

### crates.io

    cargo install mabel

## Usage

    mabel <input.eno> [-o output.png]

`mabel -h` for more information.

### Format

Mabel uses the [Eno](https://eno-lang.org) data language, you can learn more about Eno
[here](https://eno-lang.org/guide).

| Name    | Type                                                                    | Required | Notes                                                                                                                                                                                                    |
|---------|-------------------------------------------------------------------------|----------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| size    | u8                                                                      | No       | The size of each pixel in the image.                                                                                                                                                                     |
| width   | u32                                                                     | No       | The amount of pixels in the x-axis.                                                                                                                                                                      |
| height  | u32                                                                     | No       | The amount of pixels in the y-axis.                                                                                                                                                                      |
| palette | [Fieldset](https://eno-lang.org/guide/elements/fieldsets)               | No       | The color palette. Keys are the characters and values are the colors. Keys must be one character long. Colors can be anything that [color-art](https://color-art.netlify.app/guide/usage.html) supports. |
| pixels  | [Multiline Field](https://eno-lang.org/guide/elements/multiline-fields) | Yes      | The image data. Spaces and empty lines are transparent. Characters must be defined in `palette`.                                                                                                         |

To see some examples, check out the [examples](examples) directory.

## Acknowledgements

- [tep](https://github.com/gennyble/tep)
- [enolib-rs](https://codeberg.org/simonrepp/enolib-rs)
- [asefile](https://github.com/alpine-alpaca/asefile)

## License

[Apache 2.0](LICENSE)
