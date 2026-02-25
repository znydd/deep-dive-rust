# Rust Mastery: One Capstone Project Per Topic (Topics 1–14)

> Each project is designed to be the **single best exercise** for its topic — touching every major concept in one cohesive build. Work through them in order; later projects assume earlier ones are done.

---

## Topic 1 — Foundations
### Project: **CLI Unit Converter**

Build a command-line unit converter that runs as a REPL (read-eval-print loop).

**What to build:**
A program that accepts input like `42 km to miles` or `100 f to c` and prints the result, looping until the user types `quit`.

**Every concept you must use:**
- `cargo new` to set up the project, run with `cargo run`
- `const` for conversion factors (e.g., `const KM_TO_MILES: f64 = 0.621371`)
- Variables with explicit type annotations (`let input: String`)
- Shadowing: re-bind `input` after trimming whitespace
- All scalar types: use `u32` for a loop counter, `f64` for values, `bool` for a "keep running" flag, `char` to check the first character of input
- Compound types: store parsed tokens in a tuple `(f64, &str, &str)`; store supported unit names in an array
- `loop`, `while`, and `for` — use each at least once (loop for REPL, for to iterate unit names, while for retry logic)
- Functions: `fn parse_input(s: &str) -> ...`, `fn convert(value: f64, from: &str, to: &str) -> ...`, `fn print_result(...)`
- `if / else if / else` for unit dispatch
- `match` for the unit string (at least 6 conversion cases)
- `println!` with format specifiers (`{:.4}`, `{:>10}`)
- Read from stdin with `std::io::stdin().read_line()`
- Exit cleanly on `quit` using `break` and `std::process::exit`

**Supported conversions (minimum):** km↔miles, kg↔lbs, °C↔°F, liters↔gallons, meters↔feet, hours↔minutes

**Sample session:**
```
> 100 km to miles
100.0000 km = 62.1371 miles

> 37 c to f
37.0000 °C = 98.6000 °F

> quit
Goodbye!
```

---

## Topic 2 — Ownership System
### Project: **In-Memory Text Editor**

Build a simple line-based text buffer (like a stripped-down `ed`) that lets the user append, insert, delete, view, and search lines.

**What to build:**
A struct `TextBuffer` backed by `Vec<String>` with a command REPL that manipulates it.

**Every concept you must use:**
- **Move semantics**: `fn load(content: String) -> TextBuffer` takes ownership of the string; the caller can't use `content` after
- **Borrowing**: `fn display(&self)` takes `&self`; `fn append(&mut self, line: &str)` takes `&mut self` and `&str`
- **Simultaneous borrow rules**: demonstrate (in a comment) why you can't call `display` while holding a mutable ref; show the fix
- **Slices**: `fn search(&self, query: &str) -> Vec<&str>` returns borrowed slices of lines — understand the returned lifetime ties to `&self`
- **String slices**: `fn first_word(line: &str) -> &str` returns a `&str` slice into the line
- **Clone**: `fn clone_buffer(&self) -> TextBuffer` — use `.clone()` explicitly; explain cost
- **Copy types**: use `usize` indices freely without worrying about moves
- **Drop**: implement `Drop for TextBuffer` that prints `"Buffer dropped with N lines"`
- **Lifetime in function signature**: write `fn longest_line<'a>(a: &'a str, b: &'a str) -> &'a str`
- **`'static` lifetime**: define a `const WELCOME: &'static str`

**Commands:** `append <text>`, `insert <n> <text>`, `delete <n>`, `view`, `search <word>`, `undo` (keep a `Vec<TextBuffer>` snapshot using `clone`), `quit`

---

## Topic 3 — Structs & Enums
### Project: **Task Manager CLI**

Build a to-do/task manager where tasks have priorities, statuses, and due dates (as strings).

**What to build:**
A full CRUD task manager in a REPL using structs and enums for everything.

**Every concept you must use:**
- **Struct with named fields**: `struct Task { id: u32, title: String, description: String, priority: Priority, status: Status, due: Option<String> }`
- **Tuple struct**: `struct TaskId(u32)`
- **Unit struct**: `struct Separator;` with a `Display` impl that prints a divider line
- **Associated functions**: `Task::new(...)`, `Task::default()`
- **Methods**: `task.is_overdue()`, `task.summary()`, `task.mark_done()`
- **Struct update syntax**: create a "duplicate with different priority" using `Task { priority: Priority::High, ..existing_task.clone() }`
- **Enum with variants**: `enum Priority { Low, Medium, High, Critical }`, `enum Status { Todo, InProgress, Done, Cancelled }`
- **Enum with data**: `enum Command { Add { title: String, priority: Priority }, Delete(u32), Complete(u32), List(Option<Status>), Help, Quit }`
- **Pattern matching** on `Command` with all variants covered — compiler enforces exhaustiveness
- **`if let`**: `if let Some(due) = &task.due { ... }`
- **`while let`**: drain a queue of pending notifications with `while let Some(msg) = queue.pop()`
- **`Option`**: `due: Option<String>`, `fn find_task(&self, id: u32) -> Option<&Task>`
- **`Result`**: `fn parse_command(input: &str) -> Result<Command, String>`
- **Match guards**: `Priority::Critical if task.is_overdue() => ...`
- **Nested destructuring**: destructure `Command::Add { title, priority }` in a match arm

---

## Topic 4 — Collections & Iterators
### Project: **Log File Analyzer**

Build a program that parses a hardcoded (or file-based) log, extracts structured data, and produces a statistical report — entirely through iterator chains.

**What to build:**
Parse lines like `[2024-01-15 ERROR] Database connection failed (module=db, duration_ms=342)` into structs, then analyze.

**Every concept you must use:**
- **`Vec`**: store all parsed `LogEntry` structs
- **`HashMap<String, usize>`**: count occurrences by log level, by module
- **`HashMap<String, Vec<u64>>`**: group durations by module for average calculation
- **`HashSet<String>`**: collect unique error messages (dedup)
- **`BTreeMap<String, usize>`**: sorted frequency map for report (iterates in key order)
- **`entry().or_insert()`** and **`entry().or_default()`**
- **`.iter()`, `.iter_mut()`, `.into_iter()`** — use all three in different contexts
- **`map`**: transform `&str` lines into `Option<LogEntry>`
- **`filter_map`**: skip unparseable lines while transforming
- **`flat_map`**: flatten tags from each entry into one iterator
- **`fold`**: compute total duration and max duration in one pass
- **`collect::<HashMap<_, _>>()`** and **`collect::<HashSet<_>>()`**
- **`enumerate`**: add line numbers to error messages
- **`zip`**: pair log entries with their line numbers from a separate iterator
- **`chain`**: combine error and warning iterators for "issues" report
- **`take` and `skip`**: paginate results
- **`sort_by_key`**: sort modules by error count descending
- **Custom iterator**: implement `Iterator` on `struct LogWindow` that yields sliding windows of 3 consecutive entries (for spike detection)
- **`any` and `all`**: `entries.iter().any(|e| e.level == "CRITICAL")`
- **`max_by_key` and `min_by_key`**: find slowest and fastest requests

**Report output:**
```
=== Log Analysis Report ===
Total entries : 147
Errors        : 23
Warnings      : 41
Unique errors : 18

Top 3 slowest modules:
  db       avg 312ms  max 891ms
  cache    avg 87ms   max 203ms
  auth     avg 12ms   max 45ms
```

---

## Topic 5 — Error Handling
### Project: **CSV Data Pipeline**

Build a program that reads a CSV-like file (hardcoded or from args), parses records into typed structs, validates them, transforms the data, and writes a summary — with bulletproof error handling at every step.

**What to build:**
Process `employees.csv` with fields `id,name,department,salary,start_date`. Produce a department salary report.

**Every concept you must use:**
- **`panic!`** — one place it's appropriate: assert a truly impossible invariant with a comment explaining why
- **`Option`** — `fn find_employee(id: u32) -> Option<&Employee>`
- **`Result<T, E>`** — every fallible function returns `Result`
- **`?` operator** — used in at least 5 different functions
- **Custom error enum**:
  ```rust
  enum PipelineError {
      Io(std::io::Error),
      Parse { line: usize, field: String, value: String },
      Validation { id: u32, reason: String },
      EmptyDataset,
  }
  ```
- **`impl Display for PipelineError`** — human-readable messages for each variant
- **`impl Error for PipelineError`** — implement `source()` for `Io` variant
- **`From<std::io::Error> for PipelineError`** — enables `?` on IO operations
- **`From<std::num::ParseIntError> for PipelineError`** — enables `?` on parse calls
- **`thiserror` crate** — rewrite `PipelineError` with `#[derive(Error)]`; compare the two versions
- **`anyhow` crate** — write the top-level `main()` using `anyhow::Result` and `.context("...")`
- **`.map_err()`** — add context to a low-level error at the boundary
- **`collect::<Result<Vec<_>, _>>()`** — collect parsed records, failing on first error
- **Error recovery** — use `.unwrap_or_else()` to substitute a default salary if parsing fails for that field only
- **Multiple error sources** — the same `?` in one function that propagates both IO and parse errors (requires `From` impls)

**The pipeline:** read → parse lines → validate (salary > 0, name non-empty) → compute per-department stats → write report

---

## Topic 6 — Generics & Traits
### Project: **Generic Data Processing Pipeline**

Build a type-safe, reusable pipeline framework where stages can be composed generically, plus a set of concrete implementations.

**What to build:**
A `Pipeline<I, O>` that chains `Stage` implementations. Build concrete stages for: filtering, mapping, batching, deduplication, statistics.

**Every concept you must use:**
- **Generic struct**: `struct Pipeline<I, O> { stages: Vec<Box<dyn Stage<I, O>>> }` — and simpler `struct Batch<T> { items: Vec<T>, size: usize }`
- **Generic function**: `fn process<T: Clone + PartialEq>(items: Vec<T>) -> Vec<T>`
- **Trait definition**:
  ```rust
  trait Stage<I, O> {
      fn process(&self, input: Vec<I>) -> Vec<O>;
      fn name(&self) -> &str;
  }
  trait Summarize {
      fn summary(&self) -> String;
      fn word_count(&self) -> usize { self.summary().split_whitespace().count() }
  }
  ```
- **Default method in trait**: `word_count` above — test that it works without override
- **Multiple trait bounds**: `fn print_item<T: Debug + Display + Clone>(item: T)`
- **`where` clause**: use for a complex bound that's cleaner as `where`
- **`impl Trait` in parameter position**: `fn run_stage(stage: &impl Stage<i32, i32>, data: Vec<i32>)`
- **`impl Trait` in return position**: `fn make_doubler() -> impl Fn(i32) -> i32`
- **Trait objects `dyn Trait`**: `Vec<Box<dyn Stage<...>>>` — why generics alone can't work here
- **`From` / `Into`**: `struct Celsius(f64)` → `Fahrenheit` conversion
- **`Display` and `Debug`**: implement manually for at least one type
- **`Clone` and `Copy`**: derive for value types; implement `Clone` manually for a type with a custom deep copy
- **`PartialEq` / `Eq`**: implement for a struct with a custom equality rule (two records equal if same ID regardless of other fields)
- **`PartialOrd` / `Ord`**: implement for `Priority` enum
- **Operator overloading**: `Add` for a `Stats` struct (merge two stats objects)
- **Blanket implementation**: `impl<T: Display> Summarize for T`
- **Object safety**: show why a trait with `fn clone_box(&self) -> Box<Self>` is not object-safe; fix with `where Self: Sized`

---

## Topic 7 — Advanced Types
### Project: **Shared Configuration Store**

Build a thread-safe (for single-threaded use with interior mutability) configuration registry that supports nested configs, observers, and lazy defaults.

**What to build:**
`ConfigStore` holding `HashMap<String, ConfigValue>`, observable (registered callbacks fire on change), with lazy-evaluated defaults.

**Every concept you must use:**
- **`Box<T>`**: `type ConfigValue = Box<dyn Any>` — store heterogeneous values; also use `Box<dyn Fn()>` for callbacks
- **`Rc<T>`**: share a `ConfigNode` between two parts of a config tree (single-threaded sharing)
- **`Rc::clone` and `Rc::strong_count`**: track sharing; print count before and after dropping a clone
- **`Weak<T>`**: observers hold `Weak<ConfigStore>` — if the store is dropped, observers don't prevent cleanup
- **`RefCell<T>`**: `struct ConfigStore { data: RefCell<HashMap<String, String>>, observers: RefCell<Vec<Box<dyn Fn(&str, &str)>>> }` — mutate through `&self`
- **`borrow()` and `borrow_mut()`**: use both; show the runtime panic by calling `borrow_mut` twice (in a test, catch with `std::panic::catch_unwind`)
- **`Cell<T>`**: `struct AccessStats { reads: Cell<u32>, writes: Cell<u32> }` — track stats through `&self`
- **`Rc<RefCell<T>>`**: combine for a shared mutable config node in a tree
- **Reference cycles and `Weak`**: build parent↔child relationship where child holds `Weak<Parent>` to prevent cycle
- **Newtype pattern**: `struct ConfigKey(String)` — prevents mixing up key and value strings; implement `Deref<Target=String>`
- **Type alias**: `type Observer = Box<dyn Fn(&ConfigKey, &str)>`
- **`Cow<str>`**: `fn get_or_default<'a>(&'a self, key: &str, default: &'a str) -> Cow<'a, str>` — returns borrowed if found, owned if not
- **`ManuallyDrop`**: wrap a value in `ManuallyDrop` to defer its drop; call `ManuallyDrop::drop` explicitly at end of a scope
- **Never type `!`**: write `fn crash_if_missing(key: &str) -> !` that always panics
- **`?Sized`**: write `fn log_value<T: ?Sized + Debug>(val: &T)` — call with `&str` and `&[u8]`

---

## Topic 8 — Closures & Functional Patterns
### Project: **Middleware Pipeline (HTTP-like Request Handler)**

Build a composable middleware system where handlers and middlewares are closures, combined into a pipeline that processes "requests".

**What to build:**
`struct Request { path: String, method: String, headers: HashMap<String, String>, body: Option<String> }` and `struct Response { status: u16, body: String }`. Middlewares are closures that wrap a handler.

**Every concept you must use:**
- **Closure syntax variants**: same middleware written as named fn, typed closure, inferred closure, one-liner
- **Immutable capture (`Fn`)**: logging middleware captures a `&String` prefix by reference (must use `move` to own it) — use `Fn` because it's called repeatedly
- **Mutable capture (`FnMut`)**: request counter middleware mutates a `u64` counter on each call
- **Consuming capture (`FnOnce`)**: a one-time setup middleware that consumes a `Vec<String>` of allowed tokens
- **`move` keyword**: all closures stored in a struct must `move` their captures
- **`Fn`, `FnMut`, `FnOnce` trait bounds**: write `fn apply<F: Fn(&Request) -> Response>(handler: F, req: &Request) -> Response` — and variants for `FnMut` and `FnOnce`
- **Returning closures with `impl Fn`**: `fn make_auth_middleware(token: String) -> impl Fn(&Request) -> bool`
- **Returning closures with `Box<dyn Fn>`**: `fn make_middleware_chain(middlewares: Vec<Box<dyn Fn(...)>>) -> Box<dyn Fn(...)>` — necessary because sizes differ
- **Higher-order functions**: `fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C`
- **Closure as struct field**: `struct Router { routes: Vec<(String, Box<dyn Fn(&Request) -> Response>)> }`
- **Lazy evaluation**: `struct Lazy<T> { init: Option<Box<dyn FnOnce() -> T>>, value: Option<T> }` with a `get(&mut self) -> &T` method
- **Iterator with closures**: `router.routes.iter().find(|(path, _)| path == &req.path).map(|(_, handler)| handler(req))`
- **Event callbacks**: `struct EventEmitter { listeners: HashMap<String, Vec<Box<dyn Fn(&str)>>> }` with `on` and `emit`

**Build and run a pipeline:**
```
Request: GET /api/users
→ Logger middleware        [logs: "GET /api/users"]
→ Auth middleware          [checks Authorization header]
→ Rate limiter middleware  [increments counter, rejects if > 10/min]
→ Router                   [dispatches to handler]
← Response: 200 OK
```

---

## Topic 9 — Modules & Crate System
### Project: **Multi-Crate Library + CLI Workspace**

Build a Cargo workspace called `notes-app` with three crates, a feature flag, full documentation, and passing clippy/fmt.

**Workspace structure:**
```
notes-app/
├── Cargo.toml              (workspace)
├── core/                   (library crate)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── note.rs
│   │   │   └── tag.rs
│   │   ├── storage/
│   │   │   ├── mod.rs
│   │   │   └── memory.rs
│   │   └── search/
│   │       └── mod.rs
├── cli/                    (binary crate, depends on core)
│   └── src/main.rs
└── export/                 (library crate, depends on core, optional serde)
    └── src/lib.rs
```

**Every concept you must use:**
- **`mod` declarations**: every subdirectory needs a `mod.rs` or inline module
- **`pub`, `pub(crate)`, `pub(super)`**: `pub` for external API, `pub(crate)` for cross-module within `core`, `pub(super)` for storage internals
- **`use` and `pub use` re-exports**: `core/src/lib.rs` re-exports `Note`, `Tag`, `Storage` so users write `use core::{Note, Tag}` not `use core::models::note::Note`
- **Nested `use`**: `use core::{Note, storage::MemoryStorage}`
- **Workspace `Cargo.toml`**: `[workspace]` with `members = ["core", "cli", "export"]`
- **Inter-crate dependency**: `cli/Cargo.toml` has `core = { path = "../core" }` 
- **Feature flag**: `export/Cargo.toml` has `serde = ["dep:serde", "dep:serde_json"]` as optional feature; `#[cfg(feature = "serde")]` gates JSON export
- **`#[cfg(test)]` module**: inline tests in each source file
- **Integration tests**: `core/tests/integration.rs` tests the public API end-to-end
- **Doc comments `///`**: every public item has a doc comment with an `# Examples` section that passes `cargo test --doc`
- **`build.rs`**: in `core`, write a build script that embeds the build timestamp as a constant `pub const BUILD_TIME: &str`
- **`cargo clippy -- -D warnings`**: zero warnings
- **`cargo fmt --check`**: passes
- **`cargo test --workspace`**: all tests pass
- **External crates**: use `uuid` for note IDs, `chrono` for timestamps

---

## Topic 10 — Memory Management (Deep Dive)
### Project: **Arena Allocator with Safe API**

Build a region-based memory arena that allocates objects into a contiguous buffer, supports typed access, and exposes a safe API over an unsafe core.

**What to build:**
`struct Arena { buffer: Vec<u8>, bump: usize }` — allocates values by bumping a pointer, never frees individually (whole arena freed on drop).

**Every concept you must use:**
- **Stack vs heap sizes**: print `size_of` and `align_of` for every type you store in the arena
- **`unsafe` blocks with justification comments**: every `unsafe` block has a `// SAFETY:` comment
- **Raw pointers `*const T` / `*mut T`**: the internal allocation returns `*mut T`; the safe wrapper converts to `&'arena T`
- **`std::alloc::Layout`**: compute layout for each allocated type using `Layout::new::<T>()` and `Layout::array::<T>(n)`
- **Alignment**: implement `fn align_up(ptr: usize, align: usize) -> usize`; demonstrate that misaligned access is UB (document it)
- **`MaybeUninit<T>`**: `fn alloc<T>(&mut self) -> &mut MaybeUninit<T>` — caller initializes before use
- **`MaybeUninit::assume_init_ref`**: call only after confirming initialization
- **Manual `Drop`**: implement `Drop for Arena` that zeros the buffer (simulate secure erasure) and logs how many bytes were used
- **`std::mem::size_of`, `align_of`, `size_of_val`**: used throughout
- **`std::ptr::write` and `std::ptr::read`**: use instead of raw dereference for unaligned or uninitialized memory
- **`std::ptr::copy_nonoverlapping`**: implement `fn copy_slice_into_arena<T: Copy>(&mut self, slice: &[T]) -> &[T]`
- **Lifetimes tied to arena**: `fn alloc_typed<'a, T>(&'a mut self, val: T) -> &'a mut T` — the reference's lifetime is bounded by the arena's
- **RAII guard**: `struct ArenaScope<'a> { arena: &'a mut Arena, checkpoint: usize }` — `Drop` resets the bump pointer to the checkpoint (stack-like scoped alloc)
- **Newtype for safety**: `struct ArenaPtr<'a, T> { ptr: *const T, _marker: PhantomData<&'a T> }` — PhantomData carries the lifetime
- **`PhantomData`**: used in `ArenaPtr` to make the compiler track the lifetime without storing a reference
- **`transmute` — the wrong way and right way**: show `transmute` converting `[u8; 4]` to `f32`; then show `f32::from_le_bytes` as the safe equivalent
- **Safe wrapper test**: write a test that allocates 1000 integers, reads them back, and verifies correctness — zero `unsafe` in the test itself

---

## Topic 11 — Concurrency
### Project: **Parallel Web Crawler Simulator**

Build a simulated web crawler that fetches "pages" (fake, no real network), parses links, and crawls up to depth N — using real concurrency with thread pool, shared state, and cancellation.

**What to build:**
A graph of `Page { url: String, links: Vec<String>, content: String }` hardcoded in memory. Crawl it breadth-first using multiple threads.

**Every concept you must use:**
- **`std::thread::spawn`**: spawn worker threads from a pool
- **`JoinHandle`**: collect and join all workers at shutdown
- **`Arc<T>`**: share the page graph and the visited set across threads
- **`Mutex<T>`**: protect `visited: HashSet<String>` and the work queue `VecDeque<(String, usize)>` (url + depth)
- **`RwLock<T>`**: protect a `results: HashMap<String, CrawlResult>` — many readers, one writer
- **Lock ordering**: acquire queue lock, then visited lock, always in same order — comment explaining deadlock prevention
- **`MutexGuard` scope**: explicitly drop guards before `.await`-like blocking to avoid holding the lock too long
- **`mpsc::channel`**: workers send `CrawlResult` back to the main thread through a channel
- **`mpsc::Sender::clone()`**: each worker gets its own clone of the sender
- **Sentinel / shutdown signal**: send `None` through the channel to signal completion; use `Arc<AtomicBool>` as a cancellation flag
- **`AtomicBool`** with `Ordering::SeqCst`: global "should stop" flag — set when depth limit or page limit reached
- **`AtomicUsize`**: count total pages crawled without a mutex
- **`Condvar`** (condition variable): the main thread waits on a `Condvar` for the work queue to become non-empty, rather than busy-spinning
- **`Send` and `Sync` bounds**: ensure types passed across threads satisfy `Send`; show a compile error when they don't
- **Thread panic recovery**: `JoinHandle::join()` returns `Result` — handle the `Err` case when a worker panics
- **Thread naming**: `std::thread::Builder::new().name("worker-1".into()).spawn(...)`
- **`std::sync::Barrier`**: all workers synchronize before starting (ensures pool is fully up before crawl begins)

**Output:** live progress lines + final report of all crawled URLs, their depths, and crawl times.

---

## Topic 12 — Async Rust
### Project: **Async Task Queue with Workers**

Build an async job processing system: a queue accepts jobs, a pool of async workers processes them concurrently with rate limiting, timeouts, and graceful shutdown.

**What to build:**
`struct JobQueue` accepts `Job { id: Uuid, payload: String, priority: u8 }`. Workers pull jobs and "process" them (simulate with `sleep`). Collect results.

**Every concept you must use:**
- **`#[tokio::main]`**: entry point
- **`async fn`**: every worker function, the queue dispatcher, the result collector
- **`.await`**: used wherever async work happens
- **`tokio::spawn`**: spawn N worker tasks — each runs in a loop pulling from the queue
- **`JoinHandle` (tokio)**: collect handles, `join_all` at shutdown
- **`tokio::sync::Mutex`**: protect the job queue
- **`tokio::sync::RwLock`**: protect a `results: HashMap<Uuid, JobResult>`
- **`tokio::sync::mpsc`**: `Sender<Job>` for submitting jobs, `Sender<JobResult>` for returning results
- **`tokio::sync::oneshot`**: each job carries a `oneshot::Sender<JobResult>` for direct result delivery back to the submitter
- **`tokio::sync::Semaphore`**: limit concurrency to max 3 simultaneous "expensive" jobs
- **`tokio::sync::broadcast`**: shutdown signal sent to all workers simultaneously
- **`tokio::time::sleep`**: simulate job processing time
- **`tokio::time::timeout`**: wrap each job's processing in a 500ms timeout; return `Err(Timeout)` if exceeded
- **`tokio::select!`**: in each worker loop, race between "new job available" and "shutdown signal received"
- **`futures::future::join_all`**: await all worker handles at shutdown
- **`futures::stream::Stream`**: produce a stream of `JobResult` using `tokio_stream::wrappers::ReceiverStream`; consume with `while let Some(result) = stream.next().await`
- **`Pin` / `poll`**: manually poll a future once using `std::future::Future::poll` with a no-op waker (educational — show the mechanics)
- **Graceful shutdown**: on Ctrl+C (use `tokio::signal::ctrl_c()`), stop accepting new jobs, finish in-flight jobs, then exit
- **Task-local state**: use `tokio::task_local!` to store a worker ID accessible anywhere in the task without passing it explicitly

**Demo:** submit 20 jobs of varying durations from multiple async "clients" simultaneously; show real concurrency in the output timestamps.

---

## Topic 13 — Macros
### Project: **Mini Test Framework Macro**

Build a complete mini testing framework — like a tiny version of `#[test]` and `assert_eq!` — using both declarative and procedural macros.

**What to build:**
Two crates: `mini-test-macros` (proc macro crate) and `mini-test` (library using it). Users write:
```rust
#[mini_test::test]
fn test_addition() {
    mini_test::assert_eq!(2 + 2, 4);
    mini_test::assert_ne!(2 + 2, 5);
    mini_test::assert_approx!(3.14159, std::f64::consts::PI, epsilon = 0.001);
}
```
And call `mini_test::run_all!()` to execute and report.

**Every concept you must use:**

**Declarative macros (`macro_rules!`):**
- **`assert_eq!` reimplementation**: `macro_rules! assert_eq` — takes two expressions, evaluates each once (no double evaluation), prints both values on failure
- **`assert_approx!`**: named argument `epsilon =` using pattern matching on token trees
- **`assert_ne!`**: straightforward variant
- **`run_all!`**: takes a list of test function names using `$( $test:ident ),*` repetition; calls each and tracks pass/fail count
- **Recursive macro**: `macro_rules! count_tests!` that counts the number of test function names using recursion
- **`$crate::` prefix**: all macros use `$crate::` to reference items from the library, not the caller's crate
- **Hygiene**: demonstrate that a variable defined inside the macro (`let result = ...`) doesn't conflict with caller's `result`

**Procedural macros (`proc-macro` crate):**
- **`#[derive(TestSuite)]`**: applied to a struct, generates a `run_suite()` method that calls all methods starting with `test_`; uses `syn` to inspect method names
- **`#[mini_test::test]` attribute macro**: wraps a function to catch panics using `std::panic::catch_unwind`, records pass/fail, and registers the test in a global inventory using `linkme` or a static `Vec` (using `ctor` crate)
- **`syn::parse_macro_input!`**: parse the input token stream into a `syn::ItemFn`
- **`quote::quote!`**: generate the wrapper function
- **`proc_macro2::TokenStream`**: use the proc_macro2 version for testability
- **Function-like proc macro**: `sql!(SELECT * FROM {table} WHERE id = {id})` — validates that SQL starts with a known keyword at compile time; substitutes `{var}` placeholders with format args

**Final output:**
```
running 5 tests
  ✓ test_addition       (0.1ms)
  ✓ test_subtraction    (0.0ms)
  ✗ test_division       FAILED: assert_eq!(10 / 3, 3) → left=3, right=3 ✓ ... 
  ✓ test_string_ops     (0.1ms)
  ✓ test_edge_cases     (0.2ms)

Results: 4 passed, 1 failed
```

---

## Topic 14 — Testing
### Project: **Fully Tested Key-Value Store**

Build a `KvStore` — an in-memory key-value store with TTL (time-to-live), transactions, and pub/sub notifications — and achieve near-complete test coverage using every testing technique.

**What to build:**
```rust
struct KvStore {
    data: HashMap<String, Entry>,
    subscribers: HashMap<String, Vec<Box<dyn Fn(&str, &Event)>>>,
    transaction_log: Vec<Operation>,
}
```
With: `set(key, value, ttl?)`, `get(key)`, `delete(key)`, `expire(key, secs)`, `begin_transaction()` / `commit()` / `rollback()`.

**Every testing concept you must use:**

**Unit tests** (`#[cfg(test)]` module in `src/lib.rs`):
- `#[test]` on at least 15 functions
- `assert_eq!`, `assert_ne!`, `assert!`, `assert!(expr, "message with {}", var)`
- `#[should_panic(expected = "key not found")]`
- Test every `Result` variant: `assert!(result.is_ok())`, `assert!(result.is_err())`; match on `result.unwrap_err()` to check the error variant
- Test `Option`: `assert!(store.get("missing").is_none())`
- Helper function `fn make_store_with_data() -> KvStore` — used by 8+ tests (DRY setup)
- Test TTL: use `Cell<Instant>` or a clock abstraction so tests don't actually wait — inject a fake clock

**Integration tests** (`tests/kv_store.rs`):
- Test the full public API as an external user
- Test transaction commit and rollback end-to-end
- Test concurrent-ish access by running operations in sequence and checking invariants

**Doc tests**:
- Every public method has a `/// # Examples` block with a runnable assertion
- One doc test uses `# use kv_store::KvStore;` to hide boilerplate
- `cargo test --doc` must pass

**`#[should_panic]` tests**:
- Panic on get from empty store (if that's your API design)
- Panic on double-commit

**Mocking with `mockall`**:
- Extract a `Clock` trait: `trait Clock { fn now(&self) -> Instant; }`
- `#[automock]` it
- In TTL tests: `mock_clock.expect_now().returning(|| Instant::now() + Duration::from_secs(100))`
- Verify TTL expiry without sleeping

**Property-based tests with `proptest`**:
- `set` then `get` always returns the same value (for any string key/value)
- `delete` after `set` always results in `None`
- `rollback` after any sequence of operations restores the original state
- Use `prop_assume!` to skip invalid inputs

**Benchmarks with `criterion`** (`benches/kv_bench.rs`):
- `bench_set`: insert 1000 keys
- `bench_get`: get from a store with 1000 keys
- `bench_set_vs_get`: compare throughput
- Use `criterion::black_box` to prevent dead code elimination
- Add a `[[bench]]` entry to `Cargo.toml`

**Test organization**:
- Unit tests in `src/lib.rs`
- Integration tests in `tests/`
- Benchmarks in `benches/`
- A `tests/common/mod.rs` with shared helpers used by multiple integration test files
- Run specific test: `cargo test ttl`
- Run with output: `cargo test -- --nocapture`

---

## Progress Tracker

| # | Topic | Project | Status |
|---|-------|---------|--------|
| 1 | Foundations | CLI Unit Converter | ☐ |
| 2 | Ownership System | In-Memory Text Editor | ☐ |
| 3 | Structs & Enums | Task Manager CLI | ☐ |
| 4 | Collections & Iterators | Log File Analyzer | ☐ |
| 5 | Error Handling | CSV Data Pipeline | ☐ |
| 6 | Generics & Traits | Generic Data Processing Pipeline | ☐ |
| 7 | Advanced Types | Shared Configuration Store | ☐ |
| 8 | Closures | Middleware Pipeline | ☐ |
| 9 | Modules & Crate System | Multi-Crate Workspace | ☐ |
| 10 | Memory Management | Arena Allocator | ☐ |
| 11 | Concurrency | Parallel Web Crawler Simulator | ☐ |
| 12 | Async Rust | Async Task Queue | ☐ |
| 13 | Macros | Mini Test Framework | ☐ |
| 14 | Testing | Fully Tested KV Store | ☐ |

**14 projects. Every concept covered. Production-ready Rust at the end.**
