#!/bin/bash

# Ensure the B2B directory exists and is the correct project
current_dir=$(pwd)
root_dir=$(dirname $(dirname "$current_dir"))
root_dirname=$(basename "$root_dir")
if [ "$root_dirname" == "b2b" ]; then
  echo "Running inside b2b"
else
  [ -d ./b2b ] || (echo "B2B GIT repo not found, exiting"; exit 77); [ "$?" -eq 77 ]  && exit 2
fi

CONTAINER=vig4

# Make symlink to docker for this dev environment
#cp b2b/apps/tauri1/Dockerfile . || /bin/true

# Init container
#sudo docker build --tag ${CONTAINER}:latest --cache-from ${CONTAINER}:latest -t ${CONTAINER} . || sudo docker build -t ${CONTAINER} .

# Check if the Docker image already exists
if [[ "$(sudo docker images -q ${CONTAINER_NAME} 2> /dev/null)" == "" ]]; then
    # If the image does not exist, build it
    echo "Building Docker image ${CONTAINER_NAME}..."
    sudo docker build --tag ${CONTAINER_NAME} --cache-from ${CONTAINER_NAME} -t ${CONTAINER} . || sudo docker build -t ${CONTAINER} .
else
    echo "Docker image ${CONTAINER_NAME} already exists. Skipping build."
fi


# Start Dev container
#sudo docker run -v${PWD}:/usr/src/app/ -p7000:3000 -it ${CONTAINER}  /bin/bash

# Allow Docker containers to connect to the X server
xhost +local:docker

# Start the Docker container with X11 forwarding
sudo docker run -it --rm \
    -e DISPLAY=$DISPLAY \
    --env="QT_X11_NO_MITSHM=1" \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    -v ${PWD}:/usr/src/app/ \
    -p 3000:3000 \
    ${CONTAINER} \
    /bin/bash
