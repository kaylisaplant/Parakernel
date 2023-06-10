# FROM kmcbride/novnclayer:latest
FROM parakernel/novnc:latest

RUN \
    apt-get update && \
    apt-get install --yes \
        git

# RUN pip3 install --upgrade pip
WORKDIR /shft
RUN pip install --upgrade pip
ENV CONDA_DIR /opt/conda
RUN \
    wget --quiet \
    https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh \
    -O ~/miniconda.sh && \
    /bin/bash ~/miniconda.sh -b -p /opt/conda
ENV PATH=$CONDA_DIR/bin:$PATH
RUN conda install --quiet --yes -c "conda-forge/label/main" nodejs

#https://github.com/NVIDIA/ipyparaview/issues/40
RUN conda install --quiet --yes -c conda-forge ipywidgets==7.7.1

WORKDIR /shft/app
# RUN pip3 install git+https://github.com/NVIDIA/ipyparaview.git
RUN pip install jupyter numpy
RUN pip install git+https://github.com/NVIDIA/ipyparaview.git
RUN jupyter nbextension enable --py --sys-prefix ipyparaview

COPY . iparaview-kernel

# Copy updates to the paraview image
RUN cp /shft/app/iparaview-kernel/conf.d/jupyter.conf /srv/conf.d/

WORKDIR /root/.local/share/jupyter/kernels/paraview
RUN cmake -DParaView_PREFIX_PATH=/shft/app/paraview/build \
    /shft/app/iparaview-kernel
RUN make
RUN make install

WORKDIR /root
ENV PYTHONPATH=${PYTHONPATH}:/shft/app/paraview/build/lib/python3.8/site-packages
CMD ["/srv/entrypoint.sh"]
# CMD ["jupyter", "notebook", "--ip=0.0.0.0", "--no-browser", "--allow-root", "--NotebookApp.token=''", "--NotebookApp.password='Jupyter'"]

RUN cp /shft/app/iparaview-kernel/Test.ipynb .

# TODO: This is a hack to get things working on Spin -- in future, use the
# correct user
RUN chmod -R g=rwX /root
RUN chmod -R g=rwX /shft

EXPOSE 8888
