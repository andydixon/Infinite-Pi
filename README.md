# Pi Digit Generator in Rust

This Rust program generates the digits of π (pi) indefinitely, printing each digit as soon as it's calculated. It uses a spigot algorithm based on arbitrary-precision arithmetic to compute π's digits sequentially without relying on high-precision floating-point operations.

## Table of Contents

- [Overview](#overview)
- [Algorithm](#algorithm)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Example Output](#example-output)
- [Notes](#notes)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Overview

The program implements a spigot algorithm for π, which generates its digits one at a time. This is particularly useful for applications where you need π's digits sequentially or for educational purposes to understand arbitrary-precision arithmetic and algorithms for π.

## Algorithm

The algorithm is based on the work by Stanley Rabinowitz and Stan Wagon, allowing for the sequential generation of π's digits without the need for floating-point arithmetic. It uses integer arithmetic with arbitrary precision to handle the large numbers involved in the calculations.

## Prerequisites

- Rust (latest stable version recommended)
- The [`num`](https://crates.io/crates/num) crate for arbitrary-precision arithmetic

## Installation

1. **Install Rust**

   If you haven't installed Rust yet, you can do so by visiting [rust-lang.org](https://www.rust-lang.org/tools/install) and following the installation instructions.

2. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/pi-digit-generator.git
   cd pi-digit-generator
   ```

## Usage

1. **Build the Program**

   ```bash
   cargo build --release
   ```

2. **Run the Program**

   ```bash
   cargo run --release
   ```

   The program will start printing the digits of π to the standard output indefinitely.

3. **Redirect Output (Optional)**

   Since the program runs indefinitely, you might want to redirect the output to a file:

   ```bash
   cargo run --release > pi_digits.txt
   ```

   **Note:** Be cautious as this file can grow large quickly.

## Example Output

Here is a snippet of the program's output:

```
314159265358979323846264338327950288419716939937510...
```

## Notes

- **Performance:** Running the program will consume CPU resources indefinitely. Monitor your system's performance if you plan to run it for extended periods.
- **Stopping the Program:** You can stop the program by pressing `Ctrl+C` in the terminal.
- **Limiting Output:** If you only need a specific number of digits, you can modify the code to include a counter that stops the loop after the desired number of digits have been printed. For example:

  ```rust
  let mut count = 0;
  let max_digits = 1000; // Change this to the desired number of digits

  loop {
      // ... existing code ...

      if count >= max_digits {
          break;
      }
      count += 1;
  }
  ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Stanley Rabinowitz and Stan Wagon:** For their work on spigot algorithms for the digits of π.
- **The Rust Community:** For providing the `num` crate and extensive documentation.
- **Inspiration:** This project was inspired by the need to demonstrate arbitrary-precision arithmetic and algorithm implementation in Rust.

---

Feel free to contribute to this project by opening issues or submitting pull requests.

```
