#!/bin/bash

export PUBLIC_URL=https://tslyriccompletion.com
time npm run build
cd build
cp -r * /var/www/html

echo "Successfully built the project and copied files to /var/www/html. Changes should now be live."

