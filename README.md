# Learning Rust by programming a Game of Life

## Rules for [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

The game is a two-dimensional orthogonal grid of square cells. These cells can be in 1 of 2 possible states : live or dead.
At each step in time, the state of every cell updates according to the states of its 8 neighbours :

1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2. Any live cell with two or three live neighbours lives on to the next generation.
3. Any live cell with more than three live neighbours dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
