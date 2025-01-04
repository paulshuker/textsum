# textsum

Summarise text in the command line, written in Rust.

![](./example.svg)

## Install

### Binary

Download a binary from the [releases](https://github.com/paulshuker/textsum/releases)
page for your Operating System.

## Usage

In the terminal you can input a string of text to analyse

```terminal
textsum "Input text"
```

Or input a file path to an existing text file to read and analyse it

```terminal
textsum /path/to/text_file.txt
```

> [!NOTE]
> Directories are separated by `\` not `/` for Windows.

If your terminal supports piping commands with the `|` symbol, you can pipe the results
from one command into textsum, e.g.

```terminal
tail ./text_file.txt | textsum
```

## Tests (~1 ms)

Clone the source code and run the command inside the repository

```terminal
cargo test
```

## Limitations

The tool is designed for UTF-8 only.
