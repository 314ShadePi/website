cd .\frontend
mkdir .\dist
cd ..\admin-dash
mkdir .\dist

cd ..\backend
mkdir frontend
mkdir admin-dash
cd frontend
mklink /D .\dist ..\..\frontend\dist
cd ../admin-dash
mklink /D .\dist ..\..\admin-dash\dist