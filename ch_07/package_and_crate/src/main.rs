// it's usual to use full path when we introduce struct.
use std::collections::HashMap;
// unless we introduce multi struct with the same name.
/*
use std::fmt;
use std::io;

fn funtion1() -> fmt::Result {
    // --snip--
    Ok(())
}
fn funtion2() -> io::Result<()> {
    // --snip--
    Ok(())
}
*/
// or else, we can provide another name for the struct with same name with `as`
use std::fmt::Result;
use std::io::Result as IoResult;

fn funtion1() -> Result {
    Ok(())
}

fn funtion2() -> IoResult<()> {
    Ok(())
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Re-exporting Names
// see it in lib.rs
