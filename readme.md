# An Enigma Machine in Rust

This is a Rust implementation of an Engima Machine.

It is based on the [JAVA implementation by Dr Mike Pound](https://github.com/mikepound/enigma)

At the moment the code runs via unit tests rather than the main. This is OK because a GUI front end will be put on and this project was never intended to be a library. The code is now feature complete, the front end just needs creating.

## PHASE 1 - COMPLETED

- [X] Add the plugboard code
- [X] Add the Enigma code to link together
- [X] Get the Enigma code to work - at the moment it just compiles
- [X] Get code tested to working
- [X] Get a builder pattern for the plugboard, reflector, rotor structures
- [X] Improve the type structures to avoid generic parameters

# TODO

## PHASE 2

- [ ] Add a GUI to illustrate what's going on (will be using [MacroQuad](https://macroquad.rs/) for this)
