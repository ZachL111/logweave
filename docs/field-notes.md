# Field Notes

The fixture is small on purpose, which makes each domain case carry real weight.

The domain cases cover `span volume`, `latency skew`, `signal loss`, and `incident shape`. They sit beside the smaller starter fixture so the project has both a compact scoring check and a domain-flavored review check.

`baseline` tells me the happy path still works. `edge` tells me whether the guardrail still has teeth.

The extra check gives the repository a behavior path that can fail for a domain reason, not only a syntax reason.
