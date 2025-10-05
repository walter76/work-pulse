@echo off
echo Building container images for work-pulse ...

echo Building backend container image...
docker build -f src/work-pulse-service.Dockerfile -t workpulse-service ./src

echo Building frontend container image...
docker build -f src/work-pulse-app.Dockerfile -t workpulse-app ./src

echo Build complete!
