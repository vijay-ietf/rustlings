// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=eb49ff5fda98cc9d0bf8f60e04b4e6ea

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;

    *z += 1000;
    assert_eq!(x, 1200);
}
