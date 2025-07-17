#!/bin/bash

sudo nmap -sP "$1.*" > ../assets/nmap-scan-result.txt
