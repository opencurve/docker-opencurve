FROM centos:7 as fiu
LABEL maintainer="Cyber-SiKu <Cyber-SiKu@outloook.com>"
# ADD https://mirrors.163.com/.help/CentOS7-Base-163.repo /etc/yum.repos.d/CentOS-Base.repo
RUN yum clean all && yum makecache \
    && yum install -y wget \
        gcc \
        python3 \
        make \
    && wget https://github.com/albertito/libfiu/archive/refs/tags/1.00.tar.gz \
    && tar -xzvf 1.00.tar.gz \
    && cd libfiu-1.00 \
    && make  && make install

FROM centos:7 as builder

COPY --from=fiu /usr/local/include/fiu*.h /usr/local/include/
COPY --from=fiu /usr/local/lib/ /usr/local/lib
RUN yum clean all && yum makecache \
    && yum install -y wget \
        make \
        patch \
        gcc \
        gcc-c++ \
        clang \
        zlib-devel \
        openssl-devel \
        libuuid-devel \
        libnl3-devel \
        libcurl-devel \
        python-devel \
        python-setuptools \
        python3-devel \
        python3-setuptools \
        python3-wheel \
        libunwind-devel \
        sudo \
    && wget https://curve-build.nos-eastchina1.126.net/bazelbuild/bazelisk/bazelisk-linux-amd64 \
    && mv bazelisk-linux-amd64 /usr/bin/bazelisk \
    && ln -s /usr/bin/bazelisk /usr/bin/bazel \
    && wget https://curve-build.nos-eastchina1.126.net/bazelbuild/4.2.2/bazel-4.2.2-linux-x86_64 \
    && mkdir -p /root/.cache/bazelisk/downloads/bazelbuild/bazel-4.2.2-linux-x86_64/bin \
    && mv bazel-4.2.2-linux-x86_64 /root/.cache/bazelisk/downloads/bazelbuild/bazel-4.2.2-linux-x86_64/bin/bazel \
    && wget https://curve-build.nos-eastchina1.126.net/bazelbuild/0.17.2/bazel-0.17.2-linux-x86_64\
    && mkdir -p /root/.cache/bazelisk/downloads/bazelbuild/bazel-0.17.2-linux-x86_64/bin \
    && mv bazel-0.17.2-linux-x86_64 /root/.cache/bazelisk/downloads/bazelbuild/bazel-0.17.2-linux-x86_64/bin/bazel \
    && chmod +x /root/.cache/bazelisk/downloads/bazelbuild/bazel-4.2.2-linux-x86_64/bin/bazel /root/.cache/bazelisk/downloads/bazelbuild/bazel-0.17.2-linux-x86_64/bin/bazel /usr/bin/bazel /usr/bin/bazelisk \
    && yum install -y  https://packages.endpointdev.com/rhel/7/os/x86_64/endpoint-repo.x86_64.rpm \
    && yum install -y git
