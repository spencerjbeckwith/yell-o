#!/bin/bash

poetry install
cd ui
npm install
npm run build
echo "Done!"