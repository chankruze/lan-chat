### 📌 Syntax Breakdown:

```rust
Some(existing_peer_info)
    if !peer_info
        .metadata
        .is_different(&existing_peer_info.metadata) =>
```

Let’s break this down:

#### ✅ `Some(existing_peer_info)`

This is a **pattern** that matches if the result of `peers_map.get(&peer_info.id)` is a `Some(...)` variant. The `existing_peer_info` is a reference to the value inside the `Some`.

#### ✅ `if !peer_info.metadata.is_different(&existing_peer_info.metadata)`

This is the **match guard** — an additional condition that must also be true for this arm to match.

Together, this arm is only taken when:

- There **is** an existing peer (`Some(existing_peer_info)`), **and**
- The **metadata is not different**, meaning no meaningful update is needed.

### 🧠 General Match Guard Syntax:

```rust
match value {
    PATTERN if CONDITION => { ... },
    _ => { ... },
}
```

The `if` after a match arm is the guard — it's evaluated only if the pattern matches. If it evaluates to `false`, Rust will move on to try the next pattern.

### 👀 Example in Isolation:

```rust
match Some(42) {
    Some(x) if x > 50 => println!("Greater than 50: {}", x),
    Some(x) => println!("Got something: {}", x),
    None => println!("Got nothing"),
}
```

This will print: `Got something: 42`
Because although `Some(x)` matches, the guard `x > 50` fails — so it tries the next arm.
