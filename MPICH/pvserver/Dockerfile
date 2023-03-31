# FROM kmcbride/layeri:latest
FROM parakernel/base:latest

# RUN update-alternatives --install /usr/bin/python python /usr/bin/python3 10

RUN \
    apt-get update && \
    apt-get install --yes        \
        git                      \
        libgl1-mesa-dev          \
        libxt-dev                \
        qt5-default              \
        libqt5x11extras5-dev     \
        libqt5help5              \
        qttools5-dev             \
        qtxmlpatterns5-dev-tools \
        libqt5svg5-dev           \
        python3-numpy            \
        libtbb-dev               \
        ninja-build              \
        mesa-utils               \
        libgl1-mesa-glx       && \
    apt-get clean all

WORKDIR /shft/app
RUN git clone https://gitlab.kitware.com/paraview/paraview.git
WORKDIR /shft/app/paraview
RUN git checkout v5.11.0
RUN git submodule update --init --recursive

RUN mkdir -p shft/app/paraview/build
WORKDIR /shft/app/paraview/build
RUN cmake \
        -GNinja \
        -DPARAVIEW_USE_PYTHON=ON \
        -DPARAVIEW_USE_MPI=ON \
        -DVTK_SMP_IMPLEMENTATION_TYPE=TBB \
        -DCMAKE_BUILD_TYPE=Release \
        ..

RUN ninja
ENV PATH="${PATH}:/shft/app/paraview/build/bin"
RUN /sbin/ldconfig
