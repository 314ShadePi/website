#!/usr/bin/env sh

cd ./frontend || exit
mkdir ./dist
cd ../admin-dash || exit
mkdir ./dist

cd ../backend || exit
mkdir frontend
mkdir admin-dash
cd frontend || exit
ln -s ../../frontend/dist ./dist
cd ../admin-dash || exit
ln -s ../../admin-dash/dist ./dist
