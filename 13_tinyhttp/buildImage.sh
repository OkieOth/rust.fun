#!/bin/bash

scriptPos=${0%/*}


imageBase=my_http_server
imageTag=`cat $scriptPos/version.txt | grep -P '\d+\.\d+\.\d+'`

imageName="$imageBase:$imageTag"

echo "I am going to create: $imageName"

if docker build -t $imageName . -f Dockerfile.app; then
    echo -en "\033[1;34m  image created: $imageName \033[0m\n"
else
    echo -en "\033[1;31m  error while create image: $imageName \033[0m\n"
fi
