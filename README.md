# Circular Buffer

A high-performance, memory-mapped circular buffer (ring buffer) designed for Windows systems. This library leverages Windows-specific APIs for efficient data handling, making it ideal for real-time applications, streaming data, and low-latency scenarios.

- **Memory-Mapped Buffer:** Uses dual memory views for seamless wrap-around.
- **High Performance:** Optimized for fast reads and writes with minimal overhead.
- **Windows Native:** Leverages Windows API (`VirtualAlloc2`, `MapViewOfFile3`, etc.).

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
circular_buffer = "0.1.0"
```
