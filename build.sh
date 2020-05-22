#! /bin/bash

MANIFEST_DIR="${PWD}"
PGVER="12.3"
BUILD_DIR="${PWD}/target/${PGVER}-build"
POSTGRES_A="${PWD}/target/libpostgres.a"
POSTGRES_LL="${BUILD_DIR}/postgresql-${PGVER}/src/backend/postgres.ll"

if [ "x${NUM_CPUS}" == "x" ]; then
    NUM_CPUS="1"
fi

set -x

if [ -f "${POSTGRES_A}" ]; then
  # we already have libpostgres.a, so don't bother generating it again
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
if [ "x${UNAME}" == "xLinux" ] ; then
  # linux needs to use the "gold" linker
  mkdir build_bin || exit 1
  ln -s /usr/bin/ld.gold build_bin/ld || exit 1
  CFLAGS="-B${PWD}/build_bin"
fi
AR="llvm-ar" CC="clang" CFLAGS="${CFLAGS} -flto" ./configure --without-readline --without-zlib --prefix="${PWD}/temp-install" || exit 1
make -j${NUM_CPUS} || exit 1
make install || exit 1

# rename Postgres 'main' function entry point so it won't conflict
# with users of this library
sed -i'' -e 's/"main"/"pg_main"/g' "${POSTGRES_LL}" || exit 1
sed -i'' -e 's/i32 @main/i32 @pg_main/g' "${POSTGRES_LL}" || exit 1

cd "${MANIFEST_DIR}" || exit 1

# optimize postgres.ll
opt --O3 -adce "${POSTGRES_LL}" -o target/optimized.bc || exit 1

# create an archive which the Rust create will statically link
llvm-ar crv "${POSTGRES_A}" target/optimized.bc || exit 1
echo ${POSTGRES_A}
