// ============================================================================
// pointer-array-conversion-reject — Checker corpus (bounty track: "grow the
// adversarial corpus").
//
// PURPOSE: type conversions whose operand is a Pointer or an Array (#1813).
// `int(p)` / `float64(p)` / `string(p)` / `int(arr)` passed `hyb check` with
// 0 diagnostics and failed `go build` with `cannot convert ... to type ...`,
// breaking GUARANTEES.md Row 13 (BUILDABLE).
//
// RULE CITATION: Row 13 — every `hyb check`-accepted file builds cleanly with
// `go build`. Go rejects pointer-to-scalar and array-to-scalar conversions:
//   `cannot convert p (variable of type *int) to type int`
//   `cannot convert arr (variable of type [4]int) to type int`
//
// ---- Why this is a REJECT ----
// `conversion_kinds_conflict` (buildable.rs) omitted `OpKind::Pointer(_)` and
// `OpKind::Array(_)` from its incompatibility arms, so every pair involving
// them fell through to `_ => false`. Closed by adding both kinds to the
// numeric/slice/map/chan target arms plus a pointer-target guard (HYB-E1002).
//
// ---- Expected verdict ----
// EXPECTED-VERDICT: REJECT BUILDABLE
// EXPECTED-DIAGNOSTIC: HYB-E1002
//
// ---- Paired control ----
// `string(arr)` stays fail-open on purpose: Go 1.20+ accepts
// `string([N]byte)`, and OpKind does not distinguish byte elements from int
// elements, so arrays are NOT rejected as string sources (the paired accept
// control lives in `type_conversion_operand_soundness.rs::string_from_array_stays_fail_open`).
//
// ---- Annotations / managed ----
//   lifetime annotations: 0. `managed` blocks: 0.
// ============================================================================

fn main() {
    x := 42
    p := &x
    var arr [4]int
    _ = int(p)
    _ = float64(p)
    _ = string(p)
    _ = int(arr)
    _ = float64(arr)
}
