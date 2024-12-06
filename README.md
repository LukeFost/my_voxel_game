# My Voxel Game Prototype

## Overview

This is a minimal prototype of a voxel-based, spherical world game using the Bevy game engine and a simple LAN multiplayer networking solution.

**Key Features:**
- Spherical world approximated by cube-mapped chunks.
- Voxel rendering with a minimal set of block types (dirt, stone, water, and a plant block).
- Perlin noise terrain generation with simple biomes.
- Basic first-person movement & camera that orients to planet surface.
- Simple block mining and placement.
- Basic client-server networking using `bevy_networking_turbulence` for LAN.

## Architecture

- `main.rs` sets up the Bevy app, choosing between client or server mode via a command-line argument.
- `network.rs` contains networking logic, server state, and client connection code.
- `world_gen.rs` handles noise-based terrain generation and biome logic.
- `voxels.rs` defines voxel and chunk structures, along with cube-sphere mapping.
- `rendering.rs` handles mesh generation for voxel chunks.
- `player.rs` handles player input, movement, orientation, and block interaction.

## Running

### Server
```bash
cargo run -- server
