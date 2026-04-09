# ZeusI
### Watch sorting algorithms compete across your network

**ZeusI** is a distributed benchmarking tool that visualizes and compares in real time the performance of sorting algorithms running on different machines over a network.

> **ZeusI** est un outil de benchmarking distribué qui visualise et compare en temps réel les performances d'algorithmes de tri s'exécutant sur différentes machines en réseau.

---

## Demo

![ZeusI Dashboard](images\screenshot.png)

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable, 2021 edition)
- Cargo (included with Rust)

### Clone the repository

```bash
git clone https://github.com/your-username/zeusi.git
cd zeusi
```

### Build

```bash
cargo build --release
```

---

## Usage

Start the components in this order: **server first**, then **workers**, then **dashboard**.

### 1. Server

```bash
cargo run -p server -- <port> <array_size>
```

| Argument | Description |
|---|---|
| `port` | Port to listen on |
| `array_size` | Number of elements in each array sent to workers |

Example:
```bash
cargo run -p server -- 8080 10000
```

---

### 2. Workers

```bash
cargo run -p worker -- <name> <algorithm> <port>
```

| Argument | Description |
|---|---|
| `name` | Name of the worker |
| `algorithm` | `BUBBLE`, `MERGE`, or `INSERTION` |
| `port` | Server port to connect to |

Example — start three workers with different algorithms:
```bash
cargo run -p worker -- Alice BUBBLE 8080
cargo run -p worker -- Bob MERGE 8080
cargo run -p worker -- Charlie INSERTION 8080
```

---

### 3. Dashboard

```bash
cargo run -p dashboard -- <port>
```

| Argument | Description |
|---|---|
| `port` | Server port to connect to |

Example:
```bash
cargo run -p dashboard -- 8080
```

The dashboard window opens and displays the workers ranked by performance in real time.

---

## How it works

Every 100ms, the server sends the same random array to all connected workers. Each worker sorts the array using its algorithm, measures the time and number of comparisons, and sends the result back. The server broadcasts the updated rankings to all connected dashboards.

```
Worker (BUBBLE) ──┐
Worker (MERGE)  ──┼──► Server ──► Dashboard (egui)
Worker (INSERT) ──┘
```

---

## Architecture

ZeusI is organized as a Cargo workspace with four crates:

| Crate | Role |
|---|---|
| `core` | Shared types, serialization, sorting algorithms and trait |
| `server` | Receives connections, distributes arrays, broadcasts stats |
| `worker` | Connects to server, sorts arrays, returns performance results |
| `dashboard` | Connects to server, displays live rankings with egui |

### Sorting algorithms

| Algorithm | Average complexity | Comparisons (10 000 elements) |
|---|---|---|
| Merge Sort | O(n log n) | ~130 000 |
| Insertion Sort | O(n²) | ~25 000 000 |
| Bubble Sort | O(n²) | ~50 000 000 |

### TCP message protocol

All messages are newline-terminated strings with `|` as separator:

```
CONNECT|<name>|<BUBBLE|MERGE|INSERTION>
TAB|<v1>|<v2>|...|<vn>
RESULT|<time_µs>|<comparisons>
STAT|<name>|<algo>|<time>|<count>|...
```

---

## License

MIT