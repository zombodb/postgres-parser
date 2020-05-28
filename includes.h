//#include <stdint.h>
//#include <stdbool.h>
//
//#define PGDLLIMPORT
//#define FLEXIBLE_ARRAY_MEMBER
//#define USE_STDBOOL 1
//#define NULL 0
//#define PG_INT32_MAX 2147483647
//
//#define bits32 uint32_t
//#define uint32 uint32_t
//#define uint64 uint64_t
//#define int32 int32_t
//#define int16 int16_t
//
//typedef unsigned int Index;
//typedef unsigned int Oid;
//typedef uintptr_t Datum;
//typedef size_t Size;

#include "postgres.h"
#include "nodes/memnodes.h"
#include "nodes/nodes.h"
#include "nodes/parsenodes.h"
#include "parser/parser.h"
#include "utils/memutils.h"
#include "utils/palloc.h"
