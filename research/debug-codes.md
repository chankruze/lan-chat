```rust
for record in resp.records() {
    println!("📦 Record: {record:?}");

    if let RecordKind::TXT(txt_records) = &record.kind {
        println!("📝 TXT Records:");
        for txt in txt_records {
            println!("  - {txt}");
        }
    }
}
```
