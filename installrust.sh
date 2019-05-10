#!/bin/bash
#install
mkdir $HOME/local
cd $HOME/local

#download packages
wget https://dl.fedoraproject.org/pub/epel/7/x86_64/Packages/c/cargo-1.34.0-1.el7.x86_64.rpm
wget https://dl.fedoraproject.org/pub/epel/7/x86_64/Packages/r/rust-1.34.0-1.el7.x86_64.rpm

#install to local
rpm2cpio cargo-1.34.0-1.el7.x86_64.rpm  | cpio -div
rpm2cpio ../rust-1.34.0-1.el7.x86_64.rpm  | cpio -div
