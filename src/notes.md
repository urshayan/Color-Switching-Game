# Take Aways

- arrays in rust use usize indexing
- let mut rng = rand::thread_rng(); creates random engine
- Cleanup System
enemies.retain(|e| e.x + e.width > 0.0);
This line is chef-level Rust elegance 
What It Means
Keep only enemies still visible.
Anything past left edge = deleted.
C++ Equivalent
You’d write 10 lines of erase/remove loops.
Rust does it in one poetic line.

- for enemy in &enemies 
 bcs .. if you dont modify it borrow it..read only!!!

