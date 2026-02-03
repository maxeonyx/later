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

// =============================================================================
// PHASE 1: Control Flow
// =============================================================================

#[test]
fn test_if_true() {
    expect_output("if_true.later", "yes");
}

#[test]
fn test_if_false() {
    expect_output("if_false.later", "no");
}

#[test]
fn test_if_else_if() {
    expect_output("if_else_if.later", "middle");
}

#[test]
fn test_if_expression_value() {
    // if/else is an expression that returns a value
    expect_output("if_expr.later", "10");
}

#[test]
fn test_loop_basic() {
    expect_output("loop_basic.later", "5");
}

#[test]
fn test_loop_break_with_value() {
    expect_output("loop_break_value.later", "42");
}

#[test]
fn test_loop_continue() {
    expect_output("loop_continue.later", "[1, 3, 5, 7, 9]");
}

// =============================================================================
// PHASE 1: Pattern Matching
// =============================================================================

#[test]
fn test_let_destructure_list() {
    expect_output("pattern_list.later", "6");
}

#[test]
fn test_let_destructure_object() {
    expect_output("pattern_object.later", "30");
}

#[test]
fn test_let_destructure_nested() {
    expect_output("pattern_nested.later", "42");
}

#[test]
fn test_let_destructure_spread_list() {
    expect_output("pattern_spread_list.later", "[2, 3, 4]");
}

#[test]
fn test_let_destructure_spread_object() {
    expect_output("pattern_spread_object.later", "{ b: 2, c: 3 }");
}

#[test]
fn test_fn_param_destructure() {
    expect_output("pattern_fn_param.later", "15");
}

#[test]
fn test_pattern_wildcard() {
    // _ discards a value (important for linear types!)
    expect_output("pattern_wildcard.later", "kept");
}

// =============================================================================
// PHASE 1: Mutability
// =============================================================================

#[test]
fn test_mut_variable() {
    expect_output("mut_basic.later", "10");
}

#[test]
fn test_mut_reassign() {
    expect_output("mut_reassign.later", "20");
}

#[test]
fn test_immutable_reassign_error() {
    expect_error(
        "immutable_reassign.later",
        "cannot assign to immutable variable",
    );
}

#[test]
fn test_mut_in_loop() {
    expect_output("mut_loop.later", "55");
}

// =============================================================================
// PHASE 1: Comments and Edge Cases
// =============================================================================

#[test]
fn test_empty_file() {
    // Empty file should produce no output (or unit value)
    expect_output("empty.later", "");
}

#[test]
fn test_comments_ignored() {
    expect_output("comments.later", "42");
}

#[test]
fn test_trailing_comma_list() {
    expect_output("trailing_comma_list.later", "[1, 2, 3]");
}

#[test]
fn test_trailing_comma_object() {
    expect_output("trailing_comma_object.later", "{ a: 1, b: 2 }");
}

#[test]
fn test_trailing_comma_fn_params() {
    expect_output("trailing_comma_fn.later", "6");
}

#[test]
fn test_multiline_expression() {
    expect_output("multiline.later", "10");
}

// =============================================================================
// PHASE 1: Booleans and Comparisons
// =============================================================================

#[test]
fn test_boolean_true() {
    expect_output("bool_true.later", "true");
}

#[test]
fn test_boolean_false() {
    expect_output("bool_false.later", "false");
}

#[test]
fn test_boolean_and() {
    expect_output("bool_and.later", "false");
}

#[test]
fn test_boolean_or() {
    expect_output("bool_or.later", "true");
}

#[test]
fn test_boolean_not() {
    expect_output("bool_not.later", "true");
}

#[test]
fn test_comparison_eq() {
    expect_output("cmp_eq.later", "true");
}

#[test]
fn test_comparison_neq() {
    expect_output("cmp_neq.later", "true");
}

#[test]
fn test_comparison_lt() {
    expect_output("cmp_lt.later", "true");
}

#[test]
fn test_comparison_lte() {
    expect_output("cmp_lte.later", "true");
}

#[test]
fn test_comparison_gt() {
    expect_output("cmp_gt.later", "true");
}

#[test]
fn test_comparison_gte() {
    expect_output("cmp_gte.later", "true");
}

// =============================================================================
// PHASE 1: Pipe Operator
// =============================================================================

#[test]
fn test_pipe_basic() {
    expect_output("pipe_basic.later", "5");
}

#[test]
fn test_pipe_chain() {
    expect_output("pipe_chain.later", "30");
}

// =============================================================================
// PHASE 2: Linear Types - Advanced
// =============================================================================

#[test]
fn test_linear_in_struct() {
    // Linear values in structs must all be consumed
    expect_output("linear_struct.later", "done");
}

#[test]
fn test_linear_struct_partial_consume_error() {
    expect_error(
        "linear_struct_partial.later",
        "linear value `resources.b` was never consumed",
    );
}

#[test]
fn test_linear_in_list() {
    // Linear values in lists must all be consumed
    expect_output("linear_list.later", "done");
}

#[test]
fn test_linear_loop_consume() {
    // Linear value created in loop must be consumed each iteration
    expect_output("linear_loop.later", "3 resources closed");
}

#[test]
fn test_linear_passed_to_fn() {
    // Passing linear value to function transfers ownership
    expect_output("linear_fn_transfer.later", "consumed in function");
}

#[test]
fn test_linear_returned_from_fn() {
    // Function can return linear value, caller must consume
    expect_output("linear_fn_return.later", "done");
}

#[test]
fn test_linear_fn_return_not_consumed_error() {
    expect_error(
        "linear_fn_return_unused.later",
        "linear value was never consumed",
    );
}

// =============================================================================
// PHASE 3: Closures
// =============================================================================

#[test]
fn test_closure_capture() {
    expect_output("closure_capture.later", "15");
}

#[test]
fn test_closure_capture_mut() {
    expect_output("closure_capture_mut.later", "3");
}

#[test]
fn test_closure_linear_capture_error() {
    // Can't capture linear value in closure (would allow multiple use)
    expect_error("closure_linear.later", "cannot capture linear value");
}

#[test]
fn test_closure_borrow_capture() {
    // Can capture borrow in closure
    expect_output("closure_borrow.later", "42");
}

// =============================================================================
// PHASE 4: Recursion
// =============================================================================

#[test]
fn test_recursive_factorial() {
    expect_output("factorial.later", "120");
}

#[test]
fn test_recursive_fibonacci() {
    expect_output("fibonacci.later", "55");
}

#[test]
fn test_mutual_recursion() {
    expect_output("mutual_recursion.later", "true");
}

// =============================================================================
// PHASE 5: Effect System - Advanced
// =============================================================================

#[test]
fn test_effect_multiple_handlers() {
    expect_output("effect_multi_handler.later", "a: 1, b: 2");
}

#[test]
fn test_effect_nested_handlers() {
    expect_output("effect_nested_handler.later", "inner handled");
}

#[test]
fn test_effect_rethrow() {
    expect_output("effect_rethrow.later", "outer caught: inner error");
}

#[test]
fn test_effect_finally() {
    // Cleanup runs even when effect propagates
    expect_output("effect_finally.later", "cleanup ran\nerror: boom");
}

// =============================================================================
// PHASE 5: Structured Concurrency - Advanced
// =============================================================================

#[test]
fn test_timeout() {
    expect_output("timeout.later", "timed out");
}

#[test]
fn test_nested_spawn() {
    expect_output("spawn_nested.later", "inner: 1\nouter: 2");
}

#[test]
fn test_spawn_with_linear_resource() {
    // Spawned task can own linear resources
    expect_output("spawn_linear.later", "resource cleaned up");
}

#[test]
fn test_task_cancel_during_cleanup() {
    // Cancellation during cleanup should complete cleanup
    expect_output("cancel_during_cleanup.later", "cleanup completed");
}

// =============================================================================
// PHASE 8: Memory Size Tracking
// =============================================================================

#[test]
fn test_size_annotation() {
    expect_output("size_annotate.later", "size: 16");
}

#[test]
fn test_size_propagates() {
    // Size of struct is sum of field sizes (plus alignment)
    expect_output("size_struct.later", "size: 24");
}

#[test]
fn test_size_bounded_list() {
    // List with max size annotation
    expect_output("size_bounded_list.later", "max size: 100");
}

// =============================================================================
// PHASE 1: Strings
// =============================================================================

#[test]
fn test_string_literal() {
    expect_output("string_literal.later", "hello world");
}

#[test]
fn test_string_escape() {
    expect_output("string_escape.later", "line1\nline2");
}

#[test]
fn test_string_concat() {
    expect_output("string_concat.later", "hello world");
}

#[test]
fn test_string_interpolation() {
    expect_output("string_interp.later", "value is 42");
}

#[test]
fn test_string_length() {
    expect_output("string_length.later", "5");
}

#[test]
fn test_string_index() {
    expect_output("string_index.later", "e");
}

// =============================================================================
// Error Messages - Quality
// =============================================================================

#[test]
fn test_error_undefined_variable() {
    expect_error("error_undefined.later", "undefined variable `foo`");
}

#[test]
fn test_error_type_mismatch() {
    expect_error("error_type_mismatch.later", "expected number, got string");
}

#[test]
fn test_error_wrong_arg_count() {
    expect_error("error_arg_count.later", "expected 2 arguments, got 3");
}

#[test]
fn test_error_line_number() {
    // Error message should include line number
    expect_error("error_line_number.later", "line 5");
}

#[test]
fn test_error_syntax_unexpected_token() {
    expect_error("error_syntax.later", "unexpected token");
}

#[test]
fn test_error_unclosed_brace() {
    expect_error("error_unclosed_brace.later", "unclosed `{`");
}

#[test]
fn test_error_unclosed_bracket() {
    expect_error("error_unclosed_bracket.later", "unclosed `[`");
}

#[test]
fn test_error_unclosed_paren() {
    expect_error("error_unclosed_paren.later", "unclosed `(`");
}

// =============================================================================
// Pattern Matching - Edge Cases
// =============================================================================

#[test]
fn test_pattern_list_length_mismatch() {
    expect_error("pattern_list_mismatch.later", "expected 3 elements, got 2");
}

#[test]
fn test_pattern_object_missing_key() {
    expect_error("pattern_object_missing.later", "missing key `z`");
}

#[test]
fn test_pattern_nested_failure() {
    expect_error("pattern_nested_fail.later", "pattern match failed");
}

// =============================================================================
// Negative Indices
// =============================================================================

#[test]
fn test_list_negative_index() {
    // Python-style negative indexing
    expect_output("list_negative_index.later", "3");
}

#[test]
fn test_string_negative_index() {
    expect_output("string_negative_index.later", "o");
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn test_deeply_nested_structure() {
    expect_output("nested_deep.later", "42");
}

#[test]
fn test_many_params() {
    // Function with many parameters
    expect_output("many_params.later", "55");
}

#[test]
fn test_empty_object() {
    expect_output("empty_object.later", "{}");
}

#[test]
fn test_empty_list() {
    expect_output("empty_list.later", "[]");
}

#[test]
fn test_object_shorthand() {
    // { x, y } as shorthand for { x: x, y: y }
    expect_output("object_shorthand.later", "{ x: 10, y: 20 }");
}

#[test]
fn test_keyword_as_object_key() {
    // Keywords can be used as object keys
    expect_output("keyword_key.later", "1");
}

#[test]
fn test_chained_property_access() {
    expect_output("chained_access.later", "42");
}

#[test]
fn test_chained_index_access() {
    expect_output("chained_index.later", "5");
}

// =============================================================================
// Division and Modulo
// =============================================================================

#[test]
fn test_integer_division() {
    expect_output("int_div.later", "3");
}

#[test]
fn test_modulo() {
    expect_output("modulo.later", "2");
}

#[test]
fn test_division_by_zero() {
    expect_error("div_zero.later", "division by zero");
}

// =============================================================================
// Nil/Unit Value
// =============================================================================

#[test]
fn test_nil_literal() {
    expect_output("nil.later", "nil");
}

#[test]
fn test_fn_no_return() {
    // Function without explicit return gives nil
    expect_output("fn_no_return.later", "nil");
}

// =============================================================================
// Defer/Cleanup Syntax
// =============================================================================

#[test]
fn test_defer_basic() {
    expect_output("defer_basic.later", "first\nsecond\ndeferred");
}

#[test]
fn test_defer_order() {
    // Multiple defers run in reverse order
    expect_output("defer_order.later", "3\n2\n1");
}

#[test]
fn test_defer_with_value() {
    // Defer captures value at defer time, not run time
    expect_output("defer_capture.later", "x was 1");
}

#[test]
fn test_defer_on_error() {
    // Defer runs even when error propagates
    expect_output("defer_on_error.later", "deferred\nerror: boom");
}

// =============================================================================
// Type Annotations (Optional)
// =============================================================================

#[test]
fn test_type_annotation_let() {
    expect_output("type_annotation.later", "42");
}

#[test]
fn test_type_annotation_fn_param() {
    expect_output("type_annotation_fn.later", "10");
}

#[test]
fn test_type_annotation_fn_return() {
    expect_output("type_annotation_return.later", "15");
}

#[test]
fn test_type_mismatch_annotation() {
    expect_error("type_annotation_error.later", "expected Int, got String");
}
