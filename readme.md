# An Enigma Machine in Rust

This is a Rust implementation of an Engima Machine.

It is based on the [JAVA implementation by Dr Mike Pound](https://github.com/mikepound/enigma)

At the moment this exists in an incomplete form with just the Rotor and Reflector components completed.

# TODO

## PHASE 1

- [X] Add the plugboard code
- [X] Add the Enigma code to link together
- [X] Get the Enigma code to work - at the moment it just compiles
- [X] Get code tested to working
- [ ] Get a builder pattern for the plugboard, reflector, rotor structures
- [ ] Improve the type structures to avoid generic parameters

## PHASE 2

- [ ] Add a GUI to illustrate what's going on (will be using [MacroQuad](https://macroquad.rs/) for this)


# NOTE

At the moment the project is incomplete, but the source code compiles. Basic Clippy linting has been run on the code and most warnings have been fixed, however there is still a way to go. This will be fixed once the Engima tests have passed.
