// Black-box E2E tests for the Later language
//
// These tests run the `later` binary as a subprocess and verify output.
// This is the primary test strategy - testing the language as users would use it.

use std::path::PathBuf;
use std::process::Command;

/// Get the path to the `later` binary
fn later_binary() -> PathBuf {
    // When running tests, the binary is in target/debug/
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("later");
    path
}

/// Run a .later file and return (exit_code, stdout, stderr)
fn run_later_file(filename: &str) -> (i32, String, String) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples");
    path.push(filename);

    let output = Command::new(later_binary())
        .arg(&path)
        .output()
        .expect("Failed to execute later binary");

    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    (exit_code, stdout, stderr)
}

/// Run a .later file and expect it to succeed with given output
fn expect_output(filename: &str, expected: &str) {
    let (exit_code, stdout, stderr) = run_later_file(filename);

    assert_eq!(
        exit_code, 0,
        "Program {} failed with exit code {}.\nstderr: {}",
        filename, exit_code, stderr
    );

    assert_eq!(
        stdout.trim(),
        expected.trim(),
        "Output mismatch for {}.\nExpected: {}\nGot: {}",
        filename,
        expected,
        stdout
    );
}

/// Run a .later file and expect it to fail with error message containing the given string
fn expect_error(filename: &str, expected_error: &str) {
    let (exit_code, _stdout, stderr) = run_later_file(filename);

    assert_ne!(
        exit_code, 0,
        "Program {} should have failed but succeeded",
        filename
    );

    assert!(
        stderr.contains(expected_error),
        "Error message for {} should contain '{}'.\nGot: {}",
        filename,
        expected_error,
        stderr
    );
}

// =============================================================================
// PHASE 1: Basic Expressions
// =============================================================================

#[test]
fn test_integer_literal() {
    // A program that is just an integer literal should evaluate to that integer
    expect_output("int_literal.later", "42");
}

#[test]
fn test_negative_integer() {
    expect_output("int_negative.later", "-17");
}

#[test]
fn test_addition() {
    expect_output("add.later", "5");
}

#[test]
fn test_subtraction() {
    expect_output("sub.later", "3");
}

#[test]
fn test_multiplication() {
    expect_output("mul.later", "12");
}

#[test]
fn test_division() {
    expect_output("div.later", "4");
}

#[test]
fn test_arithmetic_precedence() {
    // Following Raro's "running arithmetic" style,
    // 1 + 2 * 3 = (1 + 2) * 3 = 9, NOT 1 + (2 * 3) = 7
    expect_output("precedence.later", "9");
}

#[test]
fn test_parentheses_override_precedence() {
    // Parentheses should allow traditional precedence when needed
    expect_output("parens.later", "7");
}

// =============================================================================
// PHASE 1: Let Bindings
// =============================================================================

#[test]
fn test_let_binding() {
    expect_output("let_simple.later", "42");
}

#[test]
fn test_let_binding_used_in_expression() {
    expect_output("let_use.later", "10");
}

#[test]
fn test_inline_binding_with_as() {
    // Raro-style inline binding: `1 + 2 as x`
    expect_output("as_binding.later", "6");
}

#[test]
fn test_kebab_case_identifier() {
    expect_output("kebab_case.later", "42");
}

#[test]
fn test_kebab_case_not_subtraction() {
    // `a-b` is one identifier, `a - b` is subtraction
    expect_output("kebab_vs_sub.later", "3");
}

// =============================================================================
// PHASE 1: Functions
// =============================================================================

#[test]
fn test_fn_definition_and_call() {
    expect_output("fn_simple.later", "5");
}

#[test]
fn test_fn_with_params() {
    expect_output("fn_params.later", "7");
}

#[test]
fn test_fn_implicit_return() {
    // Last expression is returned implicitly
    expect_output("fn_implicit_return.later", "10");
}

#[test]
fn test_anonymous_fn() {
    expect_output("fn_anon.later", "9");
}

#[test]
fn test_fn_single_expression() {
    // fn add(a, b) a + b  (no braces)
    expect_output("fn_expr.later", "8");
}

// =============================================================================
// PHASE 1: Objects and Lists
// =============================================================================

#[test]
fn test_object_literal() {
    expect_output("object.later", "5");
}

#[test]
fn test_object_access() {
    expect_output("object_access.later", "42");
}

#[test]
fn test_list_literal() {
    expect_output("list.later", "[1, 2, 3]");
}

#[test]
fn test_list_index() {
    expect_output("list_index.later", "2");
}

#[test]
fn test_object_spread() {
    expect_output("object_spread.later", "{ a: 1, b: 2, c: 3 }");
}

#[test]
fn test_list_spread() {
    expect_output("list_spread.later", "[1, 2, 3, 4, 5]");
}

// =============================================================================
// PHASE 2: Linear Types - Errors
// =============================================================================

#[test]
fn test_linear_unused_value_error() {
    // A linear value that is never consumed should be a compile error
    expect_error(
        "linear_unused.later",
        "linear value `file` was never consumed",
    );
}

#[test]
fn test_linear_use_after_consume_error() {
    // Using a linear value after it's been consumed should error
    expect_error(
        "linear_use_after_consume.later",
        "value `file` was already consumed",
    );
}

#[test]
fn test_linear_consumed_ok() {
    // Properly consuming a linear value should work
    expect_output("linear_ok.later", "done");
}

#[test]
fn test_linear_conditional_consume_error() {
    // Linear value consumed in only one branch should error
    expect_error(
        "linear_conditional.later",
        "linear value `file` may not be consumed",
    );
}

#[test]
fn test_linear_both_branches_ok() {
    // Linear value consumed in both branches should work
    expect_output("linear_both_branches.later", "done");
}

// =============================================================================
// PHASE 2: Borrowing
// =============================================================================

#[test]
fn test_borrow_read() {
    // Borrowing should allow read access without consuming
    expect_output("borrow_read.later", "42");
}

#[test]
fn test_borrow_then_consume() {
    // After borrow ends, owner can still consume
    expect_output("borrow_then_consume.later", "done");
}

#[test]
fn test_borrow_while_borrowed_error() {
    // Can't consume while borrowed
    expect_error(
        "borrow_consume_while_borrowed.later",
        "cannot consume `x` while borrowed",
    );
}

// =============================================================================
// PHASE 3: Effects
// =============================================================================

#[test]
fn test_effect_send_handle() {
    expect_output("effect_simple.later", "handled: 42");
}

#[test]
fn test_effect_continue_with() {
    expect_output("effect_continue.later", "resumed: 100");
}

#[test]
fn test_effect_propagates() {
    // Unhandled effects propagate up
    expect_error("effect_unhandled.later", "unhandled effect: my-effect");
}

#[test]
fn test_effect_generator() {
    // Yield effect for generators
    expect_output("generator.later", "[0, 1, 2, 3, 4]");
}

// =============================================================================
// PHASE 4: Cancellation
// =============================================================================

#[test]
fn test_cancel_cleanup_runs() {
    // When cancelled, cleanup for linear values must run
    expect_output("cancel_cleanup.later", "cleanup ran");
}

#[test]
fn test_cancel_nested_cleanup() {
    // Nested scopes clean up in reverse order
    expect_output("cancel_nested.later", "inner cleanup\nouter cleanup");
}

#[test]
fn test_cancel_propagates_to_children() {
    // Cancelling a parent cancels children
    expect_output("cancel_children.later", "child cancelled\nparent cancelled");
}

// =============================================================================
// PHASE 5: Structured Concurrency
// =============================================================================

#[test]
fn test_spawn_and_await() {
    expect_output("spawn_await.later", "42");
}

#[test]
fn test_all_combinator() {
    expect_output("all.later", "[1, 2, 3]");
}

#[test]
fn test_race_combinator() {
    // First to complete wins, others cancelled
    expect_output("race.later", "first");
}

#[test]
fn test_all_one_fails_cancels_others() {
    expect_output(
        "all_fail.later",
        "error: task 2 failed\ncleanup: task 1\ncleanup: task 3",
    );
}

// =============================================================================
// PHASE 6: Fallible Cleanup
// =============================================================================

#[test]
fn test_cleanup_can_fail() {
    expect_output(
        "cleanup_fail.later",
        "cleanup failed: disk error\nresource abandoned",
    );
}

#[test]
fn test_cleanup_retry() {
    expect_output("cleanup_retry.later", "retry 1\nretry 2\ncleanup succeeded");
}

// =============================================================================
// PHASE 7: Multistage
// =============================================================================

#[test]
fn test_comptime_evaluation() {
    // Expression marked as comptime should be evaluated at build
    expect_output("comptime.later", "120"); // 5! computed at compile time
}

#[test]
fn test_startup_config() {
    // Startup stage ingests config
    expect_output("startup.later", "configured: production");
}
