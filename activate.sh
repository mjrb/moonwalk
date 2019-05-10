#!/bin/bash
#activate
#add bin to path
export PATH=$PATH:$HOME/local/usr/bin
#rustc and cargo need to get at their dynamic libraries
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$HOME/local/usr/lib:$HOME/local/usr/lib64:$HOME/local/usr/lib64/llvm7.0/lib"
