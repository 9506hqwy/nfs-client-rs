#!/bin/bash

set -eu

RPCGEN_BIN="https://github.com/9506hqwy/xdr-rs/releases/download/0.3.0/rpcgen-0.3.0-x86_64-unknown-linux-gnu.tar.gz"

WORKDIR=`mktemp -d`
trap 'rm -rf ${WORKDIR}' EXIT

# Download rpcgen binary.
curl -sSL -o ${WORKDIR}/rpcgen.tar.gz ${RPCGEN_BIN}
tar -C ${WORKDIR} -zxf ${WORKDIR}/rpcgen.tar.gz
chmod 755 ${WORKDIR}/rpcgen

# Generate binding.
cp -f proto/rfc5531.x ${WORKDIR}
${WORKDIR}/rpcgen --use-std-trait ${WORKDIR}/rfc5531.x > src/protocol.rs
rustfmt src/protocol.rs

cp -f proto/rfc7531.x ${WORKDIR}
cat - << EOF >> ${WORKDIR}/rfc7531.x
const RPCSEC_GSS = 6;

typedef int            int32_t;
typedef unsigned int   uint32_t;
typedef hyper          int64_t;
typedef unsigned hyper uint64_t;
EOF
sed -i -e 's/typedef opaque  utf8string<>/typedef string  utf8string<>/' ${WORKDIR}/rfc7531.x
${WORKDIR}/rpcgen --use-std-trait --use-extra-trait ${WORKDIR}/rfc7531.x > src/binding.rs
rustfmt src/binding.rs
