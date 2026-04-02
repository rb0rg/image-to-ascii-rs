# image-to-ascii-rs

A fast, simple, and reliable Command Line Interface (CLI) tool written in Rust that converts images into ASCII art text files. 

## Features

- **Automatic Resizing**: Intelligently resizes the input image to fit standard terminal and text editor viewing while preserving the aspect ratio (taking into account character height/width ratios).
- **Luminosity Mapping**: Converts image pixels to grayscale using luminosity calculations and maps them to ASCII characters for accurate shading.
- **Progress Feedback**: Features a clean CLI spinner to show progress during generation and writing.
- **Robust Parsing**: Built with `clap` to provide standard CLI flags, help menus, and error handling.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed on your system.

Clone the repository and build the project:

```bash
git clone https://github.com/rb0rg/image-to-ascii-rs.git
cd image-to-ascii-rs

# Build the project in release mode for optimal performance
cargo build --release
```

You can also install the binary globally on your system:

```bash
cargo install --path .
```

## Usage

The CLI tool provides a `generate` subcommand to perform the image-to-ASCII conversion.

### Syntax

```bash
image-to-ascii generate [OPTIONS]
```

### Arguments

| Argument | Short | Long | Required | Description |
| :--- | :---: | :---: | :---: | :--- |
| **Input** | `-i` | `--input` | **Yes** | The path to the source image file you want to convert (e.g., `image.png`, `photo.jpg`). The file must exist. |
| **Output** | `-o` | `--output`| **Yes** | The path to the folder where the resulting `.txt` file will be saved. The directory must already exist. |

### Returns & Output

Upon successful execution, the program will generate a plain text file (`.txt`) containing the ASCII representation of the image. 

- The output file will inherit the **file stem** (the name without the extension) of the input image. 
- For example, if your input is `inputs/profile_picture.png` and your output directory is `outputs/`, the resulting file will be saved as `outputs/profile_picture.txt`.
- The tool will display a success message upon completion: `✔ Successfully generated ASCII version`.

### Examples

**Basic Conversion:**

```bash
image-to-ascii generate --input ./assets/logo.png --output ./documents/
```
*This command reads `logo.png` from the `assets` folder and writes the ASCII result to `./documents/logo.txt`.*

**Using Short Flags:**

```bash
image-to-ascii generate -i ./dog.jpeg -o ./
```
*This command reads `dog.jpeg` from the current directory and outputs `dog.txt` into the current directory.*

**Viewing Help:**

```bash
image-to-ascii help
image-to-ascii generate --help
```

## Error Handling

The CLI safely handles common errors and will abort with descriptive messages if:
- The input image path does not exist.
- The output directory path does not exist.
- The input file is not a valid or supported image format.

## License

This project is open-source. Feel free to modify, distribute, and contribute!
