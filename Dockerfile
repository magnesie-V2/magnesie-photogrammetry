# Image containing Ubuntu 20.04, OpenMVG, OpenMVS, rust, cargo
FROM ghcr.io/magnesie-v2/openmvgmvs:20.04

RUN apt update && apt install -y linux-tools-common linux-tools-generic
RUN mv /usr/lib/linux-tools/5.4* /usr/lib/linux-tools/$(uname -r)

# Add last camera database and export PATH for mvgmvs script
COPY ./sensor_width_camera_database.txt /usr/local/share/openMVG/
ENV PATH $PATH:/usr/local/share/openMVG

# Install pipeline
ADD MvgMvs_Pipeline.py /opt/
RUN chmod +x /opt/MvgMvs_Pipeline.py \
  && ln -s /opt/MvgMvs_Pipeline.py /usr/local/bin/mvgmvs

WORKDIR /root

# Prepare env
COPY ./run.sh /
RUN mkdir /data && mkdir /res && mkdir -p /logs/job && chmod a+x /run.sh
ENV DATA_DIR /data
ENV RES_DIR /res
ENV PHOTOGRAMMETRY_SCRIPT /run.sh

# Build webservice
COPY ./webservice /webservice
RUN cd /webservice && cargo build --release && rm -rf target/debug

# Webservice production environment launch
ENTRYPOINT cd /webservice && cargo run --release