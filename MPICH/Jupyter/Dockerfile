FROM kmcbride/novnclayer:latest
RUN apt-get -y update
RUN apt-get -y install git
RUN pip3 install --upgrade pip
RUN conda install --quiet --yes -c "conda-forge/label/main" nodejs
WORKDIR /shft/app
RUN pip3 install git+https://github.com/NVIDIA/ipyparaview.git
RUN jupyter nbextension enable --py --sys-prefix ipyparaview


COPY . iparaview-kernel

# Copy updates to the paraview image
RUN cp /shft/app/iparaview-kernel/docker/conf.d/jupyter.conf /srv/conf.d/

WORKDIR /root/.local/share/jupyter/kernels/paraview
RUN cmake -DParaView_PREFIX_PATH=/shft/app/paraview/build \
    /shft/app/iparaview-kernel
RUN make
RUN make install

WORKDIR /root
ENV PYTHONPATH=/shft/app/paraview/build/lib/python3.8/site-packages
CMD ["/srv/entrypoint.sh"]
# CMD ["jupyter", "notebook", "--ip=0.0.0.0", "--no-browser", "--allow-root", "--NotebookApp.token=''", "--NotebookApp.password=''"]

RUN cp /shft/app/iparaview-kernel/Test.ipynb .

EXPOSE 8888
