# Note Management Backend with Rust

This project provides a backend API for managing notes using the Rust programming language and the Axum web framework. It supports CRUD (Create, Read, Update, Delete) operations on notes, utilizing SeaORM for database interactions. The project also includes Docker Compose configuration for easy setup of PostgreSQL and PgAdmin.

## Built With

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum) - Web framework for Rust
- [SeaORM](https://github.com/SeaQL/sea-orm) - An async & dynamic ORM for Rust
- [PostgreSQL](https://www.postgresql.org/) - Open-source relational database system
- [PgAdmin](https://www.pgadmin.org/) - PostgreSQL administration and management tool
- [Docker Compose](https://docs.docker.com/compose/) - Tool for defining and running multi-container Docker applications

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed on your machine:

- **Rust**: Install Rust from the [official website](https://www.rust-lang.org/).
- **Docker**: Install Docker from the [official website](https://www.docker.com/).
- **Docker Compose**: Install Docker Compose from the [official website](https://docs.docker.com/compose/).

### Installation

1. Clone the repository.

```bash
   git clone https://github.com/marianaia/Notes
   cd notes
```

2. Run Docker Compose to start PostgreSQL and PgAdmin.

```bash
docker-compose up -d
```

3. Run database migrations.

```bash
sqlx migrate run
```

4. Start the server.

```bash
cargo run
```

### Usage

Once the server is running, you can access the API at http://localhost:3000.

### Contributing

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch
3. Commit your Changes
4. Push to the Branch
5. Open a Pull Request
