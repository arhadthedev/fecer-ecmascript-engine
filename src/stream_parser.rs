//! ECMA-262 15.0 LALR(1) stream parser.
//!
//! Implements sections 11-16 (every "ECMAScript Language: ...").
//!
//! A stream parser acts like the SAX XML one. Instead of creating a syntax tree
//! it creates a stream of grammar rule replacements, eagerly starting bottom-up
//! with, for example, `Terminal Identifier → Identifier` and ending
//! with `FuncList → Program`.

/// Grammar symbols recognized by the parser.
#[derive(Eq, PartialEq)]
pub enum Symbol {
}

/// Rules applied by the stream parser on another *reduce* operation.
#[derive(Eq, PartialEq)]
pub enum ParseEvent {
    NoRuleMatch
}

/// Apply ("reduce") a fitting replacement rule to a stack of grammar symbols.
///
/// To use a stream LALR parser, push one symbol into the source code stack
/// and call reduce_once in a loop until getting ParseEvent::NoRuleMatch.
/// After that, put another symbol into the stack and repeat everything
/// until you have nothing left to put into the stack. If the stack gets
/// something different from a single end grammar goal, the input code
/// has an error preventing a full reduction of the input.
///
/// # Examples
///
/// An empty stack always gives ParseEvent::NoRuleMatch:
///
/// ```
/// use fecer_ecmascript_engine::stream_parser::{ParseEvent, reduce_once};
///
/// let empty_source = Vec::new();
/// assert!(reduce_once(empty_source) == (ParseEvent::NoRuleMatch, Vec::new()))
/// ```
pub fn reduce_once(stack: Vec<Symbol>) -> (ParseEvent, Vec<Symbol>) {
    let mut top = stack.iter().rev();
    match top.next() {
        Some(_) => (ParseEvent::NoRuleMatch, stack),
        None => (ParseEvent::NoRuleMatch, stack)
    }
}
