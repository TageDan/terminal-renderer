A terminal 3D renderer. Supports loading of .obj files.

# Run
Clone the repository and run
```
cargo run --release -- -p path_to_my_obj_file -o --chars ' ' . o e @
```
Example result:

https://github.com/user-attachments/assets/8df3cdb9-a67c-4240-a76e-841b4b3b12aa

Features:
 - 3D rendering
 - Colors (not for .obj files, only manual meshes, atm)
 - Camera movement
 - character sets
 - optional octree optimisation. (have created weird lines but should work now)
 - printing triangle count

Help message:
```
Usage: terminal-renderer [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>         Path to the .obj file
  -c                        Option to list the number of triangles instead of rendering
      --chars [<CHARS>...]  Characters to use for different light levels [low..high]
  -o                        Enables octree optimisation
  -h, --help                Print help
  -V, --version             Print version
```

Can also be used as lib by adding the following lines to your Cargo.toml file:
```
[dependencies]
glam = "0.29.2"
terminal-renderer = {git="https://github.com/TageDan/terminal-renderer"}
```

Future improvements:
 - Alternative lightning types.
 - Texture loading.
 - (re-export glam Vec3 maybe)
 - Write doc comments :D
 - Fix weird lines caused by octree
 - Change camera controls (maybe)
