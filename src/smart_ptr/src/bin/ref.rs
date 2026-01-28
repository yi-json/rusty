fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y = Box::new(x); // pointing to a copied value of x
    assert_eq!(5, *y)
}
