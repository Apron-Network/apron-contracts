#!/usr/bin/env bash

VERSION=0.1
WORK_DIR=$(cd $(dirname $0); pwd)
SERVICE_MARKET=services_market
SERVICE_STATISTICS=services_statistics

function build_module() {
    m_name=$1
    m_dir=${WORK_DIR}/${m_name}
    echo "build module ${m_dir}"
    cd ${m_dir}
    cargo +nightly contract build
    if [ $? -ne 0 ];then
      echo "build module failed"
      exit 1
    fi
    echo "copy to ../target"
    cp ${m_dir}/target/${m_name}.wasm ../target/${m_name}_v$VERSION.wasm
    cp ${m_dir}/target/${m_name}.contract ../target/${m_name}_v$VERSION.contract
    cp ${m_dir}/target/metadata.json ../target/${m_name}_v$VERSION.json
    cd -
}

echo "clean target"
rm -rf ${WORK_DIR}/target
mkdir -p ${WORK_DIR}/target

build_module ${SERVICE_MARKET}
build_module ${SERVICE_STATISTICS}