#!/bin/bash
# Purpose: tar home files into backup folder (optional encryption)
################################################################################
cd $(dirname "$0")
. helpers/helper_functions.sh

checkNotInstalled whiptail
if [ $? -eq 0 ]; then
    echo "error: whiptail is not installed"
    exit 1
fi

confirmWhiptail "Running this script will remove any old backups and create new ones.\nWould you like to continue?" 9
if [ $? -eq 1 ]; then
    exit 0
fi

declare -a folderOptions
folderOptions=()
folderOptions+=("Documents" "" on)
folderOptions+=("Music" "" off)
folderOptions+=("Pictures" "" off)
folderOptions+=("Videos" "" off)

declare -a backupFolders
backupFolders=$(whiptail --title "Backup Files" --checklist "Select Folders to Backup:" --cancel-button "Cancel" 0 0 0 "${folderOptions[@]}" 3>&1 1>&2 2>&3)

declare -a encryptFolders
passphrase=""

confirmWhiptail "Would you like to encrypt the backups?"
if [ $? -eq 0 ]; then
    passphrase=$(whiptail --passwordbox "Encryption Passphrase:" 8 40 3>&1 1>&2 2>&3)
    passphraseConfirm=$(whiptail --passwordbox "Confirm Encryption Passphrase:" 8 40 3>&1 1>&2 2>&3)
    if [[ $passphrase != $passphraseConfirm ]]; then
        echo "error: passphrases do not match"
        exit 1
    fi

    declare -a encryptfolderOptions
    encryptFolderOptions=()
    for folder in $backupFolders; do
        folder=$(echo $folder | sed 's/"//g')
        encryptFolderOptions+=("$folder" "" off)
    done
    encryptFolders=$(whiptail --title "Backup Files" --checklist "Select Backups to Encrypt:" --cancel-button "Cancel" 0 0 0 "${encryptFolderOptions[@]}" 3>&1 1>&2 2>&3)
fi

cd
mkdir -p backups

for folder in $backupFolders; do
    folder=$(echo $folder | sed 's/"//g')
    rm -f backups/$folder.tar.gz
    rm -f backups/$folder.tar.gz.gpg
    echo "Compressing $folder..."
    tar --exclude='**/.git' -czf backups/$folder.tar.gz $folder
done

for folder in $encryptFolders; do
    folder=$(echo $folder | sed 's/"//g')
    echo "Encrypting $folder..."
    gpg --batch -c --passphrase $passphrase backups/$folder.tar.gz
    rm -f backups/$folder.tar.gz
done

exit 0
