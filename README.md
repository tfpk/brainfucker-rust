# Brainfucker (Rust)

This is a simple rust compiler for Brainfuck, written by @tfpk as a first attempt at Rust. 

Brainfuck is an esoteric language written by Urban Muller in 1993.

## How it works

It assumes an infinite 'tape' that is written on by a machine. The tape is 0 initialized.

| Character | Meaning |
| -------- | ------- |
| `>` | Move the machine right once. |
| `<` | Move the machine left once. |
| `+` | Add one to the value of the tape where the machine is. |
| `-` | Subtract one from the value of the tape where the machine is. |
| `.` | Print the tape's value at the machine as an ascii character. |
| `,` | Take in an ascii character, store its value where the machine is. |
| `[` | If the tape's value at the machine is 0, jump to the matching `[`.  |
| `[` | If the tape's value at the machine is not 0, jump to the matching `]`.  |
| else | Ignore. |

## Sources

 - https://esolangs.org/wiki/Brainfuck#Examples
