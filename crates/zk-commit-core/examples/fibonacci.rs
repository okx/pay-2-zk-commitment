use anyhow::Result;

use zk_commit_core::fibonacci::fibonacci;


fn main() -> Result<()> {
    let result = fibonacci()?;
    println!(
        "100th Fibonacci number mod |F| (starting with {}, {}) is: {}",
        result.input.0, result.input.1, result.output
    );
    Ok(())
}
