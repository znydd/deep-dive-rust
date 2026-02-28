# Rust Learning Projects — Full Specifications

---

## Project 1: `taskr` — CLI Todo Manager

> **Difficulty:** ⭐ Beginner
> **Key Concepts:** Ownership, structs, enums, file I/O, error handling, `serde`

### What It Does
A command-line task manager. You run commands like `taskr add "Buy groceries"` and it saves tasks to a local JSON file.

### Commands

| Command | Description |
|---------|-------------|
| `taskr add <title>` | Add a new task |
| `taskr add <title> --priority <low\|medium\|high>` | Add with priority |
| `taskr list` | List all pending tasks |
| `taskr list --all` | List all tasks including completed |
| `taskr list --priority high` | Filter by priority |
| `taskr done <id>` | Mark a task as completed |
| `taskr remove <id>` | Delete a task |
| `taskr clear` | Remove all completed tasks |
| `taskr stats` | Show summary (total, done, pending) |

### Data Model

```rust
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    completed: bool,
    created_at: String,   // e.g. "2026-02-27 10:30"
    completed_at: Option<String>,
}

enum Priority {
    Low,
    Medium,
    High,
}
```

### Storage
- Save all tasks to `~/.taskr/tasks.json`
- Create the file/directory if it doesn't exist
- Read the file on startup, write on every mutation

### Milestones (build in this order)
1. **v0.1** — Parse CLI args manually with `std::env::args()`. Support `add` and `list` only. Store in a `Vec<Task>`, print to stdout, no persistence.
2. **v0.2** — Add JSON persistence with `serde` + `serde_json`. Tasks survive between runs.
3. **v0.3** — Add `done`, `remove`, `clear`, `stats` commands. Add `--all` flag to `list`.
4. **v0.4** — Add `Priority` enum. Add `--priority` flag to `add` and `list`. Color output using `colored` crate.
5. **v0.5** — Replace manual arg parsing with `clap`. Add `--help` for every command.

---

## Project 2: `rgrep` — Grep Clone

> **Difficulty:** ⭐⭐ Beginner → Intermediate
> **Key Concepts:** Lifetimes, iterators, closures, traits, testing, modules

### What It Does
Search for a text pattern inside files. Like the real `grep`, but simpler.

```bash
rgrep "fn main" ./src         # search in directory
rgrep "TODO" myfile.rs        # search in a single file
echo "hello world" | rgrep "hello"  # search from stdin
```

### Commands & Flags

```
rgrep <pattern> [path]

Options:
  -i, --ignore-case       Case-insensitive search
  -n, --line-number        Show line numbers (on by default)
  -r, --recursive          Search directories recursively (on by default for dirs)
  -c, --count              Show only count of matching lines per file
  -l, --files-only         Show only filenames that contain a match
  -v, --invert             Show lines that do NOT match
  --no-color               Disable colored output
  --hidden                 Include hidden files (dotfiles)
  -C <N>, --context <N>    Show N lines of context around each match
```

### Output Format
```
src/main.rs:12: fn main() {
src/lib.rs:45:     fn main_loop(&self) {
```
- Filename in **green**, line number in **yellow**, matched text **highlighted in red**
- When searching a single file, skip the filename prefix

### Behavior Rules
- If `[path]` is a file → search that file
- If `[path]` is a directory → recursively search all files
- If `[path]` is omitted → read from **stdin**
- Skip binary files (files containing null bytes in first 512 bytes)
- Respect `.gitignore` patterns (bonus, can use `ignore` crate)

### Milestones
1. **v0.1** — Search a single file for an exact string. Print matching lines with line numbers.
2. **v0.2** — Add `-i` (case insensitive) and `-v` (invert match). Learn iterators: use `.lines()`, `.enumerate()`, `.filter()`.
3. **v0.3** — Support directory recursion with `std::fs::read_dir()` + recursion. This is where you'll encounter **lifetimes** in your search result struct.
4. **v0.4** — Add colored output with `colored` crate. Add `-c` (count) and `-l` (files-only) modes.
5. **v0.5** — Add stdin support. Add `--context` flag. Write **unit tests** for the matching logic and **integration tests** that run the binary.

### Test Requirements
- At least 10 unit tests for the core search function
- At least 5 integration tests that invoke the binary and check stdout
- Test edge cases: empty files, binary files, no matches, Unicode text

---

## Project 3: `fetchr` — Concurrent Web Scraper / Downloader

> **Difficulty:** ⭐⭐⭐ Intermediate
> **Key Concepts:** Async/await, `tokio`, concurrency, `Arc`/`Mutex`, generics, error handling

### What It Does
Takes a list of URLs and concurrently fetches them, extracts data, and saves results. Can work in two modes: **scrape** (extract info from HTML) or **download** (save files to disk).

```bash
fetchr scrape urls.txt --selector "h1" --output results.json
fetchr download urls.txt --output-dir ./downloads --concurrent 5
fetchr scrape https://example.com --selector "a[href]" --attr href
```

### Commands

#### `fetchr scrape`
| Flag | Description |
|------|-------------|
| `<source>` | A URL or a text file containing one URL per line |
| `--selector <css>` | CSS selector to extract (e.g., `"h1"`, `"a[href]"`, `".title"`) |
| `--attr <name>` | Extract an attribute instead of text (e.g., `href`, `src`) |
| `--output <file>` | Save results as JSON (default: print to stdout) |
| `--concurrent <N>` | Max concurrent requests (default: 10) |
| `--timeout <secs>` | Timeout per request in seconds (default: 30) |
| `--retry <N>` | Retry failed requests N times (default: 0) |

#### `fetchr download`
| Flag | Description |
|------|-------------|
| `<source>` | A URL or a text file containing one URL per line |
| `--output-dir <dir>` | Directory to save files (default: `./downloads`) |
| `--concurrent <N>` | Max concurrent downloads (default: 5) |
| `--timeout <secs>` | Timeout per request (default: 60) |
| `--retry <N>` | Retry failed downloads N times (default: 2) |

### Output (scrape mode)
```json
[
  {
    "url": "https://example.com",
    "status": 200,
    "results": ["Example Domain"],
    "elapsed_ms": 142
  }
]
```

### Behavior Rules
- Show a **progress bar** for all operations (use `indicatif`)
- Print a **summary** at the end: total URLs, successes, failures, total time
- Handle errors gracefully: timeout, DNS failure, HTTP errors — log and continue
- Respect rate limits: add `--delay <ms>` flag for minimum delay between requests

### Milestones
1. **v0.1** — Fetch a single URL using `reqwest` and print the HTML. Learn `async/await` and `tokio::main`.
2. **v0.2** — Fetch multiple URLs from a file concurrently using `tokio::spawn` + `Arc`. Add `--concurrent` with a semaphore.
3. **v0.3** — Add HTML parsing with `scraper` crate. Implement `--selector` and `--attr`. Output as JSON.
4. **v0.4** — Implement `download` subcommand. Stream large files to disk without loading into memory. Add progress bars.
5. **v0.5** — Add `--retry`, `--timeout`, `--delay`. Add proper error types with `thiserror`. Write tests using `wiremock` for mock HTTP servers.

### Key Data Structures
```rust
struct FetchResult {
    url: String,
    status: u16,
    results: Vec<String>,
    elapsed_ms: u64,
    error: Option<String>,
}

struct FetchConfig {
    concurrent: usize,
    timeout: Duration,
    retries: u32,
    delay: Option<Duration>,
}
```

---

## Project 4: `servr` — HTTP Server from Scratch

> **Difficulty:** ⭐⭐⭐⭐ Intermediate → Advanced
> **Key Concepts:** TCP networking, protocol parsing, thread pools, builder pattern, trait objects, macros

### What It Does
A working HTTP/1.1 server built from raw `TcpListener`. Not meant for production — meant to teach you how frameworks like Axum work under the hood.

```rust
fn main() {
    let app = Servr::new()
        .get("/", home_handler)
        .get("/api/users", list_users)
        .post("/api/users", create_user)
        .static_dir("/public", "./static")
        .middleware(logger)
        .middleware(cors)
        .build();

    app.listen("127.0.0.1:3000").unwrap();
}
```

### Features

#### Core HTTP
- Parse HTTP/1.1 requests (method, path, headers, body)
- Generate HTTP responses (status code, headers, body)
- Support methods: `GET`, `POST`, `PUT`, `DELETE`
- Support `Content-Type`: `text/html`, `application/json`, `text/plain`
- Serve static files from a directory

#### Routing
- Register handlers by method + path pattern
- Path parameters: `/users/:id` → extract `id`
- Query parameters: `/search?q=rust` → extract `q`
- 404 handler for unmatched routes

#### Request / Response API
```rust
// Handler signature
fn handler(req: &Request) -> Response { ... }

// Request
req.method()           // -> Method::Get
req.path()             // -> "/users/42"
req.param("id")        // -> Some("42")
req.query("q")         // -> Some("rust")
req.header("Content-Type") // -> Some("application/json")
req.body_string()      // -> body as String
req.body_json::<T>()   // -> deserialize body as T

// Response
Response::ok().text("Hello!")
Response::ok().json(&user)
Response::ok().html("<h1>Hi</h1>")
Response::status(404).text("Not Found")
Response::redirect("/login")
```

#### Middleware
```rust
fn logger(req: &Request, next: &dyn Fn(&Request) -> Response) -> Response {
    let start = Instant::now();
    let res = next(req);
    println!("{} {} → {} ({}ms)", req.method(), req.path(), res.status(), start.elapsed().as_millis());
    res
}
```

#### Concurrency
- Handle each connection in a thread (start with `std::thread::spawn`)
- Upgrade to a **thread pool** with configurable size
- Bonus: add an async version using `tokio`

### Milestones
1. **v0.1** — Accept TCP connections, read raw bytes, respond with hardcoded `200 OK` + `"Hello World"` to every request.
2. **v0.2** — Parse the HTTP request line (method, path, version). Parse headers. Build a `Request` struct.
3. **v0.3** — Build a `Response` struct with builder pattern. Implement `.text()`, `.html()`, `.json()`.
4. **v0.4** — Add a `Router`. Register handlers by method + path. Dispatch incoming requests. Return 404 for unmatched.
5. **v0.5** — Add path parameters (`:id`). Add query string parsing.
6. **v0.6** — Add static file serving. Detect MIME types.
7. **v0.7** — Add middleware chain. Implement `logger` and `cors` middleware.
8. **v0.8** — Replace `thread::spawn` with a thread pool. Add graceful shutdown.

---

## Project 5: `dashr` — TUI Dashboard

> **Difficulty:** ⭐⭐⭐⭐⭐ Advanced
> **Key Concepts:** Event-driven architecture, state machines, complex enums, trait implementations, `ratatui`

### What It Does
A terminal dashboard that shows system information in real time with a beautiful TUI.

### Screens / Tabs

Press `1`-`4` to switch between tabs, `q` to quit.

#### Tab 1: System Overview
```
┌─ CPU ──────────────┐ ┌─ Memory ────────────┐
│ ████████░░ 78%     │ │ Used: 12.4 / 16.0 GB│
│ Core 1: ███░ 65%   │ │ ████████████░░░░ 77% │
│ Core 2: █████ 90%  │ │ Swap: 0.2 / 4.0 GB  │
│ Core 3: ██░░ 40%   │ │                      │
│ Core 4: ████ 72%   │ │                      │
└────────────────────┘ └──────────────────────┘
┌─ Disk ─────────────────────────────────────────┐
│ /     ████████████████░░░░ 210G / 500G (42%)   │
│ /home ██████████░░░░░░░░░░ 120G / 500G (24%)   │
└────────────────────────────────────────────────┘
```

#### Tab 2: Process List
```
┌─ Processes (sorted by CPU) ─────────────────────────────┐
│  PID  │ Name            │ CPU%  │ Mem%  │ Status        │
│ 1234  │ firefox          │ 12.3  │ 8.2   │ Running       │
│ 5678  │ code             │  8.1  │ 5.4   │ Running       │
│ ...   │                  │       │       │               │
└─────────────────────────────────────────────────────────┘
  [s] Sort by  [k] Kill process  [/] Search
```

#### Tab 3: Network Monitor
```
┌─ Network ──────────────────────────────────────┐
│ Interface: eth0                                │
│ Download: 2.4 MB/s  ▁▃▅▇█▇▅▃▁▃▅▇ (sparkline) │
│ Upload:   0.3 MB/s  ▁▁▂▁▁▂▃▂▁▁▁▂             │
│ Total Down: 1.2 GB  Total Up: 340 MB          │
└────────────────────────────────────────────────┘
```

#### Tab 4: Logs Viewer
- Read from a log file (configurable path)
- Auto-scroll, with ability to pause and scroll manually
- Search/filter logs with `/`

### Keyboard Controls

| Key | Action |
|-----|--------|
| `1`–`4` | Switch tabs |
| `q` / `Ctrl+C` | Quit |
| `↑` / `↓` | Scroll in lists |
| `s` | Cycle sort column (process tab) |
| `k` | Kill selected process (with confirmation) |
| `/` | Enter search/filter mode |
| `Esc` | Cancel current action |
| `?` | Show help overlay |

### Architecture
```rust
// Core app state
struct App {
    current_tab: Tab,
    system_state: SystemState,
    process_list: ProcessList,
    network_state: NetworkState,
    log_viewer: LogViewer,
    should_quit: bool,
    input_mode: InputMode,
}

enum Tab { Overview, Processes, Network, Logs }

enum InputMode {
    Normal,
    Search(String),
    Confirm(ConfirmAction),
}

enum ConfirmAction {
    KillProcess(u32),
}

// Main loop
loop {
    terminal.draw(|frame| ui::render(&app, frame))?;
    
    if event::poll(Duration::from_millis(250))? {
        app.handle_event(event::read()?);
    }
    
    app.update_system_data();
}
```

### Config File (`~/.dashr/config.toml`)
```toml
[general]
refresh_rate_ms = 1000
default_tab = "overview"

[logs]
path = "/var/log/syslog"
max_lines = 1000

[theme]
accent_color = "cyan"
```

### Dependencies
- `ratatui` + `crossterm` — TUI rendering
- `sysinfo` — System data (CPU, memory, processes, disks)
- `toml` + `serde` — Config parsing

### Milestones
1. **v0.1** — Set up `ratatui` + `crossterm`. Show a single screen with "Hello TUI". Handle `q` to quit.
2. **v0.2** — Add tab bar UI. Switch between 4 empty tabs with `1`-`4` keys. Implement the `App` state machine.
3. **v0.3** — Implement **System Overview** tab. Use `sysinfo` crate for CPU and memory. Render gauge/bar widgets.
4. **v0.4** — Implement **Process List** tab. Show a table with sorting. Add `↑`/`↓` scrolling.
5. **v0.5** — Implement **Network** tab with sparkline charts for live bandwidth.
6. **v0.6** — Implement **Logs Viewer** tab. Read file, auto-scroll, add search.
7. **v0.7** — Add config file support (`~/.dashr/config.toml`). Add help overlay (`?`). Polish theming.
8. **v0.8** — Add kill process with confirmation dialog. Refactor with clean architecture (separate `ui`, `state`, `events` modules).

---

## Summary: What Each Project Teaches

| # | Project | You Will Master |
|---|---------|----------------|
| 1 | `taskr` | Ownership, structs, enums, file I/O, `serde`, CLI basics |
| 2 | `rgrep` | Lifetimes, iterators, closures, traits, testing |
| 3 | `fetchr` | Async/await, tokio, concurrency, `Arc`/`Mutex`, error types |
| 4 | `servr` | TCP, protocol parsing, builder pattern, trait objects, macros |
| 5 | `dashr` | State machines, event-driven design, complex architecture |

> [!TIP]
> For each project, **don't use `cargo generate` or templates**. Start with `cargo new <name>` and build everything yourself. The struggle is where the learning happens.
