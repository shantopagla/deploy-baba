# Zero-Cost Philosophy

deploy-baba is built on a single principle: **abstractions should cost nothing
at runtime.**

## What Zero-Cost Means in Practice

When you write generic code using deploy-baba's traits, the Rust compiler
generates specialized machine code for each concrete type through
monomorphization. There is no vtable lookup, no heap allocation for trait
objects, no runtime dispatch overhead.

```rust
// This generic function becomes specialized at compile time
// for TomlParser, YamlParser, JsonParser — no runtime cost.
fn load_config<P: ConfigParser<MyConfig>>(source: &str) -> Result<MyConfig, ConfigError> {
    P::parse(source)
}
```

The compiler produces three distinct, optimized functions — one per parser type.
The trait boundary exists only at compile time.

## Why Not `dyn Trait`?

Dynamic dispatch (`Box<dyn ConfigParser>`) adds:
- A heap allocation per trait object
- An indirect function call (vtable) on every method invocation
- Missed optimization opportunities (the compiler can't inline across vtables)

For a deployment automation tool that processes config files and generates specs,
these costs are small in absolute terms. But the zero-cost approach also gives
better error messages (concrete types in stack traces) and eliminates an entire
class of lifetime complexity (`dyn Trait + 'a`).

## Where This Shows Up

Every trait in deploy-baba is designed for static dispatch:

- `ConfigParser<T>` — generic over the config type
- `ApiSpecGenerator` — associated types for Schema and Output
- `SpecFormatConverter<T>` — generic over the target format

The merger (`api-merger`) is the one place where we use enum dispatch
(`UnifiedApiSpec`) instead of generics, because the set of formats is closed
and known at compile time. This is still zero-cost — it compiles to a match
statement, not a vtable.

## Trade-Off

The cost of monomorphization is compile time. Each generic instantiation
produces a new copy of the function in the binary. For a library with 3-5
format implementations, this is negligible. For a library with hundreds of
instantiations, you'd want to measure binary size.

deploy-baba stays well within the negligible range.
