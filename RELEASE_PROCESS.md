# Release process

1. Update `Cargo.toml` with the new version
2. Run `bin/build`
3. Create a new bump commit (e.g. `Bump up version (v0.0.0)`)
4. Create a corresponding tag (e.g. `v0.0.0`)
5. Push everything with `git push && git push --version`
6. Create a new release at https://github.com/fnando/sqlite-shortid/releases/new
7. Attach files from `build/v0.0.0/*.zip`
