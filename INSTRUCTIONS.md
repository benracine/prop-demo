# Problem Statement

You are a member of the flight software team and are responsible for writing code that manages the satellite’s propulsion system.
Firing the propulsion system involves waiting for a certain period of time before ignition.
The following is an example usage of this system:

* At absolute time t = 0, send a command to the computer to fire the propulsion in 15 seconds
* At absolute time t = 2, send a command to the computer to fire the propulsion in 30 seconds
* At absolute time t = 32, the computer begins firing the propulsion

The flight computer should accept a command with a relative time of when to fire the propulsion - once that time has elapsed, the program should print out “firing now!”.
If another command is received before the propulsion is fired then the most recently received relative time should overwrite any existing commands.
More formally, if a command _A_ is waiting to fire and another command _B_ is received before A has fired, then B should replace _A_ as the pending command and _A_ should never fire.

If a time of -1 is given, any outstanding commands to fire the propulsion are cancelled.

Note that the flight computer should be able to fire the thruster multiple times in a single execution of the program.

You may use exactly one of the following interfaces for getting data into and out of your program:

* Standard input/standard output
* TCP

You can use whichever is more convenient - we note that some languages make asynchronous IO with standard input/standard output cumbersome and have thus included TCP.

If you do choose to use TCP, please have your server listen on port `8124`.
A sample TCP client, `propulsion_tcp_client.py`, is provided that plumbs standard input to TCP writes and likewise plumbs TCP reads to standard output.
Commands should be delineated by newlines.

A sample _complete_ execution of your program using standard input and output is shown below

```
./your_program
15
30
firing now!
```
