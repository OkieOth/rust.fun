scriptPos=${0%/*}

cd $scriptPos

yacgVersion=6.3.0

# generate all the things from the business model
if ! docker run -u $(id -u ${USER}):$(id -g ${USER}) -v `pwd`:/project --rm -t \
    ghcr.io/okieoth/yacg:${yacgVersion} \
     --config /project/codegen/config.json; then
    echo "error while run codegen, cancel"
    exit 1
fi
