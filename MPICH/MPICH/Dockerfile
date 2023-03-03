FROM ubuntu:focal
SHELL ["/bin/bash", "-c"]
ENV DEBIAN_FRONTEND noninteractive
WORKDIR /tmp

ENV DEBIAN_FRONTEND noninteractive
ENV LANG C.UTF-8

ENV TZ=America/Los_Angeles
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime
RUN echo $TZ > /etc/timezone
RUN \
    apt-get update        && \
    apt-get install --yes    \
        --no-install-recommends \
        build-essential      \
        python3-dev          \
        gfortran             \
        wget              && \
    apt-get install -y python3-pip && \
    apt-get clean all

#-------------------------------------------------------------------------------
# Make COMMON WORKSPACE (that won't be clobbered by Shifter)
#

RUN mkdir -p /shft
RUN mkdir -p /img

#-------------------------------------------------------------------------------


#-------------------------------------------------------------------------------

ARG mpich=4.0.2
ARG mpich_prefix=mpich-$mpich

ARG ncpus $(( $nproc / 8 ))
RUN \
    wget https://www.mpich.org/static/downloads/$mpich/$mpich_prefix.tar.gz && \
    tar xvzf $mpich_prefix.tar.gz                                           
 WORKDIR /tmp/$mpich_prefix                                                       
   RUN\
    ./configure                                                             && \
    make -j 6                                                               && \
    make install                                                            && \
    make clean                                                              && \
    cd ..                                                                   && \
    rm -rf $mpich_prefix


#-------------------------------------------------------------------------------
WORKDIR /shft/app

RUN apt-get install --yes git python3 python3-dev pip libssl-dev
RUN wget https://github.com/Kitware/CMake/releases/download/v3.24.3/cmake-3.24.3.tar.gz
RUN tar xf cmake-3.24.3.tar.gz
WORKDIR /shft/app/cmake-3.24.3
RUN ./configure
RUN make
# Then to install it to your system finally do
RUN make install

# Do this to refresh your console
# RUN hash -r
RUN /sbin/ldconfig



