FROM node:18-alpine AS build

WORKDIR /app

# Copy package files
COPY work-pulse-app/package*.json ./

# Install dependencies
RUN npm ci

# Copy source code
COPY work-pulse-app/ ./

# Build the application
RUN npm run build

# Production stage
FROM nginx:alpine

# Copy built assets from build stage
COPY --from=build /app/dist /usr/share/nginx/html

# Copy nginx configuration if needed
# COPY work-pulse-app/nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]