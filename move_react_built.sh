#!/bin/bash

cd ../hyperfolio-frontend

ANDROID_ASSETS_BASE=../hyperfolio-mobile/rustWebview/android/app/src/main/assets
ANDROID_ASSETS_APP=../hyperfolio-mobile/rustWebview/android/app/src/main/assets/app
ANDROID_ASSETS_BUILD=../hyperfolio-mobile/rustWebview/android/app/src/main/assets/build

rm -rf $ANDROID_ASSETS_APP

cp -R build $ANDROID_ASSETS_BASE
mv $ANDROID_ASSETS_BUILD $ANDROID_ASSETS_APP

echo "React build moved to Android assets"

echo $ANDROID_ASSETS_APP

exec bash
