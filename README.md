# StepFloat

Make funny-looking f32-based loops possible in Rust:

```rust
for step in StepFloat::new(200000.0)..StepFloat::new(200001.0) {
    println!("{:x}", step.to_bits()); // prints all bit patterns IEEE754 32-bit floats have in that range :)
}
```

# Why

Don't know.
