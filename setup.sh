#!/usr/bin/env sh

cd ./frontend || exit
mkdir ./dist

cd ../backend || exit
mkdir frontend
cd frontend || exit
ln -s ../../frontend/dist ./dist
