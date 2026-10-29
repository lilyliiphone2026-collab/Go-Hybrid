// PROBE P3 — slice alias at the spawn boundary.
// In Go, `b := a` on a slice copies the header, NOT the backing array: writing
// b[0] writes a[0]. Two tasks holding a and b therefore mutate the same memory
// with no diagnostic — exactly the data race A1c exists to reject.
// EXPECTED-VERDICT: REJECT

fn writer(s []int) {
    s[0] = 99
    fmt.println("{s[0]}")
}

fn main() {
    a := []int{1, 2, 3}
    b := a
    task writer(a)
    task writer(b)
}
