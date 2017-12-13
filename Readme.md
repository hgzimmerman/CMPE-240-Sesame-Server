# Sesame

## Group Members
Henry Zimmerman

## Compilation Instructions

#### Server and Servo Control
The nightly version of Rust is required to compile the Sesame Server.
The nightly compiler can be installed by pasting `curl https://sh.rustup.rs -sSf | sh` into a terminal running on a Raspberry Pi.
Instructions should be followed so that the nightly compiler is installed.
Restarting the shell session may be required to make sure that the `cargo` build tool has been added to your `PATH` environment.

For development of this project, the code was edited on a laptop, and synced to the Pi for compilation.
A compiler target for the Raspberry Pi can be installed on an x86 desktop and the compiled binary can be `scp`ed over to a Pi, but this route of compilation was not utilized for this project.

Execute from a bash or bash-like shell: `ROCKET_ENV=production cargo run`.
This will cause the `cargo` build tool to fetch the dependencies for the project, compile them, and then run the server.
The `ROCKET_ENV=production` shell variable indicates that the server should run using the production environment, which causes the server to bind to any address instead of localhost, allowing access to the server from outside of the Pi.
`ROCKET_ENV=production cargo run --release` will cause the compiler to compile an optimized version of the server, improving performance, although the added performance isn't strictly necessary.
`cargo build --release` will build an optimized version of the server without running it


#### Android Wear App
Is is assumed that the user has set up Android Studio and the Android Debug Bridge (ADB) in their development environment, as well as enabling debug mode over USB and Bluetooth for their phone and watch respectively.
ADB should be used to connect to the phone using `adb forward tcp:4444 localabstract:/adb-hub` and  `adb connect 127.0.0.1:4444`.
Android Studio should be used to open the directory `android/Sesame2`.

If these preconditions are met, the "run" window (shift + f10) of Android Studio should offer the connected watch as a target for running the app.
Select the watch and the Gradle build tool should compile the app, and load it onto the watch via the computer's connection to the phone.
The application should launch once it has been loaded onto the watch.

## Raspberry Pi Set Up
The lab report found in `latex/main.pdf` covers the setup of the Pi in greater detail, but a brief synopsis shall be provided here.

* The Pi should be running RaspbianOS.
* Nightly Rust and Cargo should be installed.
* This repository should be cloned to the Pi and inside the cloned directory, `ROCKET_ENV=production cargo run --release` should be used to compile and start the server.
This should take 5 to 10 minutes.

* A breaker cable should connect all GPIO pins from the Pi to a breadboard.
* The servo's signal cable should be connected to pin 16 of the breaker cable.
* The servo's ground should connect to the ground of the breaker cable.
* The external battery should connect to the power and ground of the servo.


## Execution Instructions
The execution instructions were covered in the compilation instructions section.

If an Android wear watch is not available, sending a POST request with an empty body to the address of the Pi on port 8001 should cause the servo to toggle.