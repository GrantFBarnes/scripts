#!/bin/bash
# Purpose: rename and resize all pictures
################################################################################

totalPictures=0
declare -a leaveLarge=("events")

# loop through all files in Pictures
for file in $(find ~/Pictures -name '*.*'); do
    echo "Processing file $file"

    # convert file to extension jpeg
    extension="${file##*.}"
    filename="${file%.*}"
    if [ $extension != "jpeg" ]; then
        echo "    converting to a jpeg..."
        if convert "$file" "$filename".jpeg; then
            rm "$file"
        else
            echo "    Error: could not convert to a jpeg"
            continue
        fi
    fi
    file="$filename".jpeg

    # find max size
    maxSize=2400
    for i in "${leaveLarge[@]}"; do
        if [[ $file == *"$i"* ]]; then
            maxSize=10000
            break
        fi
    done

    # resize photos that are too large
    width=$(identify -format "%[w]" "$file")
    height=$(identify -format "%[h]" "$file")
    if [ $width -gt $height ]; then
        if [ $width -gt $maxSize ]; then
            echo "    too wide ($width), resizing to $maxSize..."
            convert "$file" -resize "$maxSize" "$file"
        fi
    else
        if [ $height -gt $maxSize ]; then
            echo "    too tall ($height), resizing to $maxSize..."
            convert "$file" -resize "x$maxSize" "$file"
        fi
    fi

    ((totalPictures++))
done

echo "Total Pictures: $totalPictures"
