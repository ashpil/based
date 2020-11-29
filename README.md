# Syzygy

Basic ray tracer written in Rust. So far, I've been following [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html). Once I am done with that, I intend to implement features form [PBRT](https://pbrt.org/).

Although probably not nearly as efficient as using existing libraries, I am interested in doing this all from scratch. So I'm writing all of my libraries as well, except for complicated things such as randomness and PNG encoding.

Current libraries:
```
.
├── antsy      -- Simple terminal escape code manipulation for progress bar, etc
├── glitz      -- Vector math library aimed at computer graphics
├── syzygy     -- The actual path tracer
└── xenon      -- Color QOL library, houses color struct and easy PNG writing
```
Not even sure why I made an ANSI escape code library, it's hardly on-topic, I guess the existing ones just felt overkill for my needs.

Image so far:
![Image so far](syzygy/out.png)

## // TODO
- Finish implementing _Ray Tracing in One Weekend_ book features
- Implement features from PBRT
- Make progress bar prettier (*VERY* important, this one first)
- Write more documentation comments
- Write more unit tests

