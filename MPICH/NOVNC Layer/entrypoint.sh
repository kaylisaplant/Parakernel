#!/bin/bash
set -ex

RUN_FLUXBOX=${RUN_FLUXBOX:-yes}
RUN_XTERM=${RUN_TERMINATOR:-yes}

case $RUN_FLUXBOX in
  false|no|n|0)
    rm -f /srv/conf.d/fluxbox.conf
    ;;
esac

case $RUN_TERMINATOR in
  false|no|n|0)
    rm -f /srv/conf.d/terminator.conf
    ;;
esac

exec supervisord -c /srv/supervisord.conf