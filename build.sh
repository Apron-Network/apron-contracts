#!/usr/bin/env bash

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
    echo "copy to ../release"
    cp ${m_dir}/target/${m_name}.wasm ../release/${m_name}.wasm
    cp ${m_dir}/target/${m_name}.contract ../release/${m_name}.contract
    cp ${m_dir}/target/metadata.json ../release/${m_name}.json
    cd -
}

echo "clean release"
rm -rf ${WORK_DIR}/release
mkdir -p ${WORK_DIR}/release

build_module ${SERVICE_MARKET}
build_module ${SERVICE_STATISTICS}