FROM kmcbride/paraviewlayer:latest

# Install miniconda
ENV CONDA_DIR /opt/conda
RUN wget --quiet https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh -O ~/miniconda.sh && \
     /bin/bash ~/miniconda.sh -b -p /opt/conda

# Put conda in path so we can use conda activate
ENV PATH=$CONDA_DIR/bin:$PATH

RUN conda install --quiet --yes -c conda-forge \
    ipywidgets \
    jupyter \
    ipython \
    pillow \
    nodejs \
    numpy \
    matplotlib \
    scipy

#https://github.com/NVIDIA/ipyparaview/issues/40
RUN conda install --quiet --yes -c conda-forge ipywidgets==7.7.1
RUN apt-get -y update
RUN conda install --quiet --yes -c "conda-forge/label/main" nodejs
WORKDIR /shft/app
RUN pip3 install git+https://github.com/NVIDIA/ipyparaview.git
RUN jupyter nbextension enable --py --sys-prefix ipyparaview


COPY . iparaview-kernel

# Copy updates to the paraview image
WORKDIR /root/.local/share/jupyter/kernels/paraview
RUN cmake -DParaView_PREFIX_PATH=/shft/app/paraview/build \
    /shft/app/iparaview-kernel
RUN make
RUN make install

WORKDIR /root
ENV PYTHONPATH=/shft/app/paraview/build/lib/python3.8/site-packages
CMD ["/shft/app/iparaview-kernel/start_jupyter.sh"]

RUN cp /shft/app/iparaview-kernel/Test.ipynb .

EXPOSE 8888
