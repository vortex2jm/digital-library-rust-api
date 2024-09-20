
# Library Management System

This project implements a **Library Management System** using Rust, designed with a clean architecture pattern. It includes functionalities for managing users and books, allowing users to rent and return books. The project is built with separation of concerns and uses repositories to interact with the data layer.

## Features

- **User Management**: Add, retrieve, and delete users.
- **Book Management**: Add, retrieve, and delete books.
- **Rent and Return Books**: Users can rent books, and return them when done.
- **In-Memory Repository**: For simplicity, an in-memory repository is used for both books and users.
- **Error Handling**: Uses a custom `DomainError` to handle various error cases like locking issues, book not found, user not found, and book not available.

## Project Structure

The project is organized as follows:

- `domain/`: Contains the domain models (User, Book) and trait definitions (for repository and service behaviors).
- `adapters/repositories/`: Holds the in-memory implementation of the repositories.
- `services/`: Includes the business logic for managing users and books.
- `dto/`: Data transfer objects (DTOs) for book and user interactions between layers.

## Domain Models

### User

- `id: Uuid`: Unique identifier for each user.
- `name: String`: User's name.
- `cpf: String`: User's unique CPF (tax ID).
- `loc_books: Vec<Uuid>`: A list of book IDs rented by the user.

### Book

- `id: Uuid`: Unique identifier for each book.
- `title: String`: Book title.
- `author: String`: Author of the book.
- `loc_for: Option<Uuid>`: The user ID who has rented the book (None if available).

## Repositories

### UserRepository

Provides methods for user data management:

- `find_user_by_id`: Retrieve a user by ID.
- `save_user`: Save or update a user.
- `del_user_by_id`: Delete a user by ID.
- `find_all_users`: Retrieve all users.

### BookRepository

Provides methods for book data management:

- `find_book_by_id`: Retrieve a book by ID.
- `save_book`: Save or update a book.
- `del_book_by_id`: Delete a book by ID.
- `find_all_books`: Retrieve all books.

## Services

### UserService

Manages user-related operations, including renting and returning books. Main methods:

- `add_user`: Add a new user.
- `get_user_by_id`: Get user details by ID.
- `get_all_users`: Retrieve all users.
- `remove_user_by_id`: Remove a user.
- `rent_book`: Rent a book for a user.
- `return_book`: Return a rented book.

### BookService

Handles book-related operations. Main methods:

- `add_book`: Add a new book.
- `get_book_by_id`: Get book details by ID.
- `get_all_books`: Retrieve all books.
- `remove_book_by_id`: Remove a book by ID.

## Error Handling

The project defines various domain errors to handle issues such as:

- `UserNotFound`: When a user is not found in the repository.
- `BookNotFound`: When a book is not found in the repository.
- `BookNotAvailable`: When a book is already rented by another user.
- `BookNotRentedByUser`: When a user tries to return a book they haven't rented.
- `LockError`: When a read/write lock fails (used in the in-memory repositories).

## Running the Project

To run the project, you need to have Rust installed. You can then use the following commands:

1. Clone the repository:

```bash
git clone git@github.com:ThiagoLahass/digital-library-rust-api.git
```

2. Navigate to the project directory:

```bash
cd digital-library-rust-api
```

3. Run the project:

```bash
cargo run
```

## Future Improvements

- **Persistent Storage**: Implement a database to store books and users.
- **Authentication**: Add user authentication for secure access.

