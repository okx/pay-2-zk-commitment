#!/bin/bash
MODULE="zk-commit-mobile"
ROOT_FOLDER="${PWD}"
FRAMEWORK_NAME="zk_commit_mobile"
LIB_NAME="lib${FRAMEWORK_NAME}"
CONFIG="debug"

RUST_BUILD_OUT="${ROOT_FOLDER}/target"
RUST_LIB_OUT="${RUST_BUILD_OUT}/${CONFIG}/${LIB_NAME}.dylib"
BUILD_OUT="${ROOT_FOLDER}/ios"
BUILD_OUT_BINDING="${BUILD_OUT}/bindings"
BUILD_OUT_XC="${BUILD_OUT}/${FRAMEWORK_NAME}.xcframework"

# Build the dylib
cargo build --target-dir ${RUST_BUILD_OUT}
 
# Generate bindings
cargo run --bin uniffi-bindgen generate --library ${RUST_LIB_OUT} --language swift --out-dir ${BUILD_OUT_BINDING}
 
LIB_BUILD_ARG=""
# Add the iOS targets and build
for TARGET in \
        aarch64-apple-ios \
        aarch64-apple-ios-sim 
do
    rustup target add $TARGET
    cargo build --release --target=$TARGET --target-dir ${RUST_BUILD_OUT} 
    LIB_BUILD_ARG="${LIB_BUILD_ARG} -library ${RUST_BUILD_OUT}/$TARGET/release/${LIB_NAME}.dylib -headers ${BUILD_OUT_BINDING}"
done
 
# Rename *.modulemap to module.modulemap
mv ${BUILD_OUT_BINDING}/${FRAMEWORK_NAME}FFI.modulemap ${BUILD_OUT_BINDING}/module.modulemap
 
# Move the Swift file to the project. (Optional)
 
# Recreate XCFramework
rm -rf ${BUILD_OUT_XC} || true

xcodebuild -create-xcframework ${LIB_BUILD_ARG} -output ${BUILD_OUT_XC} 
 
# Cleanup
# rm -rf bindings