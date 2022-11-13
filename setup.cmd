cd ./frontend
trunk build
cd ../admin-dash
trunk build

cd ../backend
mkdir frontend
mkdir admin-dash
cd frontend
mklink /D ../../frontend/dist ./dist
cd ../admin-dash
mklink /D ../../admin-dash/dist ./dist