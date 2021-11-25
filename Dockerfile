FROM diluka/sfm-base

WORKDIR /opt

# Install OpenMVG
ADD OpenMVG ./OpenMVG
RUN mkdir -p /opt/OpenMVG_build \
  && cd /opt/OpenMVG_build \
  && cmake . /opt/OpenMVG/src/ -DCMAKE_BUILD_TYPE=RELEASE -DLEMON_ENABLE_SOPLEX=NO \
  && make && make install

# Install OpenMVS
ADD VCG ./VCG
ADD OpenMVS ./OpenMVS
RUN mkdir -p /opt/OpenMVS_build \
  && cd /opt/OpenMVS_build \
  && cmake . /opt/OpenMVS -DCMAKE_BUILD_TYPE=Release -DVCG_ROOT="/opt/VCG" -DCMAKE_CXX_FLAGS="-w" \
  && make && make install

# Install pipeline
ADD MvgMvs_Pipeline.py .
RUN chmod +x /opt/MvgMvs_Pipeline.py \
  && ln -s /opt/MvgMvs_Pipeline.py /usr/local/bin/mvgmvs

WORKDIR /root

# install additional libraries
RUN apt update
RUN apt install -y wget graphviz

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  -s -- -y
ENV PATH $PATH:/root/.cargo/bin
RUN rustup default nightly

# Prepare env
RUN mkdir /data
ENV DATA_DIR /data
RUN mkdir /res
ENV RES_DIR /res
ENV PHOTOGRAMMETRY_SCRIPT /run.sh
RUN mkdir -p /logs/job

# Prepare webservice
RUN mkdir /webservice
COPY ./webservice /webservice
RUN cd /webservice && cargo build --release && rm -rf target/debug

COPY ./run.sh /
RUN chmod a+x /run.sh

ENTRYPOINT cd /webservice && cargo run --release
