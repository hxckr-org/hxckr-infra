# Git Service

The Git Service is a web service that interacts with a Soft Serve git server, providing HTTP endpoints to perform various git-related operations.

## What the service does

This service acts as a bridge between HTTP clients and a Soft Serve git server. It provides the following functionalities:

1. Create repositories: Allows clients to create new git repositories on the Soft Serve server.
2. Generate authentication tokens: Creates and returns authentication tokens for accessing repositories.
3. Return authenticated URLs: Provides URLs with built-in authentication tokens for easy access to repositories.

The service essentially wraps Soft Serve's SSH-based commands into HTTP endpoints, making it easier to integrate git operations into web applications or other services that prefer HTTP interactions.

## How it is published

The Git Service is containerized using Docker and published using GitHub Actions. Here's an overview of the process:

1. Dockerfile: The service uses a multi-stage Dockerfile to build the Rust application and create a lean runtime image.

2. GitHub Actions: A workflow (`.github/workflows/docker-publish.yml`) is set up to automatically build and publish the Docker image when changes are pushed to the main branch or a new release is created.

3. Multi-platform support: The Docker image is built for both `linux/amd64` and `linux/arm64` platforms.

4. DockerHub: The built images are pushed to DockerHub under the repository `${DOCKERHUB_USERNAME}/hxckr-git-service`.

5. Tagging: Each image is tagged with:
   - `latest`: Always points to the most recent build
   - The git commit SHA
   - The git reference name (branch or tag)

To use the published Docker image, you can pull it from DockerHub:
