#!/bin/bash
MODULE="zk-commit-mobile"
ROOT_FOLDER="${PWD}"
FRAMEWORK_NAME="zk_commit_mobile"
LIB_NAME="lib${FRAMEWORK_NAME}"
CONFIG="debug"

RUST_BUILD_OUT="${ROOT_FOLDER}/target"
RUST_LIB_OUT="${RUST_BUILD_OUT}/${CONFIG}/${LIB_NAME}.dylib"
BUILD_OUT="${ROOT_FOLDER}/ios_build"
BUILD_OUT_BINDING="${BUILD_OUT}/bindings"
BUILD_OUT_XC="${BUILD_OUT}/${FRAMEWORK_NAME}.xcframework"
DYLIB_NAME="${LIB_NAME}.dylib"

# Build the dylib
cargo build --target-dir ${RUST_BUILD_OUT}
 
# Generate bindings
cargo run --bin uniffi-bindgen generate --library ${RUST_LIB_OUT} --language swift --out-dir ${BUILD_OUT_BINDING}
 

# Add the iOS targets and build
for TARGET in \
        aarch64-apple-ios \
        aarch64-apple-ios-sim \
        x86_64-apple-ios
do
    rustup target add $TARGET
    cargo build --release --target=$TARGET --target-dir ${RUST_BUILD_OUT}

    install_name_tool -id "@rpath/${DYLIB_NAME}" ${RUST_BUILD_OUT}/$TARGET/release/${DYLIB_NAME}
done
 
# Rename *.modulemap to module.modulemap
mv ${BUILD_OUT_BINDING}/${FRAMEWORK_NAME}FFI.modulemap ${BUILD_OUT_BINDING}/module.modulemap
 
# Move the Swift file to the project. (Optional)
IOS_PROJECT_PATH="./crates/zk-commit-mobile/ios/ZKSimpleApp/ZKSimpleApp"
BINDING_SWIFT_FILE="${IOS_PROJECT_PATH}/${FRAMEWORK_NAME}.swift"

rm ${BINDING_SWIFT_FILE} || true
mv ${BUILD_OUT_BINDING}/${FRAMEWORK_NAME}.swift ${BINDING_SWIFT_FILE}
 
# Recreate XCFramework
rm -rf ${BUILD_OUT_XC} || true


LIB_BUILD_ARG=""
for TARGET in \
        aarch64-apple-ios \
        aarch64-apple-ios-sim
do
    LIB_BUILD_ARG="${LIB_BUILD_ARG} -library ${RUST_BUILD_OUT}/$TARGET/release/${DYLIB_NAME} -headers ${BUILD_OUT_BINDING}"
done

xcrun lipo -create -output ${RUST_BUILD_OUT}/aarch64-apple-ios-sim/release/${DYLIB_NAME} \
                          ${RUST_BUILD_OUT}/aarch64-apple-ios-sim/release/${DYLIB_NAME} \
                           ${RUST_BUILD_OUT}/x86_64-apple-ios/release/${DYLIB_NAME}

xcodebuild -create-xcframework ${LIB_BUILD_ARG} -output ${BUILD_OUT_XC}
 # Move the xcframework to ios project
 rm -rf ${IOS_PROJECT_PATH}/${FRAMEWORK_NAME}.xcframework
 mv ${BUILD_OUT_XC} ${IOS_PROJECT_PATH}

# Cleanup
# rm -rf bindings