#!bin/bash

echo $(cat reply.txt) | nc -w 1 localhost 69  #reply.txt
