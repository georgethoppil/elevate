# Elevate Application

This API allows users to sign up, log in, record game events, and retrieve game summaries. It includes authentication mechanisms, session management, and CRUD operations for user and game data.

## Prerequisites

- Rust and Cargo (latest stable version)
- Docker
- Git

## Setup

1. Clone the repository:

   ```
   git clone https://github.com/georgethoppil/elevate.git
   cd elevate-main
   ```

2. Ensure docker is running

   ```
   chmod +x scripts/init_db.sh
   chmod +x scripts/init_redis.sh

   ./scripts/init_db.sh
   ./scripts/init_redis.sh
   ```

   This will create a postgres database and perform a migration. It will also create a redis db since we are using it for session management via cookies.

3. (Optional): You can adjust some of the config settings in the configuration folder. For example for token expiration is set to 120 seconds in development.

4. Run the server
   ```
   cargo run
   ```

## Elevate apis

1. Create a user

   ```
    curl -X POST http://localhost:8080/api/user \
        -H "Content-Type: application/json" \
        -d '{
            "email": "test@example.com",
            "password": "password123"
        }'
   ```

2. Login a user:

   ```
    curl -X POST http://localhost:8080/api/sessions \
    -H "Content-Type: application/json" \
    -d '{
        "email": "test@example.com",
        "password": "password123"
    }' \
    -c cookies.txt
   ```

3. Record a Game Event: (It requries the cookie from step 2 for auth.)

   ```
    curl -X POST http://localhost:8080/api/user/game_events \
    -H "Content-Type: application/json" \
    -b cookies.txt \
    -d '{
        "occurred_at": 1724201459825
    }'
   ```

4. Get Game Summary: (It requries the cookie from step 2 for auth.)
   ```
   curl -X GET http://localhost:8080/api/user \
       -H "Content-Type: application/json" \
       -b cookies.txt
   ```

## Testing

To run the tests for the application:

```
cargo test
```
