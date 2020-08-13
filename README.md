# imagemanip

This is a repo for image processing algorithms implemented in rust.

## Algorithms

Currently the only algorithm is a "cartoonify" algorithm that flattens regions of similar color in the image creating a Miyazaki-esque cartoon or a watercolor-esque image.

It accepts 2 parameters which can be specified from the command line

1. pct - This is the "sensitivity" of the algorithm. This property tells the algorithm what percentage of the lowest volatility regions to throw away after each pass.
2. passes -  The algorithm always does one pass by subdividing the image into a grid. This parameter sets the number of times the algorithm will sweep the image with its convolution matrix.

## Build

cargo build

## Sample command

./target/debug/imagemanip in=/path/to/input/image.bmp out=/path/to/output/image.bmp passes=20 pct=.85
