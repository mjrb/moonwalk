# Moonwalk
A language that does different things going forwards and backwards.
The syntax is assembly inspired for some simplicity, but due to the
fact that things can be revered, it supports more complicated if
expressions. This is because it becomes hard to reason about or
seemingly impossible to have certain conditions work both forwards
and backwards using Assembly like compares or simple conditionals.  

**I'd like to emphasize that going into this project neither of us
had written any rust code**

## Authors
Mickey J Winters(mjw271) and Alex Parson(aep143)

## Writeup
for more detail see the [writeup](WRITEUP.md)

## Installing rust for building (rutgers ILab)
1. run `./installrust.sh` which will install rustc and cargo to your ~/local
2. run `source activate.sh` which will add bins and dynamic libraries needed for rust
to your path
3. cargo build
4. `./target/debug/moonwalk <source>.mw`

# working example files
- hello.mw