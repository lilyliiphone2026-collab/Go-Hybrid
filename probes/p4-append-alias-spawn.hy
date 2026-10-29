// PROBE P4 — slice append aliasing.
// `append(a, x)` inside a's capacity keeps the same backing array. If the
// checker treats the result as a fresh owned value, two tasks can be handed
// views of one array.
// EXPECTED-VERDICT: REJECT

fn writer(s []int) {
    s[1] = 99
    fmt.println("{s[1]}")
}

fn main() {
    a := make([]int, 0, 8)
    a = append(a, 1, 2, 3)
    b := append(a, 4)
    task writer(a)
    task writer(b)
}
