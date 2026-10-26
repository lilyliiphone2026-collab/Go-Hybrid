//! type_conversion_operand_soundness.rs — tests for HYB-E1002 rejection of
//! invalid type conversions (converting non-structs to structs, bool to/from other types,
//! string to/from non-convertibles, and void calls as conversion operands).

use hy_check::{check_file, Verdict};
use hy_syntax::parse_file;

fn check(src: &str) -> hy_check::CheckResult {
    let ast = parse_file("t.hy", src).expect("snippet parses");
    check_file(&ast)
}

fn error_codes(r: &hy_check::CheckResult) -> Vec<String> {
    r.diagnostics
        .iter()
        .filter(|d| d.severity == hy_diag::Severity::Error)
        .map(|d| d.code.clone())
        .collect()
}

#[test]
fn struct_conversion_from_int_literal_rejects_e1002() {
    let src = "struct holder {\n    n int\n}\nfn main() {\n    _ = holder(1)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn struct_conversion_from_int_var_rejects_e1002() {
    let src = "struct holder {\n    n int\n}\nfn main() {\n    x := 1\n    _ = holder(x)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn struct_conversion_from_string_rejects_e1002() {
    let src = "struct holder {\n    s string\n}\nfn main() {\n    _ = holder(\"bad\")\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn bool_conversion_from_int_literal_rejects_e1002() {
    let src = "fn main() {\n    _ = bool(1)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn bool_conversion_from_int_var_rejects_e1002() {
    let src = "fn main() {\n    x := 1\n    _ = bool(x)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn int_conversion_from_bool_var_rejects_e1002() {
    let src = "fn main() {\n    b := true\n    _ = int(b)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn string_conversion_from_bool_var_rejects_e1002() {
    let src = "fn main() {\n    b := true\n    _ = string(b)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn int_conversion_from_string_var_rejects_e1002() {
    let src = "fn main() {\n    s := \"abc\"\n    _ = int(s)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn string_conversion_from_float_var_rejects_e1002() {
    let src = "fn main() {\n    f := 1.5\n    _ = string(f)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn conversion_from_void_call_rejects_e1002() {
    let src = "fn noop() {}\nfn main() {\n    _ = int(noop())\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn valid_numeric_conversions_accept() {
    let src = "fn main() {\n    fl := 1.5\n    i := int(fl)\n    f := float64(1)\n    _ = f\n    _ = i\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Accept, "expected accept, got diagnostics: {:?}", r.diagnostics);
}

#[test]
fn valid_string_from_int_accepts() {
    let src = "fn main() {\n    s := string(65)\n    _ = s\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Accept, "expected accept, got diagnostics: {:?}", r.diagnostics);
}

#[test]
fn valid_slice_conversions_accept() {
    let src = "fn main() {\n    b := []byte(\"hello\")\n    s := string(b)\n    _ = s\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Accept, "expected accept, got diagnostics: {:?}", r.diagnostics);
}

#[test]
fn valid_interface_conversion_accepts() {
    let src = "fn main() {\n    a := any(42)\n    _ = a\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Accept, "expected accept, got diagnostics: {:?}", r.diagnostics);
}

// ---- #1813: Pointer and Array operand kinds were missing from
// conversion_kinds_conflict, so `int(p)` / `int(arr)` passed hyb check with
// 0 diagnostics and failed `go build` with `cannot convert ... to type ...`.

#[test]
fn int_conversion_from_pointer_rejects_e1002() {
    let src = "fn main() {\n    x := 42\n    p := &x\n    _ = int(p)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn float_conversion_from_pointer_rejects_e1002() {
    let src = "fn main() {\n    x := 42\n    p := &x\n    _ = float64(p)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn string_conversion_from_pointer_rejects_e1002() {
    let src = "fn main() {\n    x := 42\n    p := &x\n    _ = string(p)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn int_conversion_from_array_rejects_e1002() {
    let src = "fn main() {\n    var arr [4]int\n    _ = int(arr)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn string_conversion_from_array_to_numeric_target_rejects_e1002() {
    // float64(arr) is rejected; string(arr) stays fail-open (see below).
    let src = "fn main() {\n    var arr [4]int\n    _ = float64(arr)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Reject);
    assert!(error_codes(&r).contains(&"HYB-E1002".to_string()));
}

#[test]
fn string_from_array_stays_fail_open() {
    // Go 1.20+ accepts `string([N]byte)`, and OpKind does not distinguish
    // byte elements from int elements — so array-to-string stays fail-open
    // rather than emitting false positives on legal byte-array conversions.
    let src = "fn main() {\n    var arr [4]int\n    _ = string(arr)\n}\n";
    let r = check(src);
    assert_eq!(r.verdict, Verdict::Accept, "expected accept, got diagnostics: {:?}", r.diagnostics);
}

// Note on the Pointer-target arm of conversion_kinds_conflict (#1813): it is
// purely defensive. Neither `(*int)(5)` nor `*int(p)` parses — the .hy grammar
// rejects `*` in expression position, so a pointer type conversion is not
// expressible in source and the arm has no reachable test.
