cd .\frontend
mkdir .\dist
cd ..\admin-dash
mkdir .\dist

cd ..\backend
mkdir frontend
mkdir admin-dash
cd frontend
mklink /D ..\..\frontend\dist .\dist
cd ../admin-dash
mklink /D ..\..\admin-dash\dist .\dist