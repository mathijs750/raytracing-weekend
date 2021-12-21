# raytracing-weekend
![Build badge](https://github.com/mathijs750/raytracing-weekend/actions/workflows/build.yml/badge.svg)
[![codecov](https://codecov.io/gh/mathijs750/raytracing-weekend/branch/main/graph/badge.svg?token=K9UH1OZHIU)](https://codecov.io/gh/mathijs750/raytracing-weekend)

[Ray tracing in one weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust

## Building
1. Have a working rust compiler
2. Run `cargo build`
3. Run `cargo test`
4. ???
5. profit

## Generating an image
1. Build and test the project
2. `cargo run > [name].ppm` where _[image].ppm_ is your output file. 
3. View you image in a ppm viewer like [this one](https://marketplace.visualstudio.com/items?itemName=martingrzzler.simple-ppm-viewer)