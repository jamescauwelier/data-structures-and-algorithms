# Data structures & algorithms

## Testing

### Memory leaks

Since there is ample use of unsafe rust code, memory leaks are a possibility.
To combat this, install [miri](https://github.com/rust-lang/miri) and use it to run test suites:

```shell
cargo +nightly miri test -Znext-lockfile-bump
```

There's also a cargo alias defined in some crates that can be called like this:

```shell
cargo +nightly ub
```

Note: `ub` stands for `undefined behavior`, which is what miri is testing.