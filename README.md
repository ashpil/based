# Based

_Based on what?_

_Physics, that's what._

Basic ray tracer written in Rust. So far, I've fully implemented the features from [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html). Currently I am working on optimizing and implementing features from [PBRT](https://pbrt.org/).

Although probably not nearly as efficient as using existing libraries, I am interested in doing this all from scratch. So I'm writing all of my libraries as well, except for complicated things/things I am uninterested in such as randomness or PNG encoding.

Current libraries:
```
.
├── antsy      -- Simple terminal escape code manipulation for progress bar, etc
├── glitz      -- Vector math library aimed at computer graphics
├── based      -- The actual path tracer
└── xenon      -- Color QOL library, houses color struct and easy PNG writing
```
Not even sure why I made an ANSI escape code library, it's hardly on-topic, I guess the existing ones just felt overkill for my needs.

Image so far:

![Image so far](based/out.png)

## // TODO
- Implement features from PBRT
- Increase performance. There is never enough performance
- Write more documentation comments
- Write more unit tests

