FROM jblaschke/paraview:latest 

RUN apt-get -y update
RUN apt-get -y install git python3 python3-pip python3-dev gcc

RUN pip3 install --upgrade pip

RUN pip3 install jupyter
RUN pip3 install matplotlib numpy scipy
RUN python3 -m pip install ipykernel
RUN python3 -m ipykernel install --user
RUN apt-get install -y npm
RUN pip3 install wheel

RUN apt-get -y install software-properties-common
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys A6DCF7707EBC211F
RUN apt-add-repository "deb http://ppa.launchpad.net/ubuntu-mozilla-security/ppa/ubuntu focal main"
RUN apt-get -y update
RUN apt-get -y install firefox

RUN pip3 install git+https://github.com/NVIDIA/ipyparaview.git
RUN jupyter nbextension enable --py --sys-prefix ipyparaview

WORKDIR /shft/app
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
