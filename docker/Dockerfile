FROM ubuntu:20.04
LABEL maintainer="Johannes Blaschke <jpblaschke@lbl.gov>"

# Base Ubuntu packages

ENV DEBIAN_FRONTEND noninteractive
ENV LANG C.UTF-8

ENV TZ=America/Los_Angeles
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime
RUN echo $TZ > /etc/timezone

RUN apt-get update
RUN apt-get --yes upgrade
RUN apt-get install --yes build-essential

#-------------------------------------------------------------------------------
# Make COMMON WORKSPACE (that won't be clobbered by Shifter)
#

RUN mkdir -p /shft
RUN mkdir -p /img

#-------------------------------------------------------------------------------


#-------------------------------------------------------------------------------
# Add MPICH
#

RUN apt-get --yes install wget gfortran

RUN mkdir -p /shft/app
WORKDIR /shft/app

ARG mpich=3.3
ARG mpich_prefix=mpich-$mpich

USER root

RUN wget https://www.mpich.org/static/downloads/$mpich/$mpich_prefix.tar.gz
RUN tar xvzf $mpich_prefix.tar.gz
WORKDIR /shft/app/$mpich_prefix
RUN ./configure
RUN make -j 4
RUN make install
WORKDIR /shft/app
RUN rm -rf $mpich_prefix

#-------------------------------------------------------------------------------


RUN apt-get install --yes git python3 python3-dev pip cmake
RUN update-alternatives --install /usr/bin/python python /usr/bin/python3 10

WORKDIR /shft/app
RUN git clone https://gitlab.kitware.com/paraview/paraview.git
WORKDIR /shft/app/paraview
RUN git submodule update --init --recursive 
RUN mkdir -p shft/app/paraview/build

RUN apt-get install --yes libgl1-mesa-dev
RUN apt-get install --yes libxt-dev
RUN apt-get install --yes qt5-default
RUN apt-get install --yes libqt5x11extras5-dev
RUN apt-get install --yes libqt5help5
RUN apt-get install --yes qttools5-dev 
RUN apt-get install --yes qtxmlpatterns5-dev-tools
RUN apt-get install --yes libqt5svg5-dev

RUN apt-get install --yes python3-numpy
RUN apt-get install --yes libtbb-dev

RUN apt-get install --yes ninja-build

WORKDIR /shft/app/paraview/build
RUN cmake \
        -GNinja \
        -DPARAVIEW_USE_PYTHON=ON \
        -DPARAVIEW_USE_MPI=ON \
        -DVTK_SMP_IMPLEMENTATION_TYPE=TBB \
        -DCMAKE_BUILD_TYPE=Release \
        ..

RUN ninja


# From: https://unix.stackexchange.com/questions/589236/libgl-error-no-matching-fbconfigs-or-visuals-found-glxgears-error-docker-cu
RUN apt-get install --yes mesa-utils libgl1-mesa-glx



#-------------------------------------------------------------------------------
# LDCONFIG
#
# We recommend running an /sbin/ldconfig as part of the image build (e.g. in
# the Dockerfile) to update the cache after installing any new libraries in in
# the image build.
#

# USER root
RUN /sbin/ldconfig
# USER user

#-------------------------------------------------------------------------------



RUN apt-get update && apt-get install -y \
      bash                               \
      fluxbox                            \
      git                                \
      net-tools                          \
      novnc                              \
      supervisor                         \
      x11vnc                             \
      xterm                              \
      xvfb                               \
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

CMD ["/srv/entrypoint.sh"]
EXPOSE 8080

