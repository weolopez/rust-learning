# Rapier Physics Demo

A simple physics simulation using [Rapier 2D](https://rapier.rs/) for physics and [Macroquad](https://macroquad.rs/) for rendering.

## Features

- Full 2D physics simulation with Rapier
- Bouncing balls with restitution
- Static ground and walls
- Interactive ball spawning

## Controls

- **SPACE**: Add a ball at random position
- **Click**: Add a ball at cursor position
- **C**: Change ball color
- **R**: Reset simulation

## Run

```bash
cd hello/rapier_physics
cargo run
```

## Dependencies

- `rapier2d` - 2D physics engine with SIMD optimizations
- `macroquad` - Simple game framework for rendering
