# Step 1: Build Stage
FROM node:20.11.1 AS build

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy only package.json and lock files to leverage Docker cache for dependencies
COPY package*.json ./

# Install dependencies
# Use --production for production dependencies or a conditional installation
ARG NODE_ENV=production
RUN if [ "$NODE_ENV" = "production" ]; then npm install --only=production; else npm install; fi

# Copy the rest of the application code
COPY . .

# Step 2: Final Image
FROM node:20.11.1

# Set working directory
WORKDIR /usr/src/app

# Copy necessary files from the build stage
COPY --from=build /usr/src/app .

# Switch to a non-root user for security
USER node

# Expose the port your app runs on
EXPOSE 3000

# Start the app
CMD ["npm", "start"]
