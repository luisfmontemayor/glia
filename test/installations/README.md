# Installation Tests

This directory contains Dockerfiles and related files for testing the installation process of `glia` components in various environments. These tests ensure that the system dependencies and build processes are correctly documented and work in "clean" environments.

## R Package (`gliar`) Installation Test

The `Dockerfile.Rtest` is designed to simulate a fresh installation on a minimal Ubuntu image. It installs all necessary system dependencies, sets up `mise`, and builds the R package.

### Files
- `Dockerfile.Rtest`: Defines the test environment starting from `ubuntu:22.04`.
- `build_rtest.sh`: A helper script to automate the building and running of the test container.

### How to run
From the root of the repository, execute:
```bash
./test/installations/build_rtest.sh
```

The script will:
1. Build a Docker image named `glia-rtest`.
2. Run a container from that image.
3. The container's default command attempts to load `library(gliar)`.

If the output ends with `[1] "gliar loaded successfully."`, the installation test passed.
