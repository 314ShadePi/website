#!/usr/bin/env sh

cd ./frontend || exit
trunk build
cd ../admin-dash || exit
trunk build

cd ../backend || exit
mkdir frontend
mkdir admin-dash
cd frontend || exit
ln -s ../../frontend/dist ./dist
cd ../admin-dash || exit
ln -s ../../admin-dash/dist ./dist
