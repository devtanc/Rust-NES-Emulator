# A NES Emulator written in Rust

I'm basing this project on this [YouTube series](https://www.youtube.com/watch?v=F8kx56OZQhg) by `javidx9`. His code can be found [on his GitHub](https://github.com/OneLoneCoder/olcNES). I've done some things a little differently, but I'm not trying to be too different, since I don't really know what I'm doing yet.

- No, this is not excellent code.
- Yes, it is my first _real_ Rust project
- Yes, it's a REALLY ambitious project
- Yes, I hope that one day this is _actually_ working and is really cool

# How to run

Use the command: `cargo run roms/nestest.nes`
Use the spacebar to step through the program, one operation at a time

# Docs for the 6504 CPU and tests

Documentation for the CPU is included here in this repo in the `6502 CPU` folder [source1](http://archive.6502.org/datasheets/rockwell_r650x_r651x.pdf), [source2](https://www.chibiakumas.com/6502/CheatSheet.pdf). Additional documentation can be found on [nesdev.com](http://nesdev.com/6502_cpu.txt), or on [6502.org](http://www.6502.org/).

The docs for the tests can be found [here](https://www.qmtpro.com/~nes/misc/nestest.txt), and the logs that the cpu should match at any given program step are found [here](https://www.qmtpro.com/~nes/misc/nestest.log).

A good guide with opcodes is [here](http://www.6502.org/tutorials/6502opcodes.html).

Information about various flags can be found [here](https://wiki.nesdev.com/w/index.php/Status_flags). Information on the overflow flag logic can be found [here](http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html)
