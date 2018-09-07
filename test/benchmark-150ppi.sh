#!/bin/bash 

echo "Run Crate: Raster"
${HOME}/.cargo/bin/raster-resize ${PWD}/samples/test-150ppi.jpg 150 150
echo "Run Crate: Image - Thumbnail"
${HOME}/.cargo/bin/image-resize ${PWD}/samples/test-150ppi.jpg 150 150
echo "Run Crate: Image - Resize Lanczos3"
${HOME}/.cargo/bin/image-resize ${PWD}/samples/test-150ppi.jpg 150 150 l
echo "Run Crate: Image - Resize Gaussian"
${HOME}/.cargo/bin/image-resize ${PWD}/samples/test-150ppi.jpg 150 150 g
echo "Run Crate: Image - Resize CatmullRom"
${HOME}/.cargo/bin/image-resize ${PWD}/samples/test-150ppi.jpg 150 150 c
echo "Run Crate: Image - Resize Nearest"
${HOME}/.cargo/bin/image-resize ${PWD}/samples/test-150ppi.jpg 150 150 n
echo "Run Crate: Image - Resize Triangle"
${HOME}/.cargo/bin/image-resize ${PWD}/samples/test-150ppi.jpg 150 150 t