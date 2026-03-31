// Week 10: Mastering ownership and borrowing

fn main() {
    println!("Week 10: Mastering ownership and borrowing");
    println!("Uncomment one problem at a time and fix it!\n");

    // Uncomment problems one at a time after fixing them:
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
}

// ============================================================================
// PROBLEM 1: Value used after move
// ============================================================================
fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("  The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

// ============================================================================
// PROBLEM 2: Immutable and mutable borrow conflict
// ============================================================================
fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");
    let r1 = &s; // immutable borrow
    println!("  {}", r1); // use immutable borrow here

    let r2 = &mut s; // mutable borrow — now OK!
    println!("  {}", r2);
}

// ============================================================================
// PROBLEM 3: Mutating through an immutable reference
// ============================================================================
fn problem_3() {
    println!("Problem 3: Mutating through an immutable reference");
    let mut s = String::from("hello");
    add_to_string(&mut s);
    println!("  Result: {}", s);
}

fn add_to_string(s: &mut String) {
    s.push_str(", world");
}

// ============================================================================
// PROBLEM 4: Multiple mutable borrows
// ============================================================================
fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("  {}", r1);
    } // r1 is dropped here

    {
        let r2 = &mut s; // OK now, r1 is gone
        println!("  {}", r2);
    }
}

// ============================================================================
// PROBLEM 5: Dangling reference
// ============================================================================
fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string();
    println!("  Got: {}", r);
}

fn create_string() -> String {
    String::from("hello")
}

// ============================================================================
// PROBLEM 6: Ownership in loops
// ============================================================================
fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");

    for i in 0..3 {
        print_with_number(&data, i);
    }
}

fn print_with_number(s: &str, n: i32) {
    println!("  {}: {}", n, s);
}

// ============================================================================
// PROBLEM 7: Lifetime — reference doesn't live long enough
// ============================================================================
fn problem_7() {
    println!("Problem 7: Lifetime extension");

    let s = String::from("inner scope");
    let result = &s;

    println!("  Result: {}", result);
}

// ============================================================================
// PART 2 — Implementation exercises
// ============================================================================

/// Takes ownership of a String, converts it to uppercase, and returns it.
pub fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

/// Borrows a String immutably and returns its length.
pub fn string_length(s: &str) -> usize {
    s.len()
}

/// Borrows a String mutably and appends `suffix` to it in place.
pub fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

/// Creates a new owned String by concatenating two borrowed string slices.
pub fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase_owned() {
        let s = String::from("hello");
        let result = to_uppercase_owned(s);
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_to_uppercase_owned_already_upper() {
        let s = String::from("RUST");
        assert_eq!(to_uppercase_owned(s), "RUST");
    }

    #[test]
    fn test_string_length() {
        let s = String::from("testing");
        let len = string_length(&s);
        assert_eq!(len, 7);
        // Original string must still be usable after the borrow.
        assert_eq!(s, "testing");
    }

    #[test]
    fn test_string_length_empty() {
        let s = String::from("");
        assert_eq!(string_length(&s), 0);
    }

    #[test]
    fn test_append_suffix() {
        let mut s = String::from("hello");
        append_suffix(&mut s, ", world");
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_append_suffix_empty() {
        let mut s = String::from("");
        append_suffix(&mut s, "hi");
        assert_eq!(s, "hi");
    }

    #[test]
    fn test_concat_strings() {
        let result = concat_strings("hello", " world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_concat_strings_empty() {
        assert_eq!(concat_strings("", "abc"), "abc");
        assert_eq!(concat_strings("abc", ""), "abc");
    }
}
