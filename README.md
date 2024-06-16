
# Uniparser(CSV Visualizer)

This Rust project reads a CSV file, parses the data, and visualizes it as a PNG image using the Plotters crate with the Cairo backend.

## Features

- Parses CSV files
- Generates a 2D scatter plot
- Saves the plot as a PNG image

## Dependencies

- [csv](https://crates.io/crates/csv)
- [serde](https://crates.io/crates/serde)
- [plotters](https://crates.io/crates/plotters)
- [plotters-cairo](https://crates.io/crates/plotters-cairo)

## Getting Started

### Prerequisites

- Rust (install from [rustup.rs](https://rustup.rs/))

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/Rishi-Sudhakar/uniparser.git
   cd uniparser
   ```

3. Add your `data.csv` file in the project directory.
 *(Example data):*
   ```csv
   x,y
   1.0,2.0
   2.0,3.0
   3.0,4.0
   4.0,5.0
   5.0,6.0
   6.0,7.0
   7.0,8.0
   8.0,9.0
   9.0,10.0
   10.0,11.0
   ```

### Usage

1. Run the project:
   ```sh
   cargo run
   ```

2. After running, the `output.png` file will be generated in the project directory.

## Screenshots

### data.csv

<img width="682" alt="Screenshot 2024-06-16 at 11 30 24 AM" src="https://github.com/Rishi-Sudhakar/uniparser/assets/79398572/66afad65-f882-46b5-b5ca-3c5310231a9b">



### output.png

<img width="912" alt="Screenshot 2024-06-16 at 11 30 48 AM" src="https://github.com/Rishi-Sudhakar/uniparser/assets/79398572/a42d5012-af6f-4104-b529-0aaf2a585977">


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

