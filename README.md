# GitHub PR Reporter

A Rust application that generates a report of GitHub pull requests for a specified repository and month. The report can be output in either a table or JSON format.

## Installation

## Install Rust
Follow instructions at [Rust's official web page](https://www.rust-lang.org/tools/install)

### Clone the Repository:

`git clone git@github.com:marcosouzabarreto/Github-Reportifier.git`

### Navigate to the Project Directory:

`cd github-pr-reporter`

### Build the Project:

`cargo build --release`

## Configuration:

Create a .env file in the project root directory and add the following variables:

```sh
GITHUB_TOKEN=your_personal_access_token
REPO_OWNER=your_repo_owner
REPO_NAME=your_repo_name
```

## Usage:

Run the application using cargo run with the desired options:

cargo run -- --month <MONTH> [OPTIONS]

Options:

    --year, -y: The year for the report (default is the current year).
    --month, -m: The month number (e.g., 9 for September). Required.
    --output, -o: Output format: "table" or "json" (default is "table").

Examples:

    Generate a Table Report for September 2023

`cargo run -- --year 2023 --month 9`

    Generate a JSON Report for October 2024

`cargo run -- --year 2024 --month 10 --output json`

### Output Formats:
- Table Output

By default, the report is displayed in a table format.

Sample Table Output:
![image](https://github.com/user-attachments/assets/858b71db-8f01-4209-a376-4597c1cc6530)

- JSON Output

If you specify --output json, the report will be output in JSON format.

Sample JSON Output:
![image](https://github.com/user-attachments/assets/4ce6dcc8-9655-4cea-80d7-65168eee0537)

