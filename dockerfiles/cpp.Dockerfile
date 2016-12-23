FROM gcc:7.0-snapshot

# Boost
ENV BOOST_ARCHIVE_URL   http://sourceforge.net/projects/boost/files/boost/1.62.0/boost_1_62_0.tar.bz2/download
ENV BOOST_BUILD_DIR     /boost-build
ENV BOOST_PREFIX_DIR    /boost
ENV BOOST_INCLUDE_DIR   ${BOOST_PREFIX_DIR}/include
ENV BOOST_LIB_DIR       ${BOOST_PREFIX_DIR}/lib

RUN mkdir ${BOOST_BUILD_DIR} &&\
    curl -L ${BOOST_ARCHIVE_URL} | tar xj \
        --strip-components=1 \
        -C ${BOOST_BUILD_DIR} &&\
    cd ${BOOST_BUILD_DIR} &&\
    ./bootstrap.sh --prefix=${BOOST_PREFIX_DIR} \
        --with-libraries=program_options \
        --with-libraries=system &&\
    ./b2 -j4 --cxxflags=-stc=c++1z install > boost.build &&\
    rm -rf ${BOOST_BUILD_DIR}

RUN apt-get update -y && apt-get install -y \
    inotify-tools

WORKDIR /app

CMD ["make"]
