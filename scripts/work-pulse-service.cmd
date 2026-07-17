@echo off

echo Starting work-pulse-service ...
docker run -d -p 8080:8080 --name work-pulse-service work-pulse-service:latest
