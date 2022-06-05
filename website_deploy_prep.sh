#!/bin/bash
# Purpose: build and tar website files for deployment
################################################################################
cd $(dirname "$0")
. helper_functions.sh

checkNotInstalled whiptail
if [ $? -eq 0 ]; then
    echo "error: whiptail is not installed"
    exit 1
fi

confirmWhiptail "Running this script will remove any previous deployment files and create new ones.\nWould you like to continue?" 9
if [ $? -eq 1 ]; then
    exit 0
fi

declare -a folderOptions
folderOptions=()
folderOptions+=("home-page" "" on)
folderOptions+=("learn-vietnamese" "" off)
folderOptions+=("tractor-pulling" "" off)
folderOptions+=("vehicle-ownership-cost" "" off)

declare -a deployFolders
deployFolders=$(whiptail --title "Deploy Projects" --checklist "Select Projects to Deploy:" --cancel-button "Cancel" 0 0 0 "${folderOptions[@]}" 3>&1 1>&2 2>&3)

cd ..
mkdir -p website_deployment_files

for folder in $deployFolders; do
    folder=$(echo $folder | sed 's/"//g')
    cd $folder
    echo "Removing old build files in $folder..."
    rm -rf node_modules/ dist/
    echo "Building $folder..."
    npm i
    npm run build
    cd ..
    echo "Compressing $folder..."
    rm -f website_deployment_files/$folder.tar.gz
    tar --exclude='**/.git' -czf website_deployment_files/$folder.tar.gz $folder
done

exit 0
