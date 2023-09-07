# Speller-rs

An implementation of the Cs50x 2023 speller problem using rust's foreign function interface.
If you are interested, the speller problem description at time of writing is found at https://cs50.harvard.edu/x/2023/psets/5/speller/ .

# Requirements
All testing data - `texts`, `dictionaries`, `keys` [was provided by cs50](https://cdn.cs50.net/2022/fall/psets/5/speller.zip) for debugging. You will nead to download and extract the contents into the root folder (except dictionary.c) for the compilation and tests to work smoothly. They are essential but have been excluded from re-distribution due to licencing concerns.

# Run
- Compilation is handled by `build.sh` in the root folder. A WSL 1.2.5.0 console with Ubutnu "20.04.6 LTS (Focal Fossa)" was used to *slowly* run the build process.
The script compiles the project in the rust folder and copies the resulting `.so` file into the root folder, and then runs make which relies on the `Makefile` included to link the rust library
- `test.sh` is probably better named run.sh, it just runs the compiled executable with some arguments.
