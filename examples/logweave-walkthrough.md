# Logweave Walkthrough

I use this file as a small checklist before changing the Rust implementation.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | span volume | 217 | ship |
| stress | latency skew | 175 | ship |
| edge | signal loss | 157 | ship |
| recovery | incident shape | 162 | ship |
| stale | span volume | 211 | ship |

Start with `baseline` and `edge`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

The useful comparison is `span volume` against `signal loss`, not the raw score alone.
