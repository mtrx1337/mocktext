# What does it do

```bash
$ echo "hello world!" | mocktext
HeLlO WoRlD!
$ echo "hello world!" | mocktext -r
hElLo wOrLd!
$ mocktext
>hello world!
HeLlO WoRlD!
$ mocktext -h
<prints help>
```

# Build

```
cargo build
```

# Install

```bash
cargo build --release
cargo install --path .
```

# Why

To learn rust. I always wanted an easy way to generate mocktext and this seemed like a reasonable first project.
