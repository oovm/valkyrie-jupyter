# https://hub.docker.com/r/jupyter/minimal-notebook/tags
FROM jupyter/minimal-notebook:notebook-6.5.4
# As root, install the packages required to build the notebook
USER root
# ARG NB_USER=jovyan
# ARG NB_UID=1000
# Create jovyan user with UID=1000 and in the 'users' group
RUN userdel jovyan; exit 0
RUN adduser --disabled-password \
    --gecos "Default user" \
    --uid 1000 jovyan
RUN usermod -d /newhome/jovyan jovyan
# RUN python3 -m pip install --no-cache-dir notebook jupyterlab
# Make sure the contents of our repo are in ${HOME}
RUN git clone https://github.com/nyar-vm/valkyrie-jupyter.git --branch dev
RUN cp -r ./valkyrie-jupyter/projects/playground /home/jovyan
RUN rm -rf ./valkyrie-jupyter
RUN chown -R 1000 /home/jovyan
# Switch to jovyan, good practice
USER jovyan
