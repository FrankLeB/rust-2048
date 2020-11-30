# rust-2048

This is a console version of the 2048 game implemented in Rust.

## Game rules

You are given a 4x4 grid filled with zeros. At each turn a new two will appear on a
random square. You can tilt the board up, down, left or right to make the digits 
slide through the grid. When 2 adjacent digits slide in the same direction, they 
combine into a single digit twice the size.

You win once one of the digits in the grid reach 2048 and you lose if the grid is 
full and there are no possible moves. Upon winning, you can keep playing to reach 
a highscore.

## Controls

The program will show you the board at the current state and ask for your input. 
You can use the arrows on the keyboard or wsad to tilt the board. To quit the game, 
press q.

### Dependencies

To play this game, you must have rust installed on your computer.

### To play, simply run the following command

```
cargo run
```
