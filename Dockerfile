# https://hub.docker.com/r/jupyter/minimal-notebook/tags
FROM jupyter/minimal-notebook:notebook-6.5.4
# As root, install the packages required to build the notebook
USER root
# ARG NB_USER=valhalla
# ARG NB_UID=1000
# Create valhalla user with UID=1000 and in the 'users' group
RUN userdel jovyan; exit 0
RUN adduser --disabled-password \
    --gecos "Default user" \
    --uid 1000 valhalla
# RUN python3 -m pip install --no-cache-dir notebook jupyterlab
# Make sure the contents of our repo are in ${HOME}
COPY . /home/valhalla
RUN chown -R 1000 /home/valhalla
# Switch to valhalla, good practice
USER valhalla
RUN git clone https://github.com/nyar-vm/valkyrie-jupyter.git --branch dev
RUN cd valkyrie-jupyter/projects/playground