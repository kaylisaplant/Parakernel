# Parakernel

Based on: https://gitlab.kitware.com/paraview/iparaview-kernel

## Building the Container

* Using Docker:
```bash
docker build -t <image_name>:latest
```

* Using Podman
```bash
podman build -t <image_name>:latest --format docker
```

## Running

* On a local machine using Docker:
```bash
docker run -p 0.0.0.0:8888:8888 <image_name>:latest
```
