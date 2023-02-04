use std::fmt;

const STACK_TRACE_DEPTH: usize = 128;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// The program counter of the stack trace.
pub struct StackTrace {
    /// The program counter of the stack trace.
    pub pc: Vec<usize>,
    /// The cause stack trace of the stack trace.
    pub cause_stack_trace: Option<Box<StackTrace>>,
}

impl StackTrace {
    /// Formats the stack trace.
    pub fn format_stack_trace(stack_trace: Vec<String>) -> String {
        format!(
            "Stack trace:\n{}",
            stack_trace
                .iter()
                .map(|elem| format!("\t- {elem}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    /// Creates a new stack trace.
    pub fn new() -> StackTrace {
        let pc = Vec::with_capacity(STACK_TRACE_DEPTH);
        StackTrace {
            pc,
            cause_stack_trace: Some(Box::new(StackTrace {
                pc: Vec::with_capacity(STACK_TRACE_DEPTH),
                cause_stack_trace: None,
            })),
        }
    }
}

impl Default for StackTrace {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StackTrace {
    /// The formatted stack trace.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stack_trace = self
            .pc
            .iter()
            .map(|&pc| format!("{pc:#x}"))
            .collect::<Vec<_>>();
        let formatted_stack_trace = StackTrace::format_stack_trace(stack_trace);

        match &self.cause_stack_trace {
            Some(cause_stack_trace) => {
                write!(f, "{formatted_stack_trace}\n{cause_stack_trace}")
            }
            None => write!(f, "{formatted_stack_trace}"),
        }
    }
}
