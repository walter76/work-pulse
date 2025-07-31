# work-pulse
Track my working hours and create reports.

# How to Build the Container for the Services

The command to build the container for the services with Docker is:

```sh
docker build -t work-pulse-service --build-arg INCLUDE_CA=true .
```

The build argument `INCLUDE_CA=true` tells the build process that certificates from a different CA (Certificate
Authority) should be included. If this is set, all certificates that are in the subfolder `certificates` are copied into
the build container and registered for the build process. This might be required if you are in a company network which
is changing the Root-CA because of security reasons.

If you omit the build argument `INCLUDE_CA=true` no certificates will be copied and registered. You still require to
have the empty directory `certificates` because building based on the condition whether a directory exists or not is not
so easy with Docker.
