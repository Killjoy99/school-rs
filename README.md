# School Management System

This project is a School Management System built with [Rust](https://www.rust-lang.org/), using [Actix Web](https://actix.rs/) for the backend and [Tera](https://tera.netlify.app/) for templating.

## Features

- Student and teacher management
- Class scheduling
- Attendance tracking
- User authentication
- Security temper checks on the database: each row's contents are signed
- Audit triggers and audit trail
- Append-only ledger for data integrity
- Verification with lockouts for tamper detection

## Tech Stack

- **Backend:** Actix Web
- **Templating:** Tera
- **Language:** Rust

## Getting Started

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/school-rs.git
    cd school-rs
    ```
2. Install dependencies:
    ```bash
    cargo build
    ```
3. Run the server:
    ```bash
    cargo run
    ```

## Adding to a Remote GitHub Repository

1. Initialize git (if not already done):
    ```bash
    git init
    ```
2. Add all files and commit:
    ```bash
    git add .
    git commit -m "Initial commit"
    ```
3. Rename the local branch to `main`:
    ```bash
    git branch -m master main
    ```
4. Add the remote repository:
    ```bash
    git remote add origin https://github.com/yourusername/school-rs.git
    ```
5. Push to GitHub:
    ```bash
    git push -u origin main
    ```

handlers are functions with the logic for routes
schemas are DTOs


## License

MIT