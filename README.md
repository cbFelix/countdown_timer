# Countdown Timer

A simple Rust application that counts down to a specified date and time. The program prompts the user to enter a date and time, and then it displays a live countdown until that moment.

## Features

- Interactive countdown timer.
- Supports date and time input in the format `YYYY-MM-DD HH:MM`.
- Live updates every second.

## Prerequisites

To build and run this application, you need:

- [Rust](https://www.rust-lang.org/) installed on your system. You can install Rust using [rustup](https://rustup.rs/).

## Building the Project

1. Clone the repository:

   ```sh
   git clone https://github.com/cbFelix/countdown_timer.git
   ```

2. Navigate to the project directory:

   ```sh
   cd countdown_timer
   ```

3. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

   This will compile the project and produce an executable in the `target/release` directory.

## Running the Application

1. Run the compiled executable:

   ```sh
   ./target/release/countdown_timer
   ```

2. Follow the prompt to enter the date and time in the format `YYYY-MM-DD HH:MM`. For example:

Enter the date and time in the format YYYY-MM-DD HH:MM: 2024-12-31 23:59
```

The application will start counting down to the specified date and time, updating every second.
Example
Enter the date and time in the format YYYY-MM-DD HH:MM: 2024-12-31 23:59
Remaining: 57 hours 59 minutes 59 seconds
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Feel free to fork the repository and submit pull requests. For major changes, please open an issue first to discuss what you would like to change.

## Contact

For any questions or feedback, please contact [victorlebedev30@gmail.com](mailto:victorlebedev30@gmail.com).
