# Foxtrot
[Project writeup](https://mattkeeter.com/projects/foxtrot)

Foxtrot is a **fast** viewer for
[STEP files](https://en.wikipedia.org/wiki/ISO_10303-21),
a standard interchange format for mechanical [CAD](https://en.wikipedia.org/wiki/Computer-aided_design).
It is an _experimental_ project built from the ground up,
including new libraries for parsing and triangulation.

This repository includes a simple native GUI:

![Motherboard example](https://mattkeeter.com/projects/foxtrot/rpi.png)  
([demo model source](https://grabcad.com/library/raspberry-pi-3-reference-design-model-b-rpi-raspberrypi-raspberry-pi-1))

In addition, the same code can run in a browser (click to [see the demo](https://mattkeeter.com/projects/foxtrot/demo)):

[![Browser example](https://www.mattkeeter.com/projects/foxtrot/foxtrot365.png)](https://mattkeeter.com/projects/foxtrot/demo)
([demo model source](https://grabcad.com/library/6-dof-mechanical-arm-claw-kit-1))

## Quick start
(Prerequisite: [install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), and clone this repository)
```sh
cargo run --release -- examples/cube_hole.step
```

## WebAssembly demo
(Prerequisite: [install `wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) and clone this repository)
```sh
cd wasm
wasm-pack build --target no-modules
python3 -m http.server --directory deploy # or the simple server of your choice
```
Then, open the local server's URL (typically `127.0.0.1:8000`)
and select a sample file from the list.

## Subsystems
- `cdt`: Constrained Delaunay triangulation (standalone)
- `express`: Parser for EXPRESS schemas files and a matching code generation
  system
- `experiments`: Experiments with trait systems (unused)
- `step`: Auto-generated STEP file parser.  This take a _very_ long time to
  compile, so it is isolated into this crate.
- `triangulate`: Converts a file loaded by `step` into a triangle mesh, using
  `cdt` as its core
- `nurbs`: A handful of NURBS / B-spline algorithms used by `triangulate`
- `gui`: GUI for rendering STEP files, using WebGPU
- `wasm`: Scaffolding to run in the browser using WebAssembly

## Code generation
`step/src/ap214.rs` is automatically generated from
`10303-214e3-aim-long.exp`, which is available via [CVS](https://en.wikipedia.org/wiki/Concurrent_Versions_System) [here](http://www.steptools.com/stds/help/cvshowto.html)
(check out the `APs` folder).

To regenerate, run
```
cargo run --release --example gen_exp -- path/to/APs/10303-214e3-aim-long.exp step/src/ap214.rs
```

## License
© 2021 [Formlabs](https://formlabs.com)

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

### Disclaimer
Foxtrot is a proof-of-concept demo, not an industrial-strength CAD kernel.
It may not work for your models!
Even in the screenshots above,
there are a handful of surfaces that it fails to triangulate;
look in the console for details.

This isn't an official Formlabs project (experimental or otherwise),
it is just code that happens to be owned by Formlabs.

No one at Formlabs is paid to maintain this,
so set your expectations for support accordingly.

## References
[STEP Integrated Definitions](https://www.steptools.com/stds/stp_expg/aim.html)
