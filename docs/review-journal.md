# Review Journal

I treated `logweave` as a project where the smallest useful behavior should still be inspectable.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its observability focus without claiming live deployment or external usage.

## Cases

- `baseline`: `span volume`, score 217, lane `ship`
- `stress`: `latency skew`, score 175, lane `ship`
- `edge`: `signal loss`, score 157, lane `ship`
- `recovery`: `incident shape`, score 162, lane `ship`
- `stale`: `span volume`, score 211, lane `ship`

## Note

The repository should be understandable without pretending it is larger than it is.
