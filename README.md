# 3D Chess Game

## Introduction

This project is a 3D chess game variant called Raumschach, implemented using Rust with WebAssembly (wasm), HTML, CSS, and JavaScript. The game features three difficulty modes, move highlighting, move history tracking, captured pieces display, and evaluation of the game state. The chess AI utilizes the minimax algorithm with alpha-beta pruning for optimization. The user interface (UI) for the game is based on the one available in the following [repository](https://github.com/edweenie123/3D-Chess). However, all the game logic and AI functionality have been implemented in Rust.

Play the game at: https://mikej.site/chess-3D/

## Getting Started

To set up and run the game, follow these steps:

1. Clone the repository using the following command:
``` bash
git clone https://github.com/WR104/chess-3D
```

2. Build the WebAssembly code using wasm-pack by running the following command:
``` bash
wasm-pack build
```

Ensure that you have wasm-pack installed at
https://rustwasm.github.io/wasm-pack/installer/

3. Change to the `www` directory using the following command:
``` bash
cd www
```

4. Start the development server by running the following command:
```
npm run start
```

## Contributing

Contributions to the project are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the project's GitHub repository.

## License

This project is licensed under the MIT License. Feel free to modify and distribute
