#!/bin/bash

# Function to create the .env file with PostgreSQL credentials
create_env_file() {
    echo "Creating .env file with PostgreSQL credentials..."
    cat <<EOL > .env
# .env file with PostgreSQL credentials
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
POSTGRES_DB=mydatabase
DATABASE_URL=postgres://postgres:password@postgresdb:5432/mydatabase

EOL
    echo ".env file created with PostgreSQL credentials."
}

# Helper Functions
print_usage() {
    echo "Usage: $0 [dev|prod|logs <container_name>|clean]"
    echo "  dev           - Run the Docker Compose file for development environment"
    echo "  prod          - Run the Docker Compose file for production environment"
    echo "  logs <name>   - View logs of a specific container by name"
    echo "  clean         - Stop and remove all containers and volumes"
    exit 1
}

# Check if Docker Compose exists
check_docker_compose() {
    if ! command -v docker-compose &> /dev/null; then
        echo "Error: docker-compose is not installed. Please install it first."
        exit 1
    fi
}


# Run Docker Compose for Dev/Prod
run_docker_compose() {
    local env=$1
    if [ "$env" == "dev" ]; then
        compose_file="docker-compose.dev.yml"
    elif [ "$env" == "prod" ]; then
        compose_file="docker-compose.prod.yml"
    else
        echo "Error: Unknown environment '$env'."
        exit 1
    fi

    if [ ! -f "$compose_file" ]; then
        echo "Error: $compose_file not found."
        exit 1
    fi

    echo "Starting Docker Compose for $env environment..."
    docker-compose -f "$compose_file" up --build
}

# View Logs of a Specific Container
view_logs() {
    local container_name=$1
    if [ -z "$container_name" ]; then
        echo "Error: No container name provided."
        exit 1
    fi

    echo "Viewing logs for container: $container_name"
    docker logs -f "$container_name"
}

# Clean Containers and Volumes
clean_up() {
    echo "Stopping and removing all containers and volumes..."
    docker-compose down -v
    docker system prune -f --volumes
    echo "All containers and volumes have been removed."
}

# Main Script Logic
check_docker_compose
# Create the .env file with PostgreSQL credentials
create_env_file

case "$1" in
    dev)
        run_docker_compose "dev"
        ;;
    prod)
        run_docker_compose "prod"
        ;;
    logs)
        view_logs "$2"
        ;;
    clean)
        clean_up
        ;;
    *)
        echo "Error: Invalid command."
        print_usage
        ;;
esac
