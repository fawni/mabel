# mabel 🍁

Declarative pixel art 🎨

Sometime ago i found a really cute project called [tep](https://github.com/gennyble/tep) that generates images based on its own `.tep` format! I had problems with it that prevented me from actually using it, namely that you cannot control the scale of each "pixel".

Mabel uses the same approach to generate simple pixel images in a declarative, reproducible manner!

## Installation

### crates.io

```
cargo install mabel
```

## Usage

Mabel uses the [eno](https://eno-lang.org) data language. Initially i was going to write my own format, but i found eno and a ready parser for it in rust. It is quiet similar to what i had planned, only better. You can learn more about eno [here](https://eno-lang.org/guide).



| Name    | Type                                                                    | Required | Notes                                                                                                                                                                                                   |
| ------- | ----------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| size    | u8                                                                      | No       | The size of each pixel in the image.                                                                                                                                                                    |
| width   | u32                                                                     | No       | The amount of pixels in the x-axis.                                                                                                                                                                     |
| height  | u32                                                                     | No       | The amount of pixels in the y-axis.                                                                                                                                                                     |
| palette | [Fieldset](https://eno-lang.org/guide/elements/fieldsets)               | No       | The color palette. Keys are the characters and values are the colors. Keys must be one character long. Colors can be anything that [color-art](https://color-art.netlify.app/guide/usage.html) supports. |
| pixels  | [Multiline Field](https://eno-lang.org/guide/elements/multiline-fields) | Yes      | The image data. Spaces and empty lines are transparent. Characters must be defined in `palette`.                                                                                                        |

To see some examples, check out the [examples](examples) directory.

```
mabel <input.eno> [-o output.png]
```

`mabel -h` for more information.

## Acknowledgements

- [tep](https://github.com/gennyble/tep)
- [enolib-rs](https://codeberg.org/simonrepp/enolib-rs)

## License

[Apache 2.0](LICENSE)
