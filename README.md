# logweave

`logweave` is a Rust project in observability. Its focus is to correlate service logs into incident timelines.

## Why It Exists

This is intentionally local and self-contained so it can be inspected without credentials, services, or seeded history.

## Logweave Review Notes

The first comparison I would make is `span volume` against `signal loss` because it shows where the rule is most opinionated.

## Features

- `fixtures/domain_review.csv` adds cases for span volume and latency skew.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/logweave-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `span volume` and `signal loss`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Architecture Notes

The implementation keeps the scoring rule plain: reward signal and confidence, preserve slack, penalize drag, then classify the result into a review lane.

The Rust implementation avoids hidden state so fixture changes are easy to reason about.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Tests

The check exercises the source code and the review fixture. `baseline` is the high score at 217; `edge` is the low score at 157.

## Limitations And Roadmap

The fixture set is small enough to audit by hand. The next useful expansion is malformed input coverage, not extra surface area.
