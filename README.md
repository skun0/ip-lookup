# IP Info Checker

A simple Rust program to get detailed information about an IP address using the public API at [ip-api.com](http://ip-api.com). Users can query any IP or leave it blank to get info about their own public IP.

## Features

* Query an IP address or use your current public IP if left blank.
* Display IP details including:

  * Country
  * Region
  * City
  * ZIP code
  * Latitude and Longitude
  * ISP
* Handle API errors gracefully with error messages.

## Getting Started

### Prerequisites

* Rust installed ([Download Rust](https://www.rust-lang.org/tools/install))
* Cargo package manager (comes with Rust)

### Usage

0. Download the [Release](https://github.com/skun0/ip-lookup/releases/tag/release) (you dont have to install Rust)

   
2. Run the program:

   ```bash
   cargo run
   ```
  
3. Enter an IP address when prompted (or leave blank for own).
4. View the returned IP information in the terminal.

## Contributing

Feel free to fork this project and leave a star!

## Author

* Skuno – [GitHub Profile](https://github.com/skun0)
