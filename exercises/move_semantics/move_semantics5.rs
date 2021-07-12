// move_semantics5.rs
// Make me compile without adding, removing, or changing any of the
// lines in `main()`.
// Execute `rustlings hint move_semantics5` for hints :)

// I AM NOT DONE

fn main() {
    let mut x = 100;
    let y = &mut x;
    
    *y += 100;
    assert_eq!(*y, 200);
    let z = &mut *y;
    assert_eq!(*z, 200);
    *z += 1000;
    assert_eq!(x, 1200);
}
