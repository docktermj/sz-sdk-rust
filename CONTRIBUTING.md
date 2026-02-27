# Contributing

Thank you for your interest in contributing to `sz-sdk-rust`!

## Getting Started

1. Fork the repository and clone your fork.
2. Create a feature branch from `main`.
3. Make your changes and ensure all checks pass:

   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```

4. Open a pull request against `main`.

## Guidelines

- Follow existing code style and conventions (see [CLAUDE.md](.claude/CLAUDE.md) for details).
- Trait methods use `snake_case`; flag constants use `SCREAMING_SNAKE_CASE` with `SZ_` prefix.
- All fallible methods return `Result<T, SzError>`.
- Keep PRs focused — one logical change per pull request.
- Add tests for new functionality.

## License

By contributing, you agree that your contributions will be licensed under the
[Apache License, Version 2.0](LICENSE).
