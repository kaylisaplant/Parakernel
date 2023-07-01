# FROM kmcbride/paraviewlayer:latest
FROM parakernel/pvserver:latest

RUN \
    apt-get update && \
    apt-get install --yes \
      bash                \
      fluxbox             \
      git                 \
      net-tools           \
      novnc               \
      supervisor          \
      x11vnc              \
      xterm               \
      xvfb                \
      terminator


RUN ln -s /usr/share/novnc/vnc_auto.html /usr/share/novnc/index.html

ENV DISPLAY=:0.0 \
    DISPLAY_WIDTH=1600 \
    DISPLAY_HEIGHT=968 \
    RUN_TERMINATOR=yes \
    RUN_FLUXBOX=yes


COPY . /srv
COPY novnc/passwd /root/.vnc/passwd

RUN mkdir -p /root/.fluxbox
RUN ln -s /srv/fluxbox/startup.sh /root/.fluxbox/startup
RUN cp /srv/fluxbox/init /root/.fluxbox/init
# COPY . /srv
COPY novnc/passwd /root/.vnc/passwd

CMD ["/srv/entrypoint.sh"]
EXPOSE 8080

