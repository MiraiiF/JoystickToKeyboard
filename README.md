# Joystick to Keyboard

Program made for my Operational Systems class on Federal University of Ceará.
Before using, change the events device files being read at the start of the main function.
It maps the first joystick to the Tekken 7's defaults (in keyboard), and the second joystick to a arbitrary configuration. The configurations can be changed by altering the hashmap at the start (for buttons) and the keys being pushed in the main (for joystick).

It can't press a key continuosly, only single inputs.
I have plans to extend this (and maybe develop a full arcade) in the future.

# Usage

Change the device files (from /dev/input) in the main function. Then, the usual:

cargo run

Enjoy!