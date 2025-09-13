#!/bin/bash

set -eu

TOOL_VERSION="0.4.0"
RPCGEN_BIN="https://github.com/9506hqwy/xdr-rs/releases/download/${TOOL_VERSION}/rpcgen-${TOOL_VERSION}-x86_64-unknown-linux-gnu.tar.gz"

WORKDIR=$(mktemp -d)
trap 'rm -rf ${WORKDIR}' EXIT

# Download rpcgen binary.
curl -sSL -o "${WORKDIR}/rpcgen.tar.gz" "${RPCGEN_BIN}"
tar -C "${WORKDIR}" -zxf "${WORKDIR}/rpcgen.tar.gz"
chmod 755 "${WORKDIR}/rpcgen"

# Generate binding.
cp -f proto/rfc5531.x "${WORKDIR}"
"${WORKDIR}/rpcgen" --use-std-trait "${WORKDIR}/rfc5531.x" > src/protocol.rs
rustfmt src/protocol.rs

cp -f proto/rfc7863.txt "${WORKDIR}"
patch "${WORKDIR}/rfc7863.txt" proto/rfc7863_flatten_nest_seq.patch
cat "${WORKDIR}/rfc7863.txt" | grep "^  *///" | sed 's?^  */// ??' | sed 's?^  *///$??' | sed 's/^%.*$//' > "${WORKDIR}/rfc7863.x"
cat - << EOF >> "${WORKDIR}/rfc7863.x"
const AUTH_NONE = 0;
const AUTH_SYS = 1;
const RPCSEC_GSS = 6;

typedef int            int32_t;
typedef unsigned int   uint32_t;
typedef hyper          int64_t;
typedef unsigned hyper uint64_t;

struct authsys_parms {
    unsigned int stamp;
    string machinename<255>;
    unsigned int uid;
    unsigned int gid;
    unsigned int gids<16>;
};
EOF
sed -i -e 's/typedef opaque  utf8string<>/typedef string  utf8string<>/' "${WORKDIR}/rfc7863.x"
"${WORKDIR}/rpcgen" --use-indexer "${WORKDIR}/rfc7863.x" > src/binding.rs
rustfmt src/binding.rs
