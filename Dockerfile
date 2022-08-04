FROM jupyter/nbviewer:2eb040d822d5953dfb0181d09cdaed34d3c6e9a2a09f798c998784547c486f2c
# RUN python3 -m pip install --no-cache-dir notebook jupyterlab

ARG NB_USER=valhalla
ARG NB_UID=1000

ENV USER ${NB_USER}
ENV NB_UID ${NB_UID}
ENV HOME /home/${NB_USER}

RUN adduser --disabled-password \
    --gecos "Default user" \
    --uid ${NB_UID} \
    ${NB_USER}
# Make sure the contents of our repo are in ${HOME}
COPY . ${HOME}
USER root
RUN chown -R ${NB_UID} ${HOME}
USER ${NB_USER}