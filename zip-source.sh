cd ./rust
cargo clean
cd ..
zip -r speller-src.zip ./Makefile ./dictionary.c ./dictionary.h ./libspellers.so ./speller.c rust build.sh test.sh
