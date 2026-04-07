// Week 12: Generics and traits
//
// Implement a generic Stack<T> data structure and make it work with Rust's
// standard Display and Iterator traits.
//
// Run: cargo test

use std::fmt;

fn main() {
    println!("Week 12: Generics and traits");

    let mut s: Stack<i32> = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    // requires Display impl
    println!("Stack contents: {}", s);

    // requires IntoIterator impl
    println!("Popping items:");
    for item in s {
        println!("{}", item);
    }
}

// ============================================================================
// STACK<T> — implement all methods and trait impls below.
// ============================================================================

/// A generic last-in, first-out (LIFO) stack.
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    /// Creates a new, empty stack.
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    /// Pushes `item` onto the top of the stack.
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    /// Removes and returns the top item, or `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Returns a reference to the top item without removing it,
    /// or `None` if the stack is empty.
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Returns `true` if the stack contains no items.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of items in the stack.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// ============================================================================
// DISPLAY — format the stack as "[bottom, ..., top]"
// ============================================================================
impl<T: fmt::Debug> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Formats the internal vector using its Debug implementation
        write!(f, "{:?}", self.data)
    }
}

// ============================================================================
// ITERATOR — consume the stack from top to bottom
// ============================================================================

pub struct StackIter<T> {
    stack: Stack<T>,
}

impl<T> Iterator for StackIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // LIFO order: pop from the end of the vector (the top of the stack)
        self.stack.pop()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIter { stack: self }
    }
}
// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // --- basic operations ---

    #[test]
    fn test_new_stack_is_empty() {
        let s: Stack<i32> = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_push_increases_len() {
        let mut s = Stack::new();
        s.push(1);
        assert_eq!(s.len(), 1);
        s.push(2);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_pop_returns_lifo_order() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut s: Stack<i32> = Stack::new();
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_peek_does_not_remove() {
        let mut s = Stack::new();
        s.push(42);
        assert_eq!(s.peek(), Some(&42));
        assert_eq!(s.len(), 1); // still there
    }

    #[test]
    fn test_peek_empty_stack() {
        let s: Stack<i32> = Stack::new();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn test_is_empty_after_pop() {
        let mut s = Stack::new();
        s.push(1);
        s.pop();
        assert!(s.is_empty());
    }

    // --- works with different types ---

    #[test]
    fn test_stack_of_strings() {
        let mut s = Stack::new();
        s.push(String::from("hello"));
        s.push(String::from("world"));
        assert_eq!(s.pop(), Some(String::from("world")));
    }

    #[test]
    fn test_stack_of_floats() {
        let mut s = Stack::new();
        s.push(1.1_f64);
        s.push(2.2_f64);
        assert_eq!(s.len(), 2);
    }

    // --- Display ---

    #[test]
    fn test_display_empty() {
        let s: Stack<i32> = Stack::new();
        assert_eq!(format!("{}", s), "[]");
    }

    #[test]
    fn test_display_with_items() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        // bottom → top, so display order is [1, 2, 3]
        assert_eq!(format!("{}", s), "[1, 2, 3]");
    }
}
