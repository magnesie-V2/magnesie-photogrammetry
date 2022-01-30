FROM ubuntu:20.04

# Get dependencies
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
  cmake \
  build-essential \
  git \
  curl \
  wget \
  libpng-dev \
  libjpeg-dev \
  libtiff-dev \
  libxxf86vm1 \
  libxxf86vm-dev \
  libxi-dev \
  libxrandr-dev \
  libglu1-mesa-dev ; \
  apt-get autoclean && apt-get clean

# Build latest openMVG
RUN git clone --recursive https://github.com/openMVG/openMVG.git ; \
  mkdir openMVG_build && cd openMVG_build; \
  cmake -DCMAKE_BUILD_TYPE=RELEASE \
  -DOpenMVG_BUILD_TESTS=OFF \
  -DOpenMVG_BUILD_EXAMPLES=OFF \
  -DOpenMVG_BUILD_DOC=OFF \
  -DTARGET_ARCHITECTURE=generic \
  ../openMVG/src; \
  cmake --build . --target install; \
  cd ..; \
  rm -rf /openMVG; rm -rf /openMVG_build

# openMVS requirements
RUN git clone https://gitlab.com/libeigen/eigen.git --branch 3.2; \
  mkdir eigen_build && cd eigen_build ; \
  cmake . ../eigen ; \
  make && make install ; \
  cd .. ; \
  rm -rf /eigen; rm -rf /eigen_build

RUN apt-get -y install libboost-iostreams-dev libboost-program-options-dev libboost-system-dev libboost-serialization-dev libopencv-dev libcgal-dev libcgal-qt5-dev

RUN git clone https://github.com/cdcseacave/VCG.git vcglib

RUN apt-get -y install libatlas-base-dev libsuitesparse-dev ; \
  git clone https://ceres-solver.googlesource.com/ceres-solver ceres-solver ; \
  mkdir ceres_build && cd ceres_build ; \
  cmake . ../ceres-solver/ -DMINIGLOG=ON -DBUILD_TESTING=OFF -DBUILD_EXAMPLES=OFF ; \
  make -j2 && make install ; \
  cd ..; \
  rm -rf ceres-solver; rm -rf ceres_build

RUN apt-get -y install freeglut3-dev libglew-dev libglfw3-dev

# Build latest openMVS
RUN git clone https://github.com/cdcseacave/openMVS.git openMVS; \
  mkdir openMVS_build && cd openMVS_build; \
  cmake . ../openMVS \
  -DCMAKE_BUILD_TYPE=Release \
  -DVCG_ROOT=/vcglib ; \
  make -j2 && make install ; \
  cp -r /openMVS_build/bin/* /bin; cp /openMVS/MvgMvsPipeline.py /MvgMvsPipeline.py ; rm -rf /openMVS; rm -rf /openMVS_build

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  -s -- -y
ENV PATH $PATH:/root/.cargo/bin
RUN rustup default nightly