#!/bin/bash

# sudo docker build -t rust-image .
# sudo docker run rust-image

sudo docker build -t my-rust-app .
sudo docker run -it --rm --name my-running-app my-rust-app