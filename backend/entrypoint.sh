#!/bin/bash

# Print a message indicating that migrations are being run
echo "Running migrations..."

# Run database migrations
sqlx migrate run

# Print a message indicating that the backend is starting
echo "Starting the backend application..."

# Start the backend application (replace with the actual command to start your backend)
exec /usr/local/bin/backend
