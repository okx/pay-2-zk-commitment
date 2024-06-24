#!/bin/bash
xcodebuild -scheme ZKSimpleApp -destination 'platform=iOS Simulator,name=iPhone 15 Pro' \
  -project ./crates/zk-commit-mobile/ios/ZKSimpleApp/ZKSimpleApp.xcodeproj \
  -configuration Debug \
  -derivedDataPath ./crates/zk-commit-mobile/ios/build_ios \
  clean build install


# Get the UDID of the simulator
SIMULATOR_UDID=$(xcrun simctl list devices | grep 'iPhone 15 Pro (' | grep -oE '[0-9A-F-]{36}' | head -n 1)
# Boot the simulator (if not already running)
open -a Simulator
xcrun simctl boot "$SIMULATOR_UDID"

# Install the app
echo ${SIMULATOR_UDID}
xcrun simctl install ${SIMULATOR_UDID} ./crates/zk-commit-mobile/ios/build_ios/Build/Products/Debug-iphonesimulator/ZKSimpleApp.app

# Launch the app
xcrun simctl launch "$SIMULATOR_UDID" org.okx.ZKSimpleApp