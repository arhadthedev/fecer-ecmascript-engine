//! ECMA-262 15.0 LALR(1) stream parser.
//!
//! Implements sections 11-16 (every "ECMAScript Language: ...").
//!
//! A stream parser acts like the SAX XML one. Instead of creating a syntax tree
//! it creates a stream of grammar rule replacements, eagerly starting bottom-up
//! with, for example, `Terminal Identifier → Identifier` and ending
//! with `FuncList → Program`.

/// Grammar symbols the parser works with.
#[derive(Debug, Eq, PartialEq)]
pub enum Symbol {
    SourceCharacter(u16),

    Eof
}

/// Apply ("reduce") a fitting replacement rule to a grammar symbol stack top.
///
/// LALR(1) parsers do parsing by repeatedly consolidating input Unicode code
/// points into larger and larger abstractions like literals, expressions,
/// sentences, sentence lists, blocks, functions, and a script file.
///
/// As a result, a user of the functions needs to consolidate already fed source
/// code as much as possible by calling the function until it returns Err. Only
/// after the consolidation another source code character may be read into the
/// end of a consolidated sequence followed by another round of consolidations.
///
/// When no more input character is available, a special Symbol::Eof needs to be
/// added into the consolidated sequence. This step determines whether another
/// returned Err means success or not. For the former, the Err contains
/// a sequence with a root symbol like Script or Module. Anything else means
/// a parse error.
///
/// Returns an input sequence with the consolidation replacement applied.
///
/// # Errors
///
/// When no replacement rule can be applied, returns an unmodified input wrapped
/// into Err.
///
/// # Examples
///
/// ```
/// use fecer_ecmascript_engine::parser::{reduce_once, Symbol};
/// use std::iter::once;
/// use std::str::EncodeUtf16;
///
/// let input = "abc";
/// let tokens = input.encode_utf16()
///     .map(|c| Symbol::SourceCharacter(c))
///     .chain(once(Symbol::Eof));
///
/// let mut stack: Vec<Symbol> = Vec::new();
/// for token in tokens {
///     stack.push(token);
///     let mut can_process_more = true;
///     while can_process_more {
///         (stack, can_process_more) = match reduce_once(stack) {
///             Ok(modified_stack) => (modified_stack, true),
///             Err(old_stack) => (old_stack, false)
///         }
///     }
/// }
/// ```
///
/// When no grammar rule can be applied, the function returns Err:
///
/// ```
/// use fecer_ecmascript_engine::parser::reduce_once;
/// let empty_source = Vec::new();
/// assert!(reduce_once(empty_source) == Err(Vec::new()))
/// ```
pub fn reduce_once(stack: Vec<Symbol>) -> Result<Vec<Symbol>, Vec<Symbol>> {
    let mut top = stack.iter().rev();
    match top.next() {
        // Both branches return err until we add some proper rule
        None | Some(_) => Err(stack),
    }
}
