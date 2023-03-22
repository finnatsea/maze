# maze

This program generates a random maze using a depth-first search algorithm and then solves the maze using another depth-first search algorithm. The maze size can be customized by changing the `MAZE_WIDTH` and `MAZE_HEIGHT` values.

## How to Run

1. Install [Rust and Cargo](https://www.rust-lang.org/tools/install) if you haven't already.
2. Clone this repository or copy the source code into a new directory.
3. Run `cargo run --release`

This command will compile and run the program. The generated maze and its solution will be displayed in the terminal.

## Example Maze

```
Solution Found in: 186.167µs
█████████████████████████
█*█*******█.............█
█*█*█████*█.█████.███████
█*█***█.█*█.█***█.......█
█*███*█.█*███*█*███.███.█
█***█*█..*****█***█...█.█
███*█*███████████*███.█.█
█***█***█*********█...█.█
█*█████*█*█████████████.█
█*█*****█***█***█*******█
█*█*███████*█*█*█*█████*█
█***█.....█***█***█*****█
█████.█.███████████*█████
█.█...█.....█*****█*█   █
█.█.███████.█*███*█*█ █ █
█.█.█.....█.█***█***█ █ █
█.█.███.███.███*█████ █ █
█...█...█.....█***█   █ █
█.███.█.█.█.█████*█ ███ █
█...█.█.█.█.█*****█   █ █
███.█.█.█.███*███████ █ █
█...█.█.█...█*█..***█ █ █
█.███.█.███.█*███*█*███ █
█.....█...█..*****█****.█
█████████████████████████
```
