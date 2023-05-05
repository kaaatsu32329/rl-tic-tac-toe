# Reinforcement Learning for Tic Tac Toe game implemented in Rust.

## Run

```bash
cargo run --bin game
```

### Output example

```bash
> cargo run --bin game
   Compiling rl-tic-tac-toe v0.1.0 (C:\Users\{path}\rl-tic-tac-toe)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35s
     Running `target\debug\game.exe`

--------------------------------------
Step: 50000
Win rate: 86.534
Q table:Some([-3.084103765015378, -0.7290000000000001, 0.36450000000000005, -1.0, 0.0, -3.2109234722676154, -2.8909534246704767, 0.6561, -0.0971681721765858])

--------------------------------------
Step: 100000
Win rate: 91.124
Q table:Some([0.7290000000000001, -0.4986360000000001, 0.6561000000000001, -1.0, 0.7290000000000001, -1.0, 1.0, 0.0, 0.16135926197198236])

--------------------------------------
Step: 150000
Win rate: 92.092
Q table:Some([0.6561, 0.6561, -1.0, 0.7290000000000001, -1.0, -3.448746227716117, -2.470281854076001, -2.5989441984729007, -1.0332875741119643])
```

## Coverage

```bash
cargo llvm-cov --lcov --output-path lcov.info
```
