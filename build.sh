#! /bin/bash

MANIFEST_DIR="${PWD}"
UNAME=$(uname)
PGVER="12.3"
BUILD_DIR="${PWD}/target/${PGVER}-build"
POSTGRES_O="${BUILD_DIR}/postgres.o"
POSTGRES_A="${PWD}/target/libpostgres.a"
POSTGRES_LL="${BUILD_DIR}/postgresql-${PGVER}/src/backend/postgres.ll"

set -x

if [ -f "${POSTGRES_A}" ]; then
  # we already have postgres.ll, so don't bother generating it again
  echo ${POSTGRES_A}
  exit 0
fi

mkdir -p ${BUILD_DIR} || exit 1
cd ${BUILD_DIR} || exit 1

# download/untar Postgres
wget -q https://ftp.postgresql.org/pub/source/v12.3/postgresql-${PGVER}.tar.bz2 || exit 1
tar xjf postgresql-${PGVER}.tar.bz2 || exit 1

# patch its Makefiles
cd postgresql-${PGVER} || exit 1
patch -p1 < ../../../patches/makefiles-${PGVER}.patch || exit 1

# configure, build, and (locally) install Postgres
CC="clang" CFLAGS="-flto" ./configure --prefix="${PWD}/temp-install" || exit 1
make -j4 || exit 1
make install || exit 1

# rename Postgres 'main' function entry point so it won't conflict
# with users of this library
sed -i bak 's/"main"/"pg_main"/g' "${POSTGRES_LL}" || exit 1
sed -i bak 's/i32 @main/i32 @pg_main/g' "${POSTGRES_LL}" || exit 1

cd "${MANIFEST_DIR}" || exit 1

# optimize postgres.ll
OPT_ARGS="-adce"
if [ "${UNAME}" == "Darwin" ]; then
  # on MacOS we need to realign the stack
  OPT_ARGS="${OPT_ARGS} --stack-alignment=16 --stackrealign"
fi
opt --O3 ${OPT_ARGS} "${POSTGRES_LL}" -o target/optimized.bc || exit 1

# create an archive which the Rust create will statically link
llvm-ar crv "${POSTGRES_A}" target/optimized.bc || exit 1
echo ${POSTGRES_A}
