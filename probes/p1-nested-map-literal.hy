// PROBE P1 — three-level nesting.
// GUARANTEES row 3 says composites are DEEP: "every reference-typed field/element
// must itself be sendable". A map literal whose VALUES are map literals that hold
// a shared mutable map should therefore be rejected at the spawn site.
// EXPECTED-VERDICT: REJECT

fn worker(outer map[string]map[string]int) {
    n := len(outer)
    fmt.println("n={n}")
}

fn main() {
    inner := map[string]int{}
    outer := map[string]map[string]int{"k": inner}
    task worker(outer)
}
