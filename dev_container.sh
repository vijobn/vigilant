#!/bin/bash

# Ensure the B2B directory exists and is the correct project
current_dir=$(pwd)
root_dir=$(dirname $(dirname "$current_dir"))
root_dirname=$(basename "$root_dir")
if [ "$root_dirname" == "b2b" ]; then
  echo "Running inside b2b"
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
    sudo docker build --tag ${CONTAINER_NAME} --cache-from ${CONTAINER_NAME} -t ${CONTAINER} . || sudo docker build -t ${CONTAINER} .


# Start Dev container
#sudo docker run -v${PWD}:/usr/src/app/ -p7000:3000 -it ${CONTAINER}  /bin/bash

# Allow Docker containers to connect to the X server
xhost +local:docker

# Starting port
HOST_PORT=3000

# Check if port is already in use
while sudo lsof -i :$HOST_PORT > /dev/null 2>&1; do
  # If port is in use, increment by 10
  HOST_PORT=$((HOST_PORT + 10))
done

# Run the Docker container with the chosen port
sudo docker run -it --rm \
  -e DISPLAY=$DISPLAY \
  --env="QT_X11_NO_MITSHM=1" \
  -v /tmp/.X11-unix:/tmp/.X11-unix \
  -v ${PWD}:/usr/src/app/ \
  -p ${HOST_PORT}:3000 \
  ${CONTAINER} \
  /bin/bash -c "echo -e '\nHost port: $HOST_PORT\n'; /bin/bash"

echo "Was using port $HOST_PORT"