# threadedbufreader x86 x86_64

This is my experiments with a shared mutability in Rust.

This a threaded buffered reader library optimized for x86 and x86_64 platforms
(in fact it may not work correctly on other platforms, due to the synchronization
that has been chosen), which allows to do IO and store data into memory buffer
in one thread while accessing that data/buffer from another thread.

The idea is to use shared mutable buffer and have two objects "Reader" and "Writer"
that can access it. "Writer" is data producer, it reads data from a given Read implementor
and stores it into memory. "Reader" reads data from the memory.

That's it.
