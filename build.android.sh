# cargo ndk  -t arm64-v8a \
#     -t armeabi-v7a \
#     -t x86 \
#     -t x86_64 \
#     -o ./android/android_studio/app/src/main/jniLibs build --release --features android

cargo ndk  -t arm64-v8a \
    -o ./android/android_studio/app/src/main/jniLibs build --release --features android