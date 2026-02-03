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

// =============================================================================
// Cancellation - Core Feature Tests
// =============================================================================

#[test]
fn test_cancel_simple_task() {
    expect_output("cancel_simple.later", "cancelled");
}

#[test]
fn test_cancel_flag_check() {
    // Demonstrates that cancellation points exist and are checked
    expect_output("cancel_flag.later", "iterations: 100\ncancelled");
}

#[test]
fn test_cancel_in_loop() {
    // Loop should check cancellation and exit cleanly
    expect_output("cancel_loop.later", "loop cancelled\ncleanup done");
}

#[test]
fn test_cancel_blocks_in_cleanup() {
    // While in cleanup, further cancellation should be blocked
    expect_output("cancel_in_cleanup.later", "cleanup completed fully");
}

#[test]
fn test_cancel_effect_catchable() {
    // Cancellation is an effect that can be caught
    expect_output("cancel_catch.later", "caught cancellation");
}

// =============================================================================
// Linear Types - Comprehensive
// =============================================================================

#[test]
fn test_linear_must_use() {
    // Linear type returned from function must be used
    expect_error("linear_must_use.later", "value of type `File` must be used");
}

#[test]
fn test_linear_move_semantics() {
    // After move, original binding is invalid
    expect_error("linear_move.later", "value `f` has been moved");
}

#[test]
fn test_linear_split() {
    // Can split a linear aggregate and consume pieces separately
    expect_output("linear_split.later", "both consumed");
}

#[test]
fn test_linear_drop_explicit() {
    // Explicit drop for when you can't use a value
    expect_output("linear_drop.later", "dropped");
}

#[test]
fn test_linear_in_match() {
    // Pattern match on linear must consume in all branches
    expect_output("linear_match.later", "consumed");
}

#[test]
fn test_linear_in_match_error() {
    // Pattern match on linear with unconsumed branch
    expect_error(
        "linear_match_error.later",
        "linear value not consumed in branch",
    );
}

// =============================================================================
// Structured Concurrency - Comprehensive
// =============================================================================

#[test]
fn test_nursery_basic() {
    // Alternative name for spawn scope
    expect_output("nursery.later", "all done");
}

#[test]
fn test_child_outlives_parent_error() {
    // Child cannot escape parent scope
    expect_error("child_escape.later", "task cannot outlive its parent scope");
}

#[test]
fn test_concurrent_mutation() {
    // Concurrent tasks cannot mutate shared state (borrow checker)
    expect_error(
        "concurrent_mut.later",
        "cannot mutably borrow while task holds reference",
    );
}

#[test]
fn test_channel_send_receive() {
    expect_output("channel.later", "received: hello");
}

#[test]
fn test_channel_bounded() {
    expect_output("channel_bounded.later", "sent 3 messages");
}

// =============================================================================
// Effects - Comprehensive
// =============================================================================

#[test]
fn test_effect_as_capability() {
    // Effect types act as capabilities
    expect_output("effect_capability.later", "io performed");
}

#[test]
fn test_effect_composition() {
    // Multiple effects compose
    expect_output("effect_compose.later", "yielded: 1\nlogged: step 1");
}

#[test]
fn test_effect_shallow_handler() {
    // Shallow handler only handles once
    expect_output("effect_shallow.later", "handled once");
}

#[test]
fn test_effect_deep_handler() {
    // Deep handler handles all occurrences
    expect_output("effect_deep.later", "handled: 1\nhandled: 2\nhandled: 3");
}

#[test]
fn test_effect_state() {
    // State effect pattern
    expect_output("effect_state.later", "final: 10");
}

// =============================================================================
// Multistage - Comprehensive
// =============================================================================

#[test]
fn test_comptime_type_check() {
    // Type error at comptime is a compile error
    expect_error("comptime_type_error.later", "comptime error: type mismatch");
}

#[test]
fn test_comptime_io_error() {
    // IO at comptime is an error (IO requires runtime)
    expect_error("comptime_io.later", "cannot perform IO at compile time");
}

#[test]
fn test_startup_io_ok() {
    // IO at startup is OK
    expect_output("startup_io.later", "read config successfully");
}

#[test]
fn test_stage_residual() {
    // Show that comptime produces smaller residual
    expect_output("stage_residual.later", "120"); // 5! computed at compile time
}

#[test]
fn test_runtime_value_at_comptime_error() {
    // Can't use runtime value at comptime
    expect_error(
        "comptime_runtime.later",
        "runtime value used in comptime expression",
    );
}

// =============================================================================
// Memory/Size Tracking - Comprehensive
// =============================================================================

#[test]
fn test_size_known_at_comptime() {
    expect_output("size_comptime.later", "size known: 64");
}

#[test]
fn test_size_known_at_startup() {
    expect_output("size_startup.later", "size determined by config");
}

#[test]
fn test_size_overflow_error() {
    expect_error("size_overflow.later", "exceeds maximum size");
}

#[test]
fn test_bounded_buffer() {
    expect_output("bounded_buffer.later", "buffer full");
}

// =============================================================================
// Pipe Operator - Advanced
// =============================================================================

#[test]
fn test_pipe_with_partial() {
    // Pipe with partial application: x | add(5) means add(x, 5)
    expect_output("pipe_partial.later", "15");
}

#[test]
fn test_pipe_to_method() {
    // Method-style: x | .len()
    expect_output("pipe_method.later", "5");
}

#[test]
fn test_pipe_anonymous() {
    // Pipe into anonymous function
    expect_output("pipe_anon.later", "100");
}

// =============================================================================
// Import/Export
// =============================================================================

#[test]
fn test_import_basic() {
    expect_output("import_basic.later", "42");
}

#[test]
fn test_import_destructure() {
    expect_output("import_destructure.later", "3");
}

#[test]
fn test_export_object() {
    // A file's last expression is its export
    expect_output("export_test.later", "imported: 10");
}

// =============================================================================
// Real-World Patterns
// =============================================================================

#[test]
fn test_graceful_shutdown() {
    expect_output(
        "graceful_shutdown.later",
        "shutdown complete\nall resources released",
    );
}

#[test]
fn test_retry_with_backoff() {
    expect_output("retry_backoff.later", "succeeded after 3 attempts");
}

#[test]
fn test_resource_pool() {
    expect_output(
        "resource_pool.later",
        "acquired\nused\nreleased back to pool",
    );
}

#[test]
fn test_timeout_with_cleanup() {
    expect_output("timeout_cleanup.later", "timed out\nresource cleaned up");
}

#[test]
fn test_parallel_map() {
    expect_output("parallel_map.later", "[2, 4, 6, 8, 10]");
}

// =============================================================================
// Floats and Numbers
// =============================================================================

#[test]
fn test_float_literal() {
    expect_output("float.later", "3.14");
}

#[test]
fn test_float_arithmetic() {
    expect_output("float_arith.later", "7.5");
}

#[test]
fn test_int_to_float() {
    expect_output("int_to_float.later", "5.0");
}

#[test]
fn test_float_comparison() {
    expect_output("float_cmp.later", "true");
}

// =============================================================================
// Unary Operators
// =============================================================================

#[test]
fn test_unary_minus() {
    expect_output("unary_minus.later", "-5");
}

#[test]
fn test_unary_not() {
    expect_output("unary_not.later", "false");
}

// =============================================================================
// Index Bounds
// =============================================================================

#[test]
fn test_list_out_of_bounds() {
    expect_error("list_oob.later", "index 10 out of bounds");
}

#[test]
fn test_string_out_of_bounds() {
    expect_error("string_oob.later", "index 10 out of bounds");
}

// =============================================================================
// Object Operations
// =============================================================================

#[test]
fn test_object_has_key() {
    expect_output("object_has_key.later", "true");
}

#[test]
fn test_object_keys() {
    expect_output("object_keys.later", "[\"a\", \"b\", \"c\"]");
}

#[test]
fn test_object_values() {
    expect_output("object_values.later", "[1, 2, 3]");
}

#[test]
fn test_object_missing_key_error() {
    expect_error("object_missing_key.later", "key `z` not found");
}

// =============================================================================
// List Operations
// =============================================================================

#[test]
fn test_list_push() {
    expect_output("list_push.later", "[1, 2, 3, 4]");
}

#[test]
fn test_list_pop() {
    expect_output("list_pop.later", "[1, 2]");
}

#[test]
fn test_list_length() {
    expect_output("list_len.later", "5");
}

#[test]
fn test_list_map() {
    expect_output("list_map.later", "[2, 4, 6]");
}

#[test]
fn test_list_filter() {
    expect_output("list_filter.later", "[2, 4]");
}

#[test]
fn test_list_reduce() {
    expect_output("list_reduce.later", "15");
}

// =============================================================================
// Higher-Order Functions
// =============================================================================

#[test]
fn test_fn_as_value() {
    expect_output("fn_value.later", "10");
}

#[test]
fn test_fn_return_fn() {
    expect_output("fn_return_fn.later", "15");
}

#[test]
fn test_currying() {
    expect_output("curry.later", "12");
}

// =============================================================================
// Scope and Shadowing
// =============================================================================

#[test]
fn test_shadowing() {
    expect_output("shadowbinding.later", "20");
}

#[test]
fn test_scope_block() {
    expect_output("scope_block.later", "outer");
}

#[test]
fn test_scope_fn() {
    expect_output("scope_fn.later", "10");
}

// =============================================================================
// While Loop (if we support it as sugar)
// =============================================================================

#[test]
fn test_while_loop() {
    expect_output("while.later", "5");
}

// =============================================================================
// Assert / Debug
// =============================================================================

#[test]
fn test_assert_pass() {
    expect_output("assert_pass.later", "ok");
}

#[test]
fn test_assert_fail() {
    expect_error("assert_fail.later", "assertion failed");
}

#[test]
fn test_debug_print() {
    expect_output("debug.later", "debug: 42\n42");
}

// =============================================================================
// Numeric Edge Cases
// =============================================================================

#[test]
fn test_integer_overflow() {
    // Integer overflow should be a runtime error, not silent wraparound
    expect_error("int_overflow.later", "integer overflow");
}

#[test]
fn test_integer_underflow() {
    expect_error("int_underflow.later", "integer overflow");
}

#[test]
fn test_float_nan_comparison() {
    // NaN comparisons should be false (IEEE 754)
    expect_output("float_nan_cmp.later", "false");
}

#[test]
fn test_float_infinity() {
    expect_output("float_infinity.later", "inf");
}

#[test]
fn test_float_neg_infinity() {
    expect_output("float_neg_infinity.later", "-inf");
}

#[test]
fn test_modulo_zero() {
    expect_error("mod_zero.later", "division by zero");
}

// =============================================================================
// Linear Type Edge Cases (Critical)
// =============================================================================

#[test]
fn test_linear_conditional_both_branches_consume() {
    // Both branches of if must consume a linear value exactly once
    expect_output("linear_if_consume.later", "done");
}

#[test]
fn test_linear_conditional_only_one_branch_error() {
    // If only one branch consumes, that's an error
    expect_error(
        "linear_if_partial.later",
        "linear value `file` may not be consumed",
    );
}

#[test]
fn test_linear_loop_consume_error() {
    // Can't consume a linear value inside a loop (might run 0 or N times)
    expect_error(
        "linear_loop_consume.later",
        "cannot consume linear value in loop",
    );
}

#[test]
fn test_linear_return_without_consume() {
    // Early return must not leak linear values
    expect_error(
        "linear_early_return.later",
        "linear value `file` not consumed before return",
    );
}

#[test]
fn test_linear_break_without_consume() {
    // Break must not leak linear values from enclosing scope
    expect_error(
        "linear_break_leak.later",
        "linear value `file` not consumed before break",
    );
}

#[test]
fn test_linear_panic_cleanup() {
    // Panic should still run cleanup for linear values
    expect_output("linear_panic_cleanup.later", "cleanup ran");
}

#[test]
fn test_linear_field_partial_move() {
    // Can't move a field out of a struct without consuming the whole struct
    expect_error(
        "linear_field_move.later",
        "cannot move field out of linear value",
    );
}

#[test]
fn test_linear_list_element_move() {
    // Can't move an element out of a linear list
    expect_error(
        "linear_list_move.later",
        "cannot move element out of linear list",
    );
}

// =============================================================================
// Cancellation Edge Cases (Critical)
// =============================================================================

#[test]
fn test_cancel_during_cleanup() {
    // Cancellation during cleanup should be deferred until cleanup completes
    expect_output("cancel_during_cleanup.later", "cleanup complete");
}

#[test]
fn test_double_cancel() {
    // Cancelling an already-cancelled task is a no-op
    expect_output("double_cancel.later", "ok");
}

#[test]
fn test_cancel_after_complete() {
    // Cancelling a completed task is a no-op
    expect_output("cancel_after_complete.later", "result: 42");
}

#[test]
fn test_cancel_propagation_hierarchy() {
    // Parent cancel must cancel all children
    expect_output("cancel_propagates.later", "child cancelled");
}

#[test]
fn test_cancel_cleanup_order() {
    // Cleanup runs in reverse acquisition order even under cancellation
    expect_output("cancel_cleanup_order.later", "c\nb\na");
}

#[test]
fn test_cancel_check_frequency() {
    // CPU-bound loops must check cancellation (at loop head)
    expect_output("cancel_cpu_bound.later", "cancelled after iterations");
}

// =============================================================================
// Effect Edge Cases
// =============================================================================

#[test]
fn test_effect_handler_scope() {
    // Effect handler only active within its scope
    expect_error("effect_handler_scope.later", "unhandled effect: ask");
}

#[test]
fn test_effect_shadowing() {
    // Inner handler shadows outer handler for same effect
    expect_output("effect_nested.later", "inner");
}

#[test]
fn test_effect_resume_multiple_times() {
    // Generator-style: handler can resume multiple times
    expect_output("effect_multi_resume.later", "1\n2\n3");
}

#[test]
fn test_effect_resume_with_value() {
    // Handler can provide a value when resuming
    expect_output("effect_resume_value.later", "got: hello");
}

#[test]
fn test_effect_linear_across_boundary() {
    // Linear values can't escape through effect boundaries
    expect_error(
        "effect_linear_escape.later",
        "linear value cannot escape effect handler",
    );
}

// =============================================================================
// Concurrency Edge Cases
// =============================================================================

#[test]
fn test_spawn_return_linear() {
    // Spawned task can return linear values through proper channels
    expect_output("spawn_return_linear.later", "resource consumed");
}

#[test]
fn test_channel_closed() {
    // Sending to closed channel is an error
    expect_error("channel_closed.later", "channel closed");
}

#[test]
fn test_channel_receive_cancelled() {
    // Blocking receive respects cancellation
    expect_output("channel_receive_cancel.later", "cancelled");
}

#[test]
fn test_nursery_error_cancels_siblings() {
    // One child error cancels all siblings
    expect_output("nursery_error_cancel.later", "sibling cancelled");
}

#[test]
fn test_deadlock_detection() {
    // Obvious deadlock should be detected
    expect_error("deadlock.later", "deadlock detected");
}

// =============================================================================
// Scope Edge Cases
// =============================================================================

#[test]
fn test_use_before_define() {
    expect_error(
        "use_before_define.later",
        "variable `x` used before definition",
    );
}

#[test]
fn test_mutate_immutable() {
    expect_error(
        "mutate_immutable.later",
        "cannot mutate immutable variable `x`",
    );
}

#[test]
fn test_shadow_in_same_scope() {
    // Shadowing in same scope is allowed (like Rust)
    expect_output("shadow_same_scope.later", "20");
}

#[test]
fn test_closure_outlives_variable() {
    // Closure can't capture variable that goes out of scope
    expect_error(
        "closure_escape.later",
        "captured variable `x` does not live long enough",
    );
}

// =============================================================================
// Pattern Matching Edge Cases
// =============================================================================

#[test]
fn test_pattern_exhaustive_check() {
    // Non-exhaustive patterns should error
    expect_error(
        "pattern_nonexhaustive.later",
        "non-exhaustive pattern match",
    );
}

#[test]
fn test_pattern_duplicate_binding() {
    // Same variable bound twice in pattern
    expect_error(
        "pattern_duplicate.later",
        "variable `x` bound multiple times",
    );
}

#[test]
fn test_pattern_or() {
    // Or patterns
    expect_output("pattern_or.later", "matched");
}

#[test]
fn test_pattern_guard() {
    // Pattern with guard clause
    expect_output("pattern_guard.later", "positive");
}

// =============================================================================
// Type System Edge Cases
// =============================================================================

#[test]
fn test_type_recursive() {
    // Recursive type definition
    expect_output("type_recursive.later", "node: 1");
}

#[test]
fn test_type_infinite_recursion() {
    // Infinite type should be an error
    expect_error("type_infinite.later", "infinite type");
}

#[test]
fn test_generic_instantiation() {
    // Generic function instantiation
    expect_output("generic_fn.later", "42");
}

#[test]
fn test_generic_constraint_violation() {
    expect_error(
        "generic_constraint.later",
        "type does not satisfy constraint",
    );
}

// =============================================================================
// Identifier Edge Cases
// =============================================================================

#[test]
fn test_kebab_case_variable() {
    // Kebab-case identifiers are valid
    expect_output("kebab_ident.later", "42");
}

#[test]
fn test_kebab_minus_disambiguation() {
    // `a-b` could be identifier or subtraction - context determines
    expect_output("kebab_minus.later", "10");
}

#[test]
fn test_underscore_identifier() {
    // Underscore alone is a wildcard, can't be used as variable
    expect_error("underscore_ident.later", "cannot use `_` as variable");
}

#[test]
fn test_keyword_as_field() {
    // Keywords can be field names
    expect_output("keyword_field.later", "42");
}

// =============================================================================
// String Edge Cases
// =============================================================================

#[test]
fn test_string_empty() {
    expect_output("string_empty.later", "");
}

#[test]
fn test_string_unicode() {
    expect_output("string_unicode.later", "hello 世界 🌍");
}

#[test]
fn test_string_multiline() {
    // Strings can span multiple lines
    expect_output("string_multiline.later", "line1\nline2");
}

#[test]
fn test_string_interpolation_nested() {
    // Interpolation can contain expressions
    expect_output("string_interp_nested.later", "result: 15");
}

#[test]
fn test_string_interpolation_escape() {
    // Escaped braces in interpolated strings
    expect_output("string_interp_escape.later", "literal {brace}");
}

// =============================================================================
// Operator Precedence
// =============================================================================

#[test]
fn test_precedence_mul_add() {
    // Left-to-right precedence (no BODMAS) like Raro?
    expect_output("prec_mul_add.later", "20");
}

#[test]
fn test_precedence_comparison_chain() {
    // Chained comparisons: 1 < 2 < 3
    expect_output("prec_chain.later", "true");
}

#[test]
fn test_precedence_pipe_vs_call() {
    // Pipe has low precedence
    expect_output("prec_pipe.later", "15");
}

#[test]
fn test_precedence_and_or() {
    // Left-to-right, no special binding
    expect_output("prec_and_or.later", "false");
}

// =============================================================================
// Function Edge Cases
// =============================================================================

#[test]
fn test_fn_no_params() {
    expect_output("fn_no_params.later", "42");
}

#[test]
fn test_fn_single_expr_body() {
    // Single expression function without braces
    expect_output("fn_single_expr.later", "10");
}

#[test]
fn test_fn_rest_params() {
    // Rest parameter: fn f(a, ...rest) { }
    expect_output("fn_rest.later", "[2, 3, 4]");
}

#[test]
fn test_fn_default_params() {
    // Default parameter values
    expect_output("fn_default.later", "42");
}

#[test]
fn test_fn_named_args() {
    // Named arguments: f(x: 1, y: 2)
    expect_output("fn_named_args.later", "3");
}

#[test]
fn test_fn_recursive_tail() {
    // Tail recursive function should not overflow stack
    expect_output("fn_tail_rec.later", "1000000");
}

// =============================================================================
// Defer Edge Cases
// =============================================================================

#[test]
fn test_defer_order_nested() {
    // Nested defers in nested blocks
    expect_output("defer_nested.later", "inner\nouter");
}

#[test]
fn test_defer_in_loop() {
    // Defer in loop runs each iteration
    expect_output("defer_loop.later", "0\n1\n2");
}

#[test]
fn test_defer_captures_current_value() {
    // Defer captures value at defer time, not cleanup time
    expect_output("defer_capture.later", "1");
}

#[test]
fn test_defer_return_value() {
    // Return inside defer block should be an error
    expect_error("defer_return.later", "cannot return from defer block");
}

#[test]
fn test_defer_break() {
    // Break inside defer should be an error
    expect_error("defer_break.later", "cannot break from defer block");
}

// =============================================================================
// Block Expression Edge Cases
// =============================================================================

#[test]
fn test_block_value() {
    // Block evaluates to last expression
    expect_output("block_value.later", "3");
}

#[test]
fn test_block_empty() {
    // Empty block evaluates to nil
    expect_output("block_empty.later", "nil");
}

#[test]
fn test_block_trailing_semicolon() {
    // Trailing semicolon means nil
    expect_output("block_trailing_semi.later", "nil");
}

// =============================================================================
// Integration: Linear Types + Concurrency
// =============================================================================

#[test]
fn test_linear_concurrent_access() {
    // Two tasks cannot both hold the same linear resource
    expect_error(
        "linear_concurrent.later",
        "linear value moved to another task",
    );
}

#[test]
fn test_linear_channel_transfer() {
    // Linear values can be safely transferred via channels
    expect_output("linear_channel.later", "file closed in receiver");
}

#[test]
fn test_linear_nursery_scope() {
    // Linear values can't escape nursery scope
    expect_error(
        "linear_nursery_escape.later",
        "linear value cannot escape nursery",
    );
}

// =============================================================================
// Integration: Effects + Linear Types
// =============================================================================

#[test]
fn test_effect_linear_cleanup_on_resume() {
    // Handler that resumes: linear values stay valid
    expect_output("effect_linear_resume.later", "file closed");
}

#[test]
fn test_effect_linear_cleanup_on_abort() {
    // Handler that aborts: linear values must be cleaned up
    expect_output("effect_linear_abort.later", "cleanup ran\naborted");
}

// =============================================================================
// Integration: Cancellation + Linear Types
// =============================================================================

#[test]
fn test_cancel_linear_transfer_race() {
    // Race between cancel and linear value transfer
    expect_output("cancel_linear_race.later", "resource properly handled");
}

// =============================================================================
// Integration: Defer + Effects
// =============================================================================

#[test]
fn test_defer_can_send_effect() {
    // Defer block can send effects
    expect_output("defer_effect.later", "cleanup effect handled");
}

#[test]
fn test_defer_effect_unhandled() {
    // Unhandled effect in defer is an error
    expect_error("defer_effect_unhandled.later", "unhandled effect in defer");
}

// =============================================================================
// Integration: Multistage + Linear Types
// =============================================================================

#[test]
fn test_comptime_linear_error() {
    // Can't have linear types at compile time (no cleanup at compile time)
    expect_error(
        "comptime_linear.later",
        "linear types not allowed at compile time",
    );
}

#[test]
fn test_startup_linear_ok() {
    // Startup can use linear types (has runtime cleanup)
    expect_output("startup_linear.later", "startup resource cleaned");
}

// =============================================================================
// Error Recovery Patterns
// =============================================================================

#[test]
fn test_error_with_cleanup() {
    // Error propagates but cleanup still runs
    expect_output("error_cleanup.later", "cleanup ran\nerror: oops");
}

#[test]
fn test_error_in_cleanup() {
    // Error during cleanup - first error wins, cleanup error logged
    expect_output("error_in_cleanup.later", "original error");
}

#[test]
fn test_multiple_errors_cleanup() {
    // Multiple cleanup errors - all are logged
    expect_output("multi_error_cleanup.later", "all cleanup errors logged");
}

// =============================================================================
// Stress Tests (for implementation robustness)
// =============================================================================

#[test]
fn test_deeply_nested_blocks() {
    // 100 nested blocks
    expect_output("nested_deep.later", "100");
}

#[test]
fn test_many_defers() {
    // 100 defers in sequence
    expect_output("many_defers.later", "100 cleanups ran");
}

#[test]
fn test_many_spawns() {
    // Spawn 100 concurrent tasks
    expect_output("many_spawns.later", "100 tasks completed");
}

#[test]
fn test_long_pipe_chain() {
    // Long pipe chain
    expect_output("long_pipe.later", "42");
}

#[test]
fn test_recursive_data_structure() {
    // Deeply nested data structure
    expect_output("recursive_data.later", "depth: 100");
}

// =============================================================================
// Error Message Quality
// =============================================================================

#[test]
fn test_error_column_number() {
    // Error messages should include column number
    expect_error("error_column.later", "column 10");
}

#[test]
fn test_error_context_snippet() {
    // Error messages should show the offending code
    expect_error("error_context.later", "let x = ???");
}

#[test]
fn test_error_suggestion() {
    // Error messages should suggest fixes when possible
    expect_error("error_suggestion.later", "did you mean");
}

#[test]
fn test_error_multiple() {
    // Multiple errors should all be reported
    expect_error("error_multi.later", "2 errors");
}

#[test]
fn test_error_recovery() {
    // Parser should recover and find more errors
    expect_error("error_recovery.later", "error 1");
}

// =============================================================================
// Borrowing Rules
// =============================================================================

#[test]
fn test_borrow_read_read() {
    // Multiple read borrows are allowed
    expect_output("borrow_read_read.later", "both accessed");
}

#[test]
fn test_borrow_read_write_error() {
    // Can't write while borrowed for reading
    expect_error("borrow_read_write.later", "cannot mutate while borrowed");
}

#[test]
fn test_borrow_write_read_error() {
    // Can't read while borrowed for writing
    expect_error(
        "borrow_write_read.later",
        "cannot access while mutably borrowed",
    );
}

#[test]
fn test_borrow_write_write_error() {
    // Can't have two mutable borrows
    expect_error("borrow_write_write.later", "cannot borrow mutably twice");
}

#[test]
fn test_borrow_outlives_error() {
    // Borrow cannot outlive the borrowed value
    expect_error("borrow_outlives.later", "borrow outlives borrowed value");
}

#[test]
fn test_borrow_through_function() {
    // Function can borrow and return
    expect_output("borrow_fn.later", "42");
}

// =============================================================================
// Move Semantics
// =============================================================================

#[test]
fn test_move_after_move() {
    // Can't use a value after moving it
    expect_error("move_double.later", "value already moved");
}

#[test]
fn test_move_after_borrow() {
    // Can't move while borrowed
    expect_error("move_borrowed.later", "cannot move borrowed value");
}

#[test]
fn test_move_in_condition() {
    // Move in condition body creates uncertainty
    expect_error("move_condition.later", "value may have been moved");
}

#[test]
fn test_move_in_loop_error() {
    // Move in loop is an error (unless rebinding)
    expect_error("move_loop.later", "cannot move in loop");
}

#[test]
fn test_move_rebind_ok() {
    // Moving then rebinding is OK
    expect_output("move_rebind.later", "42");
}

// =============================================================================
// Miscellaneous Edge Cases
// =============================================================================

#[test]
fn test_zero_divide_float() {
    // Float division by zero produces infinity, not error
    expect_output("float_div_zero.later", "inf");
}

#[test]
fn test_negative_array_size() {
    // Negative array size is an error
    expect_error("neg_array_size.later", "array size cannot be negative");
}

#[test]
fn test_empty_match() {
    // Match with no arms is an error
    expect_error("empty_match.later", "match requires at least one arm");
}

#[test]
fn test_cyclic_import() {
    // Cyclic imports are detected
    expect_error("cyclic_import.later", "cyclic import detected");
}

#[test]
fn test_self_reference() {
    // Self-referential value
    expect_error(
        "self_ref.later",
        "cannot reference value during its own initialization",
    );
}
