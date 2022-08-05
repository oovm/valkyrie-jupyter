# https://hub.docker.com/r/jupyter/minimal-notebook/tags
FROM jupyter/minimal-notebook:notebook-6.5.4
# As root, install the packages required to build the notebook
ARG NB_USER=valhalla
ARG NB_UID=1000
ENV USER ${NB_USER}
ENV NB_UID ${NB_UID}
ENV HOME /home/${NB_USER}
# Create ${NB_USER} user with UID=1000 and in the 'users' group
RUN adduser --disabled-password \
    --gecos "Default user" \
    --uid ${NB_UID} \
    ${NB_USER}
# RUN python3 -m pip install --no-cache-dir notebook jupyterlab
# Make sure the contents of our repo are in ${HOME}
USER root
COPY . ${HOME}
RUN chown -R ${NB_UID} ${HOME}
# Switch to ${NB_USER}, good practice
USER ${NB_USER}