# Asynchronous Programming

#### Concurrency

When an individual works on several different tasks before any of them is complete

![concurrency](../../assets/concurrency.svg)

#### Parallelism

When the team splits up a group of tasks by having each member take one task and work on it independently

![parallelism](../../assets/parallelism.svg)

## Futures and the Async Syntax

A _future_ is avalue that may not be ready now, but will be later.

- `async` is used on blocks and functions to specify that they can be interrupted and resumed
  - When Rust sees a _block_ marked with `async`, it compiles it into a data type that implements the `Future` trait
  - `async fn` = function that returns a `Future` of the return type
- `await` is used to wait for a future to complete
- _polling_ - checking with a future to see if its value is available yet

### Executing an Async Function with a Runtime

Each `await` point represents a potential point of suspension and control is handed back to the runtime. To make that work, Rust keeps track of the state involved in the async block so that the runtime could do other work and come back when the future is ready. It's an **invisible state machine**, and something has to execute this state machine - this is known as a _runtime_.

Observe the following code in [src/bin/web_scrape.rs](src/bin/web_scrape.rs).

The following code will web scrape from the Rust Programming Website:

```bash
cargo run --bin web_scrape "https://www.rust-lang.org/"
```

Here, we typically use the `trpl::block_on` function to set up the runtime and run the future returned by the `async` block until it's done. `trpl::block_on` is typically used so our top-level function can be async.

### Racing Two URLs Against Each Other Concurrently

Observe the following code in [src/bin/concurrent_urls.rs](src/bin/concurrent_urls.rs).

We can race two URLs against each other concurrently by using the `trpl::select` function.

Try testing it using two URLs. Take this for example:

```bash
cargo run --bin concurrent_urls "https://www.rust-lang.org/" "https://cplusplus.com/"
```

## Applying Concurrency with Async

Observe the following snippet. Full code found in [src/bin/trpl_join.rs](src/bin/trpl_join.rs).

```rs
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
```

Using `trpl::join`, we wait for both `fut1` and `fut2` to finish. We see the exact same order because the function is **fair** - it alternates between the two futures. This is the exact output:

```bash
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

But let's try the following variations:

### TODO

- Remove the async block from around either or both of the loops

```bash

```

#### Difference between `trpl::join` and `trpl::select`

`trpl::join` waits for both futures to finish, while `trpl::select` returns the first future to finish.

`trpl::join` is the AND operator, while `trpl::select` is the OR operator.

## Yielding Control to the Runtime

Consider the following code in [src/bin/sleep.rs](src/bin/sleep.rs).

When we run this code, we see the following output:

```bash
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished
```

Notice how the `a` future still runs for a bit before handing off control to `b`.

We want to yield control to the runtime, so we can run other tasks while we wait for the future to finish. Observe the following code in [src/bin/yield_now.rs](src/bin/yield_now.rs).

When we run this code, we see the following output:

```bash
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished
```

We see that the outputs are identical, but the difference is that `yield_now` allows the runtime to run other tasks while we wait for the future to finish, and is ideal for _cooperative multitasking_. The `yield_now` instruction takes nanoseconds; it simply moves the task from the front of the line to the back of the line. If no other tasks is waiting, it picks the same task up again immediately.

### TODO: Building Our Own Async Abstractions

[TODO]

## Streams: Futures in Sequence

Streams are a sequence of futures that are executed in order; an asynchronous form of iteration.

Observe the following code in [src/bin/stream.rs](src/bin/stream.rs).

We get the following output:

```bash
The value was: 2
The value was: 4
The value was: 6
The value was: 8
The value was: 10
The value was: 12
The value was: 14
The value was: 16
The value was: 18
The value was: 20
```

## A Closer Look at the Traits for Async

### The `Future` Trait

Let's look at the `Future` trait implementation:

```rs
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Now let's look at the `Poll` enum:

```rs
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```
