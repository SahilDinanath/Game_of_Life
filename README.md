# Game of Life CLI
[Screencast from 2024-08-05 17-25-24.webm](https://github.com/user-attachments/assets/bd52e1e8-181f-45f4-99ff-db042c076dcb)

A terminal-based implementation of Conway's Game of Life, allowing for customizable simulations through various command-line arguments.

## Overview

Conway's Game of Life is a cellular automaton devised by mathematician John Horton Conway. The game consists of a grid of cells that can be alive or dead. The state of each cell in the next generation is determined by the number of live neighbors it has, following a set of simple rules.

This CLI tool simulates the Game of Life in the terminal, providing options to customize the speed, density, initial spawn rate, and colors of the cells and background.

## Installation

To use this CLI tool, you need to have Rust installed. You can then clone the repository and build the project:

```sh
git clone https://github.com/SahilDinanath/Game_of_Life.git
cd Game_of_life
cargo build --release
```

## Usage

Run the executable with the desired command-line arguments:

```sh
./gol [OPTIONS]
```

### Options

- `-s`, `--speed`: Speed of simulation in milliseconds (default: 50).
- `-d`, `--density`: Density of simulation, multiple of total allowable cells (default: 2).
- `-r`, `--rate`: Initial spawn rate of cells with probability between 0.0 - 1.0 (default: 0.05).
- `-c`, `--color`: Color of cells. Prefix: none or bright. Values: red, green, blue, yellow, magenta, cyan, grey, etc. (default: white).
- `-b`, `--background-color`: Color of background. Prefix: none or bright. Values: red, green, blue, yellow, magenta, cyan, grey, etc. (default: black).

### Examples

1. **Run with default settings:**

   ```sh
   ./gol
   ```

2. **Custom speed and density:**

   ```sh
   ./gol --speed 100 --density 3
   ```

3. **Custom spawn rate and cell color:**

   ```sh
   ./gol --rate 0.1 --color bright-red
   ```

4. **Custom background color:**

   ```sh
   ./gol --background-color bright-blue
   ```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## Acknowledgments

Special thanks to Conway, [ratatui](https://github.com/ratatui-org/ratatui) and [clap](https://github.com/clap-rs/clap).
