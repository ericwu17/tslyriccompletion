#!/bin/bash

cargo install --path .
sudo systemctl restart tswiftrs

echo "Successfully built the project and restarted the tswiftrs service. Changes should now be live."

