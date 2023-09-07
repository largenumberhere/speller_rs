# Compile the rust project and copy the lib to libs/libspellerrs
rm ./speller
cd ./rust && cargo build --release
cd ..
cp ./rust/target/release/libspellerrs.so ./libspellerrs.so
make
