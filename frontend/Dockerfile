# Step 1: Build React App
FROM node:18 AS build

WORKDIR /app

# Install dependencies
COPY package.json package-lock.json ./
RUN npm install

# Copy app source code
COPY . .

# Build the React app
RUN npm run build

# Step 2: Serve React App
FROM nginx:alpine

# Copy the React build folder from the previous step
COPY --from=build /app/build /usr/share/nginx/html
# Copy custom nginx.conf

# Expose port
EXPOSE 80

# Start nginx server to serve the React app
CMD ["nginx", "-g", "daemon off;"]
