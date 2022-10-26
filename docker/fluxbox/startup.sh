#!/bin/bash


fluxbox &
fbpid=$!

sleep 1
{
    # Applications you want to run after fluxbox has started
    # MAKE SURE THAT APPS THAT KEEP RUNNING HAVE AN & AT THE END
    touch ~/.Xauthority
    xauth generate :0 . trusted
} &

wait $fbpid
