# Image for testing the build

FROM centos:7

ENV RPM_ROOT=https://downloads.datastax.com/cpp-driver/centos/7

ARG LIBUV_VER=1.29.1
ARG LIBDSE_VER=1.9.0

RUN yum -y update \
    && yum -y install pkgconfig curl ca-certificates openssl gcc \
    # libuv (needs to be installed before dse)
#    && rpm -i $RPM_ROOT/dependencies/libuv/v$LIBUV_VER/libuv-$LIBUV_VER-1.el7.src.rpm \
    && rpm -i $RPM_ROOT/dependencies/libuv/v$LIBUV_VER/libuv-$LIBUV_VER-1.el7.x86_64.rpm \
    && rpm -i $RPM_ROOT/dependencies/libuv/v$LIBUV_VER/libuv-debuginfo-$LIBUV_VER-1.el7.x86_64.rpm \
    && rpm -i $RPM_ROOT/dependencies/libuv/v$LIBUV_VER/libuv-devel-$LIBUV_VER-1.el7.x86_64.rpm \
    # dse
#    && rpm -i $RPM_ROOT/dse/v$LIBDSE_VER/dse-cpp-driver-$LIBDSE_VER-1.el7.src.rpm \
    && rpm -i $RPM_ROOT/dse/v$LIBDSE_VER/dse-cpp-driver-$LIBDSE_VER-1.el7.x86_64.rpm \
    && rpm -i $RPM_ROOT/dse/v$LIBDSE_VER/dse-cpp-driver-debuginfo-$LIBDSE_VER-1.el7.x86_64.rpm \
    && rpm -i $RPM_ROOT/dse/v$LIBDSE_VER/dse-cpp-driver-devel-$LIBDSE_VER-1.el7.x86_64.rpm \
    && yum clean all \
    && rm -rf /var/cache/yum

VOLUME /root
VOLUME /code


ENV CARGO_TARGET_DIR=/root/target \
    RUSTUP_HOME=/opt/rust/rustup \
    CARGO_HOME=/opt/rust/cargo \
    PATH=/opt/rust/cargo/bin:$PATH

RUN mkdir -p /opt/rust/cargo /opt/rust/rustup

RUN curl https://sh.rustup.rs -sSf | sh -s -- \
  --default-toolchain stable --no-modify-path -y

WORKDIR /code