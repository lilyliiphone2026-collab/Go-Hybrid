// PROBE P2 — control for P1.
// Same shape, but the inner map is created INSIDE the literal and never escapes,
// so nothing shared crosses the boundary. This MUST be accepted: if it is
// rejected, the probe harness (or the checker's nesting handling) is over-strict
// and P1's result means nothing.
// EXPECTED-VERDICT: ACCEPT

fn worker(outer map[string]map[string]int) {
    n := len(outer)
    fmt.println("n={n}")
}

fn main() {
    outer := map[string]map[string]int{"k": {}}
    task worker(outer)
}
