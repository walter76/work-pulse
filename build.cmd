@echo off
echo Building container images for work-pulse ...

echo Building backend container image...
docker build -f src/work-pulse-service.Dockerfile -t work-pulse-service ./src

echo Building frontend container image...
docker build -f src/work-pulse-app.Dockerfile -t work-pulse-app ./src

echo Build complete!
