FROM buildpack-deps:jessie

RUN BUILD_DEPS="python-dev" &&\
    WATCHMAN_VERSION="v4.7.0" &&\
    WATCHMAN_SOURCE_URL="https://github.com/facebook/watchman/archive/${WATCHMAN_VERSION}.tar.gz" &&\
    WATCHMAN_BUILD_DIR="/watchman-build" &&\
    apt-get update -y && apt-get install -y $BUILD_DEPS --no-install-recommends &&\
    rm -r /var/lib/apt/lists/* &&\
    mkdir $WATCHMAN_BUILD_DIR &&\
    curl -L $WATCHMAN_SOURCE_URL | tar xz \
        --strip-components=1 \
        -C $WATCHMAN_BUILD_DIR &&\
    cd $WATCHMAN_BUILD_DIR &&\
    ./autogen.sh &&\
    ./configure \
        --with-python &&\
    make -j$(nproc) &&\
    make install &&\
    rm -rf $WATCHMAN_BUILD_DIR &&\
    apt-get purge -y --auto-remove $BUILD_DEPS

ENTRYPOINT ["watchman"]
