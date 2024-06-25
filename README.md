# Etcd Config Management Service
Etcd Config Manager Client in Rust.

This is a simple Rust web application using Actix Web and ETCD. The application provides endpoints to put, get, and delete key-value pairs in ETCD.

### CAUTION: The contents in this README file may not be maintained properly.

## Project Structure

```
src/
├── main.rs          # Entry point of the application
├── routes.rs        # Route configurations
├── handlers.rs      # Request handlers
└── state.rs         # Application state definition
tests/
└── example-requests.http  # Example HTTP requests for testing
```

## Requirements

- Rust (latest stable version recommended)
- ETCD server (running locally or accessible)

## Setup

### ETCD Server

Make sure you have an ETCD server running. You can start a local ETCD server using Docker:

```bash
docker run -p 2379:2379 -p 2380:2380 --name etcd quay.io/coreos/etcd:v3.5.0 \
  /usr/local/bin/etcd --name s1 \
  --initial-advertise-peer-urls http://localhost:2380 \
  --listen-peer-urls http://0.0.0.0:2380 \
  --advertise-client-urls http://localhost:2379 \
  --listen-client-urls http://0.0.0.0:2379 \
  --initial-cluster s1=http://localhost:2380
```

### Application

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/your-username/actix-web-etcd-example.git
cd actix-web-etcd-example
```

### Running the Application

Build and run the application:

```bash
cargo run
```

The application will start an HTTP server on `127.0.0.1:8080`.

## API Endpoints

The application provides the following API endpoints:

### PUT `/put`

Inserts a key-value pair into ETCD.

- **Request:**
  - Method: POST
  - URL: `http://127.0.0.1:8080/put`
  - Body (JSON):
    ```json
    {
        "key": "exampleKey",
        "value": "exampleValue"
    }
    ```
- **Response:**
  - Success: `200 OK` with message `"Key inserted successfully"`
  - Error: `500 Internal Server Error` with error message

### GET `/get/{key}`

Retrieves the value of a given key from ETCD.

- **Request:**
  - Method: GET
  - URL: `http://127.0.0.1:8080/get/{key}`
- **Response:**
  - Success: `200 OK` with JSON body:
    ```json
    {
        "key": "exampleKey",
        "value": "exampleValue"
    }
    ```
  - Key Not Found: `404 Not Found` with message `"Key not found"`
  - Error: `500 Internal Server Error` with error message

### DELETE `/delete/{key}`

Deletes a key-value pair from ETCD.

- **Request:**
  - Method: DELETE
  - URL: `http://127.0.0.1:8080/delete/{key}`
- **Response:**
  - Success: `200 OK` with message `"Key deleted successfully"`
  - Error: `500 Internal Server Error` with error message

## Example Requests

Example HTTP requests are provided in `tests/example-requests.http` for easy testing.

### PUT Request

```http
POST http://127.0.0.1:8080/put
Content-Type: application/json

{
    "key": "exampleKey",
    "value": "exampleValue"
}
```

### GET Request

```http
GET http://127.0.0.1:8080/get/exampleKey
```

### DELETE Request

```http
DELETE http://127.0.0.1:8080/delete/exampleKey
```

You can use tools like [HTTPie](https://httpie.io/) or [Postman](https://www.postman.com/) to make these requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any changes.

---

This README provides an overview of the project, including setup instructions, API endpoint documentation, and example requests for testing. Adjust the repository URL, project name, and other details as needed for your specific use case.
