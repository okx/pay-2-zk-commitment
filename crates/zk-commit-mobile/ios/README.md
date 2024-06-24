## Run iOS project and example
- XCode must be installed

### Build XCFramework and add to demo project
``` bash
./crates/zk-commit-mobile/scripts/build_ios.sh
```

### Build ios project without opening xcode and launch in simulator
``` bash
./crates/zk-commit-mobile/scripts/run_project.sh
```

### Test performance

Can put your code and in `./crates/zk-commit-mobile/src/lib.rs` `fn test_performance`

```rust
#[uniffi::export]
pub fn test_performance() {
    // Add your test code here
    // println!("testing performance")
}
```