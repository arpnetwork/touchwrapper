# touchwrapper

Multitouch interactive wrapper program for Android. It can detect unexpected touch by user.

## Android standalone toolchain

```bash
# Download Android NDK
curl -O http://dl.google.com/android/repository/android-ndk-r10e-linux-x86_64.zip
unzip android-ndk-r10e-linux-x86_64.zip

# Make a standalone toolchain
android-ndk-r10e/build/tools/make-standalone-toolchain.sh \
    --platform=android-18 --toolchain=arm-linux-androideabi-clang3.6 \
    --install-dir=/tmp/android-18-toolchain --ndk-dir=android-ndk-r10e/ --arch=arm
```

## Building

Clone from github:

```bash
git clone https://github.com/arpnetwork/touchwrapper.git
cd touchwrapper
```

Acquire a Rust standard library for Android platform:

```bash
rustup target add arm-linux-androideabi
```

Building with Android standalone toolchain:

```bash
export PATH=/tmp/android-18-toolchain/bin:$PATH
cargo build --release --target=arm-linux-androideabi
```

## Usage

```bash
# Replace the actual multi touch device for your device
./touchwraper /dev/input/event0 | ./arptouch
```
