#!/bin/bash

cargo build --release
sudo systemctl restart tswiftrs

echo "Successfully built the project and restarted the tswiftrs service. Changes should now be live."

