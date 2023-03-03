FROM kmcbride/layeri:latest
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
RUN apt-get install --yes mesa-utils libgl1-mesa-glx
ENV PATH="${PATH}:/shft/app/paraview/build/bin"
RUN /sbin/ldconfig

