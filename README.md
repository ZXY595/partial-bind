# Rust Partial Bind
A simple proc-macro library that provides a macro to partial bind function arguments using closures.
Which is known as partial function application.

# Syntax:
```rust
bind!(foo(1, 2, _, _, 3)); // which generates: |__1, __2| foo(1, 2, __1, __2, 3)
```

# Usage:
## Case: Add context for api handler in web backend:
```rust
let (tx, rx) = async_channel::unbounded();

tokio::spawn(async move {tx.send(1).await.unwrap()});

router.route("/api/1", get(bind!(handler(rx))))

async fn handler(rx: async_channel::Receiver<i32>) -> i32 {
    rx.recv().await.unwrap()
}
```
