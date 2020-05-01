# BUILD AND DEPLOY

The build host environment is a Debian Linux (PureOS in my case).

The two targeted phones are the Librem5 and the PinePhone. Both of these are 64 bit ARM devices so we need to cross-compile the binaries to aarch64 architecture.

Follow these steps when building the very first time to setup your build environment.  Once you get this working, you can use these scripts instead:

- build.sh - cross-builds the app for the device and prepares a flatpak
- deploy.sh - copies the flatpack to your device (NOTE: adjust the script to your device ip addr etc.)

First add the arm64 architecture to your apt:
```
$ sudo dpkg --add-architecture arm64
$ sudo apt update
```

Next, install the gcc cross-compiler with linker and ARM version of the Gtk libs for GUI development:
```
$ sudo apt-get install gcc-aarch64-linux-gnu
$ sudo apt-get install libgtk-3-dev:arm64
```

Also make sure you have this:
```
sudo apt-get install pkg-config
```

Add this target to Rust:
```
$ rustup target add aarch64-unknown-linux-gnu
```

Add this configuration to your Rust binary project's root in .cargo/config file
```
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

Add the following env var to your .bash_config or manually export in the terminal before building:
```
$ export PKG_CONFIG_ALLOW_CROSS=1
```

Now you should be able to cross build your Rust app for the device aarch64 target like this:
```
$ cargo build --release --target=aarch64-unknown-linux-gnu
```


If you're targeting the emulator instead it would be:
```
$  cargo build --release --target=x86_64-unknown-linux-gnu
```

Your device aarch64 binary should be somewhere here:
```
./target/aarch64-unknown-linux-gnu/release/
```

Copy it to your phone:
```
$ scp -r target/aarch64-unknown-linux-gnu/release/body_sculpting tom@192.168.1.98:/home/tom/
```

To package it for the phone in a flatpak, setup your dev machine:
```
$ sudo apt-get install flatpak flatpak-builder
$ flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
$ flatpak install flathub org.gnome.Platform//3.36
```

Also install a version of the qemu emulator that can be used to assist with cross-compilation:
```
$ sudo apt-get -y install qemu-user-static
```

Cross-building flatpak:
```
$ flatpak install gnome-nightly org.gnome.Platform/aarch64 org.gnome.Sdk/aarch64
```

And to build the actual flatpak in a local repository:
```
$ flatpak-builder --arch=<arch> --repo=<repo> <build-dir> <manifest>
```

Specific example:
```
$ flatpak-builder --arch=aarch64 --repo=myrepo _flatpak app.sculpting.body.json
```

The `app.sculpting.body.json` looks like this for this example:
```
{
    "app-id": "app.sculpting.body",
    "runtime": "org.gnome.Platform",
    "runtime-version": "master",
    "sdk": "org.gnome.Sdk",
    "command": "body_sculpting",
    "modules": [
        {
            "name": "bodysculpting",
            "buildsystem": "simple",
            "build-commands": [
                "install -D body_sculpting /app/bin/body_sculpting"
            ],
            "sources": [
                {
                    "type": "file",
                    "path": "./target/aarch64-unknown-linux-gnu/release/body_sculpting"
                }
            ]
        }
    ]
}
```

Then export a binary bundle from the local repository so that it can be tested on the target system:
```
$ flatpak build-bundle --arch=aarch64 myrepo bodysculpting.flatpak app.sculpting.body
```

Copy this bundle to your device:
```
$ scp -r bodysculpting.flatpak tom@192.168.1.98:/home/tom
```

Prepare the Librem5 to run flatpaks.  On the device:
```
$ sudo apt -y install flatpak
```

Or on the PinePhone:
```
$ sudo apk add flatpak
```

Then:
```
$ flatpak --user remote-add --if-not-exists gnome-nightly https://nightly.gnome.org/gnome-nightly.flatpakrepo
```

Now install your app on the phone:
```
$ flatpak --user install bodysculpting.flatpak
```

And run run it:
```
$ flatpak --user run app.sculpting.body
```
