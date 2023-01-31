#[cfg(test)]

// FIXME: Add more tests to bring the code coverage to 100%
mod tests {
    extern crate errors;
    use errors::stacktrace::StackTrace;

    #[test]
    fn test_stack_trace_new() {
        let stack_trace = StackTrace::new();
        assert_eq!(stack_trace.pc.len(), 0);
    }

    #[test]
    fn test_format_stack_trace_default() {
        let stack_trace = StackTrace::default();
        assert_eq!(stack_trace.pc.len(), 0);
    }

    #[test]
    fn test_display_stack_trace() {
        let test_display_stack_trace = StackTrace::new();
        assert!(!test_display_stack_trace.to_string().is_empty());
    }
}
