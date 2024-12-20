pub fn safe_add(a:usize, b:usize) -> usize {
    if usize::MAX - a < b || usize::MAX - b < a { usize::MAX }
    else { a + b }
}