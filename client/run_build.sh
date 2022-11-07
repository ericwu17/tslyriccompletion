#!/bin/bash

export PUBLIC_URL=https://linux.ucla.edu/tswift
time npm run build
cd build
cp -r * /var/www/html

echo "Successfully build the project and copied files to /var/www/html. Changes should now be live."

