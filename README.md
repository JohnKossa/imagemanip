# imagemanip

This is a repo for image processing algorithms implemented in rust.

## Algorithms

Currently the only algorithm is a "cartoonify" algorithm that flattens regions of similar color in the image creating a Miyazaki-esque cartoon or a watercolor-esque image.

It accepts 2 parameters which can be specified from the command line

1. pct - This is the "sensitivity" of the algorithm. This property tells the algorithm what percentage of the lowest volatility regions to "flatten" after each pass. I've found somehwere from .5 to .85 to be good choices for this.
2. passes -  The algorithm always does one pass by subdividing the image into a grid. This parameter sets the number of times the algorithm will sweep the image with its convolution matrix. I've found 25 to 100 to be good choices for this.

## Build

cargo build

## Sample command

./target/debug/imagemanip in=/path/to/input/image.bmp out=/path/to/output/image.bmp passes=20 pct=.85

## Roadmap

1. Support for gif, especially animated gifs
2. Multithread boost to parallelize processing of gif frames
3. Select algorithms via CLI param
4. Get cartoonify_v1 working as intended
5. Readonly and thread-optimized cartoonify_v1
6. Deterministic default file names for output path placed in same directory if not specified by user.
7. Support for jpeg
8. "Outlining" algorithm for drawing black borders around large color regions in an already "cartoonified" image
9. Support for non-matching input and output file extensions
10. Support for png
11. Support for some form of short video (mp4? AVI? mpeg?)
