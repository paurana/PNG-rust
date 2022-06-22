# PNG-rust
A simple CLI tool encode and decode secret messages in PNG files. 

## Project Idea
I came across [png-me](https://picklenerd.github.io/pngme_book/) and decided to give it a shot. The project has no extra features other than the ones
specified in the challenge, this is my own but exact implementation of the same.

## Setup
1. Clone the repository to your computer.
2. Navigate to the **parent** folder in a terminal by `cd path/to/repo` and then `cd ..`
3. Run `cargo install --path=pngrust` in the parent folder.

**That's it, you're good to go!**

## Usage
The tool only has implementation for four commands - `encode`, `decode`, `remove` and `print` and they can be used as follows:
```
pngrust encode <PATH> <CHUNKTYPE> <MESSAGE> [OUTPUT]
pngrust decode <PATH> <CHUNKTYPE>
pngrust remove <PATH> <CHUNKTYPE>
pngrust print <PATH>
```

Without going into too much detail, **chunktypes** are essentially just 4 alphabetic characters.
Another important detail to note is `encode` takes an optional `OUTPUT` parameter, which will store the string `"Encoded"` in the file specified by the output
parameter in case of a successful encoding.

To make things more clear, this is how the tool can be used:
```
pngrust encode ./dice.png ruSt "This is a secret message! output.txt
pngrust decode ./dice.png ruSt
pngrust remove ./dice.png ruSt
pngrust print ./dice.png
```
## Todo
- [ ] Add an option to encrypt the hidden messages
