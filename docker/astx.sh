#!/bin/bash

# ASTx only accepts connection fron localhost, so we have to proxy it

/opt/AhnLab/ASTx/astxdaemon &
socat TCP-LISTEN:55930,fork TCP:127.0.0.1:55920