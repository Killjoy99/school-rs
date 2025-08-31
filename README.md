# School Management System

This project is a School Management System built with [Rust](https://www.rust-lang.org/), using [Actix Web](https://actix.rs/) for the backend and [Tera](https://tera.netlify.app/) for templating. It caters for a wide range of institutions from ECD to Universities.

## Badges

![Rust](https://img.shields.io/badge/Rust-202020?style=for-the-badge&logo=rust)
![Actix Web](https://img.shields.io/badge/Actix%20Web-202020?style=for-the-badge&logo=actix)
![Tera](https://img.shields.io/badge/Tera-202020?style=for-the-badge)
![SQLx](https://img.shields.io/badge/SQLx-202020?style=for-the-badge&logo=postgresql)
![License: MIT](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)
![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/school-rs/ci.yml?branch=main&style=for-the-badge)
![Last Commit](https://img.shields.io/github/last-commit/yourusername/school-rs?style=for-the-badge)

## Useful Information

- **Documentation:** See [docs.rs/school-rs](https://docs.rs/school-rs) for API documentation.
- **Issue Tracker:** [GitHub Issues](https://github.com/yourusername/school-rs/issues)
- **Contributing:** Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md).
- **Contact:** For questions, open an issue or email `philani.dlamini@outlook.com`.


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
Usefull tree command
tree -I "folder|to|exclude"
