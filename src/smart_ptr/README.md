# Smart Pointers

Data structures that act like a pointer but also have additional metadata and capabilities.

Typically implemented using structs by implementing the `Deref` and `Drop` traits so they can be used like a reference.

## Using `Box<T>` to Point to Data on the Heap

Boxes allow you to store data on the heap rather than the stack.

### Defining Your Own Smart Pointer

Observe the following code:

```rs
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

This code won't compile. Why?

The `MyBox<T>` type can't be dereferenced because it doesn't implement the `Deref` trait.

When we call `*y`, Rust expands it to `*(y.deref())`. Rust keeps doing this until it has a value of type `T`.

### Using Deref Coercion in Functions and Methods

_Deref Coercion_ is a feature that Rust provides to simplify calling methods on dereferenced values.

Observe the following code:

```rs
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

Rust can turn `&MyBox<String>` into `&String` by using deref coercion.

## Running Code on Cleanup with the `Drop` Trait

### Cleaning Up a Value BEFORE the End of Scope using `std::mem::drop`

Observe the following code:

```rs
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}
```

## `RC<T>`: The Reference-Counted Smart Pointer

Keeps track of the number of references to a value on the heap. If # of references == 0, the value is dropped.

We do this via `RC::new` and `RC::clone`. To see the explicit number of references, we use `RC::strong_count`.

```rs
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

## `RefCell<T>`: The Interior-Mutability Smart Pointer

Instead of dealing with compiler rules, we deal with panics at runtime.

### Tracking Borrows at Runtime using `borrow` and `borrow_mut`

- `borrow` returns the smart pointer type `Ref<T>`
- `borrow_mut` returns the smart pointer type `RefMut<T>`
- Both types implement `Deref`, so can be treated like references

### Allowing Multiple Owners of Mutable Data Using `Rc<RefCell<T>>`

With this, `RefCell<T>` ensures that you don't break borrowing rules (like having two mutable borrows at once) by checking them at runtime. If you violate these rules, your program will panic rather than fail to compile.

To see an example, look at `src/bin/mutable_cons.rs`

## When to use `Box<T>`, `Rc<T>`, and `RefCell<T>`

- `Box<T>`: Single owner; allows immutable or mutable borrows checked at compile time
- `Rc<T>`: Multiple owners; allows ONLY immutable borrows checked at compile time
- `RefCell<T>`: Single owner; allows immutable or mutable borrows checked at runtime. Can mutate the value `T` even if `T` is immutable -- **interior mutability pattern**

## Reference Cycles Can Leak Memory

TODO
