# Maze solver

Maze generator with A* solver  
Project made to learn rust

# Dependancy

Install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Description

The program can generate a maze using a randomized Depth First Search (DFS), generate a random start and end node.
It will than solve the maze using A* Algorithm with the Manhattan heuristic.

## Demonstration

```
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
|   |     .   . | .   .   . |       |                       |
+   +   +   +   +   +---+   +   +   +---+---+---+   +---+   +
|       | . | .   . | o | . |   |   |               |       |
+   +---+   +---+---+   +   +   +   +   +---+---+   +---+---+
|       | .   . |     .   . |   |       |       |   |       |
+   +---+---+   +   +---+---+   +---+   +   +   +---+   +   +
|   | .   . | . |   |       |   |       |   |           |   |
+   +   +   +   +   +   +   +   +---+   +   +---+---+---+   +
|   | . | .   . |   |   |   |       |   |   |           |   |
+   +   +---+---+   +   +   +---+   +   +   +---+---+   +   +
|   | . |   |       |   |       |   |   |               |   |
+   +   +   +   +---+   +---+   +   +---+---+---+---+   +   +
|   | . |   |       |       |   |                       |   |
+   +   +   +---+   +---+   +   +---+   +---+---+---+---+   +
|   | . |                   |       |                   |   |
+   +   +---+---+---+---+---+---+   +---+---+---+   +---+   +
|   | . | .   . | .   . |       |               |   |       |
+   +   +   +   +   +   +---+   +---+---+---+   +---+   +   +
|   | .   . | . | . | .   . |   |           |           |   |
+   +---+---+   +   +---+   +   +   +   +---+---+---+---+   +
|   | .   . | .   . | .   . |       |       |               |
+   +   +   +---+---+   +---+   +---+---+   +   +---+---+---+
|   | . | .   .   . | . |   |       |   |   |   |           |
+   +   +---+---+   +   +   +---+   +   +   +   +---+   +   +
|   | .   .   . | .   . |       |       |   |           |   |
+   +---+---+   +---+---+---+   +   +---+   +---+---+---+   +
|   |       | .   .   .   . |   |   |               |       |
+   +---+   +---+---+---+   +   +   +   +---+---+---+   +---+
|                       | o |       |                       |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
```

## Contributing

Pull requests are welcome. This project was made to learn rust so any improvement you can think about is welcome.
For major changes, please open an issue first to discuss what you would like to change.

## Author

Evan Clausse

## License

[MIT](https://choosealicense.com/licenses/mit/)
