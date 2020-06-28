#![doc = r" Generated types and constants from Postgres' header files necessary to represent"]
#![doc = r#" a parse tree as raw "C" structures.  Also contains various enum types used by"#]
#![doc = r" this module and the `nodes` module"]
#![allow(improper_ctypes)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub const ALLOCSET_DEFAULT_MINSIZE: u32 = 0;
pub const ALLOCSET_DEFAULT_INITSIZE: u32 = 8192;
pub const ALLOCSET_DEFAULT_MAXSIZE: u32 = 8388608;
pub type Oid = ::std::os::raw::c_uint;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type size_t = __darwin_size_t;
pub type int16 = ::std::os::raw::c_short;
pub type int32 = ::std::os::raw::c_int;
pub type uint32 = ::std::os::raw::c_uint;
pub type bits32 = uint32;
pub type uint64 = ::std::os::raw::c_ulong;
pub type Size = size_t;
pub type Index = ::std::os::raw::c_uint;
pub type sigjmp_buf = [::std::os::raw::c_int; 38usize];
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ErrorContextCallback {
    pub previous: *mut ErrorContextCallback,
    pub callback: ::std::option::Option<unsafe extern "C" fn(arg: *mut ::std::os::raw::c_void)>,
    pub arg: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_ErrorContextCallback() {
    assert_eq!(
        ::std::mem::size_of::<ErrorContextCallback>(),
        24usize,
        concat!("Size of: ", stringify!(ErrorContextCallback))
    );
    assert_eq!(
        ::std::mem::align_of::<ErrorContextCallback>(),
        8usize,
        concat!("Alignment of ", stringify!(ErrorContextCallback))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorContextCallback>())).previous as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorContextCallback),
            "::",
            stringify!(previous)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorContextCallback>())).callback as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorContextCallback),
            "::",
            stringify!(callback)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorContextCallback>())).arg as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorContextCallback),
            "::",
            stringify!(arg)
        )
    );
}
impl Default for ErrorContextCallback {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub static mut error_context_stack: *mut ErrorContextCallback;
}
extern "C" {
    pub static mut PG_exception_stack: *mut sigjmp_buf;
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ErrorData {
    pub elevel: ::std::os::raw::c_int,
    pub output_to_server: bool,
    pub output_to_client: bool,
    pub show_funcname: bool,
    pub hide_stmt: bool,
    pub hide_ctx: bool,
    pub filename: *const ::std::os::raw::c_char,
    pub lineno: ::std::os::raw::c_int,
    pub funcname: *const ::std::os::raw::c_char,
    pub domain: *const ::std::os::raw::c_char,
    pub context_domain: *const ::std::os::raw::c_char,
    pub sqlerrcode: ::std::os::raw::c_int,
    pub message: *mut ::std::os::raw::c_char,
    pub detail: *mut ::std::os::raw::c_char,
    pub detail_log: *mut ::std::os::raw::c_char,
    pub hint: *mut ::std::os::raw::c_char,
    pub context: *mut ::std::os::raw::c_char,
    pub message_id: *const ::std::os::raw::c_char,
    pub schema_name: *mut ::std::os::raw::c_char,
    pub table_name: *mut ::std::os::raw::c_char,
    pub column_name: *mut ::std::os::raw::c_char,
    pub datatype_name: *mut ::std::os::raw::c_char,
    pub constraint_name: *mut ::std::os::raw::c_char,
    pub cursorpos: ::std::os::raw::c_int,
    pub internalpos: ::std::os::raw::c_int,
    pub internalquery: *mut ::std::os::raw::c_char,
    pub saved_errno: ::std::os::raw::c_int,
    pub assoc_context: *mut MemoryContextData,
}
#[test]
fn bindgen_test_layout_ErrorData() {
    assert_eq!(
        ::std::mem::size_of::<ErrorData>(),
        184usize,
        concat!("Size of: ", stringify!(ErrorData))
    );
    assert_eq!(
        ::std::mem::align_of::<ErrorData>(),
        8usize,
        concat!("Alignment of ", stringify!(ErrorData))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).elevel as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(elevel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).output_to_server as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(output_to_server)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).output_to_client as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(output_to_client)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).show_funcname as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(show_funcname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).hide_stmt as *const _ as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(hide_stmt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).hide_ctx as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(hide_ctx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).filename as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(filename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).lineno as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(lineno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).funcname as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(funcname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).domain as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(domain)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).context_domain as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(context_domain)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).sqlerrcode as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(sqlerrcode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).message as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(message)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).detail as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(detail)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).detail_log as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(detail_log)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).hint as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(hint)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).context as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).message_id as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(message_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).schema_name as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(schema_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).table_name as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(table_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).column_name as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(column_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).datatype_name as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(datatype_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).constraint_name as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(constraint_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).cursorpos as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(cursorpos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).internalpos as *const _ as usize },
        156usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(internalpos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).internalquery as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(internalquery)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).saved_errno as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(saved_errno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ErrorData>())).assoc_context as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(ErrorData),
            "::",
            stringify!(assoc_context)
        )
    );
}
impl Default for ErrorData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn CopyErrorData() -> *mut ErrorData;
}
extern "C" {
    pub fn FreeErrorData(edata: *mut ErrorData);
}
extern "C" {
    pub fn FlushErrorState();
}
pub type MemoryContext = *mut MemoryContextData;
pub type MemoryContextCallbackFunction =
    ::std::option::Option<unsafe extern "C" fn(arg: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct MemoryContextCallback {
    pub func: MemoryContextCallbackFunction,
    pub arg: *mut ::std::os::raw::c_void,
    pub next: *mut MemoryContextCallback,
}
#[test]
fn bindgen_test_layout_MemoryContextCallback() {
    assert_eq!(
        ::std::mem::size_of::<MemoryContextCallback>(),
        24usize,
        concat!("Size of: ", stringify!(MemoryContextCallback))
    );
    assert_eq!(
        ::std::mem::align_of::<MemoryContextCallback>(),
        8usize,
        concat!("Alignment of ", stringify!(MemoryContextCallback))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextCallback>())).func as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextCallback),
            "::",
            stringify!(func)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextCallback>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextCallback),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextCallback>())).next as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextCallback),
            "::",
            stringify!(next)
        )
    );
}
impl Default for MemoryContextCallback {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub static mut CurrentMemoryContext: MemoryContext;
}
pub type Datum = usize;
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum pg_enc {
    PG_SQL_ASCII = 0,
    PG_EUC_JP = 1,
    PG_EUC_CN = 2,
    PG_EUC_KR = 3,
    PG_EUC_TW = 4,
    PG_EUC_JIS_2004 = 5,
    PG_UTF8 = 6,
    PG_MULE_INTERNAL = 7,
    PG_LATIN1 = 8,
    PG_LATIN2 = 9,
    PG_LATIN3 = 10,
    PG_LATIN4 = 11,
    PG_LATIN5 = 12,
    PG_LATIN6 = 13,
    PG_LATIN7 = 14,
    PG_LATIN8 = 15,
    PG_LATIN9 = 16,
    PG_LATIN10 = 17,
    PG_WIN1256 = 18,
    PG_WIN1258 = 19,
    PG_WIN866 = 20,
    PG_WIN874 = 21,
    PG_KOI8R = 22,
    PG_WIN1251 = 23,
    PG_WIN1252 = 24,
    PG_ISO_8859_5 = 25,
    PG_ISO_8859_6 = 26,
    PG_ISO_8859_7 = 27,
    PG_ISO_8859_8 = 28,
    PG_WIN1250 = 29,
    PG_WIN1253 = 30,
    PG_WIN1254 = 31,
    PG_WIN1255 = 32,
    PG_WIN1257 = 33,
    PG_KOI8U = 34,
    PG_SJIS = 35,
    PG_BIG5 = 36,
    PG_GBK = 37,
    PG_UHC = 38,
    PG_GB18030 = 39,
    PG_JOHAB = 40,
    PG_SHIFT_JIS_2004 = 41,
    _PG_LAST_ENCODING_ = 42,
}
extern "C" {
    pub fn SetDatabaseEncoding(encoding: ::std::os::raw::c_int);
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NodeTag {
    T_Invalid = 0,
    T_IndexInfo = 1,
    T_ExprContext = 2,
    T_ProjectionInfo = 3,
    T_JunkFilter = 4,
    T_OnConflictSetState = 5,
    T_ResultRelInfo = 6,
    T_EState = 7,
    T_TupleTableSlot = 8,
    T_Plan = 9,
    T_Result = 10,
    T_ProjectSet = 11,
    T_ModifyTable = 12,
    T_Append = 13,
    T_MergeAppend = 14,
    T_RecursiveUnion = 15,
    T_BitmapAnd = 16,
    T_BitmapOr = 17,
    T_Scan = 18,
    T_SeqScan = 19,
    T_SampleScan = 20,
    T_IndexScan = 21,
    T_IndexOnlyScan = 22,
    T_BitmapIndexScan = 23,
    T_BitmapHeapScan = 24,
    T_TidScan = 25,
    T_SubqueryScan = 26,
    T_FunctionScan = 27,
    T_ValuesScan = 28,
    T_TableFuncScan = 29,
    T_CteScan = 30,
    T_NamedTuplestoreScan = 31,
    T_WorkTableScan = 32,
    T_ForeignScan = 33,
    T_CustomScan = 34,
    T_Join = 35,
    T_NestLoop = 36,
    T_MergeJoin = 37,
    T_HashJoin = 38,
    T_Material = 39,
    T_Sort = 40,
    T_Group = 41,
    T_Agg = 42,
    T_WindowAgg = 43,
    T_Unique = 44,
    T_Gather = 45,
    T_GatherMerge = 46,
    T_Hash = 47,
    T_SetOp = 48,
    T_LockRows = 49,
    T_Limit = 50,
    T_NestLoopParam = 51,
    T_PlanRowMark = 52,
    T_PartitionPruneInfo = 53,
    T_PartitionedRelPruneInfo = 54,
    T_PartitionPruneStepOp = 55,
    T_PartitionPruneStepCombine = 56,
    T_PlanInvalItem = 57,
    T_PlanState = 58,
    T_ResultState = 59,
    T_ProjectSetState = 60,
    T_ModifyTableState = 61,
    T_AppendState = 62,
    T_MergeAppendState = 63,
    T_RecursiveUnionState = 64,
    T_BitmapAndState = 65,
    T_BitmapOrState = 66,
    T_ScanState = 67,
    T_SeqScanState = 68,
    T_SampleScanState = 69,
    T_IndexScanState = 70,
    T_IndexOnlyScanState = 71,
    T_BitmapIndexScanState = 72,
    T_BitmapHeapScanState = 73,
    T_TidScanState = 74,
    T_SubqueryScanState = 75,
    T_FunctionScanState = 76,
    T_TableFuncScanState = 77,
    T_ValuesScanState = 78,
    T_CteScanState = 79,
    T_NamedTuplestoreScanState = 80,
    T_WorkTableScanState = 81,
    T_ForeignScanState = 82,
    T_CustomScanState = 83,
    T_JoinState = 84,
    T_NestLoopState = 85,
    T_MergeJoinState = 86,
    T_HashJoinState = 87,
    T_MaterialState = 88,
    T_SortState = 89,
    T_GroupState = 90,
    T_AggState = 91,
    T_WindowAggState = 92,
    T_UniqueState = 93,
    T_GatherState = 94,
    T_GatherMergeState = 95,
    T_HashState = 96,
    T_SetOpState = 97,
    T_LockRowsState = 98,
    T_LimitState = 99,
    T_Alias = 100,
    T_RangeVar = 101,
    T_TableFunc = 102,
    T_Expr = 103,
    T_Var = 104,
    T_Const = 105,
    T_Param = 106,
    T_Aggref = 107,
    T_GroupingFunc = 108,
    T_WindowFunc = 109,
    T_SubscriptingRef = 110,
    T_FuncExpr = 111,
    T_NamedArgExpr = 112,
    T_OpExpr = 113,
    T_DistinctExpr = 114,
    T_NullIfExpr = 115,
    T_ScalarArrayOpExpr = 116,
    T_BoolExpr = 117,
    T_SubLink = 118,
    T_SubPlan = 119,
    T_AlternativeSubPlan = 120,
    T_FieldSelect = 121,
    T_FieldStore = 122,
    T_RelabelType = 123,
    T_CoerceViaIO = 124,
    T_ArrayCoerceExpr = 125,
    T_ConvertRowtypeExpr = 126,
    T_CollateExpr = 127,
    T_CaseExpr = 128,
    T_CaseWhen = 129,
    T_CaseTestExpr = 130,
    T_ArrayExpr = 131,
    T_RowExpr = 132,
    T_RowCompareExpr = 133,
    T_CoalesceExpr = 134,
    T_MinMaxExpr = 135,
    T_SQLValueFunction = 136,
    T_XmlExpr = 137,
    T_NullTest = 138,
    T_BooleanTest = 139,
    T_CoerceToDomain = 140,
    T_CoerceToDomainValue = 141,
    T_SetToDefault = 142,
    T_CurrentOfExpr = 143,
    T_NextValueExpr = 144,
    T_InferenceElem = 145,
    T_TargetEntry = 146,
    T_RangeTblRef = 147,
    T_JoinExpr = 148,
    T_FromExpr = 149,
    T_OnConflictExpr = 150,
    T_IntoClause = 151,
    T_ExprState = 152,
    T_AggrefExprState = 153,
    T_WindowFuncExprState = 154,
    T_SetExprState = 155,
    T_SubPlanState = 156,
    T_AlternativeSubPlanState = 157,
    T_DomainConstraintState = 158,
    T_PlannerInfo = 159,
    T_PlannerGlobal = 160,
    T_RelOptInfo = 161,
    T_IndexOptInfo = 162,
    T_ForeignKeyOptInfo = 163,
    T_ParamPathInfo = 164,
    T_Path = 165,
    T_IndexPath = 166,
    T_BitmapHeapPath = 167,
    T_BitmapAndPath = 168,
    T_BitmapOrPath = 169,
    T_TidPath = 170,
    T_SubqueryScanPath = 171,
    T_ForeignPath = 172,
    T_CustomPath = 173,
    T_NestPath = 174,
    T_MergePath = 175,
    T_HashPath = 176,
    T_AppendPath = 177,
    T_MergeAppendPath = 178,
    T_GroupResultPath = 179,
    T_MaterialPath = 180,
    T_UniquePath = 181,
    T_GatherPath = 182,
    T_GatherMergePath = 183,
    T_ProjectionPath = 184,
    T_ProjectSetPath = 185,
    T_SortPath = 186,
    T_GroupPath = 187,
    T_UpperUniquePath = 188,
    T_AggPath = 189,
    T_GroupingSetsPath = 190,
    T_MinMaxAggPath = 191,
    T_WindowAggPath = 192,
    T_SetOpPath = 193,
    T_RecursiveUnionPath = 194,
    T_LockRowsPath = 195,
    T_ModifyTablePath = 196,
    T_LimitPath = 197,
    T_EquivalenceClass = 198,
    T_EquivalenceMember = 199,
    T_PathKey = 200,
    T_PathTarget = 201,
    T_RestrictInfo = 202,
    T_IndexClause = 203,
    T_PlaceHolderVar = 204,
    T_SpecialJoinInfo = 205,
    T_AppendRelInfo = 206,
    T_PlaceHolderInfo = 207,
    T_MinMaxAggInfo = 208,
    T_PlannerParamItem = 209,
    T_RollupData = 210,
    T_GroupingSetData = 211,
    T_StatisticExtInfo = 212,
    T_MemoryContext = 213,
    T_AllocSetContext = 214,
    T_SlabContext = 215,
    T_GenerationContext = 216,
    T_Value = 217,
    T_Integer = 218,
    T_Float = 219,
    T_String = 220,
    T_BitString = 221,
    T_Null = 222,
    T_List = 223,
    T_IntList = 224,
    T_OidList = 225,
    T_ExtensibleNode = 226,
    T_RawStmt = 227,
    T_Query = 228,
    T_PlannedStmt = 229,
    T_InsertStmt = 230,
    T_DeleteStmt = 231,
    T_UpdateStmt = 232,
    T_SelectStmt = 233,
    T_AlterTableStmt = 234,
    T_AlterTableCmd = 235,
    T_AlterDomainStmt = 236,
    T_SetOperationStmt = 237,
    T_GrantStmt = 238,
    T_GrantRoleStmt = 239,
    T_AlterDefaultPrivilegesStmt = 240,
    T_ClosePortalStmt = 241,
    T_ClusterStmt = 242,
    T_CopyStmt = 243,
    T_CreateStmt = 244,
    T_DefineStmt = 245,
    T_DropStmt = 246,
    T_TruncateStmt = 247,
    T_CommentStmt = 248,
    T_FetchStmt = 249,
    T_IndexStmt = 250,
    T_CreateFunctionStmt = 251,
    T_AlterFunctionStmt = 252,
    T_DoStmt = 253,
    T_RenameStmt = 254,
    T_RuleStmt = 255,
    T_NotifyStmt = 256,
    T_ListenStmt = 257,
    T_UnlistenStmt = 258,
    T_TransactionStmt = 259,
    T_ViewStmt = 260,
    T_LoadStmt = 261,
    T_CreateDomainStmt = 262,
    T_CreatedbStmt = 263,
    T_DropdbStmt = 264,
    T_VacuumStmt = 265,
    T_ExplainStmt = 266,
    T_CreateTableAsStmt = 267,
    T_CreateSeqStmt = 268,
    T_AlterSeqStmt = 269,
    T_VariableSetStmt = 270,
    T_VariableShowStmt = 271,
    T_DiscardStmt = 272,
    T_CreateTrigStmt = 273,
    T_CreatePLangStmt = 274,
    T_CreateRoleStmt = 275,
    T_AlterRoleStmt = 276,
    T_DropRoleStmt = 277,
    T_LockStmt = 278,
    T_ConstraintsSetStmt = 279,
    T_ReindexStmt = 280,
    T_CheckPointStmt = 281,
    T_CreateSchemaStmt = 282,
    T_AlterDatabaseStmt = 283,
    T_AlterDatabaseSetStmt = 284,
    T_AlterRoleSetStmt = 285,
    T_CreateConversionStmt = 286,
    T_CreateCastStmt = 287,
    T_CreateOpClassStmt = 288,
    T_CreateOpFamilyStmt = 289,
    T_AlterOpFamilyStmt = 290,
    T_PrepareStmt = 291,
    T_ExecuteStmt = 292,
    T_DeallocateStmt = 293,
    T_DeclareCursorStmt = 294,
    T_CreateTableSpaceStmt = 295,
    T_DropTableSpaceStmt = 296,
    T_AlterObjectDependsStmt = 297,
    T_AlterObjectSchemaStmt = 298,
    T_AlterOwnerStmt = 299,
    T_AlterOperatorStmt = 300,
    T_DropOwnedStmt = 301,
    T_ReassignOwnedStmt = 302,
    T_CompositeTypeStmt = 303,
    T_CreateEnumStmt = 304,
    T_CreateRangeStmt = 305,
    T_AlterEnumStmt = 306,
    T_AlterTSDictionaryStmt = 307,
    T_AlterTSConfigurationStmt = 308,
    T_CreateFdwStmt = 309,
    T_AlterFdwStmt = 310,
    T_CreateForeignServerStmt = 311,
    T_AlterForeignServerStmt = 312,
    T_CreateUserMappingStmt = 313,
    T_AlterUserMappingStmt = 314,
    T_DropUserMappingStmt = 315,
    T_AlterTableSpaceOptionsStmt = 316,
    T_AlterTableMoveAllStmt = 317,
    T_SecLabelStmt = 318,
    T_CreateForeignTableStmt = 319,
    T_ImportForeignSchemaStmt = 320,
    T_CreateExtensionStmt = 321,
    T_AlterExtensionStmt = 322,
    T_AlterExtensionContentsStmt = 323,
    T_CreateEventTrigStmt = 324,
    T_AlterEventTrigStmt = 325,
    T_RefreshMatViewStmt = 326,
    T_ReplicaIdentityStmt = 327,
    T_AlterSystemStmt = 328,
    T_CreatePolicyStmt = 329,
    T_AlterPolicyStmt = 330,
    T_CreateTransformStmt = 331,
    T_CreateAmStmt = 332,
    T_CreatePublicationStmt = 333,
    T_AlterPublicationStmt = 334,
    T_CreateSubscriptionStmt = 335,
    T_AlterSubscriptionStmt = 336,
    T_DropSubscriptionStmt = 337,
    T_CreateStatsStmt = 338,
    T_AlterCollationStmt = 339,
    T_CallStmt = 340,
    T_A_Expr = 341,
    T_ColumnRef = 342,
    T_ParamRef = 343,
    T_A_Const = 344,
    T_FuncCall = 345,
    T_A_Star = 346,
    T_A_Indices = 347,
    T_A_Indirection = 348,
    T_A_ArrayExpr = 349,
    T_ResTarget = 350,
    T_MultiAssignRef = 351,
    T_TypeCast = 352,
    T_CollateClause = 353,
    T_SortBy = 354,
    T_WindowDef = 355,
    T_RangeSubselect = 356,
    T_RangeFunction = 357,
    T_RangeTableSample = 358,
    T_RangeTableFunc = 359,
    T_RangeTableFuncCol = 360,
    T_TypeName = 361,
    T_ColumnDef = 362,
    T_IndexElem = 363,
    T_Constraint = 364,
    T_DefElem = 365,
    T_RangeTblEntry = 366,
    T_RangeTblFunction = 367,
    T_TableSampleClause = 368,
    T_WithCheckOption = 369,
    T_SortGroupClause = 370,
    T_GroupingSet = 371,
    T_WindowClause = 372,
    T_ObjectWithArgs = 373,
    T_AccessPriv = 374,
    T_CreateOpClassItem = 375,
    T_TableLikeClause = 376,
    T_FunctionParameter = 377,
    T_LockingClause = 378,
    T_RowMarkClause = 379,
    T_XmlSerialize = 380,
    T_WithClause = 381,
    T_InferClause = 382,
    T_OnConflictClause = 383,
    T_CommonTableExpr = 384,
    T_RoleSpec = 385,
    T_TriggerTransition = 386,
    T_PartitionElem = 387,
    T_PartitionSpec = 388,
    T_PartitionBoundSpec = 389,
    T_PartitionRangeDatum = 390,
    T_PartitionCmd = 391,
    T_VacuumRelation = 392,
    T_IdentifySystemCmd = 393,
    T_BaseBackupCmd = 394,
    T_CreateReplicationSlotCmd = 395,
    T_DropReplicationSlotCmd = 396,
    T_StartReplicationCmd = 397,
    T_TimeLineHistoryCmd = 398,
    T_SQLCmd = 399,
    T_TriggerData = 400,
    T_EventTriggerData = 401,
    T_ReturnSetInfo = 402,
    T_WindowObjectData = 403,
    T_TIDBitmap = 404,
    T_InlineCodeBlock = 405,
    T_FdwRoutine = 406,
    T_IndexAmRoutine = 407,
    T_TableAmRoutine = 408,
    T_TsmRoutine = 409,
    T_ForeignKeyCacheInfo = 410,
    T_CallContext = 411,
    T_SupportRequestSimplify = 412,
    T_SupportRequestSelectivity = 413,
    T_SupportRequestCost = 414,
    T_SupportRequestRows = 415,
    T_SupportRequestIndexCondition = 416,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Node {
    pub type_: NodeTag,
}
#[test]
fn bindgen_test_layout_Node() {
    assert_eq!(
        ::std::mem::size_of::<Node>(),
        4usize,
        concat!("Size of: ", stringify!(Node))
    );
    assert_eq!(
        ::std::mem::align_of::<Node>(),
        4usize,
        concat!("Alignment of ", stringify!(Node))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Node>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Node),
            "::",
            stringify!(type_)
        )
    );
}
impl Default for Node {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CmdType {
    CMD_UNKNOWN = 0,
    CMD_SELECT = 1,
    CMD_UPDATE = 2,
    CMD_INSERT = 3,
    CMD_DELETE = 4,
    CMD_UTILITY = 5,
    CMD_NOTHING = 6,
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JoinType {
    JOIN_INNER = 0,
    JOIN_LEFT = 1,
    JOIN_FULL = 2,
    JOIN_RIGHT = 3,
    JOIN_SEMI = 4,
    JOIN_ANTI = 5,
    JOIN_UNIQUE_OUTER = 6,
    JOIN_UNIQUE_INNER = 7,
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AggSplit {
    AGGSPLIT_SIMPLE = 0,
    AGGSPLIT_INITIAL_SERIAL = 6,
    AGGSPLIT_FINAL_DESERIAL = 9,
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OnConflictAction {
    ONCONFLICT_NONE = 0,
    ONCONFLICT_NOTHING = 1,
    ONCONFLICT_UPDATE = 2,
}
#[repr(C)]
#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct MemoryContextCounters {
    pub nblocks: Size,
    pub freechunks: Size,
    pub totalspace: Size,
    pub freespace: Size,
}
#[test]
fn bindgen_test_layout_MemoryContextCounters() {
    assert_eq!(
        ::std::mem::size_of::<MemoryContextCounters>(),
        32usize,
        concat!("Size of: ", stringify!(MemoryContextCounters))
    );
    assert_eq!(
        ::std::mem::align_of::<MemoryContextCounters>(),
        8usize,
        concat!("Alignment of ", stringify!(MemoryContextCounters))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextCounters>())).nblocks as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextCounters),
            "::",
            stringify!(nblocks)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MemoryContextCounters>())).freechunks as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextCounters),
            "::",
            stringify!(freechunks)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MemoryContextCounters>())).totalspace as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextCounters),
            "::",
            stringify!(totalspace)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextCounters>())).freespace as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextCounters),
            "::",
            stringify!(freespace)
        )
    );
}
pub type MemoryStatsPrintFunc = ::std::option::Option<
    unsafe extern "C" fn(
        context: MemoryContext,
        passthru: *mut ::std::os::raw::c_void,
        stats_string: *const ::std::os::raw::c_char,
    ),
>;
#[repr(C)]
#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct MemoryContextMethods {
    pub alloc: ::std::option::Option<
        unsafe extern "C" fn(context: MemoryContext, size: Size) -> *mut ::std::os::raw::c_void,
    >,
    pub free_p: ::std::option::Option<
        unsafe extern "C" fn(context: MemoryContext, pointer: *mut ::std::os::raw::c_void),
    >,
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(
            context: MemoryContext,
            pointer: *mut ::std::os::raw::c_void,
            size: Size,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub reset: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext)>,
    pub delete_context: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext)>,
    pub get_chunk_space: ::std::option::Option<
        unsafe extern "C" fn(context: MemoryContext, pointer: *mut ::std::os::raw::c_void) -> Size,
    >,
    pub is_empty: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext) -> bool>,
    pub stats: ::std::option::Option<
        unsafe extern "C" fn(
            context: MemoryContext,
            printfunc: MemoryStatsPrintFunc,
            passthru: *mut ::std::os::raw::c_void,
            totals: *mut MemoryContextCounters,
        ),
    >,
}
#[test]
fn bindgen_test_layout_MemoryContextMethods() {
    assert_eq!(
        ::std::mem::size_of::<MemoryContextMethods>(),
        64usize,
        concat!("Size of: ", stringify!(MemoryContextMethods))
    );
    assert_eq!(
        ::std::mem::align_of::<MemoryContextMethods>(),
        8usize,
        concat!("Alignment of ", stringify!(MemoryContextMethods))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextMethods>())).alloc as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextMethods),
            "::",
            stringify!(alloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextMethods>())).free_p as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextMethods),
            "::",
            stringify!(free_p)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextMethods>())).realloc as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextMethods),
            "::",
            stringify!(realloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextMethods>())).reset as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextMethods),
            "::",
            stringify!(reset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MemoryContextMethods>())).delete_context as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextMethods),
            "::",
            stringify!(delete_context)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MemoryContextMethods>())).get_chunk_space as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextMethods),
            "::",
            stringify!(get_chunk_space)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextMethods>())).is_empty as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextMethods),
            "::",
            stringify!(is_empty)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextMethods>())).stats as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextMethods),
            "::",
            stringify!(stats)
        )
    );
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct MemoryContextData {
    pub type_: NodeTag,
    pub isReset: bool,
    pub allowInCritSection: bool,
    pub methods: *const MemoryContextMethods,
    pub parent: MemoryContext,
    pub firstchild: MemoryContext,
    pub prevchild: MemoryContext,
    pub nextchild: MemoryContext,
    pub name: *const ::std::os::raw::c_char,
    pub ident: *const ::std::os::raw::c_char,
    pub reset_cbs: *mut MemoryContextCallback,
}
#[test]
fn bindgen_test_layout_MemoryContextData() {
    assert_eq!(
        ::std::mem::size_of::<MemoryContextData>(),
        72usize,
        concat!("Size of: ", stringify!(MemoryContextData))
    );
    assert_eq!(
        ::std::mem::align_of::<MemoryContextData>(),
        8usize,
        concat!("Alignment of ", stringify!(MemoryContextData))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextData>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextData>())).isReset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(isReset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MemoryContextData>())).allowInCritSection as *const _ as usize
        },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(allowInCritSection)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextData>())).methods as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(methods)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextData>())).parent as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextData>())).firstchild as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(firstchild)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextData>())).prevchild as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(prevchild)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextData>())).nextchild as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(nextchild)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextData>())).name as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextData>())).ident as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(ident)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MemoryContextData>())).reset_cbs as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(MemoryContextData),
            "::",
            stringify!(reset_cbs)
        )
    );
}
impl Default for MemoryContextData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type bitmapword = uint64;
#[repr(C)]
#[derive(Debug, Default)]
pub struct Bitmapset {
    pub nwords: ::std::os::raw::c_int,
    pub words: __IncompleteArrayField<bitmapword>,
}
#[test]
fn bindgen_test_layout_Bitmapset() {
    assert_eq!(
        ::std::mem::size_of::<Bitmapset>(),
        8usize,
        concat!("Size of: ", stringify!(Bitmapset))
    );
    assert_eq!(
        ::std::mem::align_of::<Bitmapset>(),
        8usize,
        concat!("Alignment of ", stringify!(Bitmapset))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Bitmapset>())).nwords as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Bitmapset),
            "::",
            stringify!(nwords)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Bitmapset>())).words as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Bitmapset),
            "::",
            stringify!(words)
        )
    );
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LockClauseStrength {
    LCS_NONE = 0,
    LCS_FORKEYSHARE = 1,
    LCS_FORSHARE = 2,
    LCS_FORNOKEYUPDATE = 3,
    LCS_FORUPDATE = 4,
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LockWaitPolicy {
    LockWaitBlock = 0,
    LockWaitSkip = 1,
    LockWaitError = 2,
}
pub type AttrNumber = int16;
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct List {
    pub type_: NodeTag,
    pub length: ::std::os::raw::c_int,
    pub head: *mut ListCell,
    pub tail: *mut ListCell,
}
#[test]
fn bindgen_test_layout_List() {
    assert_eq!(
        ::std::mem::size_of::<List>(),
        24usize,
        concat!("Size of: ", stringify!(List))
    );
    assert_eq!(
        ::std::mem::align_of::<List>(),
        8usize,
        concat!("Alignment of ", stringify!(List))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<List>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(List),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<List>())).length as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(List),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<List>())).head as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(List),
            "::",
            stringify!(head)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<List>())).tail as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(List),
            "::",
            stringify!(tail)
        )
    );
}
impl Default for List {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ListCell {
    pub data: ListCell__bindgen_ty_1,
    pub next: *mut ListCell,
}
#[repr(C)]
pub struct ListCell__bindgen_ty_1 {
    pub ptr_value: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub int_value: __BindgenUnionField<::std::os::raw::c_int>,
    pub oid_value: __BindgenUnionField<Oid>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout_ListCell__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<ListCell__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(ListCell__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<ListCell__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(ListCell__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ListCell__bindgen_ty_1>())).ptr_value as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ListCell__bindgen_ty_1),
            "::",
            stringify!(ptr_value)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ListCell__bindgen_ty_1>())).int_value as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ListCell__bindgen_ty_1),
            "::",
            stringify!(int_value)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ListCell__bindgen_ty_1>())).oid_value as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ListCell__bindgen_ty_1),
            "::",
            stringify!(oid_value)
        )
    );
}
impl Default for ListCell__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_ListCell() {
    assert_eq!(
        ::std::mem::size_of::<ListCell>(),
        16usize,
        concat!("Size of: ", stringify!(ListCell))
    );
    assert_eq!(
        ::std::mem::align_of::<ListCell>(),
        8usize,
        concat!("Alignment of ", stringify!(ListCell))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ListCell>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ListCell),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ListCell>())).next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ListCell),
            "::",
            stringify!(next)
        )
    );
}
impl Default for ListCell {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn list_nth(list: *const List, n: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void;
}
#[doc = " Alias"]
#[doc = "   specifies an alias for a range variable; the alias might also"]
#[doc = "   specify renaming of columns within the table."]
#[doc = ""]
#[doc = " Note: colnames is a list of Value nodes (always strings).  In Alias structs"]
#[doc = " associated with RTEs, there may be entries corresponding to dropped"]
#[doc = " columns; these are normally empty strings (\"\").  See parsenodes.h for info."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Alias {
    pub type_: NodeTag,
    pub aliasname: *mut ::std::os::raw::c_char,
    #[doc = " aliased rel name (never qualified)"]
    pub colnames: *mut List,
}
#[test]
fn bindgen_test_layout_Alias() {
    assert_eq!(
        ::std::mem::size_of::<Alias>(),
        24usize,
        concat!("Size of: ", stringify!(Alias))
    );
    assert_eq!(
        ::std::mem::align_of::<Alias>(),
        8usize,
        concat!("Alignment of ", stringify!(Alias))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Alias>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Alias),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Alias>())).aliasname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Alias),
            "::",
            stringify!(aliasname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Alias>())).colnames as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Alias),
            "::",
            stringify!(colnames)
        )
    );
}
impl Default for Alias {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " What to do at commit time for temporary relations"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OnCommitAction {
    ONCOMMIT_NOOP = 0,
    #[doc = " No ON COMMIT clause (do nothing)"]
    ONCOMMIT_PRESERVE_ROWS = 1,
    #[doc = " ON COMMIT PRESERVE ROWS (do nothing)"]
    ONCOMMIT_DELETE_ROWS = 2,
    #[doc = " ON COMMIT DELETE ROWS"]
    ONCOMMIT_DROP = 3,
}
#[doc = " RangeVar  range variable, used in FROM clauses"]
#[doc = ""]
#[doc = " Also used to represent table names in utility statements; there, the alias"]
#[doc = " field is not used, and inh tells whether to apply the operation"]
#[doc = " recursively to child tables.  In some contexts it is also useful to carry"]
#[doc = " a TEMP table indication here."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RangeVar {
    pub type_: NodeTag,
    pub catalogname: *mut ::std::os::raw::c_char,
    #[doc = " the catalog (database) name, or NULL"]
    pub schemaname: *mut ::std::os::raw::c_char,
    #[doc = " the schema name, or NULL"]
    pub relname: *mut ::std::os::raw::c_char,
    #[doc = " the relation/sequence name"]
    pub inh: bool,
    #[doc = " expand rel by inheritance? recursively act"]
    #[doc = " on children?"]
    pub relpersistence: ::std::os::raw::c_char,
    #[doc = " see RELPERSISTENCE_* in pg_class.h"]
    pub alias: *mut Alias,
    #[doc = " table alias & optional column aliases"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_RangeVar() {
    assert_eq!(
        ::std::mem::size_of::<RangeVar>(),
        56usize,
        concat!("Size of: ", stringify!(RangeVar))
    );
    assert_eq!(
        ::std::mem::align_of::<RangeVar>(),
        8usize,
        concat!("Alignment of ", stringify!(RangeVar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeVar>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeVar),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeVar>())).catalogname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeVar),
            "::",
            stringify!(catalogname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeVar>())).schemaname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeVar),
            "::",
            stringify!(schemaname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeVar>())).relname as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeVar),
            "::",
            stringify!(relname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeVar>())).inh as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeVar),
            "::",
            stringify!(inh)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeVar>())).relpersistence as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeVar),
            "::",
            stringify!(relpersistence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeVar>())).alias as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeVar),
            "::",
            stringify!(alias)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeVar>())).location as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeVar),
            "::",
            stringify!(location)
        )
    );
}
impl Default for RangeVar {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " TableFunc  node for a table function, such as XMLTABLE."]
#[doc = ""]
#[doc = " Entries in the ns_names list are either string Value nodes containing"]
#[doc = " literal namespace names, or NULL pointers to represent DEFAULT."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TableFunc {
    pub type_: NodeTag,
    pub ns_uris: *mut List,
    #[doc = " list of namespace URI expressions"]
    pub ns_names: *mut List,
    #[doc = " list of namespace names or NULL"]
    pub docexpr: *mut Node,
    #[doc = " input document expression"]
    pub rowexpr: *mut Node,
    #[doc = " row filter expression"]
    pub colnames: *mut List,
    #[doc = " column names (list of String)"]
    pub coltypes: *mut List,
    #[doc = " OID list of column type OIDs"]
    pub coltypmods: *mut List,
    #[doc = " integer list of column typmods"]
    pub colcollations: *mut List,
    #[doc = " OID list of column collation OIDs"]
    pub colexprs: *mut List,
    #[doc = " list of column filter expressions"]
    pub coldefexprs: *mut List,
    #[doc = " list of column default expressions"]
    pub notnulls: *mut Bitmapset,
    #[doc = " nullability flag for each output column"]
    pub ordinalitycol: ::std::os::raw::c_int,
    #[doc = " counts from 0; 1 if none specified"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_TableFunc() {
    assert_eq!(
        ::std::mem::size_of::<TableFunc>(),
        104usize,
        concat!("Size of: ", stringify!(TableFunc))
    );
    assert_eq!(
        ::std::mem::align_of::<TableFunc>(),
        8usize,
        concat!("Alignment of ", stringify!(TableFunc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).ns_uris as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(ns_uris)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).ns_names as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(ns_names)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).docexpr as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(docexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).rowexpr as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(rowexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).colnames as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(colnames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).coltypes as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(coltypes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).coltypmods as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(coltypmods)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).colcollations as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(colcollations)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).colexprs as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(colexprs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).coldefexprs as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(coldefexprs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).notnulls as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(notnulls)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).ordinalitycol as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(ordinalitycol)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableFunc>())).location as *const _ as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(TableFunc),
            "::",
            stringify!(location)
        )
    );
}
impl Default for TableFunc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " IntoClause  target information for SELECT INTO, CREATE TABLE AS, and"]
#[doc = " CREATE MATERIALIZED VIEW"]
#[doc = ""]
#[doc = " For CREATE MATERIALIZED VIEW, viewQuery is the parsedbutnotrewritten"]
#[doc = " SELECT Query for the view; otherwise it's NULL.  (Although it's actually"]
#[doc = " Query*, we declare it as Node* to avoid a forward reference.)"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct IntoClause {
    pub type_: NodeTag,
    pub rel: *mut RangeVar,
    #[doc = " target relation name"]
    pub colNames: *mut List,
    #[doc = " column names to assign, or NIL"]
    pub accessMethod: *mut ::std::os::raw::c_char,
    #[doc = " table access method"]
    pub options: *mut List,
    #[doc = " options from WITH clause"]
    pub onCommit: OnCommitAction,
    #[doc = " what do we do at COMMIT?"]
    pub tableSpaceName: *mut ::std::os::raw::c_char,
    #[doc = " table space to use, or NULL"]
    pub viewQuery: *mut Node,
    #[doc = " materialized view's SELECT query"]
    pub skipData: bool,
}
#[test]
fn bindgen_test_layout_IntoClause() {
    assert_eq!(
        ::std::mem::size_of::<IntoClause>(),
        72usize,
        concat!("Size of: ", stringify!(IntoClause))
    );
    assert_eq!(
        ::std::mem::align_of::<IntoClause>(),
        8usize,
        concat!("Alignment of ", stringify!(IntoClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IntoClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(IntoClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IntoClause>())).rel as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(IntoClause),
            "::",
            stringify!(rel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IntoClause>())).colNames as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(IntoClause),
            "::",
            stringify!(colNames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IntoClause>())).accessMethod as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(IntoClause),
            "::",
            stringify!(accessMethod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IntoClause>())).options as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(IntoClause),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IntoClause>())).onCommit as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(IntoClause),
            "::",
            stringify!(onCommit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IntoClause>())).tableSpaceName as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(IntoClause),
            "::",
            stringify!(tableSpaceName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IntoClause>())).viewQuery as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(IntoClause),
            "::",
            stringify!(viewQuery)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IntoClause>())).skipData as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(IntoClause),
            "::",
            stringify!(skipData)
        )
    );
}
impl Default for IntoClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Expr  generic superclass for executableexpression nodes"]
#[doc = ""]
#[doc = " All node types that are used in executable expression trees should derive"]
#[doc = " from Expr (that is, have Expr as their first field).  Since Expr only"]
#[doc = " contains NodeTag, this is a formality, but it is an easy form of"]
#[doc = " documentation.  See also the ExprState node types in execnodes.h."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Expr {
    pub type_: NodeTag,
}
#[test]
fn bindgen_test_layout_Expr() {
    assert_eq!(
        ::std::mem::size_of::<Expr>(),
        4usize,
        concat!("Size of: ", stringify!(Expr))
    );
    assert_eq!(
        ::std::mem::align_of::<Expr>(),
        4usize,
        concat!("Alignment of ", stringify!(Expr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Expr>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Expr),
            "::",
            stringify!(type_)
        )
    );
}
impl Default for Expr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Var {
    pub xpr: Expr,
    pub varno: Index,
    #[doc = " index of this var's relation in the range"]
    #[doc = " table, or INNER_VAR/OUTER_VAR/INDEX_VAR"]
    pub varattno: AttrNumber,
    #[doc = " attribute number of this var, or zero for"]
    #[doc = " all attrs (\"wholerow Var\")"]
    pub vartype: Oid,
    #[doc = " pg_type OID for the type of this var"]
    pub vartypmod: int32,
    #[doc = " pg_attribute typmod value"]
    pub varcollid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub varlevelsup: Index,
    #[doc = " for subquery variables referencing outer"]
    #[doc = " relations; 0 in a normal var, >0 means N"]
    #[doc = " levels up"]
    pub varnoold: Index,
    #[doc = " original value of varno, for debugging"]
    pub varoattno: AttrNumber,
    #[doc = " original value of varattno"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Var() {
    assert_eq!(
        ::std::mem::size_of::<Var>(),
        40usize,
        concat!("Size of: ", stringify!(Var))
    );
    assert_eq!(
        ::std::mem::align_of::<Var>(),
        4usize,
        concat!("Alignment of ", stringify!(Var))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Var>())).xpr as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Var), "::", stringify!(xpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Var>())).varno as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Var),
            "::",
            stringify!(varno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Var>())).varattno as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Var),
            "::",
            stringify!(varattno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Var>())).vartype as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Var),
            "::",
            stringify!(vartype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Var>())).vartypmod as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Var),
            "::",
            stringify!(vartypmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Var>())).varcollid as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Var),
            "::",
            stringify!(varcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Var>())).varlevelsup as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Var),
            "::",
            stringify!(varlevelsup)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Var>())).varnoold as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(Var),
            "::",
            stringify!(varnoold)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Var>())).varoattno as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Var),
            "::",
            stringify!(varoattno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Var>())).location as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Var),
            "::",
            stringify!(location)
        )
    );
}
impl Default for Var {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Const"]
#[doc = ""]
#[doc = " Note: for varlena data types, we make a rule that a Const node's value"]
#[doc = " must be in nonextended form (4byte header, no compression or external"]
#[doc = " references).  This ensures that the Const node is selfcontained and makes"]
#[doc = " it more likely that equal() will see logically identical values as equal."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Const {
    pub xpr: Expr,
    pub consttype: Oid,
    #[doc = " pg_type OID of the constant's datatype"]
    pub consttypmod: int32,
    #[doc = " typmod value, if any"]
    pub constcollid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub constlen: ::std::os::raw::c_int,
    #[doc = " typlen of the constant's datatype"]
    pub constvalue: Datum,
    #[doc = " the constant's value"]
    pub constisnull: bool,
    #[doc = " whether the constant is null (if true,"]
    #[doc = " constvalue is undefined)"]
    pub constbyval: bool,
    #[doc = " whether this datatype is passed by value."]
    #[doc = " If true, then all the information is stored"]
    #[doc = " in the Datum. If false, then the Datum"]
    #[doc = " contains a pointer to the information."]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Const() {
    assert_eq!(
        ::std::mem::size_of::<Const>(),
        40usize,
        concat!("Size of: ", stringify!(Const))
    );
    assert_eq!(
        ::std::mem::align_of::<Const>(),
        8usize,
        concat!("Alignment of ", stringify!(Const))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Const>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Const),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Const>())).consttype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Const),
            "::",
            stringify!(consttype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Const>())).consttypmod as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Const),
            "::",
            stringify!(consttypmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Const>())).constcollid as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Const),
            "::",
            stringify!(constcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Const>())).constlen as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Const),
            "::",
            stringify!(constlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Const>())).constvalue as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Const),
            "::",
            stringify!(constvalue)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Const>())).constisnull as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Const),
            "::",
            stringify!(constisnull)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Const>())).constbyval as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(Const),
            "::",
            stringify!(constbyval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Const>())).location as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Const),
            "::",
            stringify!(location)
        )
    );
}
impl Default for Const {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " Param"]
#[doc = ""]
#[doc = "    paramkind specifies the kind of parameter. The possible values"]
#[doc = "    for this field are:"]
#[doc = ""]
#[doc = "    PARAM_EXTERN:  The parameter value is supplied from outside the plan."]
#[doc = "          Such parameters are numbered from 1 to n."]
#[doc = ""]
#[doc = "    PARAM_EXEC:  The parameter is an internal executor parameter, used"]
#[doc = "          for passing values into and out of subqueries or from"]
#[doc = "          nestloop joins to their inner scans."]
#[doc = "          For historical reasons, such parameters are numbered from 0."]
#[doc = "          These numbers are independent of PARAM_EXTERN numbers."]
#[doc = ""]
#[doc = "    PARAM_SUBLINK: The parameter represents an output column of a SubLink"]
#[doc = "          node's subselect.  The column number is contained in the"]
#[doc = "          'paramid' field.  (This type of Param is converted to"]
#[doc = "          PARAM_EXEC during planning.)"]
#[doc = ""]
#[doc = "    PARAM_MULTIEXPR:  Like PARAM_SUBLINK, the parameter represents an"]
#[doc = "          output column of a SubLink node's subselect, but here, the"]
#[doc = "          SubLink is always a MULTIEXPR SubLink.  The highorder 16 bits"]
#[doc = "          of the 'paramid' field contain the SubLink's subLinkId, and"]
#[doc = "          the loworder 16 bits contain the column number.  (This type"]
#[doc = "          of Param is also converted to PARAM_EXEC during planning.)"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ParamKind {
    PARAM_EXTERN = 0,
    PARAM_EXEC = 1,
    PARAM_SUBLINK = 2,
    PARAM_MULTIEXPR = 3,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Param {
    pub xpr: Expr,
    pub paramkind: ParamKind,
    #[doc = " kind of parameter. See above"]
    pub paramid: ::std::os::raw::c_int,
    #[doc = " numeric ID for parameter"]
    pub paramtype: Oid,
    #[doc = " pg_type OID of parameter's datatype"]
    pub paramtypmod: int32,
    #[doc = " typmod value, if known"]
    pub paramcollid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Param() {
    assert_eq!(
        ::std::mem::size_of::<Param>(),
        28usize,
        concat!("Size of: ", stringify!(Param))
    );
    assert_eq!(
        ::std::mem::align_of::<Param>(),
        4usize,
        concat!("Alignment of ", stringify!(Param))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Param>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Param),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Param>())).paramkind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Param),
            "::",
            stringify!(paramkind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Param>())).paramid as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Param),
            "::",
            stringify!(paramid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Param>())).paramtype as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Param),
            "::",
            stringify!(paramtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Param>())).paramtypmod as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Param),
            "::",
            stringify!(paramtypmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Param>())).paramcollid as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Param),
            "::",
            stringify!(paramcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Param>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Param),
            "::",
            stringify!(location)
        )
    );
}
impl Default for Param {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Aggref"]
#[doc = ""]
#[doc = " The aggregate's args list is a targetlist, ie, a list of TargetEntry nodes."]
#[doc = ""]
#[doc = " For a normal (nonorderedset) aggregate, the nonresjunk TargetEntries"]
#[doc = " represent the aggregate's regular arguments (if any) and resjunk TLEs can"]
#[doc = " be added at the end to represent ORDER BY expressions that are not also"]
#[doc = " arguments.  As in a toplevel Query, the TLEs can be marked with"]
#[doc = " ressortgroupref indexes to let them be referenced by SortGroupClause"]
#[doc = " entries in the aggorder and/or aggdistinct lists.  This represents ORDER BY"]
#[doc = " and DISTINCT operations to be applied to the aggregate input rows before"]
#[doc = " they are passed to the transition function.  The grammar only allows a"]
#[doc = " simple \"DISTINCT\" specifier for the arguments, but we use the full"]
#[doc = " querylevel representation to allow more code sharing."]
#[doc = ""]
#[doc = " For an orderedset aggregate, the args list represents the WITHIN GROUP"]
#[doc = " (aggregated) arguments, all of which will be listed in the aggorder list."]
#[doc = " DISTINCT is not supported in this case, so aggdistinct will be NIL."]
#[doc = " The direct arguments appear in aggdirectargs (as a list of plain"]
#[doc = " expressions, not TargetEntry nodes)."]
#[doc = ""]
#[doc = " aggtranstype is the data type of the state transition values for this"]
#[doc = " aggregate (resolved to an actual type, if agg's transtype is polymorphic)."]
#[doc = " This is determined during planning and is InvalidOid before that."]
#[doc = ""]
#[doc = " aggargtypes is an OID list of the data types of the direct and regular"]
#[doc = " arguments.  Normally it's redundant with the aggdirectargs and args lists,"]
#[doc = " but in a combining aggregate, it's not because the args list has been"]
#[doc = " replaced with a single argument representing the partialaggregate"]
#[doc = " transition values."]
#[doc = ""]
#[doc = " aggsplit indicates the expected partialaggregation mode for the Aggref's"]
#[doc = " parent plan node.  It's always set to AGGSPLIT_SIMPLE in the parser, but"]
#[doc = " the planner might change it to something else.  We use this mainly as"]
#[doc = " a crosscheck that the Aggrefs match the plan; but note that when aggsplit"]
#[doc = " indicates a nonfinal mode, aggtype reflects the transition data type"]
#[doc = " not the SQLlevel output type of the aggregate."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Aggref {
    pub xpr: Expr,
    pub aggfnoid: Oid,
    #[doc = " pg_proc Oid of the aggregate"]
    pub aggtype: Oid,
    #[doc = " type Oid of result of the aggregate"]
    pub aggcollid: Oid,
    #[doc = " OID of collation of result"]
    pub inputcollid: Oid,
    #[doc = " OID of collation that function should use"]
    pub aggtranstype: Oid,
    #[doc = " type Oid of aggregate's transition value"]
    pub aggargtypes: *mut List,
    #[doc = " type Oids of direct and aggregated args"]
    pub aggdirectargs: *mut List,
    #[doc = " direct arguments, if an orderedset agg"]
    pub args: *mut List,
    #[doc = " aggregated arguments and sort expressions"]
    pub aggorder: *mut List,
    #[doc = " ORDER BY (list of SortGroupClause)"]
    pub aggdistinct: *mut List,
    #[doc = " DISTINCT (list of SortGroupClause)"]
    pub aggfilter: *mut Expr,
    #[doc = " FILTER expression, if any"]
    pub aggstar: bool,
    #[doc = " true if argument list was really '*'"]
    pub aggvariadic: bool,
    #[doc = " true if variadic arguments have been"]
    #[doc = " combined into an array last argument"]
    pub aggkind: ::std::os::raw::c_char,
    #[doc = " aggregate kind (see pg_aggregate.h)"]
    pub agglevelsup: Index,
    #[doc = " > 0 if agg belongs to outer query"]
    pub aggsplit: AggSplit,
    #[doc = " expected aggsplitting mode of parent Agg"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Aggref() {
    assert_eq!(
        ::std::mem::size_of::<Aggref>(),
        88usize,
        concat!("Size of: ", stringify!(Aggref))
    );
    assert_eq!(
        ::std::mem::align_of::<Aggref>(),
        8usize,
        concat!("Alignment of ", stringify!(Aggref))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggfnoid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggfnoid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggtype as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggcollid as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).inputcollid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(inputcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggtranstype as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggtranstype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggargtypes as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggargtypes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggdirectargs as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggdirectargs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).args as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggorder as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggorder)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggdistinct as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggdistinct)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggfilter as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggfilter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggstar as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggstar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggvariadic as *const _ as usize },
        73usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggvariadic)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggkind as *const _ as usize },
        74usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggkind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).agglevelsup as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(agglevelsup)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).aggsplit as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(aggsplit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Aggref>())).location as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(Aggref),
            "::",
            stringify!(location)
        )
    );
}
impl Default for Aggref {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " GroupingFunc"]
#[doc = ""]
#[doc = " A GroupingFunc is a GROUPING(...) expression, which behaves in many ways"]
#[doc = " like an aggregate function (e.g. it \"belongs\" to a specific query level,"]
#[doc = " which might not be the one immediately containing it), but also differs in"]
#[doc = " an important respect: it never evaluates its arguments, they merely"]
#[doc = " designate expressions from the GROUP BY clause of the query level to which"]
#[doc = " it belongs."]
#[doc = ""]
#[doc = " The spec defines the evaluation of GROUPING() purely by syntactic"]
#[doc = " replacement, but we make it a real expression for optimization purposes so"]
#[doc = " that one Agg node can handle multiple grouping sets at once.  Evaluating the"]
#[doc = " result only needs the column positions to check against the grouping set"]
#[doc = " being projected.  However, for EXPLAIN to produce meaningful output, we have"]
#[doc = " to keep the original expressions around, since expression deparse does not"]
#[doc = " give us any feasible way to get at the GROUP BY clause."]
#[doc = ""]
#[doc = " Also, we treat two GroupingFunc nodes as equal if they have equal arguments"]
#[doc = " lists and agglevelsup, without comparing the refs and cols annotations."]
#[doc = ""]
#[doc = " In raw parse output we have only the args list; parse analysis fills in the"]
#[doc = " refs list, and the planner fills in the cols list."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GroupingFunc {
    pub xpr: Expr,
    pub args: *mut List,
    #[doc = " arguments, not evaluated but kept for"]
    #[doc = " benefit of EXPLAIN etc."]
    pub refs: *mut List,
    #[doc = " ressortgrouprefs of arguments"]
    pub cols: *mut List,
    #[doc = " actual column positions set by planner"]
    pub agglevelsup: Index,
    #[doc = " same as Aggref.agglevelsup"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_GroupingFunc() {
    assert_eq!(
        ::std::mem::size_of::<GroupingFunc>(),
        40usize,
        concat!("Size of: ", stringify!(GroupingFunc))
    );
    assert_eq!(
        ::std::mem::align_of::<GroupingFunc>(),
        8usize,
        concat!("Alignment of ", stringify!(GroupingFunc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GroupingFunc>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GroupingFunc),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GroupingFunc>())).args as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GroupingFunc),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GroupingFunc>())).refs as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GroupingFunc),
            "::",
            stringify!(refs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GroupingFunc>())).cols as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(GroupingFunc),
            "::",
            stringify!(cols)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GroupingFunc>())).agglevelsup as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(GroupingFunc),
            "::",
            stringify!(agglevelsup)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GroupingFunc>())).location as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(GroupingFunc),
            "::",
            stringify!(location)
        )
    );
}
impl Default for GroupingFunc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " WindowFunc"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct WindowFunc {
    pub xpr: Expr,
    pub winfnoid: Oid,
    #[doc = " pg_proc Oid of the function"]
    pub wintype: Oid,
    #[doc = " type Oid of result of the window function"]
    pub wincollid: Oid,
    #[doc = " OID of collation of result"]
    pub inputcollid: Oid,
    #[doc = " OID of collation that function should use"]
    pub args: *mut List,
    #[doc = " arguments to the window function"]
    pub aggfilter: *mut Expr,
    #[doc = " FILTER expression, if any"]
    pub winref: Index,
    #[doc = " index of associated WindowClause"]
    pub winstar: bool,
    #[doc = " true if argument list was really '*'"]
    pub winagg: bool,
    #[doc = " is function a simple aggregate?"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_WindowFunc() {
    assert_eq!(
        ::std::mem::size_of::<WindowFunc>(),
        56usize,
        concat!("Size of: ", stringify!(WindowFunc))
    );
    assert_eq!(
        ::std::mem::align_of::<WindowFunc>(),
        8usize,
        concat!("Alignment of ", stringify!(WindowFunc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).winfnoid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(winfnoid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).wintype as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(wintype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).wincollid as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(wincollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).inputcollid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(inputcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).args as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).aggfilter as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(aggfilter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).winref as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(winref)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).winstar as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(winstar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).winagg as *const _ as usize },
        45usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(winagg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowFunc>())).location as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowFunc),
            "::",
            stringify!(location)
        )
    );
}
impl Default for WindowFunc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " SubscriptingRef: describes a subscripting operation over a container"]
#[doc = "       (array, etc)."]
#[doc = ""]
#[doc = " A SubscriptingRef can describe fetching a single element from a container,"]
#[doc = " fetching a part of container (e.g. array slice), storing a single element into"]
#[doc = " a container, or storing a slice.  The \"store\" cases work with an"]
#[doc = " initial container value and a source value that is inserted into the"]
#[doc = " appropriate part of the container; the result of the operation is an"]
#[doc = " entire new modified container value."]
#[doc = ""]
#[doc = " If reflowerindexpr = NIL, then we are fetching or storing a single container"]
#[doc = " element at the subscripts given by refupperindexpr. Otherwise we are"]
#[doc = " fetching or storing a container slice, that is a rectangular subcontainer"]
#[doc = " with lower and upper bounds given by the index expressions."]
#[doc = " reflowerindexpr must be the same length as refupperindexpr when it"]
#[doc = " is not NIL."]
#[doc = ""]
#[doc = " In the slice case, individual expressions in the subscript lists can be"]
#[doc = " NULL, meaning \"substitute the array's current lower or upper bound\"."]
#[doc = ""]
#[doc = " Note: the result datatype is the element type when fetching a single"]
#[doc = " element; but it is the array type when doing subarray fetch or either"]
#[doc = " type of store."]
#[doc = ""]
#[doc = " Note: for the cases where a container is returned, if refexpr yields a R/W"]
#[doc = " expanded container, then the implementation is allowed to modify that object"]
#[doc = " inplace and return the same object.)"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SubscriptingRef {
    pub xpr: Expr,
    pub refcontainertype: Oid,
    #[doc = " type of the container proper"]
    pub refelemtype: Oid,
    #[doc = " type of the container elements"]
    pub reftypmod: int32,
    #[doc = " typmod of the container (and elements too)"]
    pub refcollid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub refupperindexpr: *mut List,
    #[doc = " expressions that evaluate to upper"]
    #[doc = " container indexes"]
    pub reflowerindexpr: *mut List,
    #[doc = " expressions that evaluate to lower"]
    #[doc = " container indexes, or NIL for single"]
    #[doc = " container element"]
    pub refexpr: *mut Expr,
    #[doc = " the expression that evaluates to a"]
    #[doc = " container value"]
    pub refassgnexpr: *mut Expr,
}
#[test]
fn bindgen_test_layout_SubscriptingRef() {
    assert_eq!(
        ::std::mem::size_of::<SubscriptingRef>(),
        56usize,
        concat!("Size of: ", stringify!(SubscriptingRef))
    );
    assert_eq!(
        ::std::mem::align_of::<SubscriptingRef>(),
        8usize,
        concat!("Alignment of ", stringify!(SubscriptingRef))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubscriptingRef>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SubscriptingRef),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SubscriptingRef>())).refcontainertype as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SubscriptingRef),
            "::",
            stringify!(refcontainertype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubscriptingRef>())).refelemtype as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SubscriptingRef),
            "::",
            stringify!(refelemtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubscriptingRef>())).reftypmod as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SubscriptingRef),
            "::",
            stringify!(reftypmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubscriptingRef>())).refcollid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SubscriptingRef),
            "::",
            stringify!(refcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubscriptingRef>())).refupperindexpr as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SubscriptingRef),
            "::",
            stringify!(refupperindexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubscriptingRef>())).reflowerindexpr as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SubscriptingRef),
            "::",
            stringify!(reflowerindexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubscriptingRef>())).refexpr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SubscriptingRef),
            "::",
            stringify!(refexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubscriptingRef>())).refassgnexpr as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SubscriptingRef),
            "::",
            stringify!(refassgnexpr)
        )
    );
}
impl Default for SubscriptingRef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " CoercionContext  distinguishes the allowed set of type casts"]
#[doc = ""]
#[doc = " NB: ordering of the alternatives is significant; later (larger) values"]
#[doc = " allow more casts than earlier ones."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CoercionContext {
    COERCION_IMPLICIT = 0,
    #[doc = " coercion in context of expression"]
    COERCION_ASSIGNMENT = 1,
    #[doc = " coercion in context of assignment"]
    COERCION_EXPLICIT = 2,
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " CoercionForm  how to display a node that could have come from a cast"]
#[doc = ""]
#[doc = " NB: equal() ignores CoercionForm fields, therefore this *must* not carry"]
#[doc = " any semantically significant information.  We need that behavior so that"]
#[doc = " the planner will consider equivalent implicit and explicit casts to be"]
#[doc = " equivalent.  In cases where those actually behave differently, the coercion"]
#[doc = " function's arguments will be different."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CoercionForm {
    COERCE_EXPLICIT_CALL = 0,
    #[doc = " display as a function call"]
    COERCE_EXPLICIT_CAST = 1,
    #[doc = " display as an explicit cast"]
    COERCE_IMPLICIT_CAST = 2,
}
#[doc = " FuncExpr  expression node for a function call"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FuncExpr {
    pub xpr: Expr,
    pub funcid: Oid,
    #[doc = " PG_PROC OID of the function"]
    pub funcresulttype: Oid,
    #[doc = " PG_TYPE OID of result value"]
    pub funcretset: bool,
    #[doc = " true if function returns set"]
    pub funcvariadic: bool,
    #[doc = " true if variadic arguments have been"]
    #[doc = " combined into an array last argument"]
    pub funcformat: CoercionForm,
    #[doc = " how to display this function call"]
    pub funccollid: Oid,
    #[doc = " OID of collation of result"]
    pub inputcollid: Oid,
    #[doc = " OID of collation that function should use"]
    pub args: *mut List,
    #[doc = " arguments to the function"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_FuncExpr() {
    assert_eq!(
        ::std::mem::size_of::<FuncExpr>(),
        48usize,
        concat!("Size of: ", stringify!(FuncExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<FuncExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(FuncExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncExpr>())).funcid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncExpr),
            "::",
            stringify!(funcid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncExpr>())).funcresulttype as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncExpr),
            "::",
            stringify!(funcresulttype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncExpr>())).funcretset as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncExpr),
            "::",
            stringify!(funcretset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncExpr>())).funcvariadic as *const _ as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncExpr),
            "::",
            stringify!(funcvariadic)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncExpr>())).funcformat as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncExpr),
            "::",
            stringify!(funcformat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncExpr>())).funccollid as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncExpr),
            "::",
            stringify!(funccollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncExpr>())).inputcollid as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncExpr),
            "::",
            stringify!(inputcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncExpr>())).args as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncExpr),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncExpr>())).location as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for FuncExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " NamedArgExpr  a named argument of a function"]
#[doc = ""]
#[doc = " This node type can only appear in the args list of a FuncCall or FuncExpr"]
#[doc = " node.  We support pure positional call notation (no named arguments),"]
#[doc = " named notation (all arguments are named), and mixed notation (unnamed"]
#[doc = " arguments followed by named ones)."]
#[doc = ""]
#[doc = " Parse analysis sets argnumber to the positional index of the argument,"]
#[doc = " but doesn't rearrange the argument list."]
#[doc = ""]
#[doc = " The planner will convert argument lists to pure positional notation"]
#[doc = " during expression preprocessing, so execution never sees a NamedArgExpr."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NamedArgExpr {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " the argument expression"]
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " the name"]
    pub argnumber: ::std::os::raw::c_int,
    #[doc = " argument's number in positional notation"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NamedArgExpr() {
    assert_eq!(
        ::std::mem::size_of::<NamedArgExpr>(),
        32usize,
        concat!("Size of: ", stringify!(NamedArgExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<NamedArgExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(NamedArgExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NamedArgExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NamedArgExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NamedArgExpr>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(NamedArgExpr),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NamedArgExpr>())).name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(NamedArgExpr),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NamedArgExpr>())).argnumber as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(NamedArgExpr),
            "::",
            stringify!(argnumber)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NamedArgExpr>())).location as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(NamedArgExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for NamedArgExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " OpExpr  expression node for an operator invocation"]
#[doc = ""]
#[doc = " Semantically, this is essentially the same as a function call."]
#[doc = ""]
#[doc = " Note that opfuncid is not necessarily filled in immediately on creation"]
#[doc = " of the node.  The planner makes sure it is valid before passing the node"]
#[doc = " tree to the executor, but during parsing/planning opfuncid can be 0."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct OpExpr {
    pub xpr: Expr,
    pub opno: Oid,
    #[doc = " PG_OPERATOR OID of the operator"]
    pub opfuncid: Oid,
    #[doc = " PG_PROC OID of underlying function"]
    pub opresulttype: Oid,
    #[doc = " PG_TYPE OID of result value"]
    pub opretset: bool,
    #[doc = " true if operator returns set"]
    pub opcollid: Oid,
    #[doc = " OID of collation of result"]
    pub inputcollid: Oid,
    #[doc = " OID of collation that operator should use"]
    pub args: *mut List,
    #[doc = " arguments to the operator (1 or 2)"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_OpExpr() {
    assert_eq!(
        ::std::mem::size_of::<OpExpr>(),
        48usize,
        concat!("Size of: ", stringify!(OpExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<OpExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(OpExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OpExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(OpExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OpExpr>())).opno as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(OpExpr),
            "::",
            stringify!(opno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OpExpr>())).opfuncid as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(OpExpr),
            "::",
            stringify!(opfuncid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OpExpr>())).opresulttype as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(OpExpr),
            "::",
            stringify!(opresulttype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OpExpr>())).opretset as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(OpExpr),
            "::",
            stringify!(opretset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OpExpr>())).opcollid as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(OpExpr),
            "::",
            stringify!(opcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OpExpr>())).inputcollid as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(OpExpr),
            "::",
            stringify!(inputcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OpExpr>())).args as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(OpExpr),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OpExpr>())).location as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(OpExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for OpExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " ScalarArrayOpExpr  expression node for \"scalar op ANY/ALL (array)\""]
#[doc = ""]
#[doc = " The operator must yield boolean.  It is applied to the left operand"]
#[doc = " and each element of the righthand array, and the results are combined"]
#[doc = " with OR or AND (for ANY or ALL respectively).  The node representation"]
#[doc = " is almost the same as for the underlying operator, but we need a useOr"]
#[doc = " flag to remember whether it's ANY or ALL, and we don't have to store"]
#[doc = " the result type (or the collation) because it must be boolean."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ScalarArrayOpExpr {
    pub xpr: Expr,
    pub opno: Oid,
    #[doc = " PG_OPERATOR OID of the operator"]
    pub opfuncid: Oid,
    #[doc = " PG_PROC OID of underlying function"]
    pub useOr: bool,
    #[doc = " true for ANY, false for ALL"]
    pub inputcollid: Oid,
    #[doc = " OID of collation that operator should use"]
    pub args: *mut List,
    #[doc = " the scalar and array operands"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ScalarArrayOpExpr() {
    assert_eq!(
        ::std::mem::size_of::<ScalarArrayOpExpr>(),
        40usize,
        concat!("Size of: ", stringify!(ScalarArrayOpExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<ScalarArrayOpExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(ScalarArrayOpExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ScalarArrayOpExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ScalarArrayOpExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ScalarArrayOpExpr>())).opno as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ScalarArrayOpExpr),
            "::",
            stringify!(opno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ScalarArrayOpExpr>())).opfuncid as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ScalarArrayOpExpr),
            "::",
            stringify!(opfuncid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ScalarArrayOpExpr>())).useOr as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ScalarArrayOpExpr),
            "::",
            stringify!(useOr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ScalarArrayOpExpr>())).inputcollid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ScalarArrayOpExpr),
            "::",
            stringify!(inputcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ScalarArrayOpExpr>())).args as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ScalarArrayOpExpr),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ScalarArrayOpExpr>())).location as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ScalarArrayOpExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for ScalarArrayOpExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " BoolExpr  expression node for the basic Boolean operators AND, OR, NOT"]
#[doc = ""]
#[doc = " Notice the arguments are given as a List.  For NOT, of course the list"]
#[doc = " must always have exactly one element.  For AND and OR, there can be two"]
#[doc = " or more arguments."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoolExprType {
    AND_EXPR = 0,
    OR_EXPR = 1,
    NOT_EXPR = 2,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct BoolExpr {
    pub xpr: Expr,
    pub boolop: BoolExprType,
    pub args: *mut List,
    #[doc = " arguments to this expression"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_BoolExpr() {
    assert_eq!(
        ::std::mem::size_of::<BoolExpr>(),
        24usize,
        concat!("Size of: ", stringify!(BoolExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<BoolExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(BoolExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BoolExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BoolExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BoolExpr>())).boolop as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(BoolExpr),
            "::",
            stringify!(boolop)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BoolExpr>())).args as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(BoolExpr),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BoolExpr>())).location as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(BoolExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for BoolExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " SubLink"]
#[doc = ""]
#[doc = " A SubLink represents a subselect appearing in an expression, and in some"]
#[doc = " cases also the combining operator(s) just above it.  The subLinkType"]
#[doc = " indicates the form of the expression represented:"]
#[doc = " EXISTS_SUBLINK    EXISTS(SELECT ...)"]
#[doc = " ALL_SUBLINK       (lefthand) op ALL (SELECT ...)"]
#[doc = " ANY_SUBLINK       (lefthand) op ANY (SELECT ...)"]
#[doc = " ROWCOMPARE_SUBLINK   (lefthand) op (SELECT ...)"]
#[doc = " EXPR_SUBLINK      (SELECT with single targetlist item ...)"]
#[doc = " MULTIEXPR_SUBLINK (SELECT with multiple targetlist items ...)"]
#[doc = " ARRAY_SUBLINK     ARRAY(SELECT with single targetlist item ...)"]
#[doc = " CTE_SUBLINK       WITH query (never actually part of an expression)"]
#[doc = " For ALL, ANY, and ROWCOMPARE, the lefthand is a list of expressions of the"]
#[doc = " same length as the subselect's targetlist.  ROWCOMPARE will *always* have"]
#[doc = " a list with more than one entry; if the subselect has just one target"]
#[doc = " then the parser will create an EXPR_SUBLINK instead (and any operator"]
#[doc = " above the subselect will be represented separately)."]
#[doc = " ROWCOMPARE, EXPR, and MULTIEXPR require the subselect to deliver at most"]
#[doc = " one row (if it returns no rows, the result is NULL)."]
#[doc = " ALL, ANY, and ROWCOMPARE require the combining operators to deliver boolean"]
#[doc = " results.  ALL and ANY combine the perrow results using AND and OR"]
#[doc = " semantics respectively."]
#[doc = " ARRAY requires just one target column, and creates an array of the target"]
#[doc = " column's type using any number of rows resulting from the subselect."]
#[doc = ""]
#[doc = " SubLink is classed as an Expr node, but it is not actually executable;"]
#[doc = " it must be replaced in the expression tree by a SubPlan node during"]
#[doc = " planning."]
#[doc = ""]
#[doc = " NOTE: in the raw output of gram.y, testexpr contains just the raw form"]
#[doc = " of the lefthand expression (if any), and operName is the String name of"]
#[doc = " the combining operator.  Also, subselect is a raw parsetree.  During parse"]
#[doc = " analysis, the parser transforms testexpr into a complete boolean expression"]
#[doc = " that compares the lefthand value(s) to PARAM_SUBLINK nodes representing the"]
#[doc = " output columns of the subselect.  And subselect is transformed to a Query."]
#[doc = " This is the representation seen in saved rules and in the rewriter."]
#[doc = ""]
#[doc = " In EXISTS, EXPR, MULTIEXPR, and ARRAY SubLinks, testexpr and operName"]
#[doc = " are unused and are always null."]
#[doc = ""]
#[doc = " subLinkId is currently used only for MULTIEXPR SubLinks, and is zero in"]
#[doc = " other SubLinks.  This number identifies different multipleassignment"]
#[doc = " subqueries within an UPDATE statement's SET list.  It is unique only"]
#[doc = " within a particular targetlist.  The output column(s) of the MULTIEXPR"]
#[doc = " are referenced by PARAM_MULTIEXPR Params appearing elsewhere in the tlist."]
#[doc = ""]
#[doc = " The CTE_SUBLINK case never occurs in actual SubLink nodes, but it is used"]
#[doc = " in SubPlans generated for WITH subqueries."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SubLinkType {
    EXISTS_SUBLINK = 0,
    ALL_SUBLINK = 1,
    ANY_SUBLINK = 2,
    ROWCOMPARE_SUBLINK = 3,
    EXPR_SUBLINK = 4,
    MULTIEXPR_SUBLINK = 5,
    ARRAY_SUBLINK = 6,
    CTE_SUBLINK = 7,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SubLink {
    pub xpr: Expr,
    pub subLinkType: SubLinkType,
    #[doc = " see above"]
    pub subLinkId: ::std::os::raw::c_int,
    #[doc = " ID (1..n); 0 if not MULTIEXPR"]
    pub testexpr: *mut Node,
    #[doc = " outerquery test for ALL/ANY/ROWCOMPARE"]
    pub operName: *mut List,
    #[doc = " originally specified operator name"]
    pub subselect: *mut Node,
    #[doc = " subselect as Query* or raw parsetree"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_SubLink() {
    assert_eq!(
        ::std::mem::size_of::<SubLink>(),
        48usize,
        concat!("Size of: ", stringify!(SubLink))
    );
    assert_eq!(
        ::std::mem::align_of::<SubLink>(),
        8usize,
        concat!("Alignment of ", stringify!(SubLink))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubLink>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SubLink),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubLink>())).subLinkType as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SubLink),
            "::",
            stringify!(subLinkType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubLink>())).subLinkId as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SubLink),
            "::",
            stringify!(subLinkId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubLink>())).testexpr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SubLink),
            "::",
            stringify!(testexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubLink>())).operName as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SubLink),
            "::",
            stringify!(operName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubLink>())).subselect as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SubLink),
            "::",
            stringify!(subselect)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SubLink>())).location as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SubLink),
            "::",
            stringify!(location)
        )
    );
}
impl Default for SubLink {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " AlternativeSubPlan  expression node for a choice among SubPlans"]
#[doc = ""]
#[doc = " The subplans are given as a List so that the node definition need not"]
#[doc = " change if there's ever more than two alternatives.  For the moment,"]
#[doc = " though, there are always exactly two; and the first one is the faststart"]
#[doc = " plan."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlternativeSubPlan {
    pub xpr: Expr,
    pub subplans: *mut List,
}
#[test]
fn bindgen_test_layout_AlternativeSubPlan() {
    assert_eq!(
        ::std::mem::size_of::<AlternativeSubPlan>(),
        16usize,
        concat!("Size of: ", stringify!(AlternativeSubPlan))
    );
    assert_eq!(
        ::std::mem::align_of::<AlternativeSubPlan>(),
        8usize,
        concat!("Alignment of ", stringify!(AlternativeSubPlan))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlternativeSubPlan>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlternativeSubPlan),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlternativeSubPlan>())).subplans as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlternativeSubPlan),
            "::",
            stringify!(subplans)
        )
    );
}
impl Default for AlternativeSubPlan {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " FieldSelect"]
#[doc = ""]
#[doc = " FieldSelect represents the operation of extracting one field from a tuple"]
#[doc = " value.  At runtime, the input expression is expected to yield a rowtype"]
#[doc = " Datum.  The specified field number is extracted and returned as a Datum."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FieldSelect {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " input expression"]
    pub fieldnum: AttrNumber,
    #[doc = " attribute number of field to extract"]
    pub resulttype: Oid,
    #[doc = " type of the field (result type of this"]
    #[doc = " node)"]
    pub resulttypmod: int32,
    #[doc = " output typmod (usually 1)"]
    pub resultcollid: Oid,
}
#[test]
fn bindgen_test_layout_FieldSelect() {
    assert_eq!(
        ::std::mem::size_of::<FieldSelect>(),
        32usize,
        concat!("Size of: ", stringify!(FieldSelect))
    );
    assert_eq!(
        ::std::mem::align_of::<FieldSelect>(),
        8usize,
        concat!("Alignment of ", stringify!(FieldSelect))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldSelect>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldSelect),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldSelect>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldSelect),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldSelect>())).fieldnum as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldSelect),
            "::",
            stringify!(fieldnum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldSelect>())).resulttype as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldSelect),
            "::",
            stringify!(resulttype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldSelect>())).resulttypmod as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldSelect),
            "::",
            stringify!(resulttypmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldSelect>())).resultcollid as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldSelect),
            "::",
            stringify!(resultcollid)
        )
    );
}
impl Default for FieldSelect {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " FieldStore"]
#[doc = ""]
#[doc = " FieldStore represents the operation of modifying one field in a tuple"]
#[doc = " value, yielding a new tuple value (the input is not touched!).  Like"]
#[doc = " the assign case of SubscriptingRef, this is used to implement UPDATE of a"]
#[doc = " portion of a column."]
#[doc = ""]
#[doc = " resulttype is always a named composite type (not a domain).  To update"]
#[doc = " a composite domain value, apply CoerceToDomain to the FieldStore."]
#[doc = ""]
#[doc = " A single FieldStore can actually represent updates of several different"]
#[doc = " fields.  The parser only generates FieldStores with singleelement lists,"]
#[doc = " but the planner will collapse multiple updates of the same base column"]
#[doc = " into one FieldStore."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FieldStore {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " input tuple value"]
    pub newvals: *mut List,
    #[doc = " new value(s) for field(s)"]
    pub fieldnums: *mut List,
    #[doc = " integer list of field attnums"]
    pub resulttype: Oid,
}
#[test]
fn bindgen_test_layout_FieldStore() {
    assert_eq!(
        ::std::mem::size_of::<FieldStore>(),
        40usize,
        concat!("Size of: ", stringify!(FieldStore))
    );
    assert_eq!(
        ::std::mem::align_of::<FieldStore>(),
        8usize,
        concat!("Alignment of ", stringify!(FieldStore))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldStore>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldStore),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldStore>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldStore),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldStore>())).newvals as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldStore),
            "::",
            stringify!(newvals)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldStore>())).fieldnums as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldStore),
            "::",
            stringify!(fieldnums)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FieldStore>())).resulttype as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(FieldStore),
            "::",
            stringify!(resulttype)
        )
    );
}
impl Default for FieldStore {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " RelabelType"]
#[doc = ""]
#[doc = " RelabelType represents a \"dummy\" type coercion between two binary"]
#[doc = " compatible datatypes, such as reinterpreting the result of an OID"]
#[doc = " expression as an int4.  It is a noop at runtime; we only need it"]
#[doc = " to provide a place to store the correct type to be attributed to"]
#[doc = " the expression result during type resolution.  (We can't get away"]
#[doc = " with just overwriting the type field of the input expression node,"]
#[doc = " so we need a separate node to show the coercion's result type.)"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RelabelType {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " input expression"]
    pub resulttype: Oid,
    #[doc = " output type of coercion expression"]
    pub resulttypmod: int32,
    #[doc = " output typmod (usually 1)"]
    pub resultcollid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub relabelformat: CoercionForm,
    #[doc = " how to display this node"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_RelabelType() {
    assert_eq!(
        ::std::mem::size_of::<RelabelType>(),
        40usize,
        concat!("Size of: ", stringify!(RelabelType))
    );
    assert_eq!(
        ::std::mem::align_of::<RelabelType>(),
        8usize,
        concat!("Alignment of ", stringify!(RelabelType))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RelabelType>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RelabelType),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RelabelType>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RelabelType),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RelabelType>())).resulttype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RelabelType),
            "::",
            stringify!(resulttype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RelabelType>())).resulttypmod as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(RelabelType),
            "::",
            stringify!(resulttypmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RelabelType>())).resultcollid as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RelabelType),
            "::",
            stringify!(resultcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RelabelType>())).relabelformat as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(RelabelType),
            "::",
            stringify!(relabelformat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RelabelType>())).location as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RelabelType),
            "::",
            stringify!(location)
        )
    );
}
impl Default for RelabelType {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " CoerceViaIO"]
#[doc = ""]
#[doc = " CoerceViaIO represents a type coercion between two types whose textual"]
#[doc = " representations are compatible, implemented by invoking the source type's"]
#[doc = " typoutput function then the destination type's typinput function."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CoerceViaIO {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " input expression"]
    pub resulttype: Oid,
    #[doc = " output type of coercion */"]
    pub resultcollid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub coerceformat: CoercionForm,
    #[doc = " how to display this node"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CoerceViaIO() {
    assert_eq!(
        ::std::mem::size_of::<CoerceViaIO>(),
        32usize,
        concat!("Size of: ", stringify!(CoerceViaIO))
    );
    assert_eq!(
        ::std::mem::align_of::<CoerceViaIO>(),
        8usize,
        concat!("Alignment of ", stringify!(CoerceViaIO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceViaIO>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceViaIO),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceViaIO>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceViaIO),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceViaIO>())).resulttype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceViaIO),
            "::",
            stringify!(resulttype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceViaIO>())).resultcollid as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceViaIO),
            "::",
            stringify!(resultcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceViaIO>())).coerceformat as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceViaIO),
            "::",
            stringify!(coerceformat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceViaIO>())).location as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceViaIO),
            "::",
            stringify!(location)
        )
    );
}
impl Default for CoerceViaIO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " ArrayCoerceExpr"]
#[doc = ""]
#[doc = " ArrayCoerceExpr represents a type coercion from one array type to another,"]
#[doc = " which is implemented by applying the perelement coercion expression"]
#[doc = " \"elemexpr\" to each element of the source array.  Within elemexpr, the"]
#[doc = " source element is represented by a CaseTestExpr node.  Note that even if"]
#[doc = " elemexpr is a noop (that is, just CaseTestExpr + RelabelType), the"]
#[doc = " coercion still requires some effort: we have to fix the element type OID"]
#[doc = " stored in the array header."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ArrayCoerceExpr {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " input expression (yields an array)"]
    pub elemexpr: *mut Expr,
    #[doc = " expression representing perelement work"]
    pub resulttype: Oid,
    #[doc = " output type of coercion (an array type)"]
    pub resulttypmod: int32,
    #[doc = " output typmod (also element typmod)"]
    pub resultcollid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub coerceformat: CoercionForm,
    #[doc = " how to display this node"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ArrayCoerceExpr() {
    assert_eq!(
        ::std::mem::size_of::<ArrayCoerceExpr>(),
        48usize,
        concat!("Size of: ", stringify!(ArrayCoerceExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<ArrayCoerceExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(ArrayCoerceExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayCoerceExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayCoerceExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayCoerceExpr>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayCoerceExpr),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayCoerceExpr>())).elemexpr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayCoerceExpr),
            "::",
            stringify!(elemexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayCoerceExpr>())).resulttype as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayCoerceExpr),
            "::",
            stringify!(resulttype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayCoerceExpr>())).resulttypmod as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayCoerceExpr),
            "::",
            stringify!(resulttypmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayCoerceExpr>())).resultcollid as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayCoerceExpr),
            "::",
            stringify!(resultcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayCoerceExpr>())).coerceformat as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayCoerceExpr),
            "::",
            stringify!(coerceformat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayCoerceExpr>())).location as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayCoerceExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for ArrayCoerceExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " ConvertRowtypeExpr"]
#[doc = ""]
#[doc = " ConvertRowtypeExpr represents a type coercion from one composite type"]
#[doc = " to another, where the source type is guaranteed to contain all the columns"]
#[doc = " needed for the destination type plus possibly others; the columns need not"]
#[doc = " be in the same positions, but are matched up by name.  This is primarily"]
#[doc = " used to convert a wholerow value of an inheritance child table into a"]
#[doc = " valid wholerow value of its parent table's rowtype.  Both resulttype"]
#[doc = " and the exposed type of \"arg\" must be named composite types (not domains)."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ConvertRowtypeExpr {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " input expression"]
    pub resulttype: Oid,
    #[doc = " output type (always a composite type) */"]
    pub convertformat: CoercionForm,
    #[doc = " how to display this node"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ConvertRowtypeExpr() {
    assert_eq!(
        ::std::mem::size_of::<ConvertRowtypeExpr>(),
        32usize,
        concat!("Size of: ", stringify!(ConvertRowtypeExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<ConvertRowtypeExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(ConvertRowtypeExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ConvertRowtypeExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ConvertRowtypeExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ConvertRowtypeExpr>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ConvertRowtypeExpr),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ConvertRowtypeExpr>())).resulttype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ConvertRowtypeExpr),
            "::",
            stringify!(resulttype)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ConvertRowtypeExpr>())).convertformat as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(ConvertRowtypeExpr),
            "::",
            stringify!(convertformat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ConvertRowtypeExpr>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ConvertRowtypeExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for ConvertRowtypeExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " CollateExpr  COLLATE"]
#[doc = ""]
#[doc = " The planner replaces CollateExpr with RelabelType during expression"]
#[doc = " preprocessing, so execution never sees a CollateExpr."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CollateExpr {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " input expression"]
    pub collOid: Oid,
    #[doc = " collation's OID"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CollateExpr() {
    assert_eq!(
        ::std::mem::size_of::<CollateExpr>(),
        24usize,
        concat!("Size of: ", stringify!(CollateExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<CollateExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(CollateExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CollateExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CollateExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CollateExpr>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CollateExpr),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CollateExpr>())).collOid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CollateExpr),
            "::",
            stringify!(collOid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CollateExpr>())).location as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(CollateExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for CollateExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " CaseExpr  a CASE expression"]
#[doc = ""]
#[doc = " We support two distinct forms of CASE expression:"]
#[doc = "    CASE WHEN boolexpr THEN expr [ WHEN boolexpr THEN expr ... ]"]
#[doc = "    CASE testexpr WHEN compexpr THEN expr [ WHEN compexpr THEN expr ... ]"]
#[doc = " These are distinguishable by the \"arg\" field being NULL in the first case"]
#[doc = " and the testexpr in the second case."]
#[doc = ""]
#[doc = " In the raw grammar output for the second form, the condition expressions"]
#[doc = " of the WHEN clauses are just the comparison values.  Parse analysis"]
#[doc = " converts these to valid boolean expressions of the form"]
#[doc = "    CaseTestExpr '=' compexpr"]
#[doc = " where the CaseTestExpr node is a placeholder that emits the correct"]
#[doc = " value at runtime.  This structure is used so that the testexpr need be"]
#[doc = " evaluated only once.  Note that after parse analysis, the condition"]
#[doc = " expressions always yield boolean."]
#[doc = ""]
#[doc = " Note: we can test whether a CaseExpr has been through parse analysis"]
#[doc = " yet by checking whether casetype is InvalidOid or not."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CaseExpr {
    pub xpr: Expr,
    pub casetype: Oid,
    #[doc = " type of expression result"]
    pub casecollid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub arg: *mut Expr,
    #[doc = " implicit equality comparison argument"]
    pub args: *mut List,
    #[doc = " the arguments (list of WHEN clauses)"]
    pub defresult: *mut Expr,
    #[doc = " the default result (ELSE clause)"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CaseExpr() {
    assert_eq!(
        ::std::mem::size_of::<CaseExpr>(),
        48usize,
        concat!("Size of: ", stringify!(CaseExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<CaseExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(CaseExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseExpr>())).casetype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseExpr),
            "::",
            stringify!(casetype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseExpr>())).casecollid as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseExpr),
            "::",
            stringify!(casecollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseExpr>())).arg as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseExpr),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseExpr>())).args as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseExpr),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseExpr>())).defresult as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseExpr),
            "::",
            stringify!(defresult)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseExpr>())).location as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for CaseExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " CaseWhen  one arm of a CASE expression"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CaseWhen {
    pub xpr: Expr,
    pub expr: *mut Expr,
    #[doc = " condition expression"]
    pub result: *mut Expr,
    #[doc = " substitution result"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CaseWhen() {
    assert_eq!(
        ::std::mem::size_of::<CaseWhen>(),
        32usize,
        concat!("Size of: ", stringify!(CaseWhen))
    );
    assert_eq!(
        ::std::mem::align_of::<CaseWhen>(),
        8usize,
        concat!("Alignment of ", stringify!(CaseWhen))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseWhen>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseWhen),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseWhen>())).expr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseWhen),
            "::",
            stringify!(expr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseWhen>())).result as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseWhen),
            "::",
            stringify!(result)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseWhen>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseWhen),
            "::",
            stringify!(location)
        )
    );
}
impl Default for CaseWhen {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Placeholder node for the test value to be processed by a CASE expression."]
#[doc = " This is effectively like a Param, but can be implemented more simply"]
#[doc = " since we need only one replacement value at a time."]
#[doc = ""]
#[doc = " We also abuse this node type for some other purposes, including:"]
#[doc = " * Placeholder for the current array element value in ArrayCoerceExpr;"]
#[doc = "   see build_coercion_expression()."]
#[doc = " * Nested FieldStore/SubscriptingRef assignment expressions in INSERT/UPDATE;"]
#[doc = "   see transformAssignmentIndirection()."]
#[doc = ""]
#[doc = " The uses in CaseExpr and ArrayCoerceExpr are safe only to the extent that"]
#[doc = " there is not any other CaseExpr or ArrayCoerceExpr between the value source"]
#[doc = " node and its child CaseTestExpr(s).  This is true in the parse analysis"]
#[doc = " output, but the planner's functioninlining logic has to be careful not to"]
#[doc = " break it."]
#[doc = ""]
#[doc = " The nestedassignmentexpression case is safe because the only node types"]
#[doc = " that can be above such CaseTestExprs are FieldStore and SubscriptingRef."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CaseTestExpr {
    pub xpr: Expr,
    pub typeId: Oid,
    #[doc = " type for substituted value"]
    pub typeMod: int32,
    #[doc = " typemod for substituted value"]
    pub collation: Oid,
}
#[test]
fn bindgen_test_layout_CaseTestExpr() {
    assert_eq!(
        ::std::mem::size_of::<CaseTestExpr>(),
        16usize,
        concat!("Size of: ", stringify!(CaseTestExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<CaseTestExpr>(),
        4usize,
        concat!("Alignment of ", stringify!(CaseTestExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseTestExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseTestExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseTestExpr>())).typeId as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseTestExpr),
            "::",
            stringify!(typeId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseTestExpr>())).typeMod as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseTestExpr),
            "::",
            stringify!(typeMod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CaseTestExpr>())).collation as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(CaseTestExpr),
            "::",
            stringify!(collation)
        )
    );
}
impl Default for CaseTestExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " ArrayExpr  an ARRAY[] expression"]
#[doc = ""]
#[doc = " Note: if multidims is false, the constituent expressions all yield the"]
#[doc = " scalar type identified by element_typeid.  If multidims is true, the"]
#[doc = " constituent expressions all yield arrays of element_typeid (ie, the same"]
#[doc = " type as array_typeid); at runtime we must check for compatible subscripts."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ArrayExpr {
    pub xpr: Expr,
    pub array_typeid: Oid,
    #[doc = " type of expression result"]
    pub array_collid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub element_typeid: Oid,
    #[doc = " common type of array elements"]
    pub elements: *mut List,
    #[doc = " the array elements or subarrays"]
    pub multidims: bool,
    #[doc = " true if elements are subarrays"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ArrayExpr() {
    assert_eq!(
        ::std::mem::size_of::<ArrayExpr>(),
        32usize,
        concat!("Size of: ", stringify!(ArrayExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<ArrayExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(ArrayExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayExpr>())).array_typeid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayExpr),
            "::",
            stringify!(array_typeid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayExpr>())).array_collid as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayExpr),
            "::",
            stringify!(array_collid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayExpr>())).element_typeid as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayExpr),
            "::",
            stringify!(element_typeid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayExpr>())).elements as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayExpr),
            "::",
            stringify!(elements)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayExpr>())).multidims as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayExpr),
            "::",
            stringify!(multidims)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ArrayExpr>())).location as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(ArrayExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for ArrayExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " RowExpr  a ROW() expression"]
#[doc = ""]
#[doc = " Note: the list of fields must have a oneforone correspondence with"]
#[doc = " physical fields of the associated rowtype, although it is okay for it"]
#[doc = " to be shorter than the rowtype.  That is, the N'th list element must"]
#[doc = " match up with the N'th physical field.  When the N'th physical field"]
#[doc = " is a dropped column (attisdropped) then the N'th list element can just"]
#[doc = " be a NULL constant.  (This case can only occur for named composite types,"]
#[doc = " not RECORD types, since those are built from the RowExpr itself rather"]
#[doc = " than vice versa.)  It is important not to assume that length(args) is"]
#[doc = " the same as the number of columns logically present in the rowtype."]
#[doc = ""]
#[doc = " colnames provides field names in cases where the names can't easily be"]
#[doc = " obtained otherwise.  Names *must* be provided if row_typeid is RECORDOID."]
#[doc = " If row_typeid identifies a known composite type, colnames can be NIL to"]
#[doc = " indicate the type's cataloged field names apply.  Note that colnames can"]
#[doc = " be nonNIL even for a composite type, and typically is when the RowExpr"]
#[doc = " was created by expanding a wholerow Var.  This is so that we can retain"]
#[doc = " the column alias names of the RTE that the Var referenced (which would"]
#[doc = " otherwise be very difficult to extract from the parsetree).  Like the"]
#[doc = " args list, colnames is oneforone with physical fields of the rowtype."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RowExpr {
    pub xpr: Expr,
    pub args: *mut List,
    #[doc = " the fields"]
    pub row_typeid: Oid,
    #[doc = " row_typeid cannot be a domain over composite, only plain composite.  To"]
    #[doc = " create a composite domain value, apply CoerceToDomain to the RowExpr."]
    #[doc = ""]
    #[doc = " Note: we deliberately do NOT store a typmod.  Although a typmod will be"]
    #[doc = " associated with specific RECORD types at runtime, it will differ for"]
    #[doc = " different backends, and so cannot safely be stored in stored"]
    #[doc = " parsetrees.  We must assume typmod 1 for a RowExpr node."]
    #[doc = ""]
    #[doc = " We don't need to store a collation either.  The result type is"]
    #[doc = " necessarily composite, and composite types never have a collation."]
    pub row_format: CoercionForm,
    #[doc = " how to display this node"]
    pub colnames: *mut List,
    #[doc = " list of String, or NIL"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_RowExpr() {
    assert_eq!(
        ::std::mem::size_of::<RowExpr>(),
        40usize,
        concat!("Size of: ", stringify!(RowExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<RowExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(RowExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RowExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowExpr>())).args as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RowExpr),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowExpr>())).row_typeid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RowExpr),
            "::",
            stringify!(row_typeid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowExpr>())).row_format as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(RowExpr),
            "::",
            stringify!(row_format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowExpr>())).colnames as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RowExpr),
            "::",
            stringify!(colnames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowExpr>())).location as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RowExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for RowExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " RowCompareExpr  rowwise comparison, such as (a, b) <= (1, 2)"]
#[doc = ""]
#[doc = " We support row comparison for any operator that can be determined to"]
#[doc = " act like =, <>, <, <=, >, or >= (we determine this by looking for the"]
#[doc = " operator in btree opfamilies).  Note that the same operator name might"]
#[doc = " map to a different operator for each pair of row elements, since the"]
#[doc = " element datatypes can vary."]
#[doc = ""]
#[doc = " A RowCompareExpr node is only generated for the < <= > >= cases;"]
#[doc = " the = and <> cases are translated to simple AND or OR combinations"]
#[doc = " of the pairwise comparisons.  However, we include = and <> in the"]
#[doc = " RowCompareType enum for the convenience of parser logic."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RowCompareType {
    #[doc = " Values of this enum are chosen to match btree strategy numbers"]
    ROWCOMPARE_LT = 1,
    #[doc = " BTLessStrategyNumber"]
    ROWCOMPARE_LE = 2,
    #[doc = " BTLessEqualStrategyNumber"]
    ROWCOMPARE_EQ = 3,
    #[doc = " BTEqualStrategyNumber"]
    ROWCOMPARE_GE = 4,
    #[doc = " BTGreaterEqualStrategyNumber"]
    ROWCOMPARE_GT = 5,
    #[doc = " BTGreaterStrategyNumber"]
    ROWCOMPARE_NE = 6,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RowCompareExpr {
    pub xpr: Expr,
    pub rctype: RowCompareType,
    #[doc = " LT LE GE or GT, never EQ or NE"]
    pub opnos: *mut List,
    #[doc = " OID list of pairwise comparison ops"]
    pub opfamilies: *mut List,
    #[doc = " OID list of containing operator families"]
    pub inputcollids: *mut List,
    #[doc = " OID list of collations for comparisons"]
    pub largs: *mut List,
    #[doc = " the lefthand input arguments"]
    pub rargs: *mut List,
}
#[test]
fn bindgen_test_layout_RowCompareExpr() {
    assert_eq!(
        ::std::mem::size_of::<RowCompareExpr>(),
        48usize,
        concat!("Size of: ", stringify!(RowCompareExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<RowCompareExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(RowCompareExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowCompareExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RowCompareExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowCompareExpr>())).rctype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RowCompareExpr),
            "::",
            stringify!(rctype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowCompareExpr>())).opnos as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RowCompareExpr),
            "::",
            stringify!(opnos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowCompareExpr>())).opfamilies as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RowCompareExpr),
            "::",
            stringify!(opfamilies)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowCompareExpr>())).inputcollids as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RowCompareExpr),
            "::",
            stringify!(inputcollids)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowCompareExpr>())).largs as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RowCompareExpr),
            "::",
            stringify!(largs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowCompareExpr>())).rargs as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RowCompareExpr),
            "::",
            stringify!(rargs)
        )
    );
}
impl Default for RowCompareExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " CoalesceExpr  a COALESCE expression"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CoalesceExpr {
    pub xpr: Expr,
    pub coalescetype: Oid,
    #[doc = " type of expression result"]
    pub coalescecollid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub args: *mut List,
    #[doc = " the arguments"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CoalesceExpr() {
    assert_eq!(
        ::std::mem::size_of::<CoalesceExpr>(),
        32usize,
        concat!("Size of: ", stringify!(CoalesceExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<CoalesceExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(CoalesceExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoalesceExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CoalesceExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoalesceExpr>())).coalescetype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CoalesceExpr),
            "::",
            stringify!(coalescetype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoalesceExpr>())).coalescecollid as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CoalesceExpr),
            "::",
            stringify!(coalescecollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoalesceExpr>())).args as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CoalesceExpr),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoalesceExpr>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CoalesceExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for CoalesceExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " MinMaxExpr  a GREATEST or LEAST function"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MinMaxOp {
    IS_GREATEST = 0,
    IS_LEAST = 1,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct MinMaxExpr {
    pub xpr: Expr,
    pub minmaxtype: Oid,
    #[doc = " common type of arguments and result"]
    pub minmaxcollid: Oid,
    #[doc = " OID of collation of result"]
    pub inputcollid: Oid,
    #[doc = " OID of collation that function should use"]
    pub op: MinMaxOp,
    #[doc = " function to execute"]
    pub args: *mut List,
    #[doc = " the arguments"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_MinMaxExpr() {
    assert_eq!(
        ::std::mem::size_of::<MinMaxExpr>(),
        40usize,
        concat!("Size of: ", stringify!(MinMaxExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<MinMaxExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(MinMaxExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MinMaxExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MinMaxExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MinMaxExpr>())).minmaxtype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MinMaxExpr),
            "::",
            stringify!(minmaxtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MinMaxExpr>())).minmaxcollid as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MinMaxExpr),
            "::",
            stringify!(minmaxcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MinMaxExpr>())).inputcollid as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MinMaxExpr),
            "::",
            stringify!(inputcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MinMaxExpr>())).op as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MinMaxExpr),
            "::",
            stringify!(op)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MinMaxExpr>())).args as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MinMaxExpr),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MinMaxExpr>())).location as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(MinMaxExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for MinMaxExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " SQLValueFunction  parameterless functions with special grammar productions"]
#[doc = ""]
#[doc = " The SQL standard categorizes some of these as <datetime value function>"]
#[doc = " and others as <general value specification>.  We call 'em SQLValueFunctions"]
#[doc = " for lack of a better term.  We store type and typmod of the result so that"]
#[doc = " some code doesn't need to know each function individually, and because"]
#[doc = " we would need to store typmod anyway for some of the datetime functions."]
#[doc = " Note that currently, all variants return noncollating datatypes, so we do"]
#[doc = " not need a collation field; also, all these functions are stable."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SQLValueFunctionOp {
    SVFOP_CURRENT_DATE = 0,
    SVFOP_CURRENT_TIME = 1,
    SVFOP_CURRENT_TIME_N = 2,
    SVFOP_CURRENT_TIMESTAMP = 3,
    SVFOP_CURRENT_TIMESTAMP_N = 4,
    SVFOP_LOCALTIME = 5,
    SVFOP_LOCALTIME_N = 6,
    SVFOP_LOCALTIMESTAMP = 7,
    SVFOP_LOCALTIMESTAMP_N = 8,
    SVFOP_CURRENT_ROLE = 9,
    SVFOP_CURRENT_USER = 10,
    SVFOP_USER = 11,
    SVFOP_SESSION_USER = 12,
    SVFOP_CURRENT_CATALOG = 13,
    SVFOP_CURRENT_SCHEMA = 14,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SQLValueFunction {
    pub xpr: Expr,
    pub op: SQLValueFunctionOp,
    #[doc = " which function this is"]
    pub type_: Oid,
    #[doc = " result type/typmod"]
    pub typmod: int32,
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_SQLValueFunction() {
    assert_eq!(
        ::std::mem::size_of::<SQLValueFunction>(),
        20usize,
        concat!("Size of: ", stringify!(SQLValueFunction))
    );
    assert_eq!(
        ::std::mem::align_of::<SQLValueFunction>(),
        4usize,
        concat!("Alignment of ", stringify!(SQLValueFunction))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SQLValueFunction>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLValueFunction),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SQLValueFunction>())).op as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLValueFunction),
            "::",
            stringify!(op)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SQLValueFunction>())).type_ as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLValueFunction),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SQLValueFunction>())).typmod as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLValueFunction),
            "::",
            stringify!(typmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SQLValueFunction>())).location as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLValueFunction),
            "::",
            stringify!(location)
        )
    );
}
impl Default for SQLValueFunction {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " XmlExpr  various SQL/XML functions requiring special grammar productions"]
#[doc = ""]
#[doc = " 'name' carries the \"NAME foo\" argument (already XMLescaped)."]
#[doc = " 'named_args' and 'arg_names' represent an xml_attribute list."]
#[doc = " 'args' carries all other arguments."]
#[doc = ""]
#[doc = " Note: result type/typmod/collation are not stored, but can be deduced"]
#[doc = " from the XmlExprOp.  The type/typmod fields are just used for display"]
#[doc = " purposes, and are NOT necessarily the true result type of the node."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XmlExprOp {
    IS_XMLCONCAT = 0,
    #[doc = " XMLCONCAT(args)"]
    IS_XMLELEMENT = 1,
    #[doc = " XMLELEMENT(name, xml_attributes, args)"]
    IS_XMLFOREST = 2,
    #[doc = " XMLFOREST(xml_attributes)"]
    IS_XMLPARSE = 3,
    #[doc = " XMLPARSE(text, is_doc, preserve_ws)"]
    IS_XMLPI = 4,
    #[doc = " XMLPI(name [, args])"]
    IS_XMLROOT = 5,
    #[doc = " XMLROOT(xml, version, standalone)"]
    IS_XMLSERIALIZE = 6,
    #[doc = " XMLSERIALIZE(is_document, xmlval)"]
    IS_DOCUMENT = 7,
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XmlOptionType {
    XMLOPTION_DOCUMENT = 0,
    XMLOPTION_CONTENT = 1,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct XmlExpr {
    pub xpr: Expr,
    pub op: XmlExprOp,
    #[doc = " xml function ID"]
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " name in xml(NAME foo ...) syntaxes"]
    pub named_args: *mut List,
    #[doc = " nonXML expressions for xml_attributes"]
    pub arg_names: *mut List,
    #[doc = " parallel list of Value strings"]
    pub args: *mut List,
    #[doc = " list of expressions"]
    pub xmloption: XmlOptionType,
    #[doc = " DOCUMENT or CONTENT"]
    pub type_: Oid,
    #[doc = " target type/typmod for XMLSERIALIZE"]
    pub typmod: int32,
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_XmlExpr() {
    assert_eq!(
        ::std::mem::size_of::<XmlExpr>(),
        56usize,
        concat!("Size of: ", stringify!(XmlExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<XmlExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(XmlExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlExpr>())).op as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlExpr),
            "::",
            stringify!(op)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlExpr>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlExpr),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlExpr>())).named_args as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlExpr),
            "::",
            stringify!(named_args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlExpr>())).arg_names as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlExpr),
            "::",
            stringify!(arg_names)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlExpr>())).args as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlExpr),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlExpr>())).xmloption as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlExpr),
            "::",
            stringify!(xmloption)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlExpr>())).type_ as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlExpr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlExpr>())).typmod as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlExpr),
            "::",
            stringify!(typmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlExpr>())).location as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for XmlExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " NullTest"]
#[doc = ""]
#[doc = " NullTest represents the operation of testing a value for NULLness."]
#[doc = " The appropriate test is performed and returned as a boolean Datum."]
#[doc = ""]
#[doc = " When argisrow is false, this simply represents a test for the null value."]
#[doc = ""]
#[doc = " When argisrow is true, the input expression must yield a rowtype, and"]
#[doc = " the node implements \"row IS [NOT] NULL\" per the SQL standard.  This"]
#[doc = " includes checking individual fields for NULLness when the row datum"]
#[doc = " itself isn't NULL."]
#[doc = ""]
#[doc = " NOTE: the combination of a rowtype input and argisrow==false does NOT"]
#[doc = " correspond to the SQL notation \"row IS [NOT] NULL\"; instead, this case"]
#[doc = " represents the SQL notation \"row IS [NOT] DISTINCT FROM NULL\"."]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NullTestType {
    IS_NULL = 0,
    IS_NOT_NULL = 1,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NullTest {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " input expression"]
    pub nulltesttype: NullTestType,
    #[doc = " IS NULL, IS NOT NULL"]
    pub argisrow: bool,
    #[doc = " T to perform fieldbyfield null checks"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NullTest() {
    assert_eq!(
        ::std::mem::size_of::<NullTest>(),
        32usize,
        concat!("Size of: ", stringify!(NullTest))
    );
    assert_eq!(
        ::std::mem::align_of::<NullTest>(),
        8usize,
        concat!("Alignment of ", stringify!(NullTest))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NullTest>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NullTest),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NullTest>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(NullTest),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NullTest>())).nulltesttype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(NullTest),
            "::",
            stringify!(nulltesttype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NullTest>())).argisrow as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(NullTest),
            "::",
            stringify!(argisrow)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NullTest>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(NullTest),
            "::",
            stringify!(location)
        )
    );
}
impl Default for NullTest {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " BooleanTest"]
#[doc = ""]
#[doc = " BooleanTest represents the operation of determining whether a boolean"]
#[doc = " is TRUE, FALSE, or UNKNOWN (ie, NULL).  All six meaningful combinations"]
#[doc = " are supported.  Note that a NULL input does *not* cause a NULL result."]
#[doc = " The appropriate test is performed and returned as a boolean Datum."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoolTestType {
    IS_TRUE = 0,
    IS_NOT_TRUE = 1,
    IS_FALSE = 2,
    IS_NOT_FALSE = 3,
    IS_UNKNOWN = 4,
    IS_NOT_UNKNOWN = 5,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct BooleanTest {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " input expression"]
    pub booltesttype: BoolTestType,
    #[doc = " test type"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_BooleanTest() {
    assert_eq!(
        ::std::mem::size_of::<BooleanTest>(),
        24usize,
        concat!("Size of: ", stringify!(BooleanTest))
    );
    assert_eq!(
        ::std::mem::align_of::<BooleanTest>(),
        8usize,
        concat!("Alignment of ", stringify!(BooleanTest))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BooleanTest>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BooleanTest),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BooleanTest>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(BooleanTest),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BooleanTest>())).booltesttype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(BooleanTest),
            "::",
            stringify!(booltesttype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BooleanTest>())).location as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(BooleanTest),
            "::",
            stringify!(location)
        )
    );
}
impl Default for BooleanTest {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " CoerceToDomain"]
#[doc = ""]
#[doc = " CoerceToDomain represents the operation of coercing a value to a domain"]
#[doc = " type.  At runtime (and not before) the precise set of constraints to be"]
#[doc = " checked will be determined.  If the value passes, it is returned as the"]
#[doc = " result; if not, an error is raised.  Note that this is equivalent to"]
#[doc = " RelabelType in the scenario where no constraints are applied."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CoerceToDomain {
    pub xpr: Expr,
    pub arg: *mut Expr,
    #[doc = " input expression"]
    pub resulttype: Oid,
    #[doc = " domain type ID (result type)"]
    pub resulttypmod: int32,
    #[doc = " output typmod (currently always 1)"]
    pub resultcollid: Oid,
    #[doc = " OID of collation, or InvalidOid if none"]
    pub coercionformat: CoercionForm,
    #[doc = " how to display this node"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CoerceToDomain() {
    assert_eq!(
        ::std::mem::size_of::<CoerceToDomain>(),
        40usize,
        concat!("Size of: ", stringify!(CoerceToDomain))
    );
    assert_eq!(
        ::std::mem::align_of::<CoerceToDomain>(),
        8usize,
        concat!("Alignment of ", stringify!(CoerceToDomain))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomain>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomain),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomain>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomain),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomain>())).resulttype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomain),
            "::",
            stringify!(resulttype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomain>())).resulttypmod as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomain),
            "::",
            stringify!(resulttypmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomain>())).resultcollid as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomain),
            "::",
            stringify!(resultcollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomain>())).coercionformat as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomain),
            "::",
            stringify!(coercionformat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomain>())).location as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomain),
            "::",
            stringify!(location)
        )
    );
}
impl Default for CoerceToDomain {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Placeholder node for the value to be processed by a domain's check"]
#[doc = " constraint.  This is effectively like a Param, but can be implemented more"]
#[doc = " simply since we need only one replacement value at a time."]
#[doc = ""]
#[doc = " Note: the typeId/typeMod/collation will be set from the domain's base type,"]
#[doc = " not the domain itself.  This is because we shouldn't consider the value"]
#[doc = " to be a member of the domain if we haven't yet checked its constraints."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CoerceToDomainValue {
    pub xpr: Expr,
    pub typeId: Oid,
    #[doc = " type for substituted value"]
    pub typeMod: int32,
    #[doc = " typemod for substituted value"]
    pub collation: Oid,
    #[doc = " collation for the substituted value"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CoerceToDomainValue() {
    assert_eq!(
        ::std::mem::size_of::<CoerceToDomainValue>(),
        20usize,
        concat!("Size of: ", stringify!(CoerceToDomainValue))
    );
    assert_eq!(
        ::std::mem::align_of::<CoerceToDomainValue>(),
        4usize,
        concat!("Alignment of ", stringify!(CoerceToDomainValue))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomainValue>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomainValue),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomainValue>())).typeId as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomainValue),
            "::",
            stringify!(typeId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomainValue>())).typeMod as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomainValue),
            "::",
            stringify!(typeMod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomainValue>())).collation as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomainValue),
            "::",
            stringify!(collation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CoerceToDomainValue>())).location as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CoerceToDomainValue),
            "::",
            stringify!(location)
        )
    );
}
impl Default for CoerceToDomainValue {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Placeholder node for a DEFAULT marker in an INSERT or UPDATE command."]
#[doc = ""]
#[doc = " This is not an executable expression: it must be replaced by the actual"]
#[doc = " column default expression during rewriting.  But it is convenient to"]
#[doc = " treat it as an expression node during parsing and rewriting."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SetToDefault {
    pub xpr: Expr,
    pub typeId: Oid,
    #[doc = " type for substituted value"]
    pub typeMod: int32,
    #[doc = " typemod for substituted value"]
    pub collation: Oid,
    #[doc = " collation for the substituted value"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_SetToDefault() {
    assert_eq!(
        ::std::mem::size_of::<SetToDefault>(),
        20usize,
        concat!("Size of: ", stringify!(SetToDefault))
    );
    assert_eq!(
        ::std::mem::align_of::<SetToDefault>(),
        4usize,
        concat!("Alignment of ", stringify!(SetToDefault))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetToDefault>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SetToDefault),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetToDefault>())).typeId as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SetToDefault),
            "::",
            stringify!(typeId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetToDefault>())).typeMod as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SetToDefault),
            "::",
            stringify!(typeMod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetToDefault>())).collation as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SetToDefault),
            "::",
            stringify!(collation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetToDefault>())).location as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SetToDefault),
            "::",
            stringify!(location)
        )
    );
}
impl Default for SetToDefault {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Node representing [WHERE] CURRENT OF cursor_name"]
#[doc = ""]
#[doc = " CURRENT OF is a bit like a Var, in that it carries the rangetable index"]
#[doc = " of the target relation being constrained; this aids placing the expression"]
#[doc = " correctly during planning.  We can assume however that its \"levelsup\" is"]
#[doc = " always zero, due to the syntactic constraints on where it can appear."]
#[doc = ""]
#[doc = " The referenced cursor can be represented either as a hardwired string"]
#[doc = " or as a reference to a runtime parameter of type REFCURSOR.  The latter"]
#[doc = " case is for the convenience of plpgsql."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CurrentOfExpr {
    pub xpr: Expr,
    pub cvarno: Index,
    #[doc = " RT index of target relation"]
    pub cursor_name: *mut ::std::os::raw::c_char,
    #[doc = " name of referenced cursor, or NULL"]
    pub cursor_param: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CurrentOfExpr() {
    assert_eq!(
        ::std::mem::size_of::<CurrentOfExpr>(),
        24usize,
        concat!("Size of: ", stringify!(CurrentOfExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<CurrentOfExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(CurrentOfExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CurrentOfExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CurrentOfExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CurrentOfExpr>())).cvarno as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CurrentOfExpr),
            "::",
            stringify!(cvarno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CurrentOfExpr>())).cursor_name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CurrentOfExpr),
            "::",
            stringify!(cursor_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CurrentOfExpr>())).cursor_param as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CurrentOfExpr),
            "::",
            stringify!(cursor_param)
        )
    );
}
impl Default for CurrentOfExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " NextValueExpr  get next value from sequence"]
#[doc = ""]
#[doc = " This has the same effect as calling the nextval() function, but it does not"]
#[doc = " check permissions on the sequence.  This is used for identity columns,"]
#[doc = " where the sequence is an implicit dependency without its own permissions."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NextValueExpr {
    pub xpr: Expr,
    pub seqid: Oid,
    pub typeId: Oid,
}
#[test]
fn bindgen_test_layout_NextValueExpr() {
    assert_eq!(
        ::std::mem::size_of::<NextValueExpr>(),
        12usize,
        concat!("Size of: ", stringify!(NextValueExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<NextValueExpr>(),
        4usize,
        concat!("Alignment of ", stringify!(NextValueExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NextValueExpr>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NextValueExpr),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NextValueExpr>())).seqid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(NextValueExpr),
            "::",
            stringify!(seqid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NextValueExpr>())).typeId as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(NextValueExpr),
            "::",
            stringify!(typeId)
        )
    );
}
impl Default for NextValueExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " InferenceElem  an element of a unique index inference specification"]
#[doc = ""]
#[doc = " This mostly matches the structure of IndexElems, but having a dedicated"]
#[doc = " primnode allows for a clean separation between the use of index parameters"]
#[doc = " by utility commands, and this node."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct InferenceElem {
    pub xpr: Expr,
    pub expr: *mut Node,
    #[doc = " expression to infer from, or NULL"]
    pub infercollid: Oid,
    #[doc = " OID of collation, or InvalidOid"]
    pub inferopclass: Oid,
}
#[test]
fn bindgen_test_layout_InferenceElem() {
    assert_eq!(
        ::std::mem::size_of::<InferenceElem>(),
        24usize,
        concat!("Size of: ", stringify!(InferenceElem))
    );
    assert_eq!(
        ::std::mem::align_of::<InferenceElem>(),
        8usize,
        concat!("Alignment of ", stringify!(InferenceElem))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InferenceElem>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(InferenceElem),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InferenceElem>())).expr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(InferenceElem),
            "::",
            stringify!(expr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InferenceElem>())).infercollid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(InferenceElem),
            "::",
            stringify!(infercollid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InferenceElem>())).inferopclass as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(InferenceElem),
            "::",
            stringify!(inferopclass)
        )
    );
}
impl Default for InferenceElem {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " TargetEntry"]
#[doc = "    a target entry (used in query target lists)"]
#[doc = ""]
#[doc = " Strictly speaking, a TargetEntry isn't an expression node (since it can't"]
#[doc = " be evaluated by ExecEvalExpr).  But we treat it as one anyway, since in"]
#[doc = " very many places it's convenient to process a whole query targetlist as a"]
#[doc = " single expression tree."]
#[doc = ""]
#[doc = " In a SELECT's targetlist, resno should always be equal to the item's"]
#[doc = " ordinal position (counting from 1).  However, in an INSERT or UPDATE"]
#[doc = " targetlist, resno represents the attribute number of the destination"]
#[doc = " column for the item; so there may be missing or outoforder resnos."]
#[doc = " It is even legal to have duplicated resnos; consider"]
#[doc = "    UPDATE table SET arraycol[1] = ..., arraycol[2] = ..., ..."]
#[doc = " The two meanings come together in the executor, because the planner"]
#[doc = " transforms INSERT/UPDATE tlists into a normalized form with exactly"]
#[doc = " one entry for each column of the destination table.  Before that's"]
#[doc = " happened, however, it is risky to assume that resno == position."]
#[doc = " Generally get_tle_by_resno() should be used rather than list_nth()"]
#[doc = " to fetch tlist entries by resno, and only in SELECT should you assume"]
#[doc = " that resno is a unique identifier."]
#[doc = ""]
#[doc = " resname is required to represent the correct column name in nonresjunk"]
#[doc = " entries of toplevel SELECT targetlists, since it will be used as the"]
#[doc = " column title sent to the frontend.  In most other contexts it is only"]
#[doc = " a debugging aid, and may be wrong or even NULL.  (In particular, it may"]
#[doc = " be wrong in a tlist from a stored rule, if the referenced column has been"]
#[doc = " renamed by ALTER TABLE since the rule was made.  Also, the planner tends"]
#[doc = " to store NULL rather than look up a valid name for tlist entries in"]
#[doc = " nontoplevel plan nodes.)  In resjunk entries, resname should be either"]
#[doc = " a specific systemgenerated name (such as \"ctid\") or NULL; anything else"]
#[doc = " risks confusing ExecGetJunkAttribute!"]
#[doc = ""]
#[doc = " ressortgroupref is used in the representation of ORDER BY, GROUP BY, and"]
#[doc = " DISTINCT items.  Targetlist entries with ressortgroupref=0 are not"]
#[doc = " sort/group items.  If ressortgroupref>0, then this item is an ORDER BY,"]
#[doc = " GROUP BY, and/or DISTINCT target value.  No two entries in a targetlist"]
#[doc = " may have the same nonzero ressortgroupref  but there is no particular"]
#[doc = " meaning to the nonzero values, except as tags.  (For example, one must"]
#[doc = " not assume that lower ressortgroupref means a more significant sort key.)"]
#[doc = " The order of the associated SortGroupClause lists determine the semantics."]
#[doc = ""]
#[doc = " resorigtbl/resorigcol identify the source of the column, if it is a"]
#[doc = " simple reference to a column of a base table (or view).  If it is not"]
#[doc = " a simple reference, these fields are zeroes."]
#[doc = ""]
#[doc = " If resjunk is true then the column is a working column (such as a sort key)"]
#[doc = " that should be removed from the final output of the query.  Resjunk columns"]
#[doc = " must have resnos that cannot duplicate any regular column's resno.  Also"]
#[doc = " note that there are places that assume resjunk columns come after nonjunk"]
#[doc = " columns."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TargetEntry {
    pub xpr: Expr,
    pub expr: *mut Expr,
    #[doc = " expression to evaluate"]
    pub resno: AttrNumber,
    #[doc = " attribute number (see notes above)"]
    pub resname: *mut ::std::os::raw::c_char,
    #[doc = " name of the column (could be NULL)"]
    pub ressortgroupref: Index,
    #[doc = " nonzero if referenced by a sort/group"]
    #[doc = " clause"]
    pub resorigtbl: Oid,
    #[doc = " OID of column's source table"]
    pub resorigcol: AttrNumber,
    #[doc = " column's number in source table"]
    pub resjunk: bool,
}
#[test]
fn bindgen_test_layout_TargetEntry() {
    assert_eq!(
        ::std::mem::size_of::<TargetEntry>(),
        48usize,
        concat!("Size of: ", stringify!(TargetEntry))
    );
    assert_eq!(
        ::std::mem::align_of::<TargetEntry>(),
        8usize,
        concat!("Alignment of ", stringify!(TargetEntry))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TargetEntry>())).xpr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TargetEntry),
            "::",
            stringify!(xpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TargetEntry>())).expr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TargetEntry),
            "::",
            stringify!(expr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TargetEntry>())).resno as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TargetEntry),
            "::",
            stringify!(resno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TargetEntry>())).resname as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(TargetEntry),
            "::",
            stringify!(resname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TargetEntry>())).ressortgroupref as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(TargetEntry),
            "::",
            stringify!(ressortgroupref)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TargetEntry>())).resorigtbl as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(TargetEntry),
            "::",
            stringify!(resorigtbl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TargetEntry>())).resorigcol as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(TargetEntry),
            "::",
            stringify!(resorigcol)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TargetEntry>())).resjunk as *const _ as usize },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(TargetEntry),
            "::",
            stringify!(resjunk)
        )
    );
}
impl Default for TargetEntry {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " RangeTblRef  reference to an entry in the query's rangetable"]
#[doc = ""]
#[doc = " We could use direct pointers to the RT entries and skip having these"]
#[doc = " nodes, but multiple pointers to the same node in a querytree cause"]
#[doc = " lots of headaches, so it seems better to store an index into the RT."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RangeTblRef {
    pub type_: NodeTag,
    pub rtindex: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_RangeTblRef() {
    assert_eq!(
        ::std::mem::size_of::<RangeTblRef>(),
        8usize,
        concat!("Size of: ", stringify!(RangeTblRef))
    );
    assert_eq!(
        ::std::mem::align_of::<RangeTblRef>(),
        4usize,
        concat!("Alignment of ", stringify!(RangeTblRef))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTblRef>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTblRef),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTblRef>())).rtindex as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTblRef),
            "::",
            stringify!(rtindex)
        )
    );
}
impl Default for RangeTblRef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " JoinExpr  for SQL JOIN expressions"]
#[doc = ""]
#[doc = " isNatural, usingClause, and quals are interdependent.  The user can write"]
#[doc = " only one of NATURAL, USING(), or ON() (this is enforced by the grammar)."]
#[doc = " If he writes NATURAL then parse analysis generates the equivalent USING()"]
#[doc = " list, and from that fills in \"quals\" with the right equality comparisons."]
#[doc = " If he writes USING() then \"quals\" is filled with equality comparisons."]
#[doc = " If he writes ON() then only \"quals\" is set.  Note that NATURAL/USING"]
#[doc = " are not equivalent to ON() since they also affect the output column list."]
#[doc = ""]
#[doc = " alias is an Alias node representing the AS aliasclause attached to the"]
#[doc = " join expression, or NULL if no clause.  NB: presence or absence of the"]
#[doc = " alias has a critical impact on semantics, because a join with an alias"]
#[doc = " restricts visibility of the tables/columns inside it."]
#[doc = ""]
#[doc = " During parse analysis, an RTE is created for the Join, and its index"]
#[doc = " is filled into rtindex.  This RTE is present mainly so that Vars can"]
#[doc = " be created that refer to the outputs of the join.  The planner sometimes"]
#[doc = " generates JoinExprs internally; these can have rtindex = 0 if there are"]
#[doc = " no join alias variables referencing such joins."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct JoinExpr {
    pub type_: NodeTag,
    pub jointype: JoinType,
    #[doc = " type of join"]
    pub isNatural: bool,
    #[doc = " Natural join? Will need to shape table"]
    pub larg: *mut Node,
    #[doc = " left subtree"]
    pub rarg: *mut Node,
    #[doc = " right subtree"]
    pub usingClause: *mut List,
    #[doc = " USING clause, if any (list of String)"]
    pub quals: *mut Node,
    #[doc = " qualifiers on join, if any"]
    pub alias: *mut Alias,
    #[doc = " userwritten alias clause, if any"]
    pub rtindex: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_JoinExpr() {
    assert_eq!(
        ::std::mem::size_of::<JoinExpr>(),
        64usize,
        concat!("Size of: ", stringify!(JoinExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<JoinExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(JoinExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoinExpr>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JoinExpr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoinExpr>())).jointype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JoinExpr),
            "::",
            stringify!(jointype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoinExpr>())).isNatural as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(JoinExpr),
            "::",
            stringify!(isNatural)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoinExpr>())).larg as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(JoinExpr),
            "::",
            stringify!(larg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoinExpr>())).rarg as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(JoinExpr),
            "::",
            stringify!(rarg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoinExpr>())).usingClause as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(JoinExpr),
            "::",
            stringify!(usingClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoinExpr>())).quals as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(JoinExpr),
            "::",
            stringify!(quals)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoinExpr>())).alias as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(JoinExpr),
            "::",
            stringify!(alias)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<JoinExpr>())).rtindex as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(JoinExpr),
            "::",
            stringify!(rtindex)
        )
    );
}
impl Default for JoinExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " FromExpr  represents a FROM ... WHERE ... construct"]
#[doc = ""]
#[doc = " This is both more flexible than a JoinExpr (it can have any number of"]
#[doc = " children, including zero) and less so  we don't need to deal with"]
#[doc = " aliases and so on.  The output column set is implicitly just the union"]
#[doc = " of the outputs of the children."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FromExpr {
    pub type_: NodeTag,
    pub fromlist: *mut List,
    #[doc = " List of join subtrees"]
    pub quals: *mut Node,
}
#[test]
fn bindgen_test_layout_FromExpr() {
    assert_eq!(
        ::std::mem::size_of::<FromExpr>(),
        24usize,
        concat!("Size of: ", stringify!(FromExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<FromExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(FromExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FromExpr>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FromExpr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FromExpr>())).fromlist as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FromExpr),
            "::",
            stringify!(fromlist)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FromExpr>())).quals as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FromExpr),
            "::",
            stringify!(quals)
        )
    );
}
impl Default for FromExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " OnConflictExpr  represents an ON CONFLICT DO ... expression"]
#[doc = ""]
#[doc = " The optimizer requires a list of inference elements, and optionally a WHERE"]
#[doc = " clause to infer a unique index.  The unique index (or, occasionally,"]
#[doc = " indexes) inferred are used to arbitrate whether or not the alternative ON"]
#[doc = " CONFLICT path is taken."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct OnConflictExpr {
    pub type_: NodeTag,
    pub action: OnConflictAction,
    #[doc = " Arbiter"]
    pub arbiterElems: *mut List,
    #[doc = " unique index arbiter list (of"]
    #[doc = " InferenceElem's)"]
    pub arbiterWhere: *mut Node,
    #[doc = " unique index arbiter WHERE clause"]
    pub constraint: Oid,
    #[doc = " ON CONFLICT UPDATE"]
    pub onConflictSet: *mut List,
    #[doc = " List of ON CONFLICT SET TargetEntrys"]
    pub onConflictWhere: *mut Node,
    #[doc = " qualifiers to restrict UPDATE to"]
    pub exclRelIndex: ::std::os::raw::c_int,
    #[doc = " RT index of 'excluded' relation"]
    pub exclRelTlist: *mut List,
}
#[test]
fn bindgen_test_layout_OnConflictExpr() {
    assert_eq!(
        ::std::mem::size_of::<OnConflictExpr>(),
        64usize,
        concat!("Size of: ", stringify!(OnConflictExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<OnConflictExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(OnConflictExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictExpr>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictExpr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictExpr>())).action as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictExpr),
            "::",
            stringify!(action)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictExpr>())).arbiterElems as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictExpr),
            "::",
            stringify!(arbiterElems)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictExpr>())).arbiterWhere as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictExpr),
            "::",
            stringify!(arbiterWhere)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictExpr>())).constraint as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictExpr),
            "::",
            stringify!(constraint)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictExpr>())).onConflictSet as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictExpr),
            "::",
            stringify!(onConflictSet)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictExpr>())).onConflictWhere as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictExpr),
            "::",
            stringify!(onConflictWhere)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictExpr>())).exclRelIndex as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictExpr),
            "::",
            stringify!(exclRelIndex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictExpr>())).exclRelTlist as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictExpr),
            "::",
            stringify!(exclRelTlist)
        )
    );
}
impl Default for OnConflictExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Value {
    pub type_: NodeTag,
    pub val: Value_ValUnion,
}
#[repr(C)]
pub struct Value_ValUnion {
    pub ival: __BindgenUnionField<::std::os::raw::c_int>,
    pub str: __BindgenUnionField<*mut ::std::os::raw::c_char>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout_Value_ValUnion() {
    assert_eq!(
        ::std::mem::size_of::<Value_ValUnion>(),
        8usize,
        concat!("Size of: ", stringify!(Value_ValUnion))
    );
    assert_eq!(
        ::std::mem::align_of::<Value_ValUnion>(),
        8usize,
        concat!("Alignment of ", stringify!(Value_ValUnion))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Value_ValUnion>())).ival as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Value_ValUnion),
            "::",
            stringify!(ival)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Value_ValUnion>())).str as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Value_ValUnion),
            "::",
            stringify!(str)
        )
    );
}
impl Default for Value_ValUnion {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_Value() {
    assert_eq!(
        ::std::mem::size_of::<Value>(),
        16usize,
        concat!("Size of: ", stringify!(Value))
    );
    assert_eq!(
        ::std::mem::align_of::<Value>(),
        8usize,
        concat!("Alignment of ", stringify!(Value))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Value>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Value),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Value>())).val as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Value),
            "::",
            stringify!(val)
        )
    );
}
impl Default for Value {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OverridingKind {
    OVERRIDING_NOT_SET = 0,
    OVERRIDING_USER_VALUE = 1,
    OVERRIDING_SYSTEM_VALUE = 2,
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " Possible sources of a Query"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum QuerySource {
    QSRC_ORIGINAL = 0,
    #[doc = " original parsetree (explicit query)"]
    QSRC_PARSER = 1,
    #[doc = " added by parse analysis (now unused)"]
    QSRC_INSTEAD_RULE = 2,
    #[doc = " added by unconditional INSTEAD rule"]
    QSRC_QUAL_INSTEAD_RULE = 3,
    #[doc = " added by conditional INSTEAD rule"]
    QSRC_NON_INSTEAD_RULE = 4,
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " Sort ordering options for ORDER BY and CREATE INDEX"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SortByDir {
    SORTBY_DEFAULT = 0,
    SORTBY_ASC = 1,
    SORTBY_DESC = 2,
    SORTBY_USING = 3,
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SortByNulls {
    SORTBY_NULLS_DEFAULT = 0,
    SORTBY_NULLS_FIRST = 1,
    SORTBY_NULLS_LAST = 2,
}
#[doc = " TypeName  specifies a type in definitions"]
#[doc = ""]
#[doc = " For TypeName structures generated internally, it is often easier to"]
#[doc = " specify the type by OID than by name.  If \"names\" is NIL then the"]
#[doc = " actual type OID is given by typeOid, otherwise typeOid is unused."]
#[doc = " Similarly, if \"typmods\" is NIL then the actual typmod is expected to"]
#[doc = " be prespecified in typemod, otherwise typemod is unused."]
#[doc = ""]
#[doc = " If pct_type is true, then names is actually a field name and we look up"]
#[doc = " the type of that field.  Otherwise (the normal case), names is a type"]
#[doc = " name possibly qualified with schema and database name."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TypeName {
    pub type_: NodeTag,
    pub names: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub typeOid: Oid,
    #[doc = " type identified by OID"]
    pub setof: bool,
    #[doc = " is a set?"]
    pub pct_type: bool,
    #[doc = " %TYPE specified?"]
    pub typmods: *mut List,
    #[doc = " type modifier expression(s)"]
    pub typemod: int32,
    #[doc = " prespecified type modifier"]
    pub arrayBounds: *mut List,
    #[doc = " array bounds"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_TypeName() {
    assert_eq!(
        ::std::mem::size_of::<TypeName>(),
        56usize,
        concat!("Size of: ", stringify!(TypeName))
    );
    assert_eq!(
        ::std::mem::align_of::<TypeName>(),
        8usize,
        concat!("Alignment of ", stringify!(TypeName))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeName>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeName),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeName>())).names as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeName),
            "::",
            stringify!(names)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeName>())).typeOid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeName),
            "::",
            stringify!(typeOid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeName>())).setof as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeName),
            "::",
            stringify!(setof)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeName>())).pct_type as *const _ as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeName),
            "::",
            stringify!(pct_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeName>())).typmods as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeName),
            "::",
            stringify!(typmods)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeName>())).typemod as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeName),
            "::",
            stringify!(typemod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeName>())).arrayBounds as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeName),
            "::",
            stringify!(arrayBounds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeName>())).location as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeName),
            "::",
            stringify!(location)
        )
    );
}
impl Default for TypeName {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " ColumnRef  specifies a reference to a column, or possibly a whole tuple"]
#[doc = ""]
#[doc = " The \"fields\" list must be nonempty.  It can contain string Value nodes"]
#[doc = " (representing names) and A_Star nodes (representing occurrence of a '*')."]
#[doc = " Currently, A_Star must appear only as the last list element  the grammar"]
#[doc = " is responsible for enforcing this!"]
#[doc = ""]
#[doc = " Note: any container subscripting or selection of fields from composite columns"]
#[doc = " is represented by an A_Indirection node above the ColumnRef.  However,"]
#[doc = " for simplicity in the normal case, initial field selection from a table"]
#[doc = " name is represented within ColumnRef and not by adding A_Indirection."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ColumnRef {
    pub type_: NodeTag,
    pub fields: *mut List,
    #[doc = " field names (Value strings) or A_Star"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ColumnRef() {
    assert_eq!(
        ::std::mem::size_of::<ColumnRef>(),
        24usize,
        concat!("Size of: ", stringify!(ColumnRef))
    );
    assert_eq!(
        ::std::mem::align_of::<ColumnRef>(),
        8usize,
        concat!("Alignment of ", stringify!(ColumnRef))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnRef>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnRef),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnRef>())).fields as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnRef),
            "::",
            stringify!(fields)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnRef>())).location as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnRef),
            "::",
            stringify!(location)
        )
    );
}
impl Default for ColumnRef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " ParamRef  specifies a $n parameter reference"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ParamRef {
    pub type_: NodeTag,
    pub number: ::std::os::raw::c_int,
    #[doc = " the number of the parameter"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ParamRef() {
    assert_eq!(
        ::std::mem::size_of::<ParamRef>(),
        12usize,
        concat!("Size of: ", stringify!(ParamRef))
    );
    assert_eq!(
        ::std::mem::align_of::<ParamRef>(),
        4usize,
        concat!("Alignment of ", stringify!(ParamRef))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ParamRef>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ParamRef),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ParamRef>())).number as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ParamRef),
            "::",
            stringify!(number)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ParamRef>())).location as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ParamRef),
            "::",
            stringify!(location)
        )
    );
}
impl Default for ParamRef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " A_Expr  infix, prefix, and postfix expressions"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum A_Expr_Kind {
    AEXPR_OP = 0,
    #[doc = " normal operator"]
    AEXPR_OP_ANY = 1,
    #[doc = " scalar op ANY (array)"]
    AEXPR_OP_ALL = 2,
    #[doc = " scalar op ALL (array)"]
    AEXPR_DISTINCT = 3,
    #[doc = " IS DISTINCT FROM  name must be \"=\""]
    AEXPR_NOT_DISTINCT = 4,
    #[doc = " IS NOT DISTINCT FROM  name must be \"=\""]
    AEXPR_NULLIF = 5,
    #[doc = " NULLIF  name must be \"=\""]
    AEXPR_OF = 6,
    #[doc = " IS [NOT] OF  name must be \"=\" or \"<>\""]
    AEXPR_IN = 7,
    #[doc = " [NOT] IN  name must be \"=\" or \"<>\""]
    AEXPR_LIKE = 8,
    #[doc = " [NOT] LIKE  name must be \"~~\" or \"!~~\""]
    AEXPR_ILIKE = 9,
    #[doc = " [NOT] ILIKE  name must be \"~~*\" or \"!~~*\""]
    AEXPR_SIMILAR = 10,
    #[doc = " [NOT] SIMILAR  name must be \"~\" or \"!~\""]
    AEXPR_BETWEEN = 11,
    #[doc = " name must be \"BETWEEN\""]
    AEXPR_NOT_BETWEEN = 12,
    #[doc = " name must be \"NOT BETWEEN\""]
    AEXPR_BETWEEN_SYM = 13,
    #[doc = " name must be \"BETWEEN SYMMETRIC\""]
    AEXPR_NOT_BETWEEN_SYM = 14,
    #[doc = " name must be \"NOT BETWEEN SYMMETRIC\""]
    AEXPR_PAREN = 15,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct A_Expr {
    pub type_: NodeTag,
    pub kind: A_Expr_Kind,
    #[doc = " see above"]
    pub name: *mut List,
    #[doc = " possiblyqualified name of operator"]
    pub lexpr: *mut Node,
    #[doc = " left argument, or NULL if none"]
    pub rexpr: *mut Node,
    #[doc = " right argument, or NULL if none"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_Expr() {
    assert_eq!(
        ::std::mem::size_of::<A_Expr>(),
        40usize,
        concat!("Size of: ", stringify!(A_Expr))
    );
    assert_eq!(
        ::std::mem::align_of::<A_Expr>(),
        8usize,
        concat!("Alignment of ", stringify!(A_Expr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Expr>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Expr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Expr>())).kind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Expr),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Expr>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Expr),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Expr>())).lexpr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Expr),
            "::",
            stringify!(lexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Expr>())).rexpr as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Expr),
            "::",
            stringify!(rexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Expr>())).location as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Expr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for A_Expr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " A_Const  a literal constant"]
#[repr(C)]
pub struct A_Const {
    pub type_: NodeTag,
    pub val: Value,
    #[doc = " value (includes type info, see value.h)"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_Const() {
    assert_eq!(
        ::std::mem::size_of::<A_Const>(),
        32usize,
        concat!("Size of: ", stringify!(A_Const))
    );
    assert_eq!(
        ::std::mem::align_of::<A_Const>(),
        8usize,
        concat!("Alignment of ", stringify!(A_Const))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Const>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Const),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Const>())).val as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Const),
            "::",
            stringify!(val)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Const>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Const),
            "::",
            stringify!(location)
        )
    );
}
impl Default for A_Const {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " TypeCast  a CAST expression"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TypeCast {
    pub type_: NodeTag,
    pub arg: *mut Node,
    #[doc = " the expression being casted"]
    pub typeName: *mut TypeName,
    #[doc = " the target type"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_TypeCast() {
    assert_eq!(
        ::std::mem::size_of::<TypeCast>(),
        32usize,
        concat!("Size of: ", stringify!(TypeCast))
    );
    assert_eq!(
        ::std::mem::align_of::<TypeCast>(),
        8usize,
        concat!("Alignment of ", stringify!(TypeCast))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeCast>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeCast),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeCast>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeCast),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeCast>())).typeName as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeCast),
            "::",
            stringify!(typeName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TypeCast>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(TypeCast),
            "::",
            stringify!(location)
        )
    );
}
impl Default for TypeCast {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " CollateClause  a COLLATE expression"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CollateClause {
    pub type_: NodeTag,
    pub arg: *mut Node,
    #[doc = " input expression"]
    pub collname: *mut List,
    #[doc = " possiblyqualified collation name"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CollateClause() {
    assert_eq!(
        ::std::mem::size_of::<CollateClause>(),
        32usize,
        concat!("Size of: ", stringify!(CollateClause))
    );
    assert_eq!(
        ::std::mem::align_of::<CollateClause>(),
        8usize,
        concat!("Alignment of ", stringify!(CollateClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CollateClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CollateClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CollateClause>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CollateClause),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CollateClause>())).collname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CollateClause),
            "::",
            stringify!(collname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CollateClause>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CollateClause),
            "::",
            stringify!(location)
        )
    );
}
impl Default for CollateClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " RoleSpec  a role name or one of a few special values."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RoleSpecType {
    ROLESPEC_CSTRING = 0,
    #[doc = " role name is stored as a C string"]
    ROLESPEC_CURRENT_USER = 1,
    #[doc = " role spec is CURRENT_USER"]
    ROLESPEC_SESSION_USER = 2,
    #[doc = " role spec is SESSION_USER"]
    ROLESPEC_PUBLIC = 3,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RoleSpec {
    pub type_: NodeTag,
    pub roletype: RoleSpecType,
    #[doc = " Type of this rolespec"]
    pub rolename: *mut ::std::os::raw::c_char,
    #[doc = " filled only for ROLESPEC_CSTRING"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_RoleSpec() {
    assert_eq!(
        ::std::mem::size_of::<RoleSpec>(),
        24usize,
        concat!("Size of: ", stringify!(RoleSpec))
    );
    assert_eq!(
        ::std::mem::align_of::<RoleSpec>(),
        8usize,
        concat!("Alignment of ", stringify!(RoleSpec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RoleSpec>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RoleSpec),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RoleSpec>())).roletype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RoleSpec),
            "::",
            stringify!(roletype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RoleSpec>())).rolename as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RoleSpec),
            "::",
            stringify!(rolename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RoleSpec>())).location as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RoleSpec),
            "::",
            stringify!(location)
        )
    );
}
impl Default for RoleSpec {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " FuncCall  a function or aggregate invocation"]
#[doc = ""]
#[doc = " agg_order (if not NIL) indicates we saw 'foo(... ORDER BY ...)', or if"]
#[doc = " agg_within_group is true, it was 'foo(...) WITHIN GROUP (ORDER BY ...)'."]
#[doc = " agg_star indicates we saw a 'foo(*)' construct, while agg_distinct"]
#[doc = " indicates we saw 'foo(DISTINCT ...)'.  In any of these cases, the"]
#[doc = " construct *must* be an aggregate call.  Otherwise, it might be either an"]
#[doc = " aggregate or some other kind of function.  However, if FILTER or OVER is"]
#[doc = " present it had better be an aggregate or window function."]
#[doc = ""]
#[doc = " Normally, you'd initialize this via makeFuncCall() and then only change the"]
#[doc = " parts of the struct its defaults don't match afterwards, as needed."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FuncCall {
    pub type_: NodeTag,
    pub funcname: *mut List,
    #[doc = " qualified name of function"]
    pub args: *mut List,
    #[doc = " the arguments (list of exprs)"]
    pub agg_order: *mut List,
    #[doc = " ORDER BY (list of SortBy)"]
    pub agg_filter: *mut Node,
    #[doc = " FILTER clause, if any"]
    pub agg_within_group: bool,
    #[doc = " ORDER BY appeared in WITHIN GROUP"]
    pub agg_star: bool,
    #[doc = " argument was really '*'"]
    pub agg_distinct: bool,
    #[doc = " arguments were labeled DISTINCT"]
    pub func_variadic: bool,
    #[doc = " last argument was labeled VARIADIC"]
    pub over: *mut WindowDef,
    #[doc = " OVER clause, if any"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_FuncCall() {
    assert_eq!(
        ::std::mem::size_of::<FuncCall>(),
        64usize,
        concat!("Size of: ", stringify!(FuncCall))
    );
    assert_eq!(
        ::std::mem::align_of::<FuncCall>(),
        8usize,
        concat!("Alignment of ", stringify!(FuncCall))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).funcname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(funcname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).args as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).agg_order as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(agg_order)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).agg_filter as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(agg_filter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).agg_within_group as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(agg_within_group)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).agg_star as *const _ as usize },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(agg_star)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).agg_distinct as *const _ as usize },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(agg_distinct)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).func_variadic as *const _ as usize },
        43usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(func_variadic)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).over as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(over)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FuncCall>())).location as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(FuncCall),
            "::",
            stringify!(location)
        )
    );
}
impl Default for FuncCall {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " A_Star  '*' representing all columns of a table or compound field"]
#[doc = ""]
#[doc = " This can appear within ColumnRef.fields, A_Indirection.indirection, and"]
#[doc = " ResTarget.indirection lists."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct A_Star {
    pub type_: NodeTag,
}
#[test]
fn bindgen_test_layout_A_Star() {
    assert_eq!(
        ::std::mem::size_of::<A_Star>(),
        4usize,
        concat!("Size of: ", stringify!(A_Star))
    );
    assert_eq!(
        ::std::mem::align_of::<A_Star>(),
        4usize,
        concat!("Alignment of ", stringify!(A_Star))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Star>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Star),
            "::",
            stringify!(type_)
        )
    );
}
impl Default for A_Star {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " A_Indices  array subscript or slice bounds ([idx] or [lidx:uidx])"]
#[doc = ""]
#[doc = " In slice case, either or both of lidx and uidx can be NULL (omitted)."]
#[doc = " In nonslice case, uidx holds the single subscript and lidx is always NULL."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct A_Indices {
    pub type_: NodeTag,
    pub is_slice: bool,
    #[doc = " true if slice (i.e., colon present)"]
    pub lidx: *mut Node,
    #[doc = " slice lower bound, if any"]
    pub uidx: *mut Node,
}
#[test]
fn bindgen_test_layout_A_Indices() {
    assert_eq!(
        ::std::mem::size_of::<A_Indices>(),
        24usize,
        concat!("Size of: ", stringify!(A_Indices))
    );
    assert_eq!(
        ::std::mem::align_of::<A_Indices>(),
        8usize,
        concat!("Alignment of ", stringify!(A_Indices))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Indices>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Indices),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Indices>())).is_slice as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Indices),
            "::",
            stringify!(is_slice)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Indices>())).lidx as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Indices),
            "::",
            stringify!(lidx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Indices>())).uidx as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Indices),
            "::",
            stringify!(uidx)
        )
    );
}
impl Default for A_Indices {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " A_Indirection  select a field and/or array element from an expression"]
#[doc = ""]
#[doc = " The indirection list can contain A_Indices nodes (representing"]
#[doc = " subscripting), string Value nodes (representing field selection  the"]
#[doc = " string value is the name of the field to select), and A_Star nodes"]
#[doc = " (representing selection of all fields of a composite type)."]
#[doc = " For example, a complex selection operation like"]
#[doc = "          (foo).field1[42][7].field2"]
#[doc = " would be represented with a single A_Indirection node having a 4element"]
#[doc = " indirection list."]
#[doc = ""]
#[doc = " Currently, A_Star must appear only as the last list element  the grammar"]
#[doc = " is responsible for enforcing this!"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct A_Indirection {
    pub type_: NodeTag,
    pub arg: *mut Node,
    #[doc = " the thing being selected from"]
    pub indirection: *mut List,
}
#[test]
fn bindgen_test_layout_A_Indirection() {
    assert_eq!(
        ::std::mem::size_of::<A_Indirection>(),
        24usize,
        concat!("Size of: ", stringify!(A_Indirection))
    );
    assert_eq!(
        ::std::mem::align_of::<A_Indirection>(),
        8usize,
        concat!("Alignment of ", stringify!(A_Indirection))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Indirection>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Indirection),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Indirection>())).arg as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Indirection),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_Indirection>())).indirection as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(A_Indirection),
            "::",
            stringify!(indirection)
        )
    );
}
impl Default for A_Indirection {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " A_ArrayExpr  an ARRAY[] construct"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct A_ArrayExpr {
    pub type_: NodeTag,
    pub elements: *mut List,
    #[doc = " array element expressions"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_ArrayExpr() {
    assert_eq!(
        ::std::mem::size_of::<A_ArrayExpr>(),
        24usize,
        concat!("Size of: ", stringify!(A_ArrayExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<A_ArrayExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(A_ArrayExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_ArrayExpr>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(A_ArrayExpr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_ArrayExpr>())).elements as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(A_ArrayExpr),
            "::",
            stringify!(elements)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A_ArrayExpr>())).location as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(A_ArrayExpr),
            "::",
            stringify!(location)
        )
    );
}
impl Default for A_ArrayExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " ResTarget"]
#[doc = "   result target (used in target list of pretransformed parse trees)"]
#[doc = ""]
#[doc = " In a SELECT target list, 'name' is the column label from an"]
#[doc = " 'AS ColumnLabel' clause, or NULL if there was none, and 'val' is the"]
#[doc = " value expression itself.  The 'indirection' field is not used."]
#[doc = ""]
#[doc = " INSERT uses ResTarget in its targetcolumnnames list.  Here, 'name' is"]
#[doc = " the name of the destination column, 'indirection' stores any subscripts"]
#[doc = " attached to the destination, and 'val' is not used."]
#[doc = ""]
#[doc = " In an UPDATE target list, 'name' is the name of the destination column,"]
#[doc = " 'indirection' stores any subscripts attached to the destination, and"]
#[doc = " 'val' is the expression to assign."]
#[doc = ""]
#[doc = " See A_Indirection for more info about what can appear in 'indirection'."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ResTarget {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " column name or NULL"]
    pub indirection: *mut List,
    #[doc = " subscripts, field names, and '*', or NIL"]
    pub val: *mut Node,
    #[doc = " the value expression to compute or assign"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ResTarget() {
    assert_eq!(
        ::std::mem::size_of::<ResTarget>(),
        40usize,
        concat!("Size of: ", stringify!(ResTarget))
    );
    assert_eq!(
        ::std::mem::align_of::<ResTarget>(),
        8usize,
        concat!("Alignment of ", stringify!(ResTarget))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ResTarget>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ResTarget),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ResTarget>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ResTarget),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ResTarget>())).indirection as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ResTarget),
            "::",
            stringify!(indirection)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ResTarget>())).val as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ResTarget),
            "::",
            stringify!(val)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ResTarget>())).location as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ResTarget),
            "::",
            stringify!(location)
        )
    );
}
impl Default for ResTarget {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " MultiAssignRef  element of a row source expression for UPDATE"]
#[doc = ""]
#[doc = " In an UPDATE target list, when we have SET (a,b,c) = rowvaluedexpression,"]
#[doc = " we generate separate ResTarget items for each of a,b,c.  Their \"val\" trees"]
#[doc = " are MultiAssignRef nodes numbered 1..n, linking to a common copy of the"]
#[doc = " rowvaluedexpression (which parse analysis will process only once, when"]
#[doc = " handling the MultiAssignRef with colno=1)."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct MultiAssignRef {
    pub type_: NodeTag,
    pub source: *mut Node,
    #[doc = " the rowvalued expression"]
    pub colno: ::std::os::raw::c_int,
    #[doc = " column number for this target (1..n)"]
    pub ncolumns: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_MultiAssignRef() {
    assert_eq!(
        ::std::mem::size_of::<MultiAssignRef>(),
        24usize,
        concat!("Size of: ", stringify!(MultiAssignRef))
    );
    assert_eq!(
        ::std::mem::align_of::<MultiAssignRef>(),
        8usize,
        concat!("Alignment of ", stringify!(MultiAssignRef))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MultiAssignRef>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MultiAssignRef),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MultiAssignRef>())).source as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MultiAssignRef),
            "::",
            stringify!(source)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MultiAssignRef>())).colno as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MultiAssignRef),
            "::",
            stringify!(colno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MultiAssignRef>())).ncolumns as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(MultiAssignRef),
            "::",
            stringify!(ncolumns)
        )
    );
}
impl Default for MultiAssignRef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " SortBy  for ORDER BY clause"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SortBy {
    pub type_: NodeTag,
    pub node: *mut Node,
    #[doc = " expression to sort on"]
    pub sortby_dir: SortByDir,
    #[doc = " ASC/DESC/USING/default"]
    pub sortby_nulls: SortByNulls,
    #[doc = " NULLS FIRST/LAST"]
    pub useOp: *mut List,
    #[doc = " name of op to use, if SORTBY_USING"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_SortBy() {
    assert_eq!(
        ::std::mem::size_of::<SortBy>(),
        40usize,
        concat!("Size of: ", stringify!(SortBy))
    );
    assert_eq!(
        ::std::mem::align_of::<SortBy>(),
        8usize,
        concat!("Alignment of ", stringify!(SortBy))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortBy>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SortBy),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortBy>())).node as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SortBy),
            "::",
            stringify!(node)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortBy>())).sortby_dir as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SortBy),
            "::",
            stringify!(sortby_dir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortBy>())).sortby_nulls as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(SortBy),
            "::",
            stringify!(sortby_nulls)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortBy>())).useOp as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SortBy),
            "::",
            stringify!(useOp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortBy>())).location as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SortBy),
            "::",
            stringify!(location)
        )
    );
}
impl Default for SortBy {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " WindowDef  raw representation of WINDOW and OVER clauses"]
#[doc = ""]
#[doc = " For entries in a WINDOW list, \"name\" is the window name being defined."]
#[doc = " For OVER clauses, we use \"name\" for the \"OVER window\" syntax, or \"refname\""]
#[doc = " for the \"OVER (window)\" syntax, which is subtly different  the latter"]
#[doc = " implies overriding the window frame clause."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct WindowDef {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " window's own name"]
    pub refname: *mut ::std::os::raw::c_char,
    #[doc = " referenced window name, if any"]
    pub partitionClause: *mut List,
    #[doc = " PARTITION BY expression list"]
    pub orderClause: *mut List,
    #[doc = " ORDER BY (list of SortBy)"]
    pub frameOptions: ::std::os::raw::c_int,
    #[doc = " frame_clause options, see below"]
    pub startOffset: *mut Node,
    #[doc = " expression for starting bound, if any"]
    pub endOffset: *mut Node,
    #[doc = " expression for ending bound, if any"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_WindowDef() {
    assert_eq!(
        ::std::mem::size_of::<WindowDef>(),
        72usize,
        concat!("Size of: ", stringify!(WindowDef))
    );
    assert_eq!(
        ::std::mem::align_of::<WindowDef>(),
        8usize,
        concat!("Alignment of ", stringify!(WindowDef))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowDef>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowDef),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowDef>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowDef),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowDef>())).refname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowDef),
            "::",
            stringify!(refname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowDef>())).partitionClause as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowDef),
            "::",
            stringify!(partitionClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowDef>())).orderClause as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowDef),
            "::",
            stringify!(orderClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowDef>())).frameOptions as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowDef),
            "::",
            stringify!(frameOptions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowDef>())).startOffset as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowDef),
            "::",
            stringify!(startOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowDef>())).endOffset as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowDef),
            "::",
            stringify!(endOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowDef>())).location as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowDef),
            "::",
            stringify!(location)
        )
    );
}
impl Default for WindowDef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " RangeSubselect  subquery appearing in a FROM clause"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RangeSubselect {
    pub type_: NodeTag,
    pub lateral: bool,
    #[doc = " does it have LATERAL prefix?"]
    pub subquery: *mut Node,
    #[doc = " the untransformed subselect clause"]
    pub alias: *mut Alias,
}
#[test]
fn bindgen_test_layout_RangeSubselect() {
    assert_eq!(
        ::std::mem::size_of::<RangeSubselect>(),
        24usize,
        concat!("Size of: ", stringify!(RangeSubselect))
    );
    assert_eq!(
        ::std::mem::align_of::<RangeSubselect>(),
        8usize,
        concat!("Alignment of ", stringify!(RangeSubselect))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeSubselect>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeSubselect),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeSubselect>())).lateral as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeSubselect),
            "::",
            stringify!(lateral)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeSubselect>())).subquery as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeSubselect),
            "::",
            stringify!(subquery)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeSubselect>())).alias as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeSubselect),
            "::",
            stringify!(alias)
        )
    );
}
impl Default for RangeSubselect {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " RangeFunction  function call appearing in a FROM clause"]
#[doc = ""]
#[doc = " functions is a List because we use this to represent the construct"]
#[doc = " ROWS FROM(func1(...), func2(...), ...).  Each element of this list is a"]
#[doc = " twoelement sublist, the first element being the untransformed function"]
#[doc = " call tree, and the second element being a possiblyempty list of ColumnDef"]
#[doc = " nodes representing any columndef list attached to that function within the"]
#[doc = " ROWS FROM() syntax."]
#[doc = ""]
#[doc = " alias and coldeflist represent any alias and/or columndef list attached"]
#[doc = " at the top level.  (We disallow coldeflist appearing both here and"]
#[doc = " perfunction, but that's checked in parse analysis, not by the grammar.)"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RangeFunction {
    pub type_: NodeTag,
    pub lateral: bool,
    #[doc = " does it have LATERAL prefix?"]
    pub ordinality: bool,
    #[doc = " does it have WITH ORDINALITY suffix?"]
    pub is_rowsfrom: bool,
    #[doc = " is result of ROWS FROM() syntax?"]
    pub functions: *mut List,
    #[doc = " perfunction information, see above"]
    pub alias: *mut Alias,
    #[doc = " table alias & optional column aliases"]
    pub coldeflist: *mut List,
}
#[test]
fn bindgen_test_layout_RangeFunction() {
    assert_eq!(
        ::std::mem::size_of::<RangeFunction>(),
        32usize,
        concat!("Size of: ", stringify!(RangeFunction))
    );
    assert_eq!(
        ::std::mem::align_of::<RangeFunction>(),
        8usize,
        concat!("Alignment of ", stringify!(RangeFunction))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeFunction>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeFunction),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeFunction>())).lateral as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeFunction),
            "::",
            stringify!(lateral)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeFunction>())).ordinality as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeFunction),
            "::",
            stringify!(ordinality)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeFunction>())).is_rowsfrom as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeFunction),
            "::",
            stringify!(is_rowsfrom)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeFunction>())).functions as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeFunction),
            "::",
            stringify!(functions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeFunction>())).alias as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeFunction),
            "::",
            stringify!(alias)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeFunction>())).coldeflist as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeFunction),
            "::",
            stringify!(coldeflist)
        )
    );
}
impl Default for RangeFunction {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " RangeTableFunc  raw form of \"table functions\" such as XMLTABLE"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RangeTableFunc {
    pub type_: NodeTag,
    pub lateral: bool,
    #[doc = " does it have LATERAL prefix?"]
    pub docexpr: *mut Node,
    #[doc = " document expression"]
    pub rowexpr: *mut Node,
    #[doc = " row generator expression"]
    pub namespaces: *mut List,
    #[doc = " list of namespaces as ResTarget"]
    pub columns: *mut List,
    #[doc = " list of RangeTableFuncCol"]
    pub alias: *mut Alias,
    #[doc = " table alias & optional column aliases"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_RangeTableFunc() {
    assert_eq!(
        ::std::mem::size_of::<RangeTableFunc>(),
        56usize,
        concat!("Size of: ", stringify!(RangeTableFunc))
    );
    assert_eq!(
        ::std::mem::align_of::<RangeTableFunc>(),
        8usize,
        concat!("Alignment of ", stringify!(RangeTableFunc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFunc>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFunc),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFunc>())).lateral as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFunc),
            "::",
            stringify!(lateral)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFunc>())).docexpr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFunc),
            "::",
            stringify!(docexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFunc>())).rowexpr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFunc),
            "::",
            stringify!(rowexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFunc>())).namespaces as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFunc),
            "::",
            stringify!(namespaces)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFunc>())).columns as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFunc),
            "::",
            stringify!(columns)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFunc>())).alias as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFunc),
            "::",
            stringify!(alias)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFunc>())).location as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFunc),
            "::",
            stringify!(location)
        )
    );
}
impl Default for RangeTableFunc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " RangeTableFuncCol  one column in a RangeTableFunc>columns"]
#[doc = ""]
#[doc = " If for_ordinality is true (FOR ORDINALITY), then the column is an int4"]
#[doc = " column and the rest of the fields are ignored."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RangeTableFuncCol {
    pub type_: NodeTag,
    pub colname: *mut ::std::os::raw::c_char,
    #[doc = " name of generated column"]
    pub typeName: *mut TypeName,
    #[doc = " type of generated column"]
    pub for_ordinality: bool,
    #[doc = " does it have FOR ORDINALITY?"]
    pub is_not_null: bool,
    #[doc = " does it have NOT NULL?"]
    pub colexpr: *mut Node,
    #[doc = " column filter expression"]
    pub coldefexpr: *mut Node,
    #[doc = " column default value expression"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_RangeTableFuncCol() {
    assert_eq!(
        ::std::mem::size_of::<RangeTableFuncCol>(),
        56usize,
        concat!("Size of: ", stringify!(RangeTableFuncCol))
    );
    assert_eq!(
        ::std::mem::align_of::<RangeTableFuncCol>(),
        8usize,
        concat!("Alignment of ", stringify!(RangeTableFuncCol))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFuncCol>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFuncCol),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFuncCol>())).colname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFuncCol),
            "::",
            stringify!(colname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFuncCol>())).typeName as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFuncCol),
            "::",
            stringify!(typeName)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<RangeTableFuncCol>())).for_ordinality as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFuncCol),
            "::",
            stringify!(for_ordinality)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFuncCol>())).is_not_null as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFuncCol),
            "::",
            stringify!(is_not_null)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFuncCol>())).colexpr as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFuncCol),
            "::",
            stringify!(colexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFuncCol>())).coldefexpr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFuncCol),
            "::",
            stringify!(coldefexpr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableFuncCol>())).location as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableFuncCol),
            "::",
            stringify!(location)
        )
    );
}
impl Default for RangeTableFuncCol {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " RangeTableSample  TABLESAMPLE appearing in a raw FROM clause"]
#[doc = ""]
#[doc = " This node, appearing only in raw parse trees, represents"]
#[doc = "    <relation> TABLESAMPLE <method> (<params>) REPEATABLE (<num>)"]
#[doc = " Currently, the <relation> can only be a RangeVar, but we might in future"]
#[doc = " allow RangeSubselect and other options.  Note that the RangeTableSample"]
#[doc = " is wrapped around the node representing the <relation>, rather than being"]
#[doc = " a subfield of it."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RangeTableSample {
    pub type_: NodeTag,
    pub relation: *mut Node,
    #[doc = " relation to be sampled"]
    pub method: *mut List,
    #[doc = " sampling method name (possibly qualified)"]
    pub args: *mut List,
    #[doc = " argument(s) for sampling method"]
    pub repeatable: *mut Node,
    #[doc = " REPEATABLE expression, or NULL if none"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_RangeTableSample() {
    assert_eq!(
        ::std::mem::size_of::<RangeTableSample>(),
        48usize,
        concat!("Size of: ", stringify!(RangeTableSample))
    );
    assert_eq!(
        ::std::mem::align_of::<RangeTableSample>(),
        8usize,
        concat!("Alignment of ", stringify!(RangeTableSample))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableSample>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableSample),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableSample>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableSample),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableSample>())).method as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableSample),
            "::",
            stringify!(method)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableSample>())).args as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableSample),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableSample>())).repeatable as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableSample),
            "::",
            stringify!(repeatable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RangeTableSample>())).location as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RangeTableSample),
            "::",
            stringify!(location)
        )
    );
}
impl Default for RangeTableSample {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " ColumnDef  column definition (used in various creates)"]
#[doc = ""]
#[doc = " If the column has a default value, we may have the value expression"]
#[doc = " in either \"raw\" form (an untransformed parse tree) or \"cooked\" form"]
#[doc = " (a postparseanalysis, executable expression tree), depending on"]
#[doc = " how this ColumnDef node was created (by parsing, or by inheritance"]
#[doc = " from an existing relation).  We should never have both in the same node!"]
#[doc = ""]
#[doc = " Similarly, we may have a COLLATE specification in either raw form"]
#[doc = " (represented as a CollateClause with arg==NULL) or cooked form"]
#[doc = " (the collation's OID)."]
#[doc = ""]
#[doc = " The constraints list may contain a CONSTR_DEFAULT item in a raw"]
#[doc = " parsetree produced by gram.y, but transformCreateStmt will remove"]
#[doc = " the item and set raw_default instead.  CONSTR_DEFAULT items"]
#[doc = " should not appear in any subsequent processing."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ColumnDef {
    pub type_: NodeTag,
    pub colname: *mut ::std::os::raw::c_char,
    #[doc = " name of column"]
    pub typeName: *mut TypeName,
    #[doc = " type of column"]
    pub inhcount: ::std::os::raw::c_int,
    #[doc = " number of times column is inherited"]
    pub is_local: bool,
    #[doc = " column has local (noninherited) def'n"]
    pub is_not_null: bool,
    #[doc = " NOT NULL constraint specified?"]
    pub is_from_type: bool,
    #[doc = " column definition came from table type"]
    pub storage: ::std::os::raw::c_char,
    #[doc = " attstorage setting, or 0 for default"]
    pub raw_default: *mut Node,
    #[doc = " default value (untransformed parse tree)"]
    pub cooked_default: *mut Node,
    #[doc = " default value (transformed expr tree)"]
    pub identity: ::std::os::raw::c_char,
    #[doc = " attidentity setting"]
    pub identitySequence: *mut RangeVar,
    #[doc = " to store identity sequence name for"]
    #[doc = " ALTER TABLE ... ADD COLUMN"]
    pub generated: ::std::os::raw::c_char,
    #[doc = " attgenerated setting"]
    pub collClause: *mut CollateClause,
    #[doc = " untransformed COLLATE spec, if any"]
    pub collOid: Oid,
    #[doc = " collation OID (InvalidOid if not set)"]
    pub constraints: *mut List,
    #[doc = " other constraints on column"]
    pub fdwoptions: *mut List,
    #[doc = " percolumn FDW options"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ColumnDef() {
    assert_eq!(
        ::std::mem::size_of::<ColumnDef>(),
        112usize,
        concat!("Size of: ", stringify!(ColumnDef))
    );
    assert_eq!(
        ::std::mem::align_of::<ColumnDef>(),
        8usize,
        concat!("Alignment of ", stringify!(ColumnDef))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).colname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(colname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).typeName as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(typeName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).inhcount as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(inhcount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).is_local as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(is_local)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).is_not_null as *const _ as usize },
        29usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(is_not_null)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).is_from_type as *const _ as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(is_from_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).storage as *const _ as usize },
        31usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(storage)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).raw_default as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(raw_default)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).cooked_default as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(cooked_default)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).identity as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(identity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).identitySequence as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(identitySequence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).generated as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(generated)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).collClause as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(collClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).collOid as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(collOid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).constraints as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(constraints)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).fdwoptions as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(fdwoptions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ColumnDef>())).location as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(ColumnDef),
            "::",
            stringify!(location)
        )
    );
}
impl Default for ColumnDef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " TableLikeClause  CREATE TABLE ( ... LIKE ... ) clause"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TableLikeClause {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    pub options: bits32,
}
#[test]
fn bindgen_test_layout_TableLikeClause() {
    assert_eq!(
        ::std::mem::size_of::<TableLikeClause>(),
        24usize,
        concat!("Size of: ", stringify!(TableLikeClause))
    );
    assert_eq!(
        ::std::mem::align_of::<TableLikeClause>(),
        8usize,
        concat!("Alignment of ", stringify!(TableLikeClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableLikeClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TableLikeClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableLikeClause>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TableLikeClause),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableLikeClause>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TableLikeClause),
            "::",
            stringify!(options)
        )
    );
}
impl Default for TableLikeClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TableLikeOption {
    CREATE_TABLE_LIKE_COMMENTS = 1,
    CREATE_TABLE_LIKE_CONSTRAINTS = 2,
    CREATE_TABLE_LIKE_DEFAULTS = 4,
    CREATE_TABLE_LIKE_GENERATED = 8,
    CREATE_TABLE_LIKE_IDENTITY = 16,
    CREATE_TABLE_LIKE_INDEXES = 32,
    CREATE_TABLE_LIKE_STATISTICS = 64,
    CREATE_TABLE_LIKE_STORAGE = 128,
    CREATE_TABLE_LIKE_ALL = 2147483647,
}
#[doc = " IndexElem  index parameters (used in CREATE INDEX, and in ON CONFLICT)"]
#[doc = ""]
#[doc = " For a plain index attribute, 'name' is the name of the table column to"]
#[doc = " index, and 'expr' is NULL.  For an index expression, 'name' is NULL and"]
#[doc = " 'expr' is the expression tree."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct IndexElem {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " name of attribute to index, or NULL"]
    pub expr: *mut Node,
    #[doc = " expression to index, or NULL"]
    pub indexcolname: *mut ::std::os::raw::c_char,
    #[doc = " name for index column; NULL = default"]
    pub collation: *mut List,
    #[doc = " name of collation; NIL = default"]
    pub opclass: *mut List,
    #[doc = " name of desired opclass; NIL = default"]
    pub ordering: SortByDir,
    #[doc = " ASC/DESC/default"]
    pub nulls_ordering: SortByNulls,
}
#[test]
fn bindgen_test_layout_IndexElem() {
    assert_eq!(
        ::std::mem::size_of::<IndexElem>(),
        56usize,
        concat!("Size of: ", stringify!(IndexElem))
    );
    assert_eq!(
        ::std::mem::align_of::<IndexElem>(),
        8usize,
        concat!("Alignment of ", stringify!(IndexElem))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexElem>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexElem),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexElem>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexElem),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexElem>())).expr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexElem),
            "::",
            stringify!(expr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexElem>())).indexcolname as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexElem),
            "::",
            stringify!(indexcolname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexElem>())).collation as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexElem),
            "::",
            stringify!(collation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexElem>())).opclass as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexElem),
            "::",
            stringify!(opclass)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexElem>())).ordering as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexElem),
            "::",
            stringify!(ordering)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexElem>())).nulls_ordering as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexElem),
            "::",
            stringify!(nulls_ordering)
        )
    );
}
impl Default for IndexElem {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " DefElem  a generic \"name = value\" option definition"]
#[doc = ""]
#[doc = " In some contexts the name can be qualified.  Also, certain SQL commands"]
#[doc = " allow a SET/ADD/DROP action to be attached to option settings, so it's"]
#[doc = " convenient to carry a field for that too.  (Note: currently, it is our"]
#[doc = " practice that the grammar allows namespace and action only in statements"]
#[doc = " where they are relevant; C code can just ignore those fields in other"]
#[doc = " statements.)"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DefElemAction {
    DEFELEM_UNSPEC = 0,
    #[doc = " no action given"]
    DEFELEM_SET = 1,
    #[doc = " no action given"]
    DEFELEM_ADD = 2,
    #[doc = " no action given"]
    DEFELEM_DROP = 3,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DefElem {
    pub type_: NodeTag,
    pub defnamespace: *mut ::std::os::raw::c_char,
    #[doc = " NULL if unqualified name"]
    pub defname: *mut ::std::os::raw::c_char,
    pub arg: *mut Node,
    #[doc = " a (Value *) or a (TypeName *)"]
    pub defaction: DefElemAction,
    #[doc = " unspecified action, or SET/ADD/DROP"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_DefElem() {
    assert_eq!(
        ::std::mem::size_of::<DefElem>(),
        40usize,
        concat!("Size of: ", stringify!(DefElem))
    );
    assert_eq!(
        ::std::mem::align_of::<DefElem>(),
        8usize,
        concat!("Alignment of ", stringify!(DefElem))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefElem>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DefElem),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefElem>())).defnamespace as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DefElem),
            "::",
            stringify!(defnamespace)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefElem>())).defname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DefElem),
            "::",
            stringify!(defname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefElem>())).arg as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DefElem),
            "::",
            stringify!(arg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefElem>())).defaction as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(DefElem),
            "::",
            stringify!(defaction)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefElem>())).location as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(DefElem),
            "::",
            stringify!(location)
        )
    );
}
impl Default for DefElem {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " LockingClause  raw representation of FOR [NO KEY] UPDATE/[KEY] SHARE"]
#[doc = "    options"]
#[doc = ""]
#[doc = " Note: lockedRels == NIL means \"all relations in query\".  Otherwise it"]
#[doc = " is a list of RangeVar nodes.  (We use RangeVar mainly because it carries"]
#[doc = " a location field  currently, parse analysis insists on unqualified"]
#[doc = " names in LockingClause.)"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct LockingClause {
    pub type_: NodeTag,
    pub lockedRels: *mut List,
    #[doc = " FOR [KEY] UPDATE/SHARE relations"]
    pub strength: LockClauseStrength,
    pub waitPolicy: LockWaitPolicy,
}
#[test]
fn bindgen_test_layout_LockingClause() {
    assert_eq!(
        ::std::mem::size_of::<LockingClause>(),
        24usize,
        concat!("Size of: ", stringify!(LockingClause))
    );
    assert_eq!(
        ::std::mem::align_of::<LockingClause>(),
        8usize,
        concat!("Alignment of ", stringify!(LockingClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<LockingClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(LockingClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<LockingClause>())).lockedRels as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(LockingClause),
            "::",
            stringify!(lockedRels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<LockingClause>())).strength as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(LockingClause),
            "::",
            stringify!(strength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<LockingClause>())).waitPolicy as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(LockingClause),
            "::",
            stringify!(waitPolicy)
        )
    );
}
impl Default for LockingClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " XMLSERIALIZE (in raw parse tree only)"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct XmlSerialize {
    pub type_: NodeTag,
    pub xmloption: XmlOptionType,
    #[doc = " DOCUMENT or CONTENT"]
    pub expr: *mut Node,
    pub typeName: *mut TypeName,
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_XmlSerialize() {
    assert_eq!(
        ::std::mem::size_of::<XmlSerialize>(),
        32usize,
        concat!("Size of: ", stringify!(XmlSerialize))
    );
    assert_eq!(
        ::std::mem::align_of::<XmlSerialize>(),
        8usize,
        concat!("Alignment of ", stringify!(XmlSerialize))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlSerialize>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlSerialize),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlSerialize>())).xmloption as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlSerialize),
            "::",
            stringify!(xmloption)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlSerialize>())).expr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlSerialize),
            "::",
            stringify!(expr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlSerialize>())).typeName as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlSerialize),
            "::",
            stringify!(typeName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<XmlSerialize>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(XmlSerialize),
            "::",
            stringify!(location)
        )
    );
}
impl Default for XmlSerialize {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " PartitionElem  parsetime representation of a single partition key"]
#[doc = ""]
#[doc = " expr can be either a raw expression tree or a parseanalyzed expression."]
#[doc = " We don't store these ondisk, though."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PartitionElem {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " name of column to partition on, or NULL"]
    pub expr: *mut Node,
    #[doc = " expression to partition on, or NULL"]
    pub collation: *mut List,
    #[doc = " name of collation; NIL = default"]
    pub opclass: *mut List,
    #[doc = " name of desired opclass; NIL = default"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_PartitionElem() {
    assert_eq!(
        ::std::mem::size_of::<PartitionElem>(),
        48usize,
        concat!("Size of: ", stringify!(PartitionElem))
    );
    assert_eq!(
        ::std::mem::align_of::<PartitionElem>(),
        8usize,
        concat!("Alignment of ", stringify!(PartitionElem))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionElem>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionElem),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionElem>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionElem),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionElem>())).expr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionElem),
            "::",
            stringify!(expr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionElem>())).collation as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionElem),
            "::",
            stringify!(collation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionElem>())).opclass as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionElem),
            "::",
            stringify!(opclass)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionElem>())).location as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionElem),
            "::",
            stringify!(location)
        )
    );
}
impl Default for PartitionElem {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " PartitionSpec  parsetime representation of a partition key specification"]
#[doc = ""]
#[doc = " This represents the key space we will be partitioning on."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PartitionSpec {
    pub type_: NodeTag,
    pub strategy: *mut ::std::os::raw::c_char,
    #[doc = " partitioning strategy ('hash', 'list' or"]
    #[doc = " 'range')"]
    pub partParams: *mut List,
    #[doc = " List of PartitionElems"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_PartitionSpec() {
    assert_eq!(
        ::std::mem::size_of::<PartitionSpec>(),
        32usize,
        concat!("Size of: ", stringify!(PartitionSpec))
    );
    assert_eq!(
        ::std::mem::align_of::<PartitionSpec>(),
        8usize,
        concat!("Alignment of ", stringify!(PartitionSpec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionSpec>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionSpec),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionSpec>())).strategy as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionSpec),
            "::",
            stringify!(strategy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionSpec>())).partParams as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionSpec),
            "::",
            stringify!(partParams)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionSpec>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionSpec),
            "::",
            stringify!(location)
        )
    );
}
impl Default for PartitionSpec {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " PartitionBoundSpec  a partition bound specification"]
#[doc = ""]
#[doc = " This represents the portion of the partition key space assigned to a"]
#[doc = " particular partition.  These are stored on disk in pg_class.relpartbound."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PartitionBoundSpec {
    pub type_: NodeTag,
    pub strategy: ::std::os::raw::c_char,
    #[doc = " see PARTITION_STRATEGY codes above"]
    pub is_default: bool,
    #[doc = " Partitioning info for HASH strategy:"]
    pub modulus: ::std::os::raw::c_int,
    pub remainder: ::std::os::raw::c_int,
    #[doc = " Partitioning info for LIST strategy:"]
    pub listdatums: *mut List,
    #[doc = " Partitioning info for RANGE strategy:"]
    pub lowerdatums: *mut List,
    #[doc = " List of PartitionRangeDatums"]
    pub upperdatums: *mut List,
    #[doc = " List of PartitionRangeDatums"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_PartitionBoundSpec() {
    assert_eq!(
        ::std::mem::size_of::<PartitionBoundSpec>(),
        48usize,
        concat!("Size of: ", stringify!(PartitionBoundSpec))
    );
    assert_eq!(
        ::std::mem::align_of::<PartitionBoundSpec>(),
        8usize,
        concat!("Alignment of ", stringify!(PartitionBoundSpec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionBoundSpec>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionBoundSpec),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionBoundSpec>())).strategy as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionBoundSpec),
            "::",
            stringify!(strategy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionBoundSpec>())).is_default as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionBoundSpec),
            "::",
            stringify!(is_default)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionBoundSpec>())).modulus as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionBoundSpec),
            "::",
            stringify!(modulus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionBoundSpec>())).remainder as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionBoundSpec),
            "::",
            stringify!(remainder)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionBoundSpec>())).listdatums as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionBoundSpec),
            "::",
            stringify!(listdatums)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionBoundSpec>())).lowerdatums as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionBoundSpec),
            "::",
            stringify!(lowerdatums)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionBoundSpec>())).upperdatums as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionBoundSpec),
            "::",
            stringify!(upperdatums)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionBoundSpec>())).location as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionBoundSpec),
            "::",
            stringify!(location)
        )
    );
}
impl Default for PartitionBoundSpec {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl PartitionRangeDatumKind {
    pub const PARTITION_RANGE_DATUM_MAXVALUE: PartitionRangeDatumKind =
        PartitionRangeDatumKind::PARTITION_RANGE_DATUM_MINVALUE;
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " PartitionRangeDatum  one of the values in a range partition bound"]
#[doc = ""]
#[doc = " This can be MINVALUE, MAXVALUE or a specific bounded value."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PartitionRangeDatumKind {
    PARTITION_RANGE_DATUM_MINVALUE = 1,
    #[doc = " less than any other value"]
    PARTITION_RANGE_DATUM_VALUE = 0,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PartitionRangeDatum {
    pub type_: NodeTag,
    pub kind: PartitionRangeDatumKind,
    pub value: *mut Node,
    #[doc = " Const (or A_Const in raw tree), if kind is"]
    #[doc = " PARTITION_RANGE_DATUM_VALUE, else NULL"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_PartitionRangeDatum() {
    assert_eq!(
        ::std::mem::size_of::<PartitionRangeDatum>(),
        24usize,
        concat!("Size of: ", stringify!(PartitionRangeDatum))
    );
    assert_eq!(
        ::std::mem::align_of::<PartitionRangeDatum>(),
        8usize,
        concat!("Alignment of ", stringify!(PartitionRangeDatum))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionRangeDatum>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionRangeDatum),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionRangeDatum>())).kind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionRangeDatum),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionRangeDatum>())).value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionRangeDatum),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionRangeDatum>())).location as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionRangeDatum),
            "::",
            stringify!(location)
        )
    );
}
impl Default for PartitionRangeDatum {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " PartitionCmd  info for ALTER TABLE/INDEX ATTACH/DETACH PARTITION commands"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PartitionCmd {
    pub type_: NodeTag,
    pub name: *mut RangeVar,
    #[doc = " name of partition to attach/detach"]
    pub bound: *mut PartitionBoundSpec,
}
#[test]
fn bindgen_test_layout_PartitionCmd() {
    assert_eq!(
        ::std::mem::size_of::<PartitionCmd>(),
        24usize,
        concat!("Size of: ", stringify!(PartitionCmd))
    );
    assert_eq!(
        ::std::mem::align_of::<PartitionCmd>(),
        8usize,
        concat!("Alignment of ", stringify!(PartitionCmd))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionCmd>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionCmd),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionCmd>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionCmd),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PartitionCmd>())).bound as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PartitionCmd),
            "::",
            stringify!(bound)
        )
    );
}
impl Default for PartitionCmd {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " RangeTblEntry"]
#[doc = "   A range table is a List of RangeTblEntry nodes."]
#[doc = ""]
#[doc = "   A range table entry may represent a plain relation, a subselect in"]
#[doc = "   FROM, or the result of a JOIN clause.  (Only explicit JOIN syntax"]
#[doc = "   produces an RTE, not the implicit join resulting from multiple FROM"]
#[doc = "   items.  This is because we only need the RTE to deal with SQL features"]
#[doc = "   like outer joins and joinoutputcolumn aliasing.)  Other special"]
#[doc = "   RTE types also exist, as indicated by RTEKind."]
#[doc = ""]
#[doc = "   Note that we consider RTE_RELATION to cover anything that has a pg_class"]
#[doc = "   entry.  relkind distinguishes the subcases."]
#[doc = ""]
#[doc = "   alias is an Alias node representing the AS aliasclause attached to the"]
#[doc = "   FROM expression, or NULL if no clause."]
#[doc = ""]
#[doc = "   eref is the table reference name and column reference names (either"]
#[doc = "   real or aliases).  Note that system columns (OID etc) are not included"]
#[doc = "   in the column list."]
#[doc = "   eref>aliasname is required to be present, and should generally be used"]
#[doc = "   to identify the RTE for error messages etc."]
#[doc = ""]
#[doc = "   In RELATION RTEs, the colnames in both alias and eref are indexed by"]
#[doc = "   physical attribute number; this means there must be colname entries for"]
#[doc = "   dropped columns.  When building an RTE we insert empty strings (\"\") for"]
#[doc = "   dropped columns.  Note however that a stored rule may have nonempty"]
#[doc = "   colnames for columns dropped since the rule was created (and for that"]
#[doc = "   matter the colnames might be out of date due to column renamings)."]
#[doc = "   The same comments apply to FUNCTION RTEs when a function's return type"]
#[doc = "   is a named composite type."]
#[doc = ""]
#[doc = "   In JOIN RTEs, the colnames in both alias and eref are onetoone with"]
#[doc = "   joinaliasvars entries.  A JOIN RTE will omit columns of its inputs when"]
#[doc = "   those columns are known to be dropped at parse time.  Again, however,"]
#[doc = "   a stored rule might contain entries for columns dropped since the rule"]
#[doc = "   was created.  (This is only possible for columns not actually referenced"]
#[doc = "   in the rule.)  When loading a stored rule, we replace the joinaliasvars"]
#[doc = "   items for any such columns with null pointers.  (We can't simply delete"]
#[doc = "   them from the joinaliasvars list, because that would affect the attnums"]
#[doc = "   of Vars referencing the rest of the list.)"]
#[doc = ""]
#[doc = "   inh is true for relation references that should be expanded to include"]
#[doc = "   inheritance children, if the rel has any.  This *must* be false for"]
#[doc = "   RTEs other than RTE_RELATION entries."]
#[doc = ""]
#[doc = "   inFromCl marks those range variables that are listed in the FROM clause."]
#[doc = "   It's false for RTEs that are added to a query behind the scenes, such"]
#[doc = "   as the NEW and OLD variables for a rule, or the subqueries of a UNION."]
#[doc = "   This flag is not used anymore during parsing, since the parser now uses"]
#[doc = "   a separate \"namespace\" data structure to control visibility, but it is"]
#[doc = "   needed by ruleutils.c to determine whether RTEs should be shown in"]
#[doc = "   decompiled queries."]
#[doc = ""]
#[doc = "   requiredPerms and checkAsUser specify runtime access permissions"]
#[doc = "   checks to be performed at query startup.  The user must have *all*"]
#[doc = "   of the permissions that are OR'd together in requiredPerms (zero"]
#[doc = "   indicates no permissions checking).  If checkAsUser is not zero,"]
#[doc = "   then do the permissions checks using the access rights of that user,"]
#[doc = "   not the current effective user ID.  (This allows rules to act as"]
#[doc = "   setuid gateways.)  Permissions checks only apply to RELATION RTEs."]
#[doc = ""]
#[doc = "   For SELECT/INSERT/UPDATE permissions, if the user doesn't have"]
#[doc = "   tablewide permissions then it is sufficient to have the permissions"]
#[doc = "   on all columns identified in selectedCols (for SELECT) and/or"]
#[doc = "   insertedCols and/or updatedCols (INSERT with ON CONFLICT DO UPDATE may"]
#[doc = "   have all 3).  selectedCols, insertedCols and updatedCols are bitmapsets,"]
#[doc = "   which cannot have negative integer members, so we subtract"]
#[doc = "   FirstLowInvalidHeapAttributeNumber from column numbers before storing"]
#[doc = "   them in these fields.  A wholerow Var reference is represented by"]
#[doc = "   setting the bit for InvalidAttrNumber."]
#[doc = ""]
#[doc = "   updatedCols is also used in some other places, for example, to determine"]
#[doc = "   which triggers to fire and in FDWs to know which changed columns they"]
#[doc = "   need to ship off.  Generated columns that are caused to be updated by an"]
#[doc = "   update to a base column are collected in extraUpdatedCols.  This is not"]
#[doc = "   considered for permission checking, but it is useful in those places"]
#[doc = "   that want to know the full set of columns being updated as opposed to"]
#[doc = "   only the ones the user explicitly mentioned in the query.  (There is"]
#[doc = "   currently no need for an extraInsertedCols, but it could exist.)"]
#[doc = ""]
#[doc = "   securityQuals is a list of security barrier quals (boolean expressions),"]
#[doc = "   to be tested in the listed order before returning a row from the"]
#[doc = "   relation.  It is always NIL in parser output.  Entries are added by the"]
#[doc = "   rewriter to implement securitybarrier views and/or rowlevel security."]
#[doc = "   Note that the planner turns each boolean expression into an implicitly"]
#[doc = "   AND'ed sublist, as is its usual habit with qualification expressions."]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RTEKind {
    RTE_RELATION = 0,
    #[doc = " ordinary relation reference"]
    RTE_SUBQUERY = 1,
    #[doc = " subquery in FROM"]
    RTE_JOIN = 2,
    #[doc = " join"]
    RTE_FUNCTION = 3,
    #[doc = " function in FROM"]
    RTE_TABLEFUNC = 4,
    #[doc = " TableFunc(.., column list)"]
    RTE_VALUES = 5,
    #[doc = " VALUES (<exprlist>), (<exprlist>), ..."]
    RTE_CTE = 6,
    #[doc = " common table expr (WITH list element)"]
    RTE_NAMEDTUPLESTORE = 7,
    #[doc = " tuplestore, e.g. for AFTER triggers"]
    RTE_RESULT = 8,
}
#[doc = " TableSampleClause  TABLESAMPLE appearing in a transformed FROM clause"]
#[doc = ""]
#[doc = " Unlike RangeTableSample, this is a subnode of the relevant RangeTblEntry."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TableSampleClause {
    pub type_: NodeTag,
    pub tsmhandler: Oid,
    #[doc = " OID of the tablesample handler function"]
    pub args: *mut List,
    #[doc = " tablesample argument expression(s)"]
    pub repeatable: *mut Expr,
}
#[test]
fn bindgen_test_layout_TableSampleClause() {
    assert_eq!(
        ::std::mem::size_of::<TableSampleClause>(),
        24usize,
        concat!("Size of: ", stringify!(TableSampleClause))
    );
    assert_eq!(
        ::std::mem::align_of::<TableSampleClause>(),
        8usize,
        concat!("Alignment of ", stringify!(TableSampleClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableSampleClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TableSampleClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableSampleClause>())).tsmhandler as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TableSampleClause),
            "::",
            stringify!(tsmhandler)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableSampleClause>())).args as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TableSampleClause),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TableSampleClause>())).repeatable as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TableSampleClause),
            "::",
            stringify!(repeatable)
        )
    );
}
impl Default for TableSampleClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " WithCheckOption"]
#[doc = "    representation of WITH CHECK OPTION checks to be applied to new tuples"]
#[doc = "    when inserting/updating an autoupdatable view, or RLS WITH CHECK"]
#[doc = "    policies to be applied when inserting/updating a relation with RLS."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WCOKind {
    WCO_VIEW_CHECK = 0,
    #[doc = " WCO on an autoupdatable view"]
    WCO_RLS_INSERT_CHECK = 1,
    #[doc = " RLS INSERT WITH CHECK policy"]
    WCO_RLS_UPDATE_CHECK = 2,
    #[doc = " RLS UPDATE WITH CHECK policy"]
    WCO_RLS_CONFLICT_CHECK = 3,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct WithCheckOption {
    pub type_: NodeTag,
    pub kind: WCOKind,
    #[doc = " kind of WCO"]
    pub relname: *mut ::std::os::raw::c_char,
    #[doc = " name of relation that specified the WCO"]
    pub polname: *mut ::std::os::raw::c_char,
    #[doc = " name of RLS policy being checked"]
    pub qual: *mut Node,
    #[doc = " constraint qual to check"]
    pub cascaded: bool,
}
#[test]
fn bindgen_test_layout_WithCheckOption() {
    assert_eq!(
        ::std::mem::size_of::<WithCheckOption>(),
        40usize,
        concat!("Size of: ", stringify!(WithCheckOption))
    );
    assert_eq!(
        ::std::mem::align_of::<WithCheckOption>(),
        8usize,
        concat!("Alignment of ", stringify!(WithCheckOption))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithCheckOption>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithCheckOption),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithCheckOption>())).kind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(WithCheckOption),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithCheckOption>())).relname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WithCheckOption),
            "::",
            stringify!(relname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithCheckOption>())).polname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WithCheckOption),
            "::",
            stringify!(polname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithCheckOption>())).qual as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(WithCheckOption),
            "::",
            stringify!(qual)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithCheckOption>())).cascaded as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(WithCheckOption),
            "::",
            stringify!(cascaded)
        )
    );
}
impl Default for WithCheckOption {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " SortGroupClause"]
#[doc = "    representation of ORDER BY, GROUP BY, PARTITION BY,"]
#[doc = "    DISTINCT, DISTINCT ON items"]
#[doc = ""]
#[doc = " You might think that ORDER BY is only interested in defining ordering,"]
#[doc = " and GROUP/DISTINCT are only interested in defining equality.  However,"]
#[doc = " one way to implement grouping is to sort and then apply a \"uniq\"like"]
#[doc = " filter.  So it's also interesting to keep track of possible sort operators"]
#[doc = " for GROUP/DISTINCT, and in particular to try to sort for the grouping"]
#[doc = " in a way that will also yield a requested ORDER BY ordering.  So we need"]
#[doc = " to be able to compare ORDER BY and GROUP/DISTINCT lists, which motivates"]
#[doc = " the decision to give them the same representation."]
#[doc = ""]
#[doc = " tleSortGroupRef must match ressortgroupref of exactly one entry of the"]
#[doc = "    query's targetlist; that is the expression to be sorted or grouped by."]
#[doc = " eqop is the OID of the equality operator."]
#[doc = " sortop is the OID of the ordering operator (a \"<\" or \">\" operator),"]
#[doc = "    or InvalidOid if not available."]
#[doc = " nulls_first means about what you'd expect.  If sortop is InvalidOid"]
#[doc = "    then nulls_first is meaningless and should be set to false."]
#[doc = " hashable is true if eqop is hashable (note this condition also depends"]
#[doc = "    on the datatype of the input expression)."]
#[doc = ""]
#[doc = " In an ORDER BY item, all fields must be valid.  (The eqop isn't essential"]
#[doc = " here, but it's cheap to get it along with the sortop, and requiring it"]
#[doc = " to be valid eases comparisons to grouping items.)  Note that this isn't"]
#[doc = " actually enough information to determine an ordering: if the sortop is"]
#[doc = " collationsensitive, a collation OID is needed too.  We don't store the"]
#[doc = " collation in SortGroupClause because it's not available at the time the"]
#[doc = " parser builds the SortGroupClause; instead, consult the exposed collation"]
#[doc = " of the referenced targetlist expression to find out what it is."]
#[doc = ""]
#[doc = " In a grouping item, eqop must be valid.  If the eqop is a btree equality"]
#[doc = " operator, then sortop should be set to a compatible ordering operator."]
#[doc = " We prefer to set eqop/sortop/nulls_first to match any ORDER BY item that"]
#[doc = " the query presents for the same tlist item.  If there is none, we just"]
#[doc = " use the default ordering op for the datatype."]
#[doc = ""]
#[doc = " If the tlist item's type has a hash opclass but no btree opclass, then"]
#[doc = " we will set eqop to the hash equality operator, sortop to InvalidOid,"]
#[doc = " and nulls_first to false.  A grouping item of this kind can only be"]
#[doc = " implemented by hashing, and of course it'll never match an ORDER BY item."]
#[doc = ""]
#[doc = " The hashable flag is provided since we generally have the requisite"]
#[doc = " information readily available when the SortGroupClause is constructed,"]
#[doc = " and it's relatively expensive to get it again later.  Note there is no"]
#[doc = " need for a \"sortable\" flag since OidIsValid(sortop) serves the purpose."]
#[doc = ""]
#[doc = " A query might have both ORDER BY and DISTINCT (or DISTINCT ON) clauses."]
#[doc = " In SELECT DISTINCT, the distinctClause list is as long or longer than the"]
#[doc = " sortClause list, while in SELECT DISTINCT ON it's typically shorter."]
#[doc = " The two lists must match up to the end of the shorter one  the parser"]
#[doc = " rearranges the distinctClause if necessary to make this true.  (This"]
#[doc = " restriction ensures that only one sort step is needed to both satisfy the"]
#[doc = " ORDER BY and set up for the Unique step.  This is semantically necessary"]
#[doc = " for DISTINCT ON, and presents no real drawback for DISTINCT.)"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SortGroupClause {
    pub type_: NodeTag,
    pub tleSortGroupRef: Index,
    #[doc = " reference into targetlist"]
    pub eqop: Oid,
    #[doc = " the equality operator ('=' op)"]
    pub sortop: Oid,
    #[doc = " the ordering operator ('<' op), or 0"]
    pub nulls_first: bool,
    #[doc = " do NULLs come before normal values?"]
    pub hashable: bool,
}
#[test]
fn bindgen_test_layout_SortGroupClause() {
    assert_eq!(
        ::std::mem::size_of::<SortGroupClause>(),
        20usize,
        concat!("Size of: ", stringify!(SortGroupClause))
    );
    assert_eq!(
        ::std::mem::align_of::<SortGroupClause>(),
        4usize,
        concat!("Alignment of ", stringify!(SortGroupClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortGroupClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SortGroupClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortGroupClause>())).tleSortGroupRef as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SortGroupClause),
            "::",
            stringify!(tleSortGroupRef)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortGroupClause>())).eqop as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SortGroupClause),
            "::",
            stringify!(eqop)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortGroupClause>())).sortop as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SortGroupClause),
            "::",
            stringify!(sortop)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortGroupClause>())).nulls_first as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SortGroupClause),
            "::",
            stringify!(nulls_first)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SortGroupClause>())).hashable as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(SortGroupClause),
            "::",
            stringify!(hashable)
        )
    );
}
impl Default for SortGroupClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " GroupingSet"]
#[doc = "    representation of CUBE, ROLLUP and GROUPING SETS clauses"]
#[doc = ""]
#[doc = " In a Query with grouping sets, the groupClause contains a flat list of"]
#[doc = " SortGroupClause nodes for each distinct expression used.  The actual"]
#[doc = " structure of the GROUP BY clause is given by the groupingSets tree."]
#[doc = ""]
#[doc = " In the raw parser output, GroupingSet nodes (of all types except SIMPLE"]
#[doc = " which is not used) are potentially mixed in with the expressions in the"]
#[doc = " groupClause of the SelectStmt.  (An expression can't contain a GroupingSet,"]
#[doc = " but a list may mix GroupingSet and expression nodes.)  At this stage, the"]
#[doc = " content of each node is a list of expressions, some of which may be RowExprs"]
#[doc = " which represent sublists rather than actual row constructors, and nested"]
#[doc = " GroupingSet nodes where legal in the grammar.  The structure directly"]
#[doc = " reflects the query syntax."]
#[doc = ""]
#[doc = " In parse analysis, the transformed expressions are used to build the tlist"]
#[doc = " and groupClause list (of SortGroupClause nodes), and the groupingSets tree"]
#[doc = " is eventually reduced to a fixed format:"]
#[doc = ""]
#[doc = " EMPTY nodes represent (), and obviously have no content"]
#[doc = ""]
#[doc = " SIMPLE nodes represent a list of one or more expressions to be treated as an"]
#[doc = " atom by the enclosing structure; the content is an integer list of"]
#[doc = " ressortgroupref values (see SortGroupClause)"]
#[doc = ""]
#[doc = " CUBE and ROLLUP nodes contain a list of one or more SIMPLE nodes."]
#[doc = ""]
#[doc = " SETS nodes contain a list of EMPTY, SIMPLE, CUBE or ROLLUP nodes, but after"]
#[doc = " parse analysis they cannot contain more SETS nodes; enough of the syntactic"]
#[doc = " transforms of the spec have been applied that we no longer have arbitrarily"]
#[doc = " deep nesting (though we still preserve the use of cube/rollup)."]
#[doc = ""]
#[doc = " Note that if the groupingSets tree contains no SIMPLE nodes (only EMPTY"]
#[doc = " nodes at the leaves), then the groupClause will be empty, but this is still"]
#[doc = " an aggregation query (similar to using aggs or HAVING without GROUP BY)."]
#[doc = ""]
#[doc = " As an example, the following clause:"]
#[doc = ""]
#[doc = " GROUP BY GROUPING SETS ((a,b), CUBE(c,(d,e)))"]
#[doc = ""]
#[doc = " looks like this after raw parsing:"]
#[doc = ""]
#[doc = " SETS( RowExpr(a,b) , CUBE( c, RowExpr(d,e) ) )"]
#[doc = ""]
#[doc = " and parse analysis converts it to:"]
#[doc = ""]
#[doc = " SETS( SIMPLE(1,2), CUBE( SIMPLE(3), SIMPLE(4,5) ) )"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupingSetKind {
    GROUPING_SET_EMPTY = 0,
    GROUPING_SET_SIMPLE = 1,
    GROUPING_SET_ROLLUP = 2,
    GROUPING_SET_CUBE = 3,
    GROUPING_SET_SETS = 4,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GroupingSet {
    pub type_: NodeTag,
    pub kind: GroupingSetKind,
    pub content: *mut List,
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_GroupingSet() {
    assert_eq!(
        ::std::mem::size_of::<GroupingSet>(),
        24usize,
        concat!("Size of: ", stringify!(GroupingSet))
    );
    assert_eq!(
        ::std::mem::align_of::<GroupingSet>(),
        8usize,
        concat!("Alignment of ", stringify!(GroupingSet))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GroupingSet>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GroupingSet),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GroupingSet>())).kind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GroupingSet),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GroupingSet>())).content as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GroupingSet),
            "::",
            stringify!(content)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GroupingSet>())).location as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GroupingSet),
            "::",
            stringify!(location)
        )
    );
}
impl Default for GroupingSet {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " WindowClause"]
#[doc = "    transformed representation of WINDOW and OVER clauses"]
#[doc = ""]
#[doc = " A parsed Query's windowClause list contains these structs.  \"name\" is set"]
#[doc = " if the clause originally came from WINDOW, and is NULL if it originally"]
#[doc = " was an OVER clause (but note that we collapse out duplicate OVERs)."]
#[doc = " partitionClause and orderClause are lists of SortGroupClause structs."]
#[doc = " If we have RANGE with offset PRECEDING/FOLLOWING, the semantics of that are"]
#[doc = " specified by startInRangeFunc/inRangeColl/inRangeAsc/inRangeNullsFirst"]
#[doc = " for the start offset, or endInRangeFunc/inRange* for the end offset."]
#[doc = " winref is an ID number referenced by WindowFunc nodes; it must be unique"]
#[doc = " among the members of a Query's windowClause list."]
#[doc = " When refname isn't null, the partitionClause is always copied from there;"]
#[doc = " the orderClause might or might not be copied (see copiedOrder); the framing"]
#[doc = " options are never copied, per spec."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct WindowClause {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " window name (NULL in an OVER clause)"]
    pub refname: *mut ::std::os::raw::c_char,
    #[doc = " referenced window name, if any"]
    pub partitionClause: *mut List,
    #[doc = " PARTITION BY list"]
    pub orderClause: *mut List,
    #[doc = " ORDER BY list"]
    pub frameOptions: ::std::os::raw::c_int,
    #[doc = " frame_clause options, see WindowDef"]
    pub startOffset: *mut Node,
    #[doc = " expression for starting bound, if any"]
    pub endOffset: *mut Node,
    #[doc = " expression for ending bound, if any"]
    pub startInRangeFunc: Oid,
    #[doc = " in_range function for startOffset"]
    pub endInRangeFunc: Oid,
    #[doc = " in_range function for endOffset"]
    pub inRangeColl: Oid,
    #[doc = " collation for in_range tests"]
    pub inRangeAsc: bool,
    #[doc = " use ASC sort order for in_range tests?"]
    pub inRangeNullsFirst: bool,
    #[doc = " nulls sort first for in_range tests?"]
    pub winref: Index,
    #[doc = " ID referenced by window functions"]
    pub copiedOrder: bool,
}
#[test]
fn bindgen_test_layout_WindowClause() {
    assert_eq!(
        ::std::mem::size_of::<WindowClause>(),
        88usize,
        concat!("Size of: ", stringify!(WindowClause))
    );
    assert_eq!(
        ::std::mem::align_of::<WindowClause>(),
        8usize,
        concat!("Alignment of ", stringify!(WindowClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).refname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(refname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).partitionClause as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(partitionClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).orderClause as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(orderClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).frameOptions as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(frameOptions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).startOffset as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(startOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).endOffset as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(endOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).startInRangeFunc as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(startInRangeFunc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).endInRangeFunc as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(endInRangeFunc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).inRangeColl as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(inRangeColl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).inRangeAsc as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(inRangeAsc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).inRangeNullsFirst as *const _ as usize },
        77usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(inRangeNullsFirst)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).winref as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(winref)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WindowClause>())).copiedOrder as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(WindowClause),
            "::",
            stringify!(copiedOrder)
        )
    );
}
impl Default for WindowClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " RowMarkClause"]
#[doc = "    parser output representation of FOR [KEY] UPDATE/SHARE clauses"]
#[doc = ""]
#[doc = " Query.rowMarks contains a separate RowMarkClause node for each relation"]
#[doc = " identified as a FOR [KEY] UPDATE/SHARE target.  If one of these clauses"]
#[doc = " is applied to a subquery, we generate RowMarkClauses for all normal and"]
#[doc = " subquery rels in the subquery, but they are marked pushedDown = true to"]
#[doc = " distinguish them from clauses that were explicitly written at this query"]
#[doc = " level.  Also, Query.hasForUpdate tells whether there were explicit FOR"]
#[doc = " UPDATE/SHARE/KEY SHARE clauses in the current query level."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RowMarkClause {
    pub type_: NodeTag,
    pub rti: Index,
    #[doc = " range table index of target relation"]
    pub strength: LockClauseStrength,
    pub waitPolicy: LockWaitPolicy,
    #[doc = " NOWAIT and SKIP LOCKED"]
    pub pushedDown: bool,
}
#[test]
fn bindgen_test_layout_RowMarkClause() {
    assert_eq!(
        ::std::mem::size_of::<RowMarkClause>(),
        20usize,
        concat!("Size of: ", stringify!(RowMarkClause))
    );
    assert_eq!(
        ::std::mem::align_of::<RowMarkClause>(),
        4usize,
        concat!("Alignment of ", stringify!(RowMarkClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowMarkClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RowMarkClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowMarkClause>())).rti as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RowMarkClause),
            "::",
            stringify!(rti)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowMarkClause>())).strength as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RowMarkClause),
            "::",
            stringify!(strength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowMarkClause>())).waitPolicy as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(RowMarkClause),
            "::",
            stringify!(waitPolicy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RowMarkClause>())).pushedDown as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RowMarkClause),
            "::",
            stringify!(pushedDown)
        )
    );
}
impl Default for RowMarkClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " WithClause"]
#[doc = "    representation of WITH clause"]
#[doc = ""]
#[doc = " Note: WithClause does not propagate into the Query representation;"]
#[doc = " but CommonTableExpr does."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct WithClause {
    pub type_: NodeTag,
    pub ctes: *mut List,
    #[doc = " list of CommonTableExprs"]
    pub recursive: bool,
    #[doc = " true = WITH RECURSIVE"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_WithClause() {
    assert_eq!(
        ::std::mem::size_of::<WithClause>(),
        24usize,
        concat!("Size of: ", stringify!(WithClause))
    );
    assert_eq!(
        ::std::mem::align_of::<WithClause>(),
        8usize,
        concat!("Alignment of ", stringify!(WithClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithClause>())).ctes as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WithClause),
            "::",
            stringify!(ctes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithClause>())).recursive as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(WithClause),
            "::",
            stringify!(recursive)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WithClause>())).location as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(WithClause),
            "::",
            stringify!(location)
        )
    );
}
impl Default for WithClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " InferClause"]
#[doc = "    ON CONFLICT unique index inference clause"]
#[doc = ""]
#[doc = " Note: InferClause does not propagate into the Query representation."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct InferClause {
    pub type_: NodeTag,
    pub indexElems: *mut List,
    #[doc = " IndexElems to infer unique index"]
    pub whereClause: *mut Node,
    #[doc = " qualification (partialindex predicate)"]
    pub conname: *mut ::std::os::raw::c_char,
    #[doc = " Constraint name, or NULL if unnamed"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_InferClause() {
    assert_eq!(
        ::std::mem::size_of::<InferClause>(),
        40usize,
        concat!("Size of: ", stringify!(InferClause))
    );
    assert_eq!(
        ::std::mem::align_of::<InferClause>(),
        8usize,
        concat!("Alignment of ", stringify!(InferClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InferClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(InferClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InferClause>())).indexElems as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(InferClause),
            "::",
            stringify!(indexElems)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InferClause>())).whereClause as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(InferClause),
            "::",
            stringify!(whereClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InferClause>())).conname as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(InferClause),
            "::",
            stringify!(conname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InferClause>())).location as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(InferClause),
            "::",
            stringify!(location)
        )
    );
}
impl Default for InferClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " OnConflictClause"]
#[doc = "    representation of ON CONFLICT clause"]
#[doc = ""]
#[doc = " Note: OnConflictClause does not propagate into the Query representation."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct OnConflictClause {
    pub type_: NodeTag,
    pub action: OnConflictAction,
    #[doc = " DO NOTHING or UPDATE?"]
    pub infer: *mut InferClause,
    #[doc = " Optional index inference clause"]
    pub targetList: *mut List,
    #[doc = " the target list (of ResTarget)"]
    pub whereClause: *mut Node,
    #[doc = " qualifications"]
    pub location: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_OnConflictClause() {
    assert_eq!(
        ::std::mem::size_of::<OnConflictClause>(),
        40usize,
        concat!("Size of: ", stringify!(OnConflictClause))
    );
    assert_eq!(
        ::std::mem::align_of::<OnConflictClause>(),
        8usize,
        concat!("Alignment of ", stringify!(OnConflictClause))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictClause>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictClause),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictClause>())).action as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictClause),
            "::",
            stringify!(action)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictClause>())).infer as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictClause),
            "::",
            stringify!(infer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictClause>())).targetList as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictClause),
            "::",
            stringify!(targetList)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictClause>())).whereClause as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictClause),
            "::",
            stringify!(whereClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<OnConflictClause>())).location as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(OnConflictClause),
            "::",
            stringify!(location)
        )
    );
}
impl Default for OnConflictClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " CommonTableExpr"]
#[doc = "    representation of WITH list element"]
#[doc = ""]
#[doc = " We don't currently support the SEARCH or CYCLE clause."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CTEMaterialize {
    CTEMaterializeDefault = 0,
    #[doc = " no option specified"]
    CTEMaterializeAlways = 1,
    #[doc = " MATERIALIZED"]
    CTEMaterializeNever = 2,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CommonTableExpr {
    pub type_: NodeTag,
    pub ctename: *mut ::std::os::raw::c_char,
    #[doc = " query name (never qualified)"]
    pub aliascolnames: *mut List,
    #[doc = " optional list of column names"]
    pub ctematerialized: CTEMaterialize,
    #[doc = " is this an optimization fence? */"]
    pub ctequery: *mut Node,
    #[doc = " the CTE's subquery"]
    pub location: ::std::os::raw::c_int,
    #[doc = " token location, or 1 if unknown */"]
    pub cterecursive: bool,
    #[doc = " is this CTE actually recursive?"]
    pub cterefcount: ::std::os::raw::c_int,
    #[doc = " number of RTEs referencing this CTE"]
    #[doc = " (excluding internal selfreferences)"]
    pub ctecolnames: *mut List,
    #[doc = " list of output column names"]
    pub ctecoltypes: *mut List,
    #[doc = " OID list of output column type OIDs"]
    pub ctecoltypmods: *mut List,
    #[doc = " integer list of output column typmods"]
    pub ctecolcollations: *mut List,
}
#[test]
fn bindgen_test_layout_CommonTableExpr() {
    assert_eq!(
        ::std::mem::size_of::<CommonTableExpr>(),
        88usize,
        concat!("Size of: ", stringify!(CommonTableExpr))
    );
    assert_eq!(
        ::std::mem::align_of::<CommonTableExpr>(),
        8usize,
        concat!("Alignment of ", stringify!(CommonTableExpr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).ctename as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(ctename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).aliascolnames as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(aliascolnames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).ctematerialized as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(ctematerialized)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).ctequery as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(ctequery)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).location as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(location)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).cterecursive as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(cterecursive)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).cterefcount as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(cterefcount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).ctecolnames as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(ctecolnames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).ctecoltypes as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(ctecoltypes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommonTableExpr>())).ctecoltypmods as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(ctecoltypmods)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CommonTableExpr>())).ctecolcollations as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(CommonTableExpr),
            "::",
            stringify!(ctecolcollations)
        )
    );
}
impl Default for CommonTableExpr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " TriggerTransition"]
#[doc = "    representation of transition row or table naming clause"]
#[doc = ""]
#[doc = " Only transition tables are initially supported in the syntax, and only for"]
#[doc = " AFTER triggers, but other permutations are accepted by the parser so we can"]
#[doc = " give a meaningful message from C code."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TriggerTransition {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    pub isNew: bool,
    pub isTable: bool,
}
#[test]
fn bindgen_test_layout_TriggerTransition() {
    assert_eq!(
        ::std::mem::size_of::<TriggerTransition>(),
        24usize,
        concat!("Size of: ", stringify!(TriggerTransition))
    );
    assert_eq!(
        ::std::mem::align_of::<TriggerTransition>(),
        8usize,
        concat!("Alignment of ", stringify!(TriggerTransition))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TriggerTransition>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TriggerTransition),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TriggerTransition>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TriggerTransition),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TriggerTransition>())).isNew as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TriggerTransition),
            "::",
            stringify!(isNew)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TriggerTransition>())).isTable as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(TriggerTransition),
            "::",
            stringify!(isTable)
        )
    );
}
impl Default for TriggerTransition {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    RawStmt  container for any one statement's raw parse tree"]
#[doc = ""]
#[doc = " Parse analysis converts a raw parse tree headed by a RawStmt node into"]
#[doc = " an analyzed statement headed by a Query node.  For optimizable statements,"]
#[doc = " the conversion is complex.  For utility statements, the parser usually just"]
#[doc = " transfers the raw parse tree (sans RawStmt) into the utilityStmt field of"]
#[doc = " the Query node, and all the useful work happens at execution time."]
#[doc = ""]
#[doc = " stmt_location/stmt_len identify the portion of the source text string"]
#[doc = " containing this raw statement (useful for multistatement strings)."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RawStmt {
    pub type_: NodeTag,
    pub stmt: *mut Node,
    #[doc = " raw parse tree"]
    pub stmt_location: ::std::os::raw::c_int,
    #[doc = " start location, or 1 if unknown"]
    pub stmt_len: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_RawStmt() {
    assert_eq!(
        ::std::mem::size_of::<RawStmt>(),
        24usize,
        concat!("Size of: ", stringify!(RawStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<RawStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(RawStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RawStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RawStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RawStmt>())).stmt as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RawStmt),
            "::",
            stringify!(stmt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RawStmt>())).stmt_location as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RawStmt),
            "::",
            stringify!(stmt_location)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RawStmt>())).stmt_len as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(RawStmt),
            "::",
            stringify!(stmt_len)
        )
    );
}
impl Default for RawStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Insert Statement"]
#[doc = ""]
#[doc = " The source expression is represented by SelectStmt for both the"]
#[doc = " SELECT and VALUES cases.  If selectStmt is NULL, then the query"]
#[doc = " is INSERT ... DEFAULT VALUES."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct InsertStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    #[doc = " relation to insert into"]
    pub cols: *mut List,
    #[doc = " optional: names of the target columns"]
    pub selectStmt: *mut Node,
    #[doc = " the source SELECT/VALUES, or NULL"]
    pub onConflictClause: *mut OnConflictClause,
    #[doc = " ON CONFLICT clause"]
    pub returningList: *mut List,
    #[doc = " list of expressions to return"]
    pub withClause: *mut WithClause,
    #[doc = " WITH clause"]
    pub override_: OverridingKind,
}
#[test]
fn bindgen_test_layout_InsertStmt() {
    assert_eq!(
        ::std::mem::size_of::<InsertStmt>(),
        64usize,
        concat!("Size of: ", stringify!(InsertStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<InsertStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(InsertStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InsertStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(InsertStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InsertStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(InsertStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InsertStmt>())).cols as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(InsertStmt),
            "::",
            stringify!(cols)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InsertStmt>())).selectStmt as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(InsertStmt),
            "::",
            stringify!(selectStmt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InsertStmt>())).onConflictClause as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(InsertStmt),
            "::",
            stringify!(onConflictClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InsertStmt>())).returningList as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(InsertStmt),
            "::",
            stringify!(returningList)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InsertStmt>())).withClause as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(InsertStmt),
            "::",
            stringify!(withClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InsertStmt>())).override_ as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(InsertStmt),
            "::",
            stringify!(override_)
        )
    );
}
impl Default for InsertStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Delete Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DeleteStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    #[doc = " relation to delete from"]
    pub usingClause: *mut List,
    #[doc = " optional using clause for more tables"]
    pub whereClause: *mut Node,
    #[doc = " qualifications"]
    pub returningList: *mut List,
    #[doc = " list of expressions to return"]
    pub withClause: *mut WithClause,
}
#[test]
fn bindgen_test_layout_DeleteStmt() {
    assert_eq!(
        ::std::mem::size_of::<DeleteStmt>(),
        48usize,
        concat!("Size of: ", stringify!(DeleteStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DeleteStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DeleteStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeleteStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeleteStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeleteStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeleteStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeleteStmt>())).usingClause as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DeleteStmt),
            "::",
            stringify!(usingClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeleteStmt>())).whereClause as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DeleteStmt),
            "::",
            stringify!(whereClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeleteStmt>())).returningList as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(DeleteStmt),
            "::",
            stringify!(returningList)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeleteStmt>())).withClause as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(DeleteStmt),
            "::",
            stringify!(withClause)
        )
    );
}
impl Default for DeleteStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Update Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct UpdateStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    #[doc = " relation to update"]
    pub targetList: *mut List,
    #[doc = " the target list (of ResTarget)"]
    pub whereClause: *mut Node,
    #[doc = " qualifications"]
    pub fromClause: *mut List,
    #[doc = " optional from clause for more tables"]
    pub returningList: *mut List,
    #[doc = " list of expressions to return"]
    pub withClause: *mut WithClause,
}
#[test]
fn bindgen_test_layout_UpdateStmt() {
    assert_eq!(
        ::std::mem::size_of::<UpdateStmt>(),
        56usize,
        concat!("Size of: ", stringify!(UpdateStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<UpdateStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(UpdateStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UpdateStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(UpdateStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UpdateStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(UpdateStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UpdateStmt>())).targetList as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(UpdateStmt),
            "::",
            stringify!(targetList)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UpdateStmt>())).whereClause as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(UpdateStmt),
            "::",
            stringify!(whereClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UpdateStmt>())).fromClause as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(UpdateStmt),
            "::",
            stringify!(fromClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UpdateStmt>())).returningList as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(UpdateStmt),
            "::",
            stringify!(returningList)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UpdateStmt>())).withClause as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(UpdateStmt),
            "::",
            stringify!(withClause)
        )
    );
}
impl Default for UpdateStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = "    Select Statement"]
#[doc = ""]
#[doc = " A \"simple\" SELECT is represented in the output of gram.y by a single"]
#[doc = " SelectStmt node; so is a VALUES construct.  A query containing set"]
#[doc = " operators (UNION, INTERSECT, EXCEPT) is represented by a tree of SelectStmt"]
#[doc = " nodes, in which the leaf nodes are component SELECTs and the internal nodes"]
#[doc = " represent UNION, INTERSECT, or EXCEPT operators.  Using the same node"]
#[doc = " type for both leaf and internal nodes allows gram.y to stick ORDER BY,"]
#[doc = " LIMIT, etc, clause values into a SELECT statement without worrying"]
#[doc = " whether it is a simple or compound SELECT."]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SetOperation {
    SETOP_NONE = 0,
    SETOP_UNION = 1,
    SETOP_INTERSECT = 2,
    SETOP_EXCEPT = 3,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SelectStmt {
    pub type_: NodeTag,
    #[doc = " These fields are used only in \"leaf\" SelectStmts."]
    pub distinctClause: *mut List,
    #[doc = " NULL, list of DISTINCT ON exprs, or"]
    #[doc = " lcons(NIL,NIL) for all (SELECT DISTINCT)"]
    pub intoClause: *mut IntoClause,
    #[doc = " target for SELECT INTO"]
    pub targetList: *mut List,
    #[doc = " the target list (of ResTarget)"]
    pub fromClause: *mut List,
    #[doc = " the FROM clause"]
    pub whereClause: *mut Node,
    #[doc = " WHERE qualification"]
    pub groupClause: *mut List,
    #[doc = " GROUP BY clauses"]
    pub havingClause: *mut Node,
    #[doc = " HAVING conditionalexpression"]
    pub windowClause: *mut List,
    #[doc = " In a \"leaf\" node representing a VALUES list, the above fields are all"]
    #[doc = " null, and instead this field is set.  Note that the elements of the"]
    #[doc = " sublists are just expressions, without ResTarget decoration. Also note"]
    #[doc = " that a list element can be DEFAULT (represented as a SetToDefault"]
    #[doc = " node), regardless of the context of the VALUES list. It's up to parse"]
    #[doc = " analysis to reject that where not valid."]
    pub valuesLists: *mut List,
    #[doc = " These fields are used in both \"leaf\" SelectStmts and upperlevel"]
    #[doc = " SelectStmts."]
    pub sortClause: *mut List,
    #[doc = " sort clause (a list of SortBy's)"]
    pub limitOffset: *mut Node,
    #[doc = " # of result tuples to skip"]
    pub limitCount: *mut Node,
    #[doc = " # of result tuples to return"]
    pub lockingClause: *mut List,
    #[doc = " FOR UPDATE (list of LockingClause's)"]
    pub withClause: *mut WithClause,
    #[doc = " These fields are used only in upperlevel SelectStmts."]
    pub op: SetOperation,
    #[doc = " type of set op"]
    pub all: bool,
    #[doc = " ALL specified?"]
    pub larg: *mut SelectStmt,
    #[doc = " left child"]
    pub rarg: *mut SelectStmt,
}
#[test]
fn bindgen_test_layout_SelectStmt() {
    assert_eq!(
        ::std::mem::size_of::<SelectStmt>(),
        144usize,
        concat!("Size of: ", stringify!(SelectStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<SelectStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(SelectStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).distinctClause as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(distinctClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).intoClause as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(intoClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).targetList as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(targetList)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).fromClause as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(fromClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).whereClause as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(whereClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).groupClause as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(groupClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).havingClause as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(havingClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).windowClause as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(windowClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).valuesLists as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(valuesLists)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).sortClause as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(sortClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).limitOffset as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(limitOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).limitCount as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(limitCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).lockingClause as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(lockingClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).withClause as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(withClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).op as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(op)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).all as *const _ as usize },
        124usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(all)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).larg as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(larg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SelectStmt>())).rarg as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(SelectStmt),
            "::",
            stringify!(rarg)
        )
    );
}
impl Default for SelectStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Set Operation node for postanalysis query trees"]
#[doc = ""]
#[doc = " After parse analysis, a SELECT with set operations is represented by a"]
#[doc = " toplevel Query node containing the leaf SELECTs as subqueries in its"]
#[doc = " range table.  Its setOperations field shows the tree of set operations,"]
#[doc = " with leaf SelectStmt nodes replaced by RangeTblRef nodes, and internal"]
#[doc = " nodes replaced by SetOperationStmt nodes.  Information about the output"]
#[doc = " column types is added, too.  (Note that the child nodes do not necessarily"]
#[doc = " produce these types directly, but we've checked that their output types"]
#[doc = " can be coerced to the output column type.)  Also, if it's not UNION ALL,"]
#[doc = " information about the types' sort/group semantics is provided in the form"]
#[doc = " of a SortGroupClause list (same representation as, eg, DISTINCT)."]
#[doc = " The resolved common column collations are provided too; but note that if"]
#[doc = " it's not UNION ALL, it's okay for a column to not have a common collation,"]
#[doc = " so a member of the colCollations list could be InvalidOid even though the"]
#[doc = " column has a collatable type."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SetOperationStmt {
    pub type_: NodeTag,
    pub op: SetOperation,
    #[doc = " type of set op"]
    pub all: bool,
    #[doc = " ALL specified?"]
    pub larg: *mut Node,
    #[doc = " left child"]
    pub rarg: *mut Node,
    #[doc = " Fields derived during parse analysis:"]
    pub colTypes: *mut List,
    #[doc = " OID list of output column type OIDs"]
    pub colTypmods: *mut List,
    #[doc = " integer list of output column typmods"]
    pub colCollations: *mut List,
    #[doc = " OID list of output column collation OIDs"]
    pub groupClauses: *mut List,
}
#[test]
fn bindgen_test_layout_SetOperationStmt() {
    assert_eq!(
        ::std::mem::size_of::<SetOperationStmt>(),
        64usize,
        concat!("Size of: ", stringify!(SetOperationStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<SetOperationStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(SetOperationStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetOperationStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SetOperationStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetOperationStmt>())).op as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SetOperationStmt),
            "::",
            stringify!(op)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetOperationStmt>())).all as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SetOperationStmt),
            "::",
            stringify!(all)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetOperationStmt>())).larg as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SetOperationStmt),
            "::",
            stringify!(larg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetOperationStmt>())).rarg as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SetOperationStmt),
            "::",
            stringify!(rarg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetOperationStmt>())).colTypes as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SetOperationStmt),
            "::",
            stringify!(colTypes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetOperationStmt>())).colTypmods as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SetOperationStmt),
            "::",
            stringify!(colTypmods)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetOperationStmt>())).colCollations as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SetOperationStmt),
            "::",
            stringify!(colCollations)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SetOperationStmt>())).groupClauses as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SetOperationStmt),
            "::",
            stringify!(groupClauses)
        )
    );
}
impl Default for SetOperationStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " When a command can act on several kinds of objects with only one"]
#[doc = " parse structure required, use these constants to designate the"]
#[doc = " object type.  Note that commands typically don't support all the types."]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ObjectType {
    OBJECT_ACCESS_METHOD = 0,
    OBJECT_AGGREGATE = 1,
    OBJECT_AMOP = 2,
    OBJECT_AMPROC = 3,
    OBJECT_ATTRIBUTE = 4,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_CAST = 5,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_COLUMN = 6,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_COLLATION = 7,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_CONVERSION = 8,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_DATABASE = 9,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_DEFAULT = 10,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_DEFACL = 11,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_DOMAIN = 12,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_DOMCONSTRAINT = 13,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_EVENT_TRIGGER = 14,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_EXTENSION = 15,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_FDW = 16,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_FOREIGN_SERVER = 17,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_FOREIGN_TABLE = 18,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_FUNCTION = 19,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_INDEX = 20,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_LANGUAGE = 21,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_LARGEOBJECT = 22,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_MATVIEW = 23,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_OPCLASS = 24,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_OPERATOR = 25,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_OPFAMILY = 26,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_POLICY = 27,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_PROCEDURE = 28,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_PUBLICATION = 29,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_PUBLICATION_REL = 30,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_ROLE = 31,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_ROUTINE = 32,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_RULE = 33,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_SCHEMA = 34,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_SEQUENCE = 35,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_SUBSCRIPTION = 36,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_STATISTIC_EXT = 37,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_TABCONSTRAINT = 38,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_TABLE = 39,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_TABLESPACE = 40,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_TRANSFORM = 41,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_TRIGGER = 42,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_TSCONFIGURATION = 43,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_TSDICTIONARY = 44,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_TSPARSER = 45,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_TSTEMPLATE = 46,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_TYPE = 47,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_USER_MAPPING = 48,
    #[doc = " type's attribute, when distinct from column"]
    OBJECT_VIEW = 49,
}
#[doc = "    Create Schema Statement"]
#[doc = ""]
#[doc = " NOTE: the schemaElts list contains raw parsetrees for component statements"]
#[doc = " of the schema, such as CREATE TABLE, GRANT, etc.  These are analyzed and"]
#[doc = " executed after the schema itself is created."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateSchemaStmt {
    pub type_: NodeTag,
    pub schemaname: *mut ::std::os::raw::c_char,
    #[doc = " the name of the schema to create"]
    pub authrole: *mut RoleSpec,
    #[doc = " the owner of the created schema"]
    pub schemaElts: *mut List,
    #[doc = " schema components (list of parsenodes)"]
    pub if_not_exists: bool,
}
#[test]
fn bindgen_test_layout_CreateSchemaStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateSchemaStmt>(),
        40usize,
        concat!("Size of: ", stringify!(CreateSchemaStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateSchemaStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateSchemaStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSchemaStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSchemaStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSchemaStmt>())).schemaname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSchemaStmt),
            "::",
            stringify!(schemaname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSchemaStmt>())).authrole as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSchemaStmt),
            "::",
            stringify!(authrole)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSchemaStmt>())).schemaElts as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSchemaStmt),
            "::",
            stringify!(schemaElts)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSchemaStmt>())).if_not_exists as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSchemaStmt),
            "::",
            stringify!(if_not_exists)
        )
    );
}
impl Default for CreateSchemaStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DropBehavior {
    DROP_RESTRICT = 0,
    #[doc = " drop fails if any dependent objects"]
    DROP_CASCADE = 1,
}
#[doc = " Alter Table"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterTableStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    #[doc = " table to work on"]
    pub cmds: *mut List,
    #[doc = " list of subcommands"]
    pub relkind: ObjectType,
    #[doc = " type of object"]
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_AlterTableStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterTableStmt>(),
        32usize,
        concat!("Size of: ", stringify!(AlterTableStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterTableStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterTableStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableStmt>())).cmds as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableStmt),
            "::",
            stringify!(cmds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableStmt>())).relkind as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableStmt),
            "::",
            stringify!(relkind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableStmt>())).missing_ok as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableStmt),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for AlterTableStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AlterTableType {
    AT_AddColumn = 0,
    #[doc = " add column"]
    AT_AddColumnRecurse = 1,
    #[doc = " internal to commands/tablecmds.c"]
    AT_AddColumnToView = 2,
    #[doc = " implicitly via CREATE OR REPLACE VIEW"]
    AT_ColumnDefault = 3,
    #[doc = " alter column default"]
    AT_DropNotNull = 4,
    #[doc = " alter column drop not null"]
    AT_SetNotNull = 5,
    #[doc = " alter column set not null"]
    AT_CheckNotNull = 6,
    #[doc = " check column is already marked not null"]
    AT_SetStatistics = 7,
    #[doc = " alter column set statistics"]
    AT_SetOptions = 8,
    #[doc = " alter column set ( options )"]
    AT_ResetOptions = 9,
    #[doc = " alter column reset ( options )"]
    AT_SetStorage = 10,
    #[doc = " alter column set storage"]
    AT_DropColumn = 11,
    #[doc = " drop column"]
    AT_DropColumnRecurse = 12,
    #[doc = " internal to commands/tablecmds.c"]
    AT_AddIndex = 13,
    #[doc = " add index"]
    AT_ReAddIndex = 14,
    #[doc = " internal to commands/tablecmds.c"]
    AT_AddConstraint = 15,
    #[doc = " add constraint"]
    AT_AddConstraintRecurse = 16,
    #[doc = " internal to commands/tablecmds.c"]
    AT_ReAddConstraint = 17,
    #[doc = " internal to commands/tablecmds.c"]
    AT_ReAddDomainConstraint = 18,
    #[doc = " internal to commands/tablecmds.c"]
    AT_AlterConstraint = 19,
    #[doc = " alter constraint"]
    AT_ValidateConstraint = 20,
    #[doc = " validate constraint"]
    AT_ValidateConstraintRecurse = 21,
    #[doc = " internal to commands/tablecmds.c"]
    AT_ProcessedConstraint = 22,
    #[doc = " preprocessed add constraint (local in"]
    #[doc = " parser/parse_utilcmd.c)"]
    AT_AddIndexConstraint = 23,
    #[doc = " add constraint using existing index"]
    AT_DropConstraint = 24,
    #[doc = " drop constraint"]
    AT_DropConstraintRecurse = 25,
    #[doc = " internal to commands/tablecmds.c"]
    AT_ReAddComment = 26,
    #[doc = " internal to commands/tablecmds.c"]
    AT_AlterColumnType = 27,
    #[doc = " alter column type"]
    AT_AlterColumnGenericOptions = 28,
    #[doc = " alter column OPTIONS (...)"]
    AT_ChangeOwner = 29,
    #[doc = " change owner"]
    AT_ClusterOn = 30,
    #[doc = " CLUSTER ON"]
    AT_DropCluster = 31,
    #[doc = " SET WITHOUT CLUSTER"]
    AT_SetLogged = 32,
    #[doc = " SET LOGGED"]
    AT_SetUnLogged = 33,
    #[doc = " SET UNLOGGED"]
    AT_DropOids = 34,
    #[doc = " SET WITHOUT OIDS"]
    AT_SetTableSpace = 35,
    #[doc = " SET TABLESPACE"]
    AT_SetRelOptions = 36,
    #[doc = " SET (...)  AM specific parameters"]
    AT_ResetRelOptions = 37,
    #[doc = " RESET (...)  AM specific parameters"]
    AT_ReplaceRelOptions = 38,
    #[doc = " replace reloption list in its entirety"]
    AT_EnableTrig = 39,
    #[doc = " ENABLE TRIGGER name"]
    AT_EnableAlwaysTrig = 40,
    #[doc = " ENABLE ALWAYS TRIGGER name"]
    AT_EnableReplicaTrig = 41,
    #[doc = " ENABLE REPLICA TRIGGER name"]
    AT_DisableTrig = 42,
    #[doc = " DISABLE TRIGGER name"]
    AT_EnableTrigAll = 43,
    #[doc = " ENABLE TRIGGER ALL"]
    AT_DisableTrigAll = 44,
    #[doc = " DISABLE TRIGGER ALL"]
    AT_EnableTrigUser = 45,
    #[doc = " ENABLE TRIGGER USER"]
    AT_DisableTrigUser = 46,
    #[doc = " DISABLE TRIGGER USER"]
    AT_EnableRule = 47,
    #[doc = " ENABLE RULE name"]
    AT_EnableAlwaysRule = 48,
    #[doc = " ENABLE ALWAYS RULE name"]
    AT_EnableReplicaRule = 49,
    #[doc = " ENABLE REPLICA RULE name"]
    AT_DisableRule = 50,
    #[doc = " DISABLE RULE name"]
    AT_AddInherit = 51,
    #[doc = " INHERIT parent"]
    AT_DropInherit = 52,
    #[doc = " NO INHERIT parent"]
    AT_AddOf = 53,
    #[doc = " OF <type_name>"]
    AT_DropOf = 54,
    #[doc = " NOT OF"]
    AT_ReplicaIdentity = 55,
    #[doc = " REPLICA IDENTITY"]
    AT_EnableRowSecurity = 56,
    #[doc = " ENABLE ROW SECURITY"]
    AT_DisableRowSecurity = 57,
    #[doc = " DISABLE ROW SECURITY"]
    AT_ForceRowSecurity = 58,
    #[doc = " FORCE ROW SECURITY"]
    AT_NoForceRowSecurity = 59,
    #[doc = " NO FORCE ROW SECURITY"]
    AT_GenericOptions = 60,
    #[doc = " OPTIONS (...)"]
    AT_AttachPartition = 61,
    #[doc = " ATTACH PARTITION"]
    AT_DetachPartition = 62,
    #[doc = " DETACH PARTITION"]
    AT_AddIdentity = 63,
    #[doc = " ADD IDENTITY"]
    AT_SetIdentity = 64,
    #[doc = " SET identity column options"]
    AT_DropIdentity = 65,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ReplicaIdentityStmt {
    pub type_: NodeTag,
    pub identity_type: ::std::os::raw::c_char,
    pub name: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ReplicaIdentityStmt() {
    assert_eq!(
        ::std::mem::size_of::<ReplicaIdentityStmt>(),
        16usize,
        concat!("Size of: ", stringify!(ReplicaIdentityStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ReplicaIdentityStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ReplicaIdentityStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReplicaIdentityStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ReplicaIdentityStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ReplicaIdentityStmt>())).identity_type as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ReplicaIdentityStmt),
            "::",
            stringify!(identity_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReplicaIdentityStmt>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ReplicaIdentityStmt),
            "::",
            stringify!(name)
        )
    );
}
impl Default for ReplicaIdentityStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterTableCmd {
    pub type_: NodeTag,
    pub subtype: AlterTableType,
    #[doc = " Type of table alteration to apply"]
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " column, constraint, or trigger to act on,"]
    #[doc = " or tablespace"]
    pub num: int16,
    #[doc = " attribute number for columns referenced by"]
    #[doc = " number"]
    pub newowner: *mut RoleSpec,
    pub def: *mut Node,
    #[doc = " definition of new column, index,"]
    #[doc = " constraint, or parent table"]
    pub behavior: DropBehavior,
    #[doc = " RESTRICT or CASCADE for DROP cases"]
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_AlterTableCmd() {
    assert_eq!(
        ::std::mem::size_of::<AlterTableCmd>(),
        48usize,
        concat!("Size of: ", stringify!(AlterTableCmd))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterTableCmd>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterTableCmd))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableCmd>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableCmd),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableCmd>())).subtype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableCmd),
            "::",
            stringify!(subtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableCmd>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableCmd),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableCmd>())).num as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableCmd),
            "::",
            stringify!(num)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableCmd>())).newowner as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableCmd),
            "::",
            stringify!(newowner)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableCmd>())).def as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableCmd),
            "::",
            stringify!(def)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableCmd>())).behavior as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableCmd),
            "::",
            stringify!(behavior)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableCmd>())).missing_ok as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableCmd),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for AlterTableCmd {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Alter Collation"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterCollationStmt {
    pub type_: NodeTag,
    pub collname: *mut List,
}
#[test]
fn bindgen_test_layout_AlterCollationStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterCollationStmt>(),
        16usize,
        concat!("Size of: ", stringify!(AlterCollationStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterCollationStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterCollationStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterCollationStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterCollationStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterCollationStmt>())).collname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterCollationStmt),
            "::",
            stringify!(collname)
        )
    );
}
impl Default for AlterCollationStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Alter Domain"]
#[doc = ""]
#[doc = " The fields are used in different ways by the different variants of"]
#[doc = " this command."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterDomainStmt {
    pub type_: NodeTag,
    pub subtype: ::std::os::raw::c_char,
    #[doc = " T = alter column default"]
    #[doc = " N = alter column drop not null"]
    #[doc = " O = alter column set not null"]
    #[doc = " C = add constraint"]
    #[doc = " X = drop constraint"]
    #[doc = ""]
    pub typeName: *mut List,
    #[doc = " domain to work on"]
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " column or constraint name to act on"]
    pub def: *mut Node,
    #[doc = " definition of default or constraint"]
    pub behavior: DropBehavior,
    #[doc = " RESTRICT or CASCADE for DROP cases"]
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_AlterDomainStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterDomainStmt>(),
        40usize,
        concat!("Size of: ", stringify!(AlterDomainStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterDomainStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterDomainStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDomainStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDomainStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDomainStmt>())).subtype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDomainStmt),
            "::",
            stringify!(subtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDomainStmt>())).typeName as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDomainStmt),
            "::",
            stringify!(typeName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDomainStmt>())).name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDomainStmt),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDomainStmt>())).def as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDomainStmt),
            "::",
            stringify!(def)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDomainStmt>())).behavior as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDomainStmt),
            "::",
            stringify!(behavior)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDomainStmt>())).missing_ok as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDomainStmt),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for AlterDomainStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = "    Grant|Revoke Statement"]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GrantTargetType {
    ACL_TARGET_OBJECT = 0,
    #[doc = " grant on specific named object(s)"]
    ACL_TARGET_ALL_IN_SCHEMA = 1,
    #[doc = " grant on all objects in given schema(s)"]
    ACL_TARGET_DEFAULTS = 2,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GrantStmt {
    pub type_: NodeTag,
    pub is_grant: bool,
    #[doc = " true = GRANT, false = REVOKE"]
    pub targtype: GrantTargetType,
    #[doc = " type of the grant target"]
    pub objtype: ObjectType,
    #[doc = " kind of object being operated on"]
    pub objects: *mut List,
    #[doc = " list of RangeVar nodes, ObjectWithArgs"]
    #[doc = " nodes, or plain names (as Value strings)"]
    pub privileges: *mut List,
    #[doc = " list of AccessPriv nodes */"]
    pub grantees: *mut List,
    #[doc = " list of RoleSpec nodes"]
    pub grant_option: bool,
    #[doc = " grant or revoke grant option"]
    pub behavior: DropBehavior,
}
#[test]
fn bindgen_test_layout_GrantStmt() {
    assert_eq!(
        ::std::mem::size_of::<GrantStmt>(),
        48usize,
        concat!("Size of: ", stringify!(GrantStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<GrantStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(GrantStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantStmt>())).is_grant as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantStmt),
            "::",
            stringify!(is_grant)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantStmt>())).targtype as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantStmt),
            "::",
            stringify!(targtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantStmt>())).objtype as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantStmt),
            "::",
            stringify!(objtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantStmt>())).objects as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantStmt),
            "::",
            stringify!(objects)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantStmt>())).privileges as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantStmt),
            "::",
            stringify!(privileges)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantStmt>())).grantees as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantStmt),
            "::",
            stringify!(grantees)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantStmt>())).grant_option as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantStmt),
            "::",
            stringify!(grant_option)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantStmt>())).behavior as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantStmt),
            "::",
            stringify!(behavior)
        )
    );
}
impl Default for GrantStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Note: ObjectWithArgs carries only the types of the input parameters of the"]
#[doc = " function.  So it is sufficient to identify an existing function, but it"]
#[doc = " is not enough info to define a function nor to call it."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ObjectWithArgs {
    pub type_: NodeTag,
    pub objname: *mut List,
    #[doc = " qualified name of function/operator"]
    pub objargs: *mut List,
    #[doc = " list of Typename nodes"]
    pub args_unspecified: bool,
}
#[test]
fn bindgen_test_layout_ObjectWithArgs() {
    assert_eq!(
        ::std::mem::size_of::<ObjectWithArgs>(),
        32usize,
        concat!("Size of: ", stringify!(ObjectWithArgs))
    );
    assert_eq!(
        ::std::mem::align_of::<ObjectWithArgs>(),
        8usize,
        concat!("Alignment of ", stringify!(ObjectWithArgs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ObjectWithArgs>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ObjectWithArgs),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ObjectWithArgs>())).objname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ObjectWithArgs),
            "::",
            stringify!(objname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ObjectWithArgs>())).objargs as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ObjectWithArgs),
            "::",
            stringify!(objargs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ObjectWithArgs>())).args_unspecified as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ObjectWithArgs),
            "::",
            stringify!(args_unspecified)
        )
    );
}
impl Default for ObjectWithArgs {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " An access privilege, with optional list of column names"]
#[doc = " priv_name == NULL denotes ALL PRIVILEGES (only used with a column list)"]
#[doc = " cols == NIL denotes \"all columns\""]
#[doc = " Note that simple \"ALL PRIVILEGES\" is represented as a NIL list, not"]
#[doc = " an AccessPriv with both fields null."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AccessPriv {
    pub type_: NodeTag,
    pub priv_name: *mut ::std::os::raw::c_char,
    #[doc = " string name of privilege"]
    pub cols: *mut List,
}
#[test]
fn bindgen_test_layout_AccessPriv() {
    assert_eq!(
        ::std::mem::size_of::<AccessPriv>(),
        24usize,
        concat!("Size of: ", stringify!(AccessPriv))
    );
    assert_eq!(
        ::std::mem::align_of::<AccessPriv>(),
        8usize,
        concat!("Alignment of ", stringify!(AccessPriv))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccessPriv>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AccessPriv),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccessPriv>())).priv_name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AccessPriv),
            "::",
            stringify!(priv_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AccessPriv>())).cols as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AccessPriv),
            "::",
            stringify!(cols)
        )
    );
}
impl Default for AccessPriv {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Grant/Revoke Role Statement"]
#[doc = ""]
#[doc = " Note: because of the parsing ambiguity with the GRANT <privileges>"]
#[doc = " statement, granted_roles is a list of AccessPriv; the execution code"]
#[doc = " should complain if any column lists appear.  grantee_roles is a list"]
#[doc = " of role names, as Value strings."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GrantRoleStmt {
    pub type_: NodeTag,
    pub granted_roles: *mut List,
    #[doc = " list of roles to be granted/revoked"]
    pub grantee_roles: *mut List,
    #[doc = " list of member roles to add/delete"]
    pub is_grant: bool,
    #[doc = " true = GRANT, false = REVOKE"]
    pub admin_opt: bool,
    #[doc = " with admin option"]
    pub grantor: *mut RoleSpec,
    #[doc = " set grantor to other than current role"]
    pub behavior: DropBehavior,
}
#[test]
fn bindgen_test_layout_GrantRoleStmt() {
    assert_eq!(
        ::std::mem::size_of::<GrantRoleStmt>(),
        48usize,
        concat!("Size of: ", stringify!(GrantRoleStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<GrantRoleStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(GrantRoleStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantRoleStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantRoleStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantRoleStmt>())).granted_roles as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantRoleStmt),
            "::",
            stringify!(granted_roles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantRoleStmt>())).grantee_roles as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantRoleStmt),
            "::",
            stringify!(grantee_roles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantRoleStmt>())).is_grant as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantRoleStmt),
            "::",
            stringify!(is_grant)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantRoleStmt>())).admin_opt as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantRoleStmt),
            "::",
            stringify!(admin_opt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantRoleStmt>())).grantor as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantRoleStmt),
            "::",
            stringify!(grantor)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GrantRoleStmt>())).behavior as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(GrantRoleStmt),
            "::",
            stringify!(behavior)
        )
    );
}
impl Default for GrantRoleStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Alter Default Privileges Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterDefaultPrivilegesStmt {
    pub type_: NodeTag,
    pub options: *mut List,
    #[doc = " list of DefElem"]
    pub action: *mut GrantStmt,
}
#[test]
fn bindgen_test_layout_AlterDefaultPrivilegesStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterDefaultPrivilegesStmt>(),
        24usize,
        concat!("Size of: ", stringify!(AlterDefaultPrivilegesStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterDefaultPrivilegesStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterDefaultPrivilegesStmt))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterDefaultPrivilegesStmt>())).type_ as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDefaultPrivilegesStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterDefaultPrivilegesStmt>())).options as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDefaultPrivilegesStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterDefaultPrivilegesStmt>())).action as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDefaultPrivilegesStmt),
            "::",
            stringify!(action)
        )
    );
}
impl Default for AlterDefaultPrivilegesStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Copy Statement"]
#[doc = ""]
#[doc = " We support \"COPY relation FROM file\", \"COPY relation TO file\", and"]
#[doc = " \"COPY (query) TO file\".  In any given CopyStmt, exactly one of \"relation\""]
#[doc = " and \"query\" must be nonNULL."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CopyStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    #[doc = " the relation to copy"]
    pub query: *mut Node,
    #[doc = " the query (SELECT or DML statement with"]
    #[doc = " RETURNING) to copy, as a raw parse tree"]
    pub attlist: *mut List,
    #[doc = " List of column names (as Strings), or NIL"]
    #[doc = " for all columns"]
    pub is_from: bool,
    #[doc = " TO or FROM"]
    pub is_program: bool,
    #[doc = " is 'filename' a program to popen?"]
    pub filename: *mut ::std::os::raw::c_char,
    #[doc = " filename, or NULL for STDIN/STDOUT"]
    pub options: *mut List,
    #[doc = " List of DefElem nodes"]
    pub whereClause: *mut Node,
}
#[test]
fn bindgen_test_layout_CopyStmt() {
    assert_eq!(
        ::std::mem::size_of::<CopyStmt>(),
        64usize,
        concat!("Size of: ", stringify!(CopyStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CopyStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CopyStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CopyStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CopyStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CopyStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CopyStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CopyStmt>())).query as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CopyStmt),
            "::",
            stringify!(query)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CopyStmt>())).attlist as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CopyStmt),
            "::",
            stringify!(attlist)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CopyStmt>())).is_from as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CopyStmt),
            "::",
            stringify!(is_from)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CopyStmt>())).is_program as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(CopyStmt),
            "::",
            stringify!(is_program)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CopyStmt>())).filename as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CopyStmt),
            "::",
            stringify!(filename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CopyStmt>())).options as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(CopyStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CopyStmt>())).whereClause as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(CopyStmt),
            "::",
            stringify!(whereClause)
        )
    );
}
impl Default for CopyStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " SET Statement (includes RESET)"]
#[doc = ""]
#[doc = " \"SET var TO DEFAULT\" and \"RESET var\" are semantically equivalent, but we"]
#[doc = " preserve the distinction in VariableSetKind for CreateCommandTag()."]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VariableSetKind {
    VAR_SET_VALUE = 0,
    #[doc = " SET var = value"]
    VAR_SET_DEFAULT = 1,
    #[doc = " SET var TO DEFAULT"]
    VAR_SET_CURRENT = 2,
    #[doc = " SET var FROM CURRENT"]
    VAR_SET_MULTI = 3,
    #[doc = " special case for SET TRANSACTION ..."]
    VAR_RESET = 4,
    #[doc = " RESET var"]
    VAR_RESET_ALL = 5,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VariableSetStmt {
    pub type_: NodeTag,
    pub kind: VariableSetKind,
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " variable to be set"]
    pub args: *mut List,
    #[doc = " List of A_Const nodes"]
    pub is_local: bool,
}
#[test]
fn bindgen_test_layout_VariableSetStmt() {
    assert_eq!(
        ::std::mem::size_of::<VariableSetStmt>(),
        32usize,
        concat!("Size of: ", stringify!(VariableSetStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<VariableSetStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(VariableSetStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VariableSetStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VariableSetStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VariableSetStmt>())).kind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(VariableSetStmt),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VariableSetStmt>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VariableSetStmt),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VariableSetStmt>())).args as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VariableSetStmt),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VariableSetStmt>())).is_local as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VariableSetStmt),
            "::",
            stringify!(is_local)
        )
    );
}
impl Default for VariableSetStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Show Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VariableShowStmt {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_VariableShowStmt() {
    assert_eq!(
        ::std::mem::size_of::<VariableShowStmt>(),
        16usize,
        concat!("Size of: ", stringify!(VariableShowStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<VariableShowStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(VariableShowStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VariableShowStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VariableShowStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VariableShowStmt>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VariableShowStmt),
            "::",
            stringify!(name)
        )
    );
}
impl Default for VariableShowStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Table Statement"]
#[doc = ""]
#[doc = " NOTE: in the raw gram.y output, ColumnDef and Constraint nodes are"]
#[doc = " intermixed in tableElts, and constraints is NIL.  After parse analysis,"]
#[doc = " tableElts contains just ColumnDefs, and constraints contains just"]
#[doc = " Constraint nodes (in fact, only CONSTR_CHECK nodes, in the present"]
#[doc = " implementation)."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    #[doc = " relation to create"]
    pub tableElts: *mut List,
    #[doc = " column definitions (list of ColumnDef)"]
    pub inhRelations: *mut List,
    #[doc = " relations to inherit from (list of"]
    #[doc = " inhRelation)"]
    pub partbound: *mut PartitionBoundSpec,
    #[doc = " FOR VALUES clause"]
    pub partspec: *mut PartitionSpec,
    #[doc = " PARTITION BY clause"]
    pub ofTypename: *mut TypeName,
    #[doc = " OF typename"]
    pub constraints: *mut List,
    #[doc = " constraints (list of Constraint nodes)"]
    pub options: *mut List,
    #[doc = " options from WITH clause"]
    pub oncommit: OnCommitAction,
    #[doc = " what do we do at COMMIT?"]
    pub tablespacename: *mut ::std::os::raw::c_char,
    #[doc = " table space to use, or NULL"]
    pub accessMethod: *mut ::std::os::raw::c_char,
    #[doc = " table access method"]
    pub if_not_exists: bool,
}
#[test]
fn bindgen_test_layout_CreateStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateStmt>(),
        104usize,
        concat!("Size of: ", stringify!(CreateStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).tableElts as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(tableElts)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).inhRelations as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(inhRelations)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).partbound as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(partbound)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).partspec as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(partspec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).ofTypename as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(ofTypename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).constraints as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(constraints)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).options as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).oncommit as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(oncommit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).tablespacename as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(tablespacename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).accessMethod as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(accessMethod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStmt>())).if_not_exists as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStmt),
            "::",
            stringify!(if_not_exists)
        )
    );
}
impl Default for CreateStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " Definitions for constraints in CreateStmt"]
#[doc = ""]
#[doc = " Note that column defaults are treated as a type of constraint,"]
#[doc = " even though that's a bit odd semantically."]
#[doc = ""]
#[doc = " For constraints that use expressions (CONSTR_CHECK, CONSTR_DEFAULT)"]
#[doc = " we may have the expression in either \"raw\" form (an untransformed"]
#[doc = " parse tree) or \"cooked\" form (the nodeToString representation of"]
#[doc = " an executable expression tree), depending on how this Constraint"]
#[doc = " node was created (by parsing, or by inheritance from an existing"]
#[doc = " relation).  We should never have both in the same node!"]
#[doc = ""]
#[doc = " FKCONSTR_ACTION_xxx values are stored into pg_constraint.confupdtype"]
#[doc = " and pg_constraint.confdeltype columns; FKCONSTR_MATCH_xxx values are"]
#[doc = " stored into pg_constraint.confmatchtype.  Changing the code values may"]
#[doc = " require an initdb!"]
#[doc = ""]
#[doc = " If skip_validation is true then we skip checking that the existing rows"]
#[doc = " in the table satisfy the constraint, and just install the catalog entries"]
#[doc = " for the constraint.  A new FK constraint is marked as valid iff"]
#[doc = " initially_valid is true.  (Usually skip_validation and initially_valid"]
#[doc = " are inverses, but we can set both true if the table is known empty.)"]
#[doc = ""]
#[doc = " Constraint attributes (DEFERRABLE etc) are initially represented as"]
#[doc = " separate Constraint nodes for simplicity of parsing.  parse_utilcmd.c makes"]
#[doc = " a pass through the constraints list to insert the info into the appropriate"]
#[doc = " Constraint node."]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConstrType {
    CONSTR_NULL = 0,
    #[doc = " not standard SQL, but a lot of people"]
    #[doc = " expect it"]
    CONSTR_NOTNULL = 1,
    #[doc = " not standard SQL, but a lot of people"]
    #[doc = " expect it"]
    CONSTR_DEFAULT = 2,
    #[doc = " not standard SQL, but a lot of people"]
    #[doc = " expect it"]
    CONSTR_IDENTITY = 3,
    #[doc = " not standard SQL, but a lot of people"]
    #[doc = " expect it"]
    CONSTR_GENERATED = 4,
    #[doc = " not standard SQL, but a lot of people"]
    #[doc = " expect it"]
    CONSTR_CHECK = 5,
    #[doc = " not standard SQL, but a lot of people"]
    #[doc = " expect it"]
    CONSTR_PRIMARY = 6,
    #[doc = " not standard SQL, but a lot of people"]
    #[doc = " expect it"]
    CONSTR_UNIQUE = 7,
    #[doc = " not standard SQL, but a lot of people"]
    #[doc = " expect it"]
    CONSTR_EXCLUSION = 8,
    #[doc = " not standard SQL, but a lot of people"]
    #[doc = " expect it"]
    CONSTR_FOREIGN = 9,
    #[doc = " not standard SQL, but a lot of people"]
    #[doc = " expect it"]
    CONSTR_ATTR_DEFERRABLE = 10,
    #[doc = " attributes for previous constraint node"]
    CONSTR_ATTR_NOT_DEFERRABLE = 11,
    #[doc = " attributes for previous constraint node"]
    CONSTR_ATTR_DEFERRED = 12,
    #[doc = " attributes for previous constraint node"]
    CONSTR_ATTR_IMMEDIATE = 13,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Constraint {
    pub type_: NodeTag,
    pub contype: ConstrType,
    #[doc = " Fields used for most/all constraint types:"]
    pub conname: *mut ::std::os::raw::c_char,
    #[doc = " Constraint name, or NULL if unnamed"]
    pub deferrable: bool,
    #[doc = " DEFERRABLE?"]
    pub initdeferred: bool,
    #[doc = " INITIALLY DEFERRED?"]
    pub location: ::std::os::raw::c_int,
    #[doc = " Fields used for constraints with expressions (CHECK and DEFAULT):"]
    pub is_no_inherit: bool,
    #[doc = " is constraint noninheritable?"]
    pub raw_expr: *mut Node,
    #[doc = " expr, as untransformed parse tree"]
    pub cooked_expr: *mut ::std::os::raw::c_char,
    #[doc = " expr, as nodeToString representation"]
    pub generated_when: ::std::os::raw::c_char,
    #[doc = " Fields used for unique constraints (UNIQUE and PRIMARY KEY):"]
    pub keys: *mut List,
    #[doc = " String nodes naming referenced key"]
    #[doc = " column(s)"]
    pub including: *mut List,
    #[doc = " Fields used for EXCLUSION constraints:"]
    pub exclusions: *mut List,
    #[doc = " Fields used for index constraints (UNIQUE, PRIMARY KEY, EXCLUSION):"]
    pub options: *mut List,
    #[doc = " options from WITH clause"]
    pub indexname: *mut ::std::os::raw::c_char,
    #[doc = " existing index to use; otherwise NULL"]
    pub indexspace: *mut ::std::os::raw::c_char,
    #[doc = " index tablespace; NULL for default"]
    pub reset_default_tblspc: bool,
    #[doc = " reset default_tablespace prior to"]
    #[doc = " creating the index */"]
    pub access_method: *mut ::std::os::raw::c_char,
    #[doc = " index access method; NULL for default"]
    pub where_clause: *mut Node,
    #[doc = " Fields used for FOREIGN KEY constraints:"]
    pub pktable: *mut RangeVar,
    #[doc = " Primary key table"]
    pub fk_attrs: *mut List,
    #[doc = " Attributes of foreign key"]
    pub pk_attrs: *mut List,
    #[doc = " Corresponding attrs in PK table"]
    pub fk_matchtype: ::std::os::raw::c_char,
    #[doc = " FULL, PARTIAL, SIMPLE"]
    pub fk_upd_action: ::std::os::raw::c_char,
    #[doc = " ON UPDATE action"]
    pub fk_del_action: ::std::os::raw::c_char,
    #[doc = " ON DELETE action"]
    pub old_conpfeqop: *mut List,
    #[doc = " pg_constraint.conpfeqop of my former self"]
    pub old_pktable_oid: Oid,
    #[doc = " Fields used for constraints that allow a NOT VALID specification"]
    pub skip_validation: bool,
    #[doc = " skip validation of existing rows?"]
    pub initially_valid: bool,
}
#[test]
fn bindgen_test_layout_Constraint() {
    assert_eq!(
        ::std::mem::size_of::<Constraint>(),
        176usize,
        concat!("Size of: ", stringify!(Constraint))
    );
    assert_eq!(
        ::std::mem::align_of::<Constraint>(),
        8usize,
        concat!("Alignment of ", stringify!(Constraint))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).contype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(contype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).conname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(conname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).deferrable as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(deferrable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).initdeferred as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(initdeferred)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).location as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(location)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).is_no_inherit as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(is_no_inherit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).raw_expr as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(raw_expr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).cooked_expr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(cooked_expr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).generated_when as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(generated_when)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).keys as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(keys)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).including as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(including)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).exclusions as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(exclusions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).options as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).indexname as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(indexname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).indexspace as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(indexspace)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).reset_default_tblspc as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(reset_default_tblspc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).access_method as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(access_method)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).where_clause as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(where_clause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).pktable as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(pktable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).fk_attrs as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(fk_attrs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).pk_attrs as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(pk_attrs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).fk_matchtype as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(fk_matchtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).fk_upd_action as *const _ as usize },
        153usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(fk_upd_action)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).fk_del_action as *const _ as usize },
        154usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(fk_del_action)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).old_conpfeqop as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(old_conpfeqop)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).old_pktable_oid as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(old_pktable_oid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).skip_validation as *const _ as usize },
        172usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(skip_validation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Constraint>())).initially_valid as *const _ as usize },
        173usize,
        concat!(
            "Offset of field: ",
            stringify!(Constraint),
            "::",
            stringify!(initially_valid)
        )
    );
}
impl Default for Constraint {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create/Drop Table Space Statements"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateTableSpaceStmt {
    pub type_: NodeTag,
    pub tablespacename: *mut ::std::os::raw::c_char,
    pub owner: *mut RoleSpec,
    pub location: *mut ::std::os::raw::c_char,
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_CreateTableSpaceStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateTableSpaceStmt>(),
        40usize,
        concat!("Size of: ", stringify!(CreateTableSpaceStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateTableSpaceStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateTableSpaceStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTableSpaceStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableSpaceStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateTableSpaceStmt>())).tablespacename as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableSpaceStmt),
            "::",
            stringify!(tablespacename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTableSpaceStmt>())).owner as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableSpaceStmt),
            "::",
            stringify!(owner)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTableSpaceStmt>())).location as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableSpaceStmt),
            "::",
            stringify!(location)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTableSpaceStmt>())).options as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableSpaceStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for CreateTableSpaceStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DropTableSpaceStmt {
    pub type_: NodeTag,
    pub tablespacename: *mut ::std::os::raw::c_char,
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_DropTableSpaceStmt() {
    assert_eq!(
        ::std::mem::size_of::<DropTableSpaceStmt>(),
        24usize,
        concat!("Size of: ", stringify!(DropTableSpaceStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DropTableSpaceStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DropTableSpaceStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropTableSpaceStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DropTableSpaceStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<DropTableSpaceStmt>())).tablespacename as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DropTableSpaceStmt),
            "::",
            stringify!(tablespacename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropTableSpaceStmt>())).missing_ok as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DropTableSpaceStmt),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for DropTableSpaceStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterTableSpaceOptionsStmt {
    pub type_: NodeTag,
    pub tablespacename: *mut ::std::os::raw::c_char,
    pub options: *mut List,
    pub isReset: bool,
}
#[test]
fn bindgen_test_layout_AlterTableSpaceOptionsStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterTableSpaceOptionsStmt>(),
        32usize,
        concat!("Size of: ", stringify!(AlterTableSpaceOptionsStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterTableSpaceOptionsStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterTableSpaceOptionsStmt))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTableSpaceOptionsStmt>())).type_ as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableSpaceOptionsStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTableSpaceOptionsStmt>())).tablespacename as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableSpaceOptionsStmt),
            "::",
            stringify!(tablespacename)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTableSpaceOptionsStmt>())).options as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableSpaceOptionsStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTableSpaceOptionsStmt>())).isReset as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableSpaceOptionsStmt),
            "::",
            stringify!(isReset)
        )
    );
}
impl Default for AlterTableSpaceOptionsStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterTableMoveAllStmt {
    pub type_: NodeTag,
    pub orig_tablespacename: *mut ::std::os::raw::c_char,
    pub objtype: ObjectType,
    #[doc = " Object type to move"]
    pub roles: *mut List,
    #[doc = " List of roles to move objects of"]
    pub new_tablespacename: *mut ::std::os::raw::c_char,
    pub nowait: bool,
}
#[test]
fn bindgen_test_layout_AlterTableMoveAllStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterTableMoveAllStmt>(),
        48usize,
        concat!("Size of: ", stringify!(AlterTableMoveAllStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterTableMoveAllStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterTableMoveAllStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableMoveAllStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableMoveAllStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTableMoveAllStmt>())).orig_tablespacename as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableMoveAllStmt),
            "::",
            stringify!(orig_tablespacename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableMoveAllStmt>())).objtype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableMoveAllStmt),
            "::",
            stringify!(objtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableMoveAllStmt>())).roles as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableMoveAllStmt),
            "::",
            stringify!(roles)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTableMoveAllStmt>())).new_tablespacename as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableMoveAllStmt),
            "::",
            stringify!(new_tablespacename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTableMoveAllStmt>())).nowait as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTableMoveAllStmt),
            "::",
            stringify!(nowait)
        )
    );
}
impl Default for AlterTableMoveAllStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create/Alter Extension Statements"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateExtensionStmt {
    pub type_: NodeTag,
    pub extname: *mut ::std::os::raw::c_char,
    pub if_not_exists: bool,
    #[doc = " just do nothing if it already exists?"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_CreateExtensionStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateExtensionStmt>(),
        32usize,
        concat!("Size of: ", stringify!(CreateExtensionStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateExtensionStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateExtensionStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateExtensionStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateExtensionStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateExtensionStmt>())).extname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateExtensionStmt),
            "::",
            stringify!(extname)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateExtensionStmt>())).if_not_exists as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateExtensionStmt),
            "::",
            stringify!(if_not_exists)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateExtensionStmt>())).options as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateExtensionStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for CreateExtensionStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Only used for ALTER EXTENSION UPDATE; later might need an action field"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterExtensionStmt {
    pub type_: NodeTag,
    pub extname: *mut ::std::os::raw::c_char,
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_AlterExtensionStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterExtensionStmt>(),
        24usize,
        concat!("Size of: ", stringify!(AlterExtensionStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterExtensionStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterExtensionStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterExtensionStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterExtensionStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterExtensionStmt>())).extname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterExtensionStmt),
            "::",
            stringify!(extname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterExtensionStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterExtensionStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for AlterExtensionStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterExtensionContentsStmt {
    pub type_: NodeTag,
    pub extname: *mut ::std::os::raw::c_char,
    #[doc = " Extension's name"]
    pub action: ::std::os::raw::c_int,
    #[doc = " +1 = add object, 1 = drop object"]
    pub objtype: ObjectType,
    #[doc = " Object's type"]
    pub object: *mut Node,
}
#[test]
fn bindgen_test_layout_AlterExtensionContentsStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterExtensionContentsStmt>(),
        32usize,
        concat!("Size of: ", stringify!(AlterExtensionContentsStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterExtensionContentsStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterExtensionContentsStmt))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterExtensionContentsStmt>())).type_ as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterExtensionContentsStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterExtensionContentsStmt>())).extname as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterExtensionContentsStmt),
            "::",
            stringify!(extname)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterExtensionContentsStmt>())).action as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterExtensionContentsStmt),
            "::",
            stringify!(action)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterExtensionContentsStmt>())).objtype as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterExtensionContentsStmt),
            "::",
            stringify!(objtype)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterExtensionContentsStmt>())).object as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterExtensionContentsStmt),
            "::",
            stringify!(object)
        )
    );
}
impl Default for AlterExtensionContentsStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create/Alter FOREIGN DATA WRAPPER Statements"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateFdwStmt {
    pub type_: NodeTag,
    pub fdwname: *mut ::std::os::raw::c_char,
    #[doc = " foreigndata wrapper name"]
    pub func_options: *mut List,
    #[doc = " HANDLER/VALIDATOR options"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_CreateFdwStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateFdwStmt>(),
        32usize,
        concat!("Size of: ", stringify!(CreateFdwStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateFdwStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateFdwStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFdwStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFdwStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFdwStmt>())).fdwname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFdwStmt),
            "::",
            stringify!(fdwname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFdwStmt>())).func_options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFdwStmt),
            "::",
            stringify!(func_options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFdwStmt>())).options as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFdwStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for CreateFdwStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterFdwStmt {
    pub type_: NodeTag,
    pub fdwname: *mut ::std::os::raw::c_char,
    #[doc = " foreigndata wrapper name"]
    pub func_options: *mut List,
    #[doc = " HANDLER/VALIDATOR options"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_AlterFdwStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterFdwStmt>(),
        32usize,
        concat!("Size of: ", stringify!(AlterFdwStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterFdwStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterFdwStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterFdwStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterFdwStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterFdwStmt>())).fdwname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterFdwStmt),
            "::",
            stringify!(fdwname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterFdwStmt>())).func_options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterFdwStmt),
            "::",
            stringify!(func_options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterFdwStmt>())).options as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterFdwStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for AlterFdwStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create/Alter FOREIGN SERVER Statements"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateForeignServerStmt {
    pub type_: NodeTag,
    pub servername: *mut ::std::os::raw::c_char,
    #[doc = " server name"]
    pub servertype: *mut ::std::os::raw::c_char,
    #[doc = " optional server type"]
    pub version: *mut ::std::os::raw::c_char,
    #[doc = " optional server version"]
    pub fdwname: *mut ::std::os::raw::c_char,
    #[doc = " FDW name"]
    pub if_not_exists: bool,
    #[doc = " just do nothing if it already exists?"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_CreateForeignServerStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateForeignServerStmt>(),
        56usize,
        concat!("Size of: ", stringify!(CreateForeignServerStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateForeignServerStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateForeignServerStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateForeignServerStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateForeignServerStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateForeignServerStmt>())).servername as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateForeignServerStmt),
            "::",
            stringify!(servername)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateForeignServerStmt>())).servertype as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateForeignServerStmt),
            "::",
            stringify!(servertype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateForeignServerStmt>())).version as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateForeignServerStmt),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateForeignServerStmt>())).fdwname as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateForeignServerStmt),
            "::",
            stringify!(fdwname)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateForeignServerStmt>())).if_not_exists as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateForeignServerStmt),
            "::",
            stringify!(if_not_exists)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateForeignServerStmt>())).options as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateForeignServerStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for CreateForeignServerStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterForeignServerStmt {
    pub type_: NodeTag,
    pub servername: *mut ::std::os::raw::c_char,
    #[doc = " server name"]
    pub version: *mut ::std::os::raw::c_char,
    #[doc = " optional server version"]
    pub options: *mut List,
    #[doc = " generic options to server"]
    pub has_version: bool,
}
#[test]
fn bindgen_test_layout_AlterForeignServerStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterForeignServerStmt>(),
        40usize,
        concat!("Size of: ", stringify!(AlterForeignServerStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterForeignServerStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterForeignServerStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterForeignServerStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterForeignServerStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterForeignServerStmt>())).servername as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterForeignServerStmt),
            "::",
            stringify!(servername)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterForeignServerStmt>())).version as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterForeignServerStmt),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterForeignServerStmt>())).options as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterForeignServerStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterForeignServerStmt>())).has_version as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterForeignServerStmt),
            "::",
            stringify!(has_version)
        )
    );
}
impl Default for AlterForeignServerStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create FOREIGN TABLE Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateForeignTableStmt {
    pub base: CreateStmt,
    pub servername: *mut ::std::os::raw::c_char,
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_CreateForeignTableStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateForeignTableStmt>(),
        120usize,
        concat!("Size of: ", stringify!(CreateForeignTableStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateForeignTableStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateForeignTableStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateForeignTableStmt>())).base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateForeignTableStmt),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateForeignTableStmt>())).servername as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateForeignTableStmt),
            "::",
            stringify!(servername)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateForeignTableStmt>())).options as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateForeignTableStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for CreateForeignTableStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create/Drop USER MAPPING Statements"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateUserMappingStmt {
    pub type_: NodeTag,
    pub user: *mut RoleSpec,
    #[doc = " user role"]
    pub servername: *mut ::std::os::raw::c_char,
    #[doc = " server name"]
    pub if_not_exists: bool,
    #[doc = " just do nothing if it already exists?"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_CreateUserMappingStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateUserMappingStmt>(),
        40usize,
        concat!("Size of: ", stringify!(CreateUserMappingStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateUserMappingStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateUserMappingStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateUserMappingStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateUserMappingStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateUserMappingStmt>())).user as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateUserMappingStmt),
            "::",
            stringify!(user)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateUserMappingStmt>())).servername as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateUserMappingStmt),
            "::",
            stringify!(servername)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateUserMappingStmt>())).if_not_exists as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateUserMappingStmt),
            "::",
            stringify!(if_not_exists)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateUserMappingStmt>())).options as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateUserMappingStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for CreateUserMappingStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterUserMappingStmt {
    pub type_: NodeTag,
    pub user: *mut RoleSpec,
    #[doc = " user role"]
    pub servername: *mut ::std::os::raw::c_char,
    #[doc = " server name"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_AlterUserMappingStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterUserMappingStmt>(),
        32usize,
        concat!("Size of: ", stringify!(AlterUserMappingStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterUserMappingStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterUserMappingStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterUserMappingStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterUserMappingStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterUserMappingStmt>())).user as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterUserMappingStmt),
            "::",
            stringify!(user)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterUserMappingStmt>())).servername as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterUserMappingStmt),
            "::",
            stringify!(servername)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterUserMappingStmt>())).options as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterUserMappingStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for AlterUserMappingStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DropUserMappingStmt {
    pub type_: NodeTag,
    pub user: *mut RoleSpec,
    #[doc = " user role"]
    pub servername: *mut ::std::os::raw::c_char,
    #[doc = " server name"]
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_DropUserMappingStmt() {
    assert_eq!(
        ::std::mem::size_of::<DropUserMappingStmt>(),
        32usize,
        concat!("Size of: ", stringify!(DropUserMappingStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DropUserMappingStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DropUserMappingStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropUserMappingStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DropUserMappingStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropUserMappingStmt>())).user as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DropUserMappingStmt),
            "::",
            stringify!(user)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropUserMappingStmt>())).servername as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DropUserMappingStmt),
            "::",
            stringify!(servername)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropUserMappingStmt>())).missing_ok as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DropUserMappingStmt),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for DropUserMappingStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = "    Import Foreign Schema Statement"]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImportForeignSchemaType {
    FDW_IMPORT_SCHEMA_ALL = 0,
    #[doc = " all relations wanted"]
    FDW_IMPORT_SCHEMA_LIMIT_TO = 1,
    #[doc = " include only listed tables in import"]
    FDW_IMPORT_SCHEMA_EXCEPT = 2,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ImportForeignSchemaStmt {
    pub type_: NodeTag,
    pub server_name: *mut ::std::os::raw::c_char,
    #[doc = " FDW server name"]
    pub remote_schema: *mut ::std::os::raw::c_char,
    #[doc = " remote schema name to query"]
    pub local_schema: *mut ::std::os::raw::c_char,
    #[doc = " local schema to create objects in"]
    pub list_type: ImportForeignSchemaType,
    #[doc = " type of table list"]
    pub table_list: *mut List,
    #[doc = " List of RangeVar"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_ImportForeignSchemaStmt() {
    assert_eq!(
        ::std::mem::size_of::<ImportForeignSchemaStmt>(),
        56usize,
        concat!("Size of: ", stringify!(ImportForeignSchemaStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ImportForeignSchemaStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ImportForeignSchemaStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ImportForeignSchemaStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ImportForeignSchemaStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ImportForeignSchemaStmt>())).server_name as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ImportForeignSchemaStmt),
            "::",
            stringify!(server_name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ImportForeignSchemaStmt>())).remote_schema as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ImportForeignSchemaStmt),
            "::",
            stringify!(remote_schema)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ImportForeignSchemaStmt>())).local_schema as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ImportForeignSchemaStmt),
            "::",
            stringify!(local_schema)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ImportForeignSchemaStmt>())).list_type as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ImportForeignSchemaStmt),
            "::",
            stringify!(list_type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ImportForeignSchemaStmt>())).table_list as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ImportForeignSchemaStmt),
            "::",
            stringify!(table_list)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ImportForeignSchemaStmt>())).options as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ImportForeignSchemaStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for ImportForeignSchemaStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create POLICY Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreatePolicyStmt {
    pub type_: NodeTag,
    pub policy_name: *mut ::std::os::raw::c_char,
    #[doc = " Policy's name"]
    pub table: *mut RangeVar,
    #[doc = " the table name the policy applies to"]
    pub cmd_name: *mut ::std::os::raw::c_char,
    #[doc = " the command name the policy applies to"]
    pub permissive: bool,
    #[doc = " restrictive or permissive policy"]
    pub roles: *mut List,
    #[doc = " the roles associated with the policy"]
    pub qual: *mut Node,
    #[doc = " the policy's condition"]
    pub with_check: *mut Node,
}
#[test]
fn bindgen_test_layout_CreatePolicyStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreatePolicyStmt>(),
        64usize,
        concat!("Size of: ", stringify!(CreatePolicyStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreatePolicyStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreatePolicyStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePolicyStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePolicyStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePolicyStmt>())).policy_name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePolicyStmt),
            "::",
            stringify!(policy_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePolicyStmt>())).table as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePolicyStmt),
            "::",
            stringify!(table)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePolicyStmt>())).cmd_name as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePolicyStmt),
            "::",
            stringify!(cmd_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePolicyStmt>())).permissive as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePolicyStmt),
            "::",
            stringify!(permissive)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePolicyStmt>())).roles as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePolicyStmt),
            "::",
            stringify!(roles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePolicyStmt>())).qual as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePolicyStmt),
            "::",
            stringify!(qual)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePolicyStmt>())).with_check as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePolicyStmt),
            "::",
            stringify!(with_check)
        )
    );
}
impl Default for CreatePolicyStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Alter POLICY Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterPolicyStmt {
    pub type_: NodeTag,
    pub policy_name: *mut ::std::os::raw::c_char,
    #[doc = " Policy's name"]
    pub table: *mut RangeVar,
    #[doc = " the table name the policy applies to"]
    pub roles: *mut List,
    #[doc = " the roles associated with the policy"]
    pub qual: *mut Node,
    #[doc = " the policy's condition"]
    pub with_check: *mut Node,
}
#[test]
fn bindgen_test_layout_AlterPolicyStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterPolicyStmt>(),
        48usize,
        concat!("Size of: ", stringify!(AlterPolicyStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterPolicyStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterPolicyStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterPolicyStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPolicyStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterPolicyStmt>())).policy_name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPolicyStmt),
            "::",
            stringify!(policy_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterPolicyStmt>())).table as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPolicyStmt),
            "::",
            stringify!(table)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterPolicyStmt>())).roles as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPolicyStmt),
            "::",
            stringify!(roles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterPolicyStmt>())).qual as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPolicyStmt),
            "::",
            stringify!(qual)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterPolicyStmt>())).with_check as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPolicyStmt),
            "::",
            stringify!(with_check)
        )
    );
}
impl Default for AlterPolicyStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create ACCESS METHOD Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateAmStmt {
    pub type_: NodeTag,
    pub amname: *mut ::std::os::raw::c_char,
    #[doc = " access method name"]
    pub handler_name: *mut List,
    #[doc = " handler function name"]
    pub amtype: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_CreateAmStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateAmStmt>(),
        32usize,
        concat!("Size of: ", stringify!(CreateAmStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateAmStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateAmStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateAmStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateAmStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateAmStmt>())).amname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateAmStmt),
            "::",
            stringify!(amname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateAmStmt>())).handler_name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateAmStmt),
            "::",
            stringify!(handler_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateAmStmt>())).amtype as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateAmStmt),
            "::",
            stringify!(amtype)
        )
    );
}
impl Default for CreateAmStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create TRIGGER Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateTrigStmt {
    pub type_: NodeTag,
    pub trigname: *mut ::std::os::raw::c_char,
    #[doc = " TRIGGER's name"]
    pub relation: *mut RangeVar,
    #[doc = " relation trigger is on"]
    pub funcname: *mut List,
    #[doc = " qual. name of function to call"]
    pub args: *mut List,
    #[doc = " list of (T_String) Values or NIL"]
    pub row: bool,
    #[doc = " ROW/STATEMENT */"]
    pub timing: int16,
    #[doc = " BEFORE, AFTER, or INSTEAD */"]
    pub events: int16,
    #[doc = " \"OR\" of INSERT/UPDATE/DELETE/TRUNCATE"]
    pub columns: *mut List,
    #[doc = " column names, or NIL for all columns"]
    pub whenClause: *mut Node,
    #[doc = " qual expression, or NULL if none"]
    pub isconstraint: bool,
    #[doc = " This is a constraint trigger */"]
    pub transitionRels: *mut List,
    #[doc = " TriggerTransition nodes, or NIL if none */"]
    pub deferrable: bool,
    #[doc = " [NOT] DEFERRABLE"]
    pub initdeferred: bool,
    #[doc = " INITIALLY {DEFERRED|IMMEDIATE}"]
    pub constrrel: *mut RangeVar,
}
#[test]
fn bindgen_test_layout_CreateTrigStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateTrigStmt>(),
        96usize,
        concat!("Size of: ", stringify!(CreateTrigStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateTrigStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateTrigStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).trigname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(trigname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).relation as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).funcname as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(funcname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).args as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).row as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(row)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).timing as *const _ as usize },
        42usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(timing)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).events as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(events)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).columns as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(columns)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).whenClause as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(whenClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).isconstraint as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(isconstraint)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).transitionRels as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(transitionRels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).deferrable as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(deferrable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).initdeferred as *const _ as usize },
        81usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(initdeferred)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTrigStmt>())).constrrel as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTrigStmt),
            "::",
            stringify!(constrrel)
        )
    );
}
impl Default for CreateTrigStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create EVENT TRIGGER Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateEventTrigStmt {
    pub type_: NodeTag,
    pub trigname: *mut ::std::os::raw::c_char,
    #[doc = " TRIGGER's name"]
    pub eventname: *mut ::std::os::raw::c_char,
    #[doc = " event's identifier"]
    pub whenclause: *mut List,
    #[doc = " list of DefElems indicating filtering"]
    pub funcname: *mut List,
}
#[test]
fn bindgen_test_layout_CreateEventTrigStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateEventTrigStmt>(),
        40usize,
        concat!("Size of: ", stringify!(CreateEventTrigStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateEventTrigStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateEventTrigStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateEventTrigStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateEventTrigStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateEventTrigStmt>())).trigname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateEventTrigStmt),
            "::",
            stringify!(trigname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateEventTrigStmt>())).eventname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateEventTrigStmt),
            "::",
            stringify!(eventname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateEventTrigStmt>())).whenclause as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateEventTrigStmt),
            "::",
            stringify!(whenclause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateEventTrigStmt>())).funcname as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateEventTrigStmt),
            "::",
            stringify!(funcname)
        )
    );
}
impl Default for CreateEventTrigStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Alter EVENT TRIGGER Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterEventTrigStmt {
    pub type_: NodeTag,
    pub trigname: *mut ::std::os::raw::c_char,
    #[doc = " TRIGGER's name"]
    pub tgenabled: ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_AlterEventTrigStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterEventTrigStmt>(),
        24usize,
        concat!("Size of: ", stringify!(AlterEventTrigStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterEventTrigStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterEventTrigStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterEventTrigStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterEventTrigStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterEventTrigStmt>())).trigname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterEventTrigStmt),
            "::",
            stringify!(trigname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterEventTrigStmt>())).tgenabled as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterEventTrigStmt),
            "::",
            stringify!(tgenabled)
        )
    );
}
impl Default for AlterEventTrigStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create LANGUAGE Statements"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreatePLangStmt {
    pub type_: NodeTag,
    pub replace: bool,
    #[doc = " T => replace if already exists"]
    pub plname: *mut ::std::os::raw::c_char,
    #[doc = " PL name"]
    pub plhandler: *mut List,
    #[doc = " PL call handler function (qual. name)"]
    pub plinline: *mut List,
    #[doc = " optional inline function (qual. name)"]
    pub plvalidator: *mut List,
    #[doc = " optional validator function (qual. name)"]
    pub pltrusted: bool,
}
#[test]
fn bindgen_test_layout_CreatePLangStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreatePLangStmt>(),
        48usize,
        concat!("Size of: ", stringify!(CreatePLangStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreatePLangStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreatePLangStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePLangStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePLangStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePLangStmt>())).replace as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePLangStmt),
            "::",
            stringify!(replace)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePLangStmt>())).plname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePLangStmt),
            "::",
            stringify!(plname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePLangStmt>())).plhandler as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePLangStmt),
            "::",
            stringify!(plhandler)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePLangStmt>())).plinline as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePLangStmt),
            "::",
            stringify!(plinline)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePLangStmt>())).plvalidator as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePLangStmt),
            "::",
            stringify!(plvalidator)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePLangStmt>())).pltrusted as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePLangStmt),
            "::",
            stringify!(pltrusted)
        )
    );
}
impl Default for CreatePLangStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " Create/Alter/Drop Role Statements"]
#[doc = ""]
#[doc = " Note: these node types are also used for the backwardscompatible"]
#[doc = " Create/Alter/Drop User/Group statements.  In the ALTER and DROP cases"]
#[doc = " there's really no need to distinguish what the original spelling was,"]
#[doc = " but for CREATE we mark the type because the defaults vary."]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RoleStmtType {
    ROLESTMT_ROLE = 0,
    ROLESTMT_USER = 1,
    ROLESTMT_GROUP = 2,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateRoleStmt {
    pub type_: NodeTag,
    pub stmt_type: RoleStmtType,
    #[doc = " ROLE/USER/GROUP"]
    pub role: *mut ::std::os::raw::c_char,
    #[doc = " role name"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_CreateRoleStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateRoleStmt>(),
        24usize,
        concat!("Size of: ", stringify!(CreateRoleStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateRoleStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateRoleStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateRoleStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateRoleStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateRoleStmt>())).stmt_type as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateRoleStmt),
            "::",
            stringify!(stmt_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateRoleStmt>())).role as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateRoleStmt),
            "::",
            stringify!(role)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateRoleStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateRoleStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for CreateRoleStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterRoleStmt {
    pub type_: NodeTag,
    pub role: *mut RoleSpec,
    #[doc = " role"]
    pub options: *mut List,
    #[doc = " List of DefElem nodes"]
    pub action: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_AlterRoleStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterRoleStmt>(),
        32usize,
        concat!("Size of: ", stringify!(AlterRoleStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterRoleStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterRoleStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterRoleStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterRoleStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterRoleStmt>())).role as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterRoleStmt),
            "::",
            stringify!(role)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterRoleStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterRoleStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterRoleStmt>())).action as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterRoleStmt),
            "::",
            stringify!(action)
        )
    );
}
impl Default for AlterRoleStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterRoleSetStmt {
    pub type_: NodeTag,
    pub role: *mut RoleSpec,
    #[doc = " role"]
    pub database: *mut ::std::os::raw::c_char,
    #[doc = " database name, or NULL"]
    pub setstmt: *mut VariableSetStmt,
}
#[test]
fn bindgen_test_layout_AlterRoleSetStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterRoleSetStmt>(),
        32usize,
        concat!("Size of: ", stringify!(AlterRoleSetStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterRoleSetStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterRoleSetStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterRoleSetStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterRoleSetStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterRoleSetStmt>())).role as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterRoleSetStmt),
            "::",
            stringify!(role)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterRoleSetStmt>())).database as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterRoleSetStmt),
            "::",
            stringify!(database)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterRoleSetStmt>())).setstmt as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterRoleSetStmt),
            "::",
            stringify!(setstmt)
        )
    );
}
impl Default for AlterRoleSetStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DropRoleStmt {
    pub type_: NodeTag,
    pub roles: *mut List,
    #[doc = " List of roles to remove"]
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_DropRoleStmt() {
    assert_eq!(
        ::std::mem::size_of::<DropRoleStmt>(),
        24usize,
        concat!("Size of: ", stringify!(DropRoleStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DropRoleStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DropRoleStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropRoleStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DropRoleStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropRoleStmt>())).roles as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DropRoleStmt),
            "::",
            stringify!(roles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropRoleStmt>())).missing_ok as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DropRoleStmt),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for DropRoleStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    {Create|Alter} SEQUENCE Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateSeqStmt {
    pub type_: NodeTag,
    pub sequence: *mut RangeVar,
    #[doc = " the sequence to create"]
    pub options: *mut List,
    pub ownerId: Oid,
    #[doc = " ID of owner, or InvalidOid for default"]
    pub for_identity: bool,
    pub if_not_exists: bool,
}
#[test]
fn bindgen_test_layout_CreateSeqStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateSeqStmt>(),
        32usize,
        concat!("Size of: ", stringify!(CreateSeqStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateSeqStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateSeqStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSeqStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSeqStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSeqStmt>())).sequence as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSeqStmt),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSeqStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSeqStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSeqStmt>())).ownerId as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSeqStmt),
            "::",
            stringify!(ownerId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSeqStmt>())).for_identity as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSeqStmt),
            "::",
            stringify!(for_identity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSeqStmt>())).if_not_exists as *const _ as usize },
        29usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSeqStmt),
            "::",
            stringify!(if_not_exists)
        )
    );
}
impl Default for CreateSeqStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterSeqStmt {
    pub type_: NodeTag,
    pub sequence: *mut RangeVar,
    #[doc = " the sequence to alter"]
    pub options: *mut List,
    pub for_identity: bool,
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_AlterSeqStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterSeqStmt>(),
        32usize,
        concat!("Size of: ", stringify!(AlterSeqStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterSeqStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterSeqStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSeqStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSeqStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSeqStmt>())).sequence as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSeqStmt),
            "::",
            stringify!(sequence)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSeqStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSeqStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSeqStmt>())).for_identity as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSeqStmt),
            "::",
            stringify!(for_identity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSeqStmt>())).missing_ok as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSeqStmt),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for AlterSeqStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create {Aggregate|Operator|Type} Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DefineStmt {
    pub type_: NodeTag,
    pub kind: ObjectType,
    #[doc = " aggregate, operator, type"]
    pub oldstyle: bool,
    #[doc = " hack to signal old CREATE AGG syntax"]
    pub defnames: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub args: *mut List,
    #[doc = " a list of TypeName (if needed)"]
    pub definition: *mut List,
    #[doc = " a list of DefElem"]
    pub if_not_exists: bool,
    #[doc = " just do nothing if it already exists?"]
    pub replace: bool,
}
#[test]
fn bindgen_test_layout_DefineStmt() {
    assert_eq!(
        ::std::mem::size_of::<DefineStmt>(),
        48usize,
        concat!("Size of: ", stringify!(DefineStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DefineStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DefineStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefineStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DefineStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefineStmt>())).kind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(DefineStmt),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefineStmt>())).oldstyle as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DefineStmt),
            "::",
            stringify!(oldstyle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefineStmt>())).defnames as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DefineStmt),
            "::",
            stringify!(defnames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefineStmt>())).args as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DefineStmt),
            "::",
            stringify!(args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefineStmt>())).definition as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(DefineStmt),
            "::",
            stringify!(definition)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefineStmt>())).if_not_exists as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(DefineStmt),
            "::",
            stringify!(if_not_exists)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DefineStmt>())).replace as *const _ as usize },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(DefineStmt),
            "::",
            stringify!(replace)
        )
    );
}
impl Default for DefineStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Domain Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateDomainStmt {
    pub type_: NodeTag,
    pub domainname: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub typeName: *mut TypeName,
    #[doc = " the base type"]
    pub collClause: *mut CollateClause,
    #[doc = " untransformed COLLATE spec, if any"]
    pub constraints: *mut List,
}
#[test]
fn bindgen_test_layout_CreateDomainStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateDomainStmt>(),
        40usize,
        concat!("Size of: ", stringify!(CreateDomainStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateDomainStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateDomainStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateDomainStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateDomainStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateDomainStmt>())).domainname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateDomainStmt),
            "::",
            stringify!(domainname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateDomainStmt>())).typeName as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateDomainStmt),
            "::",
            stringify!(typeName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateDomainStmt>())).collClause as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateDomainStmt),
            "::",
            stringify!(collClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateDomainStmt>())).constraints as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateDomainStmt),
            "::",
            stringify!(constraints)
        )
    );
}
impl Default for CreateDomainStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Operator Class Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateOpClassStmt {
    pub type_: NodeTag,
    pub opclassname: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub opfamilyname: *mut List,
    #[doc = " qualified name (ditto); NIL if omitted"]
    pub amname: *mut ::std::os::raw::c_char,
    #[doc = " name of index AM opclass is for"]
    pub datatype: *mut TypeName,
    #[doc = " datatype of indexed column"]
    pub items: *mut List,
    #[doc = " List of CreateOpClassItem nodes"]
    pub isDefault: bool,
}
#[test]
fn bindgen_test_layout_CreateOpClassStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateOpClassStmt>(),
        56usize,
        concat!("Size of: ", stringify!(CreateOpClassStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateOpClassStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateOpClassStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassStmt>())).opclassname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassStmt),
            "::",
            stringify!(opclassname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassStmt>())).opfamilyname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassStmt),
            "::",
            stringify!(opfamilyname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassStmt>())).amname as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassStmt),
            "::",
            stringify!(amname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassStmt>())).datatype as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassStmt),
            "::",
            stringify!(datatype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassStmt>())).items as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassStmt),
            "::",
            stringify!(items)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassStmt>())).isDefault as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassStmt),
            "::",
            stringify!(isDefault)
        )
    );
}
impl Default for CreateOpClassStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateOpClassItem {
    pub type_: NodeTag,
    pub itemtype: ::std::os::raw::c_int,
    #[doc = " see codes above"]
    pub name: *mut ObjectWithArgs,
    #[doc = " operator or function name and args"]
    pub number: ::std::os::raw::c_int,
    #[doc = " strategy num or support proc num"]
    pub order_family: *mut List,
    #[doc = " only used for ordering operators"]
    pub class_args: *mut List,
    #[doc = " amproclefttype/amprocrighttype or"]
    #[doc = " amoplefttype/amoprighttype */"]
    pub storedtype: *mut TypeName,
}
#[test]
fn bindgen_test_layout_CreateOpClassItem() {
    assert_eq!(
        ::std::mem::size_of::<CreateOpClassItem>(),
        48usize,
        concat!("Size of: ", stringify!(CreateOpClassItem))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateOpClassItem>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateOpClassItem))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassItem>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassItem),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassItem>())).itemtype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassItem),
            "::",
            stringify!(itemtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassItem>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassItem),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassItem>())).number as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassItem),
            "::",
            stringify!(number)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassItem>())).order_family as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassItem),
            "::",
            stringify!(order_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassItem>())).class_args as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassItem),
            "::",
            stringify!(class_args)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpClassItem>())).storedtype as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpClassItem),
            "::",
            stringify!(storedtype)
        )
    );
}
impl Default for CreateOpClassItem {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Operator Family Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateOpFamilyStmt {
    pub type_: NodeTag,
    pub opfamilyname: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub amname: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_CreateOpFamilyStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateOpFamilyStmt>(),
        24usize,
        concat!("Size of: ", stringify!(CreateOpFamilyStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateOpFamilyStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateOpFamilyStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpFamilyStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpFamilyStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpFamilyStmt>())).opfamilyname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpFamilyStmt),
            "::",
            stringify!(opfamilyname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateOpFamilyStmt>())).amname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateOpFamilyStmt),
            "::",
            stringify!(amname)
        )
    );
}
impl Default for CreateOpFamilyStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Alter Operator Family Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterOpFamilyStmt {
    pub type_: NodeTag,
    pub opfamilyname: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub amname: *mut ::std::os::raw::c_char,
    #[doc = " name of index AM opfamily is for"]
    pub isDrop: bool,
    #[doc = " ADD or DROP the items?"]
    pub items: *mut List,
}
#[test]
fn bindgen_test_layout_AlterOpFamilyStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterOpFamilyStmt>(),
        40usize,
        concat!("Size of: ", stringify!(AlterOpFamilyStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterOpFamilyStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterOpFamilyStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOpFamilyStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOpFamilyStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOpFamilyStmt>())).opfamilyname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOpFamilyStmt),
            "::",
            stringify!(opfamilyname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOpFamilyStmt>())).amname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOpFamilyStmt),
            "::",
            stringify!(amname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOpFamilyStmt>())).isDrop as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOpFamilyStmt),
            "::",
            stringify!(isDrop)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOpFamilyStmt>())).items as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOpFamilyStmt),
            "::",
            stringify!(items)
        )
    );
}
impl Default for AlterOpFamilyStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Drop Table|Sequence|View|Index|Type|Domain|Conversion|Schema Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DropStmt {
    pub type_: NodeTag,
    pub objects: *mut List,
    #[doc = " list of names"]
    pub removeType: ObjectType,
    #[doc = " object type"]
    pub behavior: DropBehavior,
    #[doc = " RESTRICT or CASCADE behavior"]
    pub missing_ok: bool,
    #[doc = " skip error if object is missing?"]
    pub concurrent: bool,
}
#[test]
fn bindgen_test_layout_DropStmt() {
    assert_eq!(
        ::std::mem::size_of::<DropStmt>(),
        32usize,
        concat!("Size of: ", stringify!(DropStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DropStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DropStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DropStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropStmt>())).objects as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DropStmt),
            "::",
            stringify!(objects)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropStmt>())).removeType as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DropStmt),
            "::",
            stringify!(removeType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropStmt>())).behavior as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(DropStmt),
            "::",
            stringify!(behavior)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropStmt>())).missing_ok as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DropStmt),
            "::",
            stringify!(missing_ok)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropStmt>())).concurrent as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(DropStmt),
            "::",
            stringify!(concurrent)
        )
    );
}
impl Default for DropStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "          Truncate Table Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TruncateStmt {
    pub type_: NodeTag,
    pub relations: *mut List,
    #[doc = " relations (RangeVars) to be truncated"]
    pub restart_seqs: bool,
    #[doc = " restart owned sequences?"]
    pub behavior: DropBehavior,
}
#[test]
fn bindgen_test_layout_TruncateStmt() {
    assert_eq!(
        ::std::mem::size_of::<TruncateStmt>(),
        24usize,
        concat!("Size of: ", stringify!(TruncateStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<TruncateStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(TruncateStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TruncateStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TruncateStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TruncateStmt>())).relations as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TruncateStmt),
            "::",
            stringify!(relations)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TruncateStmt>())).restart_seqs as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TruncateStmt),
            "::",
            stringify!(restart_seqs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TruncateStmt>())).behavior as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(TruncateStmt),
            "::",
            stringify!(behavior)
        )
    );
}
impl Default for TruncateStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "          Comment On Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CommentStmt {
    pub type_: NodeTag,
    pub objtype: ObjectType,
    #[doc = " Object's type"]
    pub object: *mut Node,
    #[doc = " Qualified name of the object"]
    pub comment: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_CommentStmt() {
    assert_eq!(
        ::std::mem::size_of::<CommentStmt>(),
        24usize,
        concat!("Size of: ", stringify!(CommentStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CommentStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CommentStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommentStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CommentStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommentStmt>())).objtype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CommentStmt),
            "::",
            stringify!(objtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommentStmt>())).object as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CommentStmt),
            "::",
            stringify!(object)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CommentStmt>())).comment as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CommentStmt),
            "::",
            stringify!(comment)
        )
    );
}
impl Default for CommentStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "          SECURITY LABEL Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct SecLabelStmt {
    pub type_: NodeTag,
    pub objtype: ObjectType,
    #[doc = " Object's type"]
    pub object: *mut Node,
    #[doc = " Qualified name of the object"]
    pub provider: *mut ::std::os::raw::c_char,
    #[doc = " Label provider (or NULL)"]
    pub label: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_SecLabelStmt() {
    assert_eq!(
        ::std::mem::size_of::<SecLabelStmt>(),
        32usize,
        concat!("Size of: ", stringify!(SecLabelStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<SecLabelStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(SecLabelStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SecLabelStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SecLabelStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SecLabelStmt>())).objtype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SecLabelStmt),
            "::",
            stringify!(objtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SecLabelStmt>())).object as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SecLabelStmt),
            "::",
            stringify!(object)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SecLabelStmt>())).provider as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SecLabelStmt),
            "::",
            stringify!(provider)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SecLabelStmt>())).label as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SecLabelStmt),
            "::",
            stringify!(label)
        )
    );
}
impl Default for SecLabelStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " parallel mode OK"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DeclareCursorStmt {
    pub type_: NodeTag,
    pub portalname: *mut ::std::os::raw::c_char,
    #[doc = " name of the portal (cursor)"]
    pub options: ::std::os::raw::c_int,
    #[doc = " bitmask of options (see above)"]
    pub query: *mut Node,
}
#[test]
fn bindgen_test_layout_DeclareCursorStmt() {
    assert_eq!(
        ::std::mem::size_of::<DeclareCursorStmt>(),
        32usize,
        concat!("Size of: ", stringify!(DeclareCursorStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DeclareCursorStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DeclareCursorStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeclareCursorStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeclareCursorStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeclareCursorStmt>())).portalname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeclareCursorStmt),
            "::",
            stringify!(portalname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeclareCursorStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DeclareCursorStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeclareCursorStmt>())).query as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DeclareCursorStmt),
            "::",
            stringify!(query)
        )
    );
}
impl Default for DeclareCursorStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Close Portal Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ClosePortalStmt {
    pub type_: NodeTag,
    pub portalname: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ClosePortalStmt() {
    assert_eq!(
        ::std::mem::size_of::<ClosePortalStmt>(),
        16usize,
        concat!("Size of: ", stringify!(ClosePortalStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ClosePortalStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ClosePortalStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ClosePortalStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClosePortalStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ClosePortalStmt>())).portalname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClosePortalStmt),
            "::",
            stringify!(portalname)
        )
    );
}
impl Default for ClosePortalStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = "    Fetch Statement (also Move)"]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FetchDirection {
    #[doc = " for these, howMany is how many rows to fetch; FETCH_ALL means ALL"]
    FETCH_FORWARD = 0,
    #[doc = " for these, howMany is how many rows to fetch; FETCH_ALL means ALL"]
    FETCH_BACKWARD = 1,
    #[doc = " for these, howMany indicates a position; only one row is fetched"]
    FETCH_ABSOLUTE = 2,
    #[doc = " for these, howMany indicates a position; only one row is fetched"]
    FETCH_RELATIVE = 3,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FetchStmt {
    pub type_: NodeTag,
    pub direction: FetchDirection,
    #[doc = " see above"]
    pub howMany: ::std::os::raw::c_long,
    #[doc = " number of rows, or position argument"]
    pub portalname: *mut ::std::os::raw::c_char,
    #[doc = " name of portal (cursor)"]
    pub ismove: bool,
}
#[test]
fn bindgen_test_layout_FetchStmt() {
    assert_eq!(
        ::std::mem::size_of::<FetchStmt>(),
        32usize,
        concat!("Size of: ", stringify!(FetchStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<FetchStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(FetchStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FetchStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FetchStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FetchStmt>())).direction as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(FetchStmt),
            "::",
            stringify!(direction)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FetchStmt>())).howMany as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FetchStmt),
            "::",
            stringify!(howMany)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FetchStmt>())).portalname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FetchStmt),
            "::",
            stringify!(portalname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FetchStmt>())).ismove as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(FetchStmt),
            "::",
            stringify!(ismove)
        )
    );
}
impl Default for FetchStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Index Statement"]
#[doc = ""]
#[doc = " This represents creation of an index and/or an associated constraint."]
#[doc = " If isconstraint is true, we should create a pg_constraint entry along"]
#[doc = " with the index.  But if indexOid isn't InvalidOid, we are not creating an"]
#[doc = " index, just a UNIQUE/PKEY constraint using an existing index.  isconstraint"]
#[doc = " must always be true in this case, and the fields describing the index"]
#[doc = " properties are empty."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct IndexStmt {
    pub type_: NodeTag,
    pub idxname: *mut ::std::os::raw::c_char,
    #[doc = " name of new index, or NULL for default"]
    pub relation: *mut RangeVar,
    #[doc = " relation to build index on"]
    pub accessMethod: *mut ::std::os::raw::c_char,
    #[doc = " name of access method (eg. btree)"]
    pub tableSpace: *mut ::std::os::raw::c_char,
    #[doc = " tablespace, or NULL for default"]
    pub indexParams: *mut List,
    #[doc = " columns to index: a list of IndexElem"]
    pub indexIncludingParams: *mut List,
    #[doc = " additional columns to index: a list"]
    #[doc = " of IndexElem"]
    pub options: *mut List,
    #[doc = " WITH clause options: a list of DefElem"]
    pub whereClause: *mut Node,
    #[doc = " qualification (partialindex predicate)"]
    pub excludeOpNames: *mut List,
    #[doc = " exclusion operator names, or NIL if none"]
    pub idxcomment: *mut ::std::os::raw::c_char,
    #[doc = " comment to apply to index, or NULL"]
    pub indexOid: Oid,
    #[doc = " OID of an existing index, if any"]
    pub oldNode: Oid,
    #[doc = " relfilenode of existing storage, if any"]
    pub unique: bool,
    #[doc = " is index unique?"]
    pub primary: bool,
    #[doc = " is index a primary key?"]
    pub isconstraint: bool,
    #[doc = " is it for a pkey/unique constraint?"]
    pub deferrable: bool,
    #[doc = " is the constraint DEFERRABLE?"]
    pub initdeferred: bool,
    #[doc = " is the constraint INITIALLY DEFERRED?"]
    pub transformed: bool,
    #[doc = " true when transformIndexStmt is finished"]
    pub concurrent: bool,
    #[doc = " should this be a concurrent index build?"]
    pub if_not_exists: bool,
    #[doc = " just do nothing if index already exists?"]
    pub reset_default_tblspc: bool,
}
#[test]
fn bindgen_test_layout_IndexStmt() {
    assert_eq!(
        ::std::mem::size_of::<IndexStmt>(),
        112usize,
        concat!("Size of: ", stringify!(IndexStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<IndexStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(IndexStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).idxname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(idxname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).relation as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).accessMethod as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(accessMethod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).tableSpace as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(tableSpace)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).indexParams as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(indexParams)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).indexIncludingParams as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(indexIncludingParams)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).options as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).whereClause as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(whereClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).excludeOpNames as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(excludeOpNames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).idxcomment as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(idxcomment)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).indexOid as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(indexOid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).oldNode as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(oldNode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).unique as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(unique)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).primary as *const _ as usize },
        97usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(primary)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).isconstraint as *const _ as usize },
        98usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(isconstraint)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).deferrable as *const _ as usize },
        99usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(deferrable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).initdeferred as *const _ as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(initdeferred)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).transformed as *const _ as usize },
        101usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(transformed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).concurrent as *const _ as usize },
        102usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(concurrent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).if_not_exists as *const _ as usize },
        103usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(if_not_exists)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<IndexStmt>())).reset_default_tblspc as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(IndexStmt),
            "::",
            stringify!(reset_default_tblspc)
        )
    );
}
impl Default for IndexStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Statistics Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateStatsStmt {
    pub type_: NodeTag,
    pub defnames: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub stat_types: *mut List,
    #[doc = " stat types (list of Value strings)"]
    pub exprs: *mut List,
    #[doc = " expressions to build statistics on"]
    pub relations: *mut List,
    #[doc = " rels to build stats on (list of RangeVar)"]
    pub stxcomment: *mut ::std::os::raw::c_char,
    #[doc = " comment to apply to stats, or NULL"]
    pub if_not_exists: bool,
}
#[test]
fn bindgen_test_layout_CreateStatsStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateStatsStmt>(),
        56usize,
        concat!("Size of: ", stringify!(CreateStatsStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateStatsStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateStatsStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStatsStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStatsStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStatsStmt>())).defnames as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStatsStmt),
            "::",
            stringify!(defnames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStatsStmt>())).stat_types as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStatsStmt),
            "::",
            stringify!(stat_types)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStatsStmt>())).exprs as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStatsStmt),
            "::",
            stringify!(exprs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStatsStmt>())).relations as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStatsStmt),
            "::",
            stringify!(relations)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStatsStmt>())).stxcomment as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStatsStmt),
            "::",
            stringify!(stxcomment)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateStatsStmt>())).if_not_exists as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateStatsStmt),
            "::",
            stringify!(if_not_exists)
        )
    );
}
impl Default for CreateStatsStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Function Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateFunctionStmt {
    pub type_: NodeTag,
    pub is_procedure: bool,
    #[doc = " it's really CREATE PROCEDURE"]
    pub replace: bool,
    #[doc = " T => replace if already exists"]
    pub funcname: *mut List,
    #[doc = " qualified name of function to create"]
    pub parameters: *mut List,
    #[doc = " a list of FunctionParameter"]
    pub returnType: *mut TypeName,
    #[doc = " the return type"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_CreateFunctionStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateFunctionStmt>(),
        40usize,
        concat!("Size of: ", stringify!(CreateFunctionStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateFunctionStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateFunctionStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFunctionStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFunctionStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFunctionStmt>())).is_procedure as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFunctionStmt),
            "::",
            stringify!(is_procedure)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFunctionStmt>())).replace as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFunctionStmt),
            "::",
            stringify!(replace)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFunctionStmt>())).funcname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFunctionStmt),
            "::",
            stringify!(funcname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFunctionStmt>())).parameters as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFunctionStmt),
            "::",
            stringify!(parameters)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFunctionStmt>())).returnType as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFunctionStmt),
            "::",
            stringify!(returnType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateFunctionStmt>())).options as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateFunctionStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for CreateFunctionStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FunctionParameterMode {
    #[doc = " the assigned enum values appear in pg_proc, don't change 'em!"]
    FUNC_PARAM_IN = 105,
    #[doc = " input only"]
    FUNC_PARAM_OUT = 111,
    #[doc = " output only"]
    FUNC_PARAM_INOUT = 98,
    #[doc = " both"]
    FUNC_PARAM_VARIADIC = 118,
    #[doc = " variadic (always input)"]
    FUNC_PARAM_TABLE = 116,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FunctionParameter {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " parameter name, or NULL if not given"]
    pub argType: *mut TypeName,
    #[doc = " TypeName for parameter type"]
    pub mode: FunctionParameterMode,
    #[doc = " IN/OUT/etc"]
    pub defexpr: *mut Node,
}
#[test]
fn bindgen_test_layout_FunctionParameter() {
    assert_eq!(
        ::std::mem::size_of::<FunctionParameter>(),
        40usize,
        concat!("Size of: ", stringify!(FunctionParameter))
    );
    assert_eq!(
        ::std::mem::align_of::<FunctionParameter>(),
        8usize,
        concat!("Alignment of ", stringify!(FunctionParameter))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionParameter>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionParameter),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionParameter>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionParameter),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionParameter>())).argType as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionParameter),
            "::",
            stringify!(argType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionParameter>())).mode as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionParameter),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FunctionParameter>())).defexpr as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(FunctionParameter),
            "::",
            stringify!(defexpr)
        )
    );
}
impl Default for FunctionParameter {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterFunctionStmt {
    pub type_: NodeTag,
    pub objtype: ObjectType,
    pub func: *mut ObjectWithArgs,
    #[doc = " name and args of function"]
    pub actions: *mut List,
}
#[test]
fn bindgen_test_layout_AlterFunctionStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterFunctionStmt>(),
        24usize,
        concat!("Size of: ", stringify!(AlterFunctionStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterFunctionStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterFunctionStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterFunctionStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterFunctionStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterFunctionStmt>())).objtype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterFunctionStmt),
            "::",
            stringify!(objtype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterFunctionStmt>())).func as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterFunctionStmt),
            "::",
            stringify!(func)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterFunctionStmt>())).actions as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterFunctionStmt),
            "::",
            stringify!(actions)
        )
    );
}
impl Default for AlterFunctionStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    DO Statement"]
#[doc = ""]
#[doc = " DoStmt is the raw parser output, InlineCodeBlock is the executiontime API"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DoStmt {
    pub type_: NodeTag,
    pub args: *mut List,
}
#[test]
fn bindgen_test_layout_DoStmt() {
    assert_eq!(
        ::std::mem::size_of::<DoStmt>(),
        16usize,
        concat!("Size of: ", stringify!(DoStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DoStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DoStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DoStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DoStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DoStmt>())).args as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DoStmt),
            "::",
            stringify!(args)
        )
    );
}
impl Default for DoStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct InlineCodeBlock {
    pub type_: NodeTag,
    pub source_text: *mut ::std::os::raw::c_char,
    #[doc = " source text of anonymous code block"]
    pub langOid: Oid,
    #[doc = " OID of selected language"]
    pub langIsTrusted: bool,
    #[doc = " trusted property of the language"]
    pub atomic: bool,
}
#[test]
fn bindgen_test_layout_InlineCodeBlock() {
    assert_eq!(
        ::std::mem::size_of::<InlineCodeBlock>(),
        24usize,
        concat!("Size of: ", stringify!(InlineCodeBlock))
    );
    assert_eq!(
        ::std::mem::align_of::<InlineCodeBlock>(),
        8usize,
        concat!("Alignment of ", stringify!(InlineCodeBlock))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InlineCodeBlock>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(InlineCodeBlock),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InlineCodeBlock>())).source_text as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(InlineCodeBlock),
            "::",
            stringify!(source_text)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InlineCodeBlock>())).langOid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(InlineCodeBlock),
            "::",
            stringify!(langOid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InlineCodeBlock>())).langIsTrusted as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(InlineCodeBlock),
            "::",
            stringify!(langIsTrusted)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<InlineCodeBlock>())).atomic as *const _ as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(InlineCodeBlock),
            "::",
            stringify!(atomic)
        )
    );
}
impl Default for InlineCodeBlock {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    CALL statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CallStmt {
    pub type_: NodeTag,
    pub funccall: *mut FuncCall,
    #[doc = " from the parser"]
    pub funcexpr: *mut FuncExpr,
}
#[test]
fn bindgen_test_layout_CallStmt() {
    assert_eq!(
        ::std::mem::size_of::<CallStmt>(),
        24usize,
        concat!("Size of: ", stringify!(CallStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CallStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CallStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CallStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CallStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CallStmt>())).funccall as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CallStmt),
            "::",
            stringify!(funccall)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CallStmt>())).funcexpr as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CallStmt),
            "::",
            stringify!(funcexpr)
        )
    );
}
impl Default for CallStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CallContext {
    pub type_: NodeTag,
    pub atomic: bool,
}
#[test]
fn bindgen_test_layout_CallContext() {
    assert_eq!(
        ::std::mem::size_of::<CallContext>(),
        8usize,
        concat!("Size of: ", stringify!(CallContext))
    );
    assert_eq!(
        ::std::mem::align_of::<CallContext>(),
        4usize,
        concat!("Alignment of ", stringify!(CallContext))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CallContext>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CallContext),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CallContext>())).atomic as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CallContext),
            "::",
            stringify!(atomic)
        )
    );
}
impl Default for CallContext {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Alter Object Rename Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RenameStmt {
    pub type_: NodeTag,
    pub renameType: ObjectType,
    #[doc = " OBJECT_TABLE, OBJECT_COLUMN, etc"]
    pub relationType: ObjectType,
    #[doc = " if column name, associated relation type"]
    pub relation: *mut RangeVar,
    #[doc = " in case it's a table"]
    pub object: *mut Node,
    #[doc = " in case it's some other object"]
    pub subname: *mut ::std::os::raw::c_char,
    #[doc = " name of contained object (column, rule,"]
    #[doc = " trigger, etc)"]
    pub newname: *mut ::std::os::raw::c_char,
    #[doc = " the new name"]
    pub behavior: DropBehavior,
    #[doc = " RESTRICT or CASCADE behavior"]
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_RenameStmt() {
    assert_eq!(
        ::std::mem::size_of::<RenameStmt>(),
        56usize,
        concat!("Size of: ", stringify!(RenameStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<RenameStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(RenameStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RenameStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RenameStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RenameStmt>())).renameType as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RenameStmt),
            "::",
            stringify!(renameType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RenameStmt>())).relationType as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RenameStmt),
            "::",
            stringify!(relationType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RenameStmt>())).relation as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RenameStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RenameStmt>())).object as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RenameStmt),
            "::",
            stringify!(object)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RenameStmt>())).subname as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RenameStmt),
            "::",
            stringify!(subname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RenameStmt>())).newname as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RenameStmt),
            "::",
            stringify!(newname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RenameStmt>())).behavior as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(RenameStmt),
            "::",
            stringify!(behavior)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RenameStmt>())).missing_ok as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(RenameStmt),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for RenameStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " ALTER object DEPENDS ON EXTENSION extname"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterObjectDependsStmt {
    pub type_: NodeTag,
    pub objectType: ObjectType,
    #[doc = " OBJECT_FUNCTION, OBJECT_TRIGGER, etc"]
    pub relation: *mut RangeVar,
    #[doc = " in case a table is involved"]
    pub object: *mut Node,
    #[doc = " name of the object"]
    pub extname: *mut Value,
}
#[test]
fn bindgen_test_layout_AlterObjectDependsStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterObjectDependsStmt>(),
        32usize,
        concat!("Size of: ", stringify!(AlterObjectDependsStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterObjectDependsStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterObjectDependsStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterObjectDependsStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectDependsStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterObjectDependsStmt>())).objectType as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectDependsStmt),
            "::",
            stringify!(objectType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterObjectDependsStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectDependsStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterObjectDependsStmt>())).object as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectDependsStmt),
            "::",
            stringify!(object)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterObjectDependsStmt>())).extname as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectDependsStmt),
            "::",
            stringify!(extname)
        )
    );
}
impl Default for AlterObjectDependsStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    ALTER object SET SCHEMA Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterObjectSchemaStmt {
    pub type_: NodeTag,
    pub objectType: ObjectType,
    #[doc = " OBJECT_TABLE, OBJECT_TYPE, etc"]
    pub relation: *mut RangeVar,
    #[doc = " in case it's a table"]
    pub object: *mut Node,
    #[doc = " in case it's some other object"]
    pub newschema: *mut ::std::os::raw::c_char,
    #[doc = " the new schema"]
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_AlterObjectSchemaStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterObjectSchemaStmt>(),
        40usize,
        concat!("Size of: ", stringify!(AlterObjectSchemaStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterObjectSchemaStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterObjectSchemaStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterObjectSchemaStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectSchemaStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterObjectSchemaStmt>())).objectType as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectSchemaStmt),
            "::",
            stringify!(objectType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterObjectSchemaStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectSchemaStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterObjectSchemaStmt>())).object as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectSchemaStmt),
            "::",
            stringify!(object)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterObjectSchemaStmt>())).newschema as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectSchemaStmt),
            "::",
            stringify!(newschema)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterObjectSchemaStmt>())).missing_ok as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterObjectSchemaStmt),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for AlterObjectSchemaStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Alter Object Owner Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterOwnerStmt {
    pub type_: NodeTag,
    pub objectType: ObjectType,
    #[doc = " OBJECT_TABLE, OBJECT_TYPE, etc"]
    pub relation: *mut RangeVar,
    #[doc = " in case it's a table"]
    pub object: *mut Node,
    #[doc = " in case it's some other object"]
    pub newowner: *mut RoleSpec,
}
#[test]
fn bindgen_test_layout_AlterOwnerStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterOwnerStmt>(),
        32usize,
        concat!("Size of: ", stringify!(AlterOwnerStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterOwnerStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterOwnerStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOwnerStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOwnerStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOwnerStmt>())).objectType as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOwnerStmt),
            "::",
            stringify!(objectType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOwnerStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOwnerStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOwnerStmt>())).object as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOwnerStmt),
            "::",
            stringify!(object)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOwnerStmt>())).newowner as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOwnerStmt),
            "::",
            stringify!(newowner)
        )
    );
}
impl Default for AlterOwnerStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Alter Operator Set Restrict, Join"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterOperatorStmt {
    pub type_: NodeTag,
    pub opername: *mut ObjectWithArgs,
    #[doc = " operator name and argument types"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_AlterOperatorStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterOperatorStmt>(),
        24usize,
        concat!("Size of: ", stringify!(AlterOperatorStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterOperatorStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterOperatorStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOperatorStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOperatorStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOperatorStmt>())).opername as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOperatorStmt),
            "::",
            stringify!(opername)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterOperatorStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterOperatorStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for AlterOperatorStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Rule Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RuleStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    #[doc = " relation the rule is for"]
    pub rulename: *mut ::std::os::raw::c_char,
    #[doc = " name of the rule"]
    pub whereClause: *mut Node,
    #[doc = " qualifications"]
    pub event: CmdType,
    #[doc = " SELECT, INSERT, etc"]
    pub instead: bool,
    #[doc = " is a 'do instead'?"]
    pub actions: *mut List,
    #[doc = " the action statements"]
    pub replace: bool,
}
#[test]
fn bindgen_test_layout_RuleStmt() {
    assert_eq!(
        ::std::mem::size_of::<RuleStmt>(),
        56usize,
        concat!("Size of: ", stringify!(RuleStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<RuleStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(RuleStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RuleStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RuleStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RuleStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RuleStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RuleStmt>())).rulename as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(RuleStmt),
            "::",
            stringify!(rulename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RuleStmt>())).whereClause as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(RuleStmt),
            "::",
            stringify!(whereClause)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RuleStmt>())).event as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(RuleStmt),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RuleStmt>())).instead as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(RuleStmt),
            "::",
            stringify!(instead)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RuleStmt>())).actions as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(RuleStmt),
            "::",
            stringify!(actions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RuleStmt>())).replace as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(RuleStmt),
            "::",
            stringify!(replace)
        )
    );
}
impl Default for RuleStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Notify Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NotifyStmt {
    pub type_: NodeTag,
    pub conditionname: *mut ::std::os::raw::c_char,
    #[doc = " condition name to notify"]
    pub payload: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_NotifyStmt() {
    assert_eq!(
        ::std::mem::size_of::<NotifyStmt>(),
        24usize,
        concat!("Size of: ", stringify!(NotifyStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<NotifyStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(NotifyStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NotifyStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NotifyStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NotifyStmt>())).conditionname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(NotifyStmt),
            "::",
            stringify!(conditionname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<NotifyStmt>())).payload as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(NotifyStmt),
            "::",
            stringify!(payload)
        )
    );
}
impl Default for NotifyStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Listen Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ListenStmt {
    pub type_: NodeTag,
    pub conditionname: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ListenStmt() {
    assert_eq!(
        ::std::mem::size_of::<ListenStmt>(),
        16usize,
        concat!("Size of: ", stringify!(ListenStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ListenStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ListenStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ListenStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ListenStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ListenStmt>())).conditionname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ListenStmt),
            "::",
            stringify!(conditionname)
        )
    );
}
impl Default for ListenStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Unlisten Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct UnlistenStmt {
    pub type_: NodeTag,
    pub conditionname: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_UnlistenStmt() {
    assert_eq!(
        ::std::mem::size_of::<UnlistenStmt>(),
        16usize,
        concat!("Size of: ", stringify!(UnlistenStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<UnlistenStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(UnlistenStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UnlistenStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(UnlistenStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<UnlistenStmt>())).conditionname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(UnlistenStmt),
            "::",
            stringify!(conditionname)
        )
    );
}
impl Default for UnlistenStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = "    {Begin|Commit|Rollback} Transaction Statement"]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TransactionStmtKind {
    TRANS_STMT_BEGIN = 0,
    TRANS_STMT_START = 1,
    #[doc = " semantically identical to BEGIN"]
    TRANS_STMT_COMMIT = 2,
    #[doc = " semantically identical to BEGIN"]
    TRANS_STMT_ROLLBACK = 3,
    #[doc = " semantically identical to BEGIN"]
    TRANS_STMT_SAVEPOINT = 4,
    #[doc = " semantically identical to BEGIN"]
    TRANS_STMT_RELEASE = 5,
    #[doc = " semantically identical to BEGIN"]
    TRANS_STMT_ROLLBACK_TO = 6,
    #[doc = " semantically identical to BEGIN"]
    TRANS_STMT_PREPARE = 7,
    #[doc = " semantically identical to BEGIN"]
    TRANS_STMT_COMMIT_PREPARED = 8,
    #[doc = " semantically identical to BEGIN"]
    TRANS_STMT_ROLLBACK_PREPARED = 9,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TransactionStmt {
    pub type_: NodeTag,
    pub kind: TransactionStmtKind,
    #[doc = " see above"]
    pub options: *mut List,
    #[doc = " for BEGIN/START commands"]
    pub savepoint_name: *mut ::std::os::raw::c_char,
    #[doc = " for savepoint commands"]
    pub gid: *mut ::std::os::raw::c_char,
    #[doc = " for twophasecommit related commands"]
    pub chain: bool,
}
#[test]
fn bindgen_test_layout_TransactionStmt() {
    assert_eq!(
        ::std::mem::size_of::<TransactionStmt>(),
        40usize,
        concat!("Size of: ", stringify!(TransactionStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<TransactionStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(TransactionStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TransactionStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TransactionStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TransactionStmt>())).kind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TransactionStmt),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TransactionStmt>())).options as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TransactionStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TransactionStmt>())).savepoint_name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TransactionStmt),
            "::",
            stringify!(savepoint_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TransactionStmt>())).gid as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(TransactionStmt),
            "::",
            stringify!(gid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TransactionStmt>())).chain as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(TransactionStmt),
            "::",
            stringify!(chain)
        )
    );
}
impl Default for TransactionStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Type Statement, composite types"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CompositeTypeStmt {
    pub type_: NodeTag,
    pub typevar: *mut RangeVar,
    #[doc = " the composite type to be created"]
    pub coldeflist: *mut List,
}
#[test]
fn bindgen_test_layout_CompositeTypeStmt() {
    assert_eq!(
        ::std::mem::size_of::<CompositeTypeStmt>(),
        24usize,
        concat!("Size of: ", stringify!(CompositeTypeStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CompositeTypeStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CompositeTypeStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CompositeTypeStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CompositeTypeStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CompositeTypeStmt>())).typevar as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CompositeTypeStmt),
            "::",
            stringify!(typevar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CompositeTypeStmt>())).coldeflist as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CompositeTypeStmt),
            "::",
            stringify!(coldeflist)
        )
    );
}
impl Default for CompositeTypeStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Type Statement, enum types"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateEnumStmt {
    pub type_: NodeTag,
    pub typeName: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub vals: *mut List,
}
#[test]
fn bindgen_test_layout_CreateEnumStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateEnumStmt>(),
        24usize,
        concat!("Size of: ", stringify!(CreateEnumStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateEnumStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateEnumStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateEnumStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateEnumStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateEnumStmt>())).typeName as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateEnumStmt),
            "::",
            stringify!(typeName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateEnumStmt>())).vals as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateEnumStmt),
            "::",
            stringify!(vals)
        )
    );
}
impl Default for CreateEnumStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Create Type Statement, range types"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateRangeStmt {
    pub type_: NodeTag,
    pub typeName: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub params: *mut List,
}
#[test]
fn bindgen_test_layout_CreateRangeStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateRangeStmt>(),
        24usize,
        concat!("Size of: ", stringify!(CreateRangeStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateRangeStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateRangeStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateRangeStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateRangeStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateRangeStmt>())).typeName as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateRangeStmt),
            "::",
            stringify!(typeName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateRangeStmt>())).params as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateRangeStmt),
            "::",
            stringify!(params)
        )
    );
}
impl Default for CreateRangeStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Alter Type Statement, enum types"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterEnumStmt {
    pub type_: NodeTag,
    pub typeName: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub oldVal: *mut ::std::os::raw::c_char,
    #[doc = " old enum value's name, if renaming"]
    pub newVal: *mut ::std::os::raw::c_char,
    #[doc = " new enum value's name"]
    pub newValNeighbor: *mut ::std::os::raw::c_char,
    #[doc = " neighboring enum value, if specified"]
    pub newValIsAfter: bool,
    #[doc = " place new enum value after neighbor?"]
    pub skipIfNewValExists: bool,
}
#[test]
fn bindgen_test_layout_AlterEnumStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterEnumStmt>(),
        48usize,
        concat!("Size of: ", stringify!(AlterEnumStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterEnumStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterEnumStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterEnumStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterEnumStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterEnumStmt>())).typeName as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterEnumStmt),
            "::",
            stringify!(typeName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterEnumStmt>())).oldVal as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterEnumStmt),
            "::",
            stringify!(oldVal)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterEnumStmt>())).newVal as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterEnumStmt),
            "::",
            stringify!(newVal)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterEnumStmt>())).newValNeighbor as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterEnumStmt),
            "::",
            stringify!(newValNeighbor)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterEnumStmt>())).newValIsAfter as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterEnumStmt),
            "::",
            stringify!(newValIsAfter)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterEnumStmt>())).skipIfNewValExists as *const _ as usize
        },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterEnumStmt),
            "::",
            stringify!(skipIfNewValExists)
        )
    );
}
impl Default for AlterEnumStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = "    Create View Statement"]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ViewCheckOption {
    NO_CHECK_OPTION = 0,
    LOCAL_CHECK_OPTION = 1,
    CASCADED_CHECK_OPTION = 2,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ViewStmt {
    pub type_: NodeTag,
    pub view: *mut RangeVar,
    #[doc = " the view to be created"]
    pub aliases: *mut List,
    #[doc = " target column names"]
    pub query: *mut Node,
    #[doc = " the SELECT query (as a raw parse tree)"]
    pub replace: bool,
    #[doc = " replace an existing view?"]
    pub options: *mut List,
    #[doc = " options from WITH clause"]
    pub withCheckOption: ViewCheckOption,
}
#[test]
fn bindgen_test_layout_ViewStmt() {
    assert_eq!(
        ::std::mem::size_of::<ViewStmt>(),
        56usize,
        concat!("Size of: ", stringify!(ViewStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ViewStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ViewStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ViewStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ViewStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ViewStmt>())).view as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ViewStmt),
            "::",
            stringify!(view)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ViewStmt>())).aliases as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ViewStmt),
            "::",
            stringify!(aliases)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ViewStmt>())).query as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ViewStmt),
            "::",
            stringify!(query)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ViewStmt>())).replace as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ViewStmt),
            "::",
            stringify!(replace)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ViewStmt>())).options as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ViewStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ViewStmt>())).withCheckOption as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ViewStmt),
            "::",
            stringify!(withCheckOption)
        )
    );
}
impl Default for ViewStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Load Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct LoadStmt {
    pub type_: NodeTag,
    pub filename: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_LoadStmt() {
    assert_eq!(
        ::std::mem::size_of::<LoadStmt>(),
        16usize,
        concat!("Size of: ", stringify!(LoadStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<LoadStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(LoadStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<LoadStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(LoadStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<LoadStmt>())).filename as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(LoadStmt),
            "::",
            stringify!(filename)
        )
    );
}
impl Default for LoadStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Createdb Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreatedbStmt {
    pub type_: NodeTag,
    pub dbname: *mut ::std::os::raw::c_char,
    #[doc = " name of database to create"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_CreatedbStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreatedbStmt>(),
        24usize,
        concat!("Size of: ", stringify!(CreatedbStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreatedbStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreatedbStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatedbStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatedbStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatedbStmt>())).dbname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatedbStmt),
            "::",
            stringify!(dbname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatedbStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatedbStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for CreatedbStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Alter Database"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterDatabaseStmt {
    pub type_: NodeTag,
    pub dbname: *mut ::std::os::raw::c_char,
    #[doc = " name of database to alter"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_AlterDatabaseStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterDatabaseStmt>(),
        24usize,
        concat!("Size of: ", stringify!(AlterDatabaseStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterDatabaseStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterDatabaseStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDatabaseStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDatabaseStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDatabaseStmt>())).dbname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDatabaseStmt),
            "::",
            stringify!(dbname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDatabaseStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDatabaseStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for AlterDatabaseStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterDatabaseSetStmt {
    pub type_: NodeTag,
    pub dbname: *mut ::std::os::raw::c_char,
    #[doc = " database name"]
    pub setstmt: *mut VariableSetStmt,
}
#[test]
fn bindgen_test_layout_AlterDatabaseSetStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterDatabaseSetStmt>(),
        24usize,
        concat!("Size of: ", stringify!(AlterDatabaseSetStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterDatabaseSetStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterDatabaseSetStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDatabaseSetStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDatabaseSetStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDatabaseSetStmt>())).dbname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDatabaseSetStmt),
            "::",
            stringify!(dbname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterDatabaseSetStmt>())).setstmt as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterDatabaseSetStmt),
            "::",
            stringify!(setstmt)
        )
    );
}
impl Default for AlterDatabaseSetStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Dropdb Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DropdbStmt {
    pub type_: NodeTag,
    pub dbname: *mut ::std::os::raw::c_char,
    #[doc = " database to drop"]
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_DropdbStmt() {
    assert_eq!(
        ::std::mem::size_of::<DropdbStmt>(),
        24usize,
        concat!("Size of: ", stringify!(DropdbStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DropdbStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DropdbStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropdbStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DropdbStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropdbStmt>())).dbname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DropdbStmt),
            "::",
            stringify!(dbname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropdbStmt>())).missing_ok as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DropdbStmt),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for DropdbStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Alter System Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterSystemStmt {
    pub type_: NodeTag,
    pub setstmt: *mut VariableSetStmt,
}
#[test]
fn bindgen_test_layout_AlterSystemStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterSystemStmt>(),
        16usize,
        concat!("Size of: ", stringify!(AlterSystemStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterSystemStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterSystemStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSystemStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSystemStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSystemStmt>())).setstmt as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSystemStmt),
            "::",
            stringify!(setstmt)
        )
    );
}
impl Default for AlterSystemStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = "    Cluster Statement (support pbrown's cluster index implementation)"]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ClusterOption {
    CLUOPT_RECHECK = 1,
    #[doc = " recheck relation state"]
    CLUOPT_VERBOSE = 2,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ClusterStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    #[doc = " relation being indexed, or NULL if all"]
    pub indexname: *mut ::std::os::raw::c_char,
    #[doc = " original index defined"]
    pub options: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ClusterStmt() {
    assert_eq!(
        ::std::mem::size_of::<ClusterStmt>(),
        32usize,
        concat!("Size of: ", stringify!(ClusterStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ClusterStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ClusterStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ClusterStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClusterStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ClusterStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClusterStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ClusterStmt>())).indexname as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ClusterStmt),
            "::",
            stringify!(indexname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ClusterStmt>())).options as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ClusterStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for ClusterStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Vacuum and Analyze Statements"]
#[doc = ""]
#[doc = " Even though these are nominally two statements, it's convenient to use"]
#[doc = " just one node type for both."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VacuumStmt {
    pub type_: NodeTag,
    pub options: *mut List,
    #[doc = " list of DefElem nodes"]
    pub rels: *mut List,
    #[doc = " list of VacuumRelation, or NIL for all"]
    pub is_vacuumcmd: bool,
}
#[test]
fn bindgen_test_layout_VacuumStmt() {
    assert_eq!(
        ::std::mem::size_of::<VacuumStmt>(),
        32usize,
        concat!("Size of: ", stringify!(VacuumStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<VacuumStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(VacuumStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VacuumStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VacuumStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VacuumStmt>())).options as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VacuumStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VacuumStmt>())).rels as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VacuumStmt),
            "::",
            stringify!(rels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VacuumStmt>())).is_vacuumcmd as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VacuumStmt),
            "::",
            stringify!(is_vacuumcmd)
        )
    );
}
impl Default for VacuumStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Info about a single target table of VACUUM/ANALYZE."]
#[doc = ""]
#[doc = " If the OID field is set, it always identifies the table to process."]
#[doc = " Then the relation field can be NULL; if it isn't, it's used only to report"]
#[doc = " failure to open/lock the relation."]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct VacuumRelation {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    #[doc = " table name to process, or NULL"]
    pub oid: Oid,
    #[doc = " table's OID; InvalidOid if not looked up"]
    pub va_cols: *mut List,
}
#[test]
fn bindgen_test_layout_VacuumRelation() {
    assert_eq!(
        ::std::mem::size_of::<VacuumRelation>(),
        32usize,
        concat!("Size of: ", stringify!(VacuumRelation))
    );
    assert_eq!(
        ::std::mem::align_of::<VacuumRelation>(),
        8usize,
        concat!("Alignment of ", stringify!(VacuumRelation))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VacuumRelation>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VacuumRelation),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VacuumRelation>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VacuumRelation),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VacuumRelation>())).oid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VacuumRelation),
            "::",
            stringify!(oid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VacuumRelation>())).va_cols as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VacuumRelation),
            "::",
            stringify!(va_cols)
        )
    );
}
impl Default for VacuumRelation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    Explain Statement"]
#[doc = ""]
#[doc = " The \"query\" field is initially a raw parse tree, and is converted to a"]
#[doc = " Query node during parse analysis.  Note that rewriting and planning"]
#[doc = " of the query are always postponed until execution."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ExplainStmt {
    pub type_: NodeTag,
    pub query: *mut Node,
    #[doc = " the query (see comments above)"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_ExplainStmt() {
    assert_eq!(
        ::std::mem::size_of::<ExplainStmt>(),
        24usize,
        concat!("Size of: ", stringify!(ExplainStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ExplainStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ExplainStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ExplainStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ExplainStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ExplainStmt>())).query as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ExplainStmt),
            "::",
            stringify!(query)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ExplainStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ExplainStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for ExplainStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    CREATE TABLE AS Statement (a/k/a SELECT INTO)"]
#[doc = ""]
#[doc = " A query written as CREATE TABLE AS will produce this node type natively."]
#[doc = " A query written as SELECT ... INTO will be transformed to this form during"]
#[doc = " parse analysis."]
#[doc = " A query written as CREATE MATERIALIZED view will produce this node type,"]
#[doc = " during parse analysis, since it needs all the same data."]
#[doc = ""]
#[doc = " The \"query\" field is handled similarly to EXPLAIN, though note that it"]
#[doc = " can be a SELECT or an EXECUTE, but not other DML statements."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateTableAsStmt {
    pub type_: NodeTag,
    pub query: *mut Node,
    #[doc = " the query (see comments above)"]
    pub into: *mut IntoClause,
    #[doc = " destination table"]
    pub relkind: ObjectType,
    #[doc = " OBJECT_TABLE or OBJECT_MATVIEW"]
    pub is_select_into: bool,
    #[doc = " it was written as SELECT INTO"]
    pub if_not_exists: bool,
}
#[test]
fn bindgen_test_layout_CreateTableAsStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateTableAsStmt>(),
        32usize,
        concat!("Size of: ", stringify!(CreateTableAsStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateTableAsStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateTableAsStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTableAsStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableAsStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTableAsStmt>())).query as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableAsStmt),
            "::",
            stringify!(query)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTableAsStmt>())).into as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableAsStmt),
            "::",
            stringify!(into)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTableAsStmt>())).relkind as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableAsStmt),
            "::",
            stringify!(relkind)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateTableAsStmt>())).is_select_into as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableAsStmt),
            "::",
            stringify!(is_select_into)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTableAsStmt>())).if_not_exists as *const _ as usize },
        29usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTableAsStmt),
            "::",
            stringify!(if_not_exists)
        )
    );
}
impl Default for CreateTableAsStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    REFRESH MATERIALIZED VIEW Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct RefreshMatViewStmt {
    pub type_: NodeTag,
    pub concurrent: bool,
    #[doc = " allow concurrent access?"]
    pub skipData: bool,
    #[doc = " true for WITH NO DATA"]
    pub relation: *mut RangeVar,
}
#[test]
fn bindgen_test_layout_RefreshMatViewStmt() {
    assert_eq!(
        ::std::mem::size_of::<RefreshMatViewStmt>(),
        16usize,
        concat!("Size of: ", stringify!(RefreshMatViewStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<RefreshMatViewStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(RefreshMatViewStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RefreshMatViewStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(RefreshMatViewStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RefreshMatViewStmt>())).concurrent as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(RefreshMatViewStmt),
            "::",
            stringify!(concurrent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RefreshMatViewStmt>())).skipData as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(RefreshMatViewStmt),
            "::",
            stringify!(skipData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<RefreshMatViewStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(RefreshMatViewStmt),
            "::",
            stringify!(relation)
        )
    );
}
impl Default for RefreshMatViewStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " Checkpoint Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CheckPointStmt {
    pub type_: NodeTag,
}
#[test]
fn bindgen_test_layout_CheckPointStmt() {
    assert_eq!(
        ::std::mem::size_of::<CheckPointStmt>(),
        4usize,
        concat!("Size of: ", stringify!(CheckPointStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CheckPointStmt>(),
        4usize,
        concat!("Alignment of ", stringify!(CheckPointStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CheckPointStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CheckPointStmt),
            "::",
            stringify!(type_)
        )
    );
}
impl Default for CheckPointStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " Discard Statement"]
#[doc = ""]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DiscardMode {
    DISCARD_ALL = 0,
    DISCARD_PLANS = 1,
    DISCARD_SEQUENCES = 2,
    DISCARD_TEMP = 3,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DiscardStmt {
    pub type_: NodeTag,
    pub target: DiscardMode,
}
#[test]
fn bindgen_test_layout_DiscardStmt() {
    assert_eq!(
        ::std::mem::size_of::<DiscardStmt>(),
        8usize,
        concat!("Size of: ", stringify!(DiscardStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DiscardStmt>(),
        4usize,
        concat!("Alignment of ", stringify!(DiscardStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DiscardStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DiscardStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DiscardStmt>())).target as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(DiscardStmt),
            "::",
            stringify!(target)
        )
    );
}
impl Default for DiscardStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    LOCK Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct LockStmt {
    pub type_: NodeTag,
    pub relations: *mut List,
    #[doc = " relations to lock"]
    pub mode: ::std::os::raw::c_int,
    #[doc = " lock mode"]
    pub nowait: bool,
}
#[test]
fn bindgen_test_layout_LockStmt() {
    assert_eq!(
        ::std::mem::size_of::<LockStmt>(),
        24usize,
        concat!("Size of: ", stringify!(LockStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<LockStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(LockStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<LockStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(LockStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<LockStmt>())).relations as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(LockStmt),
            "::",
            stringify!(relations)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<LockStmt>())).mode as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(LockStmt),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<LockStmt>())).nowait as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(LockStmt),
            "::",
            stringify!(nowait)
        )
    );
}
impl Default for LockStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    SET CONSTRAINTS Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ConstraintsSetStmt {
    pub type_: NodeTag,
    pub constraints: *mut List,
    #[doc = " List of names as RangeVars"]
    pub deferred: bool,
}
#[test]
fn bindgen_test_layout_ConstraintsSetStmt() {
    assert_eq!(
        ::std::mem::size_of::<ConstraintsSetStmt>(),
        24usize,
        concat!("Size of: ", stringify!(ConstraintsSetStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ConstraintsSetStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ConstraintsSetStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ConstraintsSetStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ConstraintsSetStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ConstraintsSetStmt>())).constraints as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ConstraintsSetStmt),
            "::",
            stringify!(constraints)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ConstraintsSetStmt>())).deferred as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ConstraintsSetStmt),
            "::",
            stringify!(deferred)
        )
    );
}
impl Default for ConstraintsSetStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " report pgstat progress"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ReindexObjectType {
    REINDEX_OBJECT_INDEX = 0,
    #[doc = " index"]
    REINDEX_OBJECT_TABLE = 1,
    #[doc = " table or materialized view"]
    REINDEX_OBJECT_SCHEMA = 2,
    #[doc = " schema"]
    REINDEX_OBJECT_SYSTEM = 3,
    #[doc = " system catalogs"]
    REINDEX_OBJECT_DATABASE = 4,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ReindexStmt {
    pub type_: NodeTag,
    pub kind: ReindexObjectType,
    #[doc = " REINDEX_OBJECT_INDEX, REINDEX_OBJECT_TABLE,"]
    #[doc = " etc."]
    pub relation: *mut RangeVar,
    #[doc = " Table or index to reindex"]
    pub name: *const ::std::os::raw::c_char,
    #[doc = " name of database to reindex"]
    pub options: ::std::os::raw::c_int,
    #[doc = " Reindex options flags"]
    pub concurrent: bool,
}
#[test]
fn bindgen_test_layout_ReindexStmt() {
    assert_eq!(
        ::std::mem::size_of::<ReindexStmt>(),
        32usize,
        concat!("Size of: ", stringify!(ReindexStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ReindexStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ReindexStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReindexStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ReindexStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReindexStmt>())).kind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ReindexStmt),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReindexStmt>())).relation as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ReindexStmt),
            "::",
            stringify!(relation)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReindexStmt>())).name as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ReindexStmt),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReindexStmt>())).options as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ReindexStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReindexStmt>())).concurrent as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(ReindexStmt),
            "::",
            stringify!(concurrent)
        )
    );
}
impl Default for ReindexStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    CREATE CONVERSION Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateConversionStmt {
    pub type_: NodeTag,
    pub conversion_name: *mut List,
    #[doc = " Name of the conversion"]
    pub for_encoding_name: *mut ::std::os::raw::c_char,
    #[doc = " source encoding name"]
    pub to_encoding_name: *mut ::std::os::raw::c_char,
    #[doc = " destination encoding name"]
    pub func_name: *mut List,
    #[doc = " qualified conversion function name"]
    pub def: bool,
}
#[test]
fn bindgen_test_layout_CreateConversionStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateConversionStmt>(),
        48usize,
        concat!("Size of: ", stringify!(CreateConversionStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateConversionStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateConversionStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateConversionStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateConversionStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateConversionStmt>())).conversion_name as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateConversionStmt),
            "::",
            stringify!(conversion_name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateConversionStmt>())).for_encoding_name as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateConversionStmt),
            "::",
            stringify!(for_encoding_name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateConversionStmt>())).to_encoding_name as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateConversionStmt),
            "::",
            stringify!(to_encoding_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateConversionStmt>())).func_name as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateConversionStmt),
            "::",
            stringify!(func_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateConversionStmt>())).def as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateConversionStmt),
            "::",
            stringify!(def)
        )
    );
}
impl Default for CreateConversionStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " CREATE CAST Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateCastStmt {
    pub type_: NodeTag,
    pub sourcetype: *mut TypeName,
    pub targettype: *mut TypeName,
    pub func: *mut ObjectWithArgs,
    pub context: CoercionContext,
    pub inout: bool,
}
#[test]
fn bindgen_test_layout_CreateCastStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateCastStmt>(),
        40usize,
        concat!("Size of: ", stringify!(CreateCastStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateCastStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateCastStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateCastStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateCastStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateCastStmt>())).sourcetype as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateCastStmt),
            "::",
            stringify!(sourcetype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateCastStmt>())).targettype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateCastStmt),
            "::",
            stringify!(targettype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateCastStmt>())).func as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateCastStmt),
            "::",
            stringify!(func)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateCastStmt>())).context as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateCastStmt),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateCastStmt>())).inout as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateCastStmt),
            "::",
            stringify!(inout)
        )
    );
}
impl Default for CreateCastStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " CREATE TRANSFORM Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateTransformStmt {
    pub type_: NodeTag,
    pub replace: bool,
    pub type_name: *mut TypeName,
    pub lang: *mut ::std::os::raw::c_char,
    pub fromsql: *mut ObjectWithArgs,
    pub tosql: *mut ObjectWithArgs,
}
#[test]
fn bindgen_test_layout_CreateTransformStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateTransformStmt>(),
        40usize,
        concat!("Size of: ", stringify!(CreateTransformStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateTransformStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateTransformStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTransformStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTransformStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTransformStmt>())).replace as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTransformStmt),
            "::",
            stringify!(replace)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTransformStmt>())).type_name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTransformStmt),
            "::",
            stringify!(type_name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTransformStmt>())).lang as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTransformStmt),
            "::",
            stringify!(lang)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTransformStmt>())).fromsql as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTransformStmt),
            "::",
            stringify!(fromsql)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateTransformStmt>())).tosql as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateTransformStmt),
            "::",
            stringify!(tosql)
        )
    );
}
impl Default for CreateTransformStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    PREPARE Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PrepareStmt {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " Name of plan, arbitrary"]
    pub argtypes: *mut List,
    #[doc = " Types of parameters (List of TypeName)"]
    pub query: *mut Node,
}
#[test]
fn bindgen_test_layout_PrepareStmt() {
    assert_eq!(
        ::std::mem::size_of::<PrepareStmt>(),
        32usize,
        concat!("Size of: ", stringify!(PrepareStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<PrepareStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(PrepareStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PrepareStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PrepareStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PrepareStmt>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PrepareStmt),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PrepareStmt>())).argtypes as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PrepareStmt),
            "::",
            stringify!(argtypes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PrepareStmt>())).query as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PrepareStmt),
            "::",
            stringify!(query)
        )
    );
}
impl Default for PrepareStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    EXECUTE Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ExecuteStmt {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " The name of the plan to execute"]
    pub params: *mut List,
}
#[test]
fn bindgen_test_layout_ExecuteStmt() {
    assert_eq!(
        ::std::mem::size_of::<ExecuteStmt>(),
        24usize,
        concat!("Size of: ", stringify!(ExecuteStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ExecuteStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ExecuteStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ExecuteStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ExecuteStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ExecuteStmt>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ExecuteStmt),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ExecuteStmt>())).params as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ExecuteStmt),
            "::",
            stringify!(params)
        )
    );
}
impl Default for ExecuteStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    DEALLOCATE Statement"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DeallocateStmt {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_DeallocateStmt() {
    assert_eq!(
        ::std::mem::size_of::<DeallocateStmt>(),
        16usize,
        concat!("Size of: ", stringify!(DeallocateStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DeallocateStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DeallocateStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeallocateStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeallocateStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DeallocateStmt>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeallocateStmt),
            "::",
            stringify!(name)
        )
    );
}
impl Default for DeallocateStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    DROP OWNED statement"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DropOwnedStmt {
    pub type_: NodeTag,
    pub roles: *mut List,
    pub behavior: DropBehavior,
}
#[test]
fn bindgen_test_layout_DropOwnedStmt() {
    assert_eq!(
        ::std::mem::size_of::<DropOwnedStmt>(),
        24usize,
        concat!("Size of: ", stringify!(DropOwnedStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DropOwnedStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DropOwnedStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropOwnedStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DropOwnedStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropOwnedStmt>())).roles as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DropOwnedStmt),
            "::",
            stringify!(roles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropOwnedStmt>())).behavior as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DropOwnedStmt),
            "::",
            stringify!(behavior)
        )
    );
}
impl Default for DropOwnedStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "    REASSIGN OWNED statement"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ReassignOwnedStmt {
    pub type_: NodeTag,
    pub roles: *mut List,
    pub newrole: *mut RoleSpec,
}
#[test]
fn bindgen_test_layout_ReassignOwnedStmt() {
    assert_eq!(
        ::std::mem::size_of::<ReassignOwnedStmt>(),
        24usize,
        concat!("Size of: ", stringify!(ReassignOwnedStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<ReassignOwnedStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(ReassignOwnedStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReassignOwnedStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ReassignOwnedStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReassignOwnedStmt>())).roles as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ReassignOwnedStmt),
            "::",
            stringify!(roles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ReassignOwnedStmt>())).newrole as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ReassignOwnedStmt),
            "::",
            stringify!(newrole)
        )
    );
}
impl Default for ReassignOwnedStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = " TS Dictionary stmts: DefineStmt, RenameStmt and DropStmt are default"]
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterTSDictionaryStmt {
    pub type_: NodeTag,
    pub dictname: *mut List,
    #[doc = " qualified name (list of Value strings)"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_AlterTSDictionaryStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterTSDictionaryStmt>(),
        24usize,
        concat!("Size of: ", stringify!(AlterTSDictionaryStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterTSDictionaryStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterTSDictionaryStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTSDictionaryStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSDictionaryStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTSDictionaryStmt>())).dictname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSDictionaryStmt),
            "::",
            stringify!(dictname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTSDictionaryStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSDictionaryStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for AlterTSDictionaryStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[doc = " TS Configuration stmts: DefineStmt, RenameStmt and DropStmt are default"]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AlterTSConfigType {
    ALTER_TSCONFIG_ADD_MAPPING = 0,
    ALTER_TSCONFIG_ALTER_MAPPING_FOR_TOKEN = 1,
    ALTER_TSCONFIG_REPLACE_DICT = 2,
    ALTER_TSCONFIG_REPLACE_DICT_FOR_TOKEN = 3,
    ALTER_TSCONFIG_DROP_MAPPING = 4,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterTSConfigurationStmt {
    pub type_: NodeTag,
    pub kind: AlterTSConfigType,
    #[doc = " ALTER_TSCONFIG_ADD_MAPPING, etc"]
    pub cfgname: *mut List,
    #[doc = " dicts will be nonNIL if ADD/ALTER MAPPING was specified. If dicts is"]
    #[doc = " NIL, but tokentype isn't, DROP MAPPING was specified."]
    pub tokentype: *mut List,
    #[doc = " list of Value strings"]
    pub dicts: *mut List,
    #[doc = " list of list of Value strings"]
    pub override_: bool,
    #[doc = " if true  remove old variant"]
    pub replace: bool,
    #[doc = " if true  replace dictionary by another"]
    pub missing_ok: bool,
}
#[test]
fn bindgen_test_layout_AlterTSConfigurationStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterTSConfigurationStmt>(),
        40usize,
        concat!("Size of: ", stringify!(AlterTSConfigurationStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterTSConfigurationStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterTSConfigurationStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTSConfigurationStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSConfigurationStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTSConfigurationStmt>())).kind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSConfigurationStmt),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTSConfigurationStmt>())).cfgname as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSConfigurationStmt),
            "::",
            stringify!(cfgname)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTSConfigurationStmt>())).tokentype as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSConfigurationStmt),
            "::",
            stringify!(tokentype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterTSConfigurationStmt>())).dicts as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSConfigurationStmt),
            "::",
            stringify!(dicts)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTSConfigurationStmt>())).override_ as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSConfigurationStmt),
            "::",
            stringify!(override_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTSConfigurationStmt>())).replace as *const _ as usize
        },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSConfigurationStmt),
            "::",
            stringify!(replace)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterTSConfigurationStmt>())).missing_ok as *const _ as usize
        },
        34usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterTSConfigurationStmt),
            "::",
            stringify!(missing_ok)
        )
    );
}
impl Default for AlterTSConfigurationStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreatePublicationStmt {
    pub type_: NodeTag,
    pub pubname: *mut ::std::os::raw::c_char,
    #[doc = " Name of the publication"]
    pub options: *mut List,
    #[doc = " List of DefElem nodes"]
    pub tables: *mut List,
    #[doc = " Optional list of tables to add"]
    pub for_all_tables: bool,
}
#[test]
fn bindgen_test_layout_CreatePublicationStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreatePublicationStmt>(),
        40usize,
        concat!("Size of: ", stringify!(CreatePublicationStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreatePublicationStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreatePublicationStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePublicationStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePublicationStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePublicationStmt>())).pubname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePublicationStmt),
            "::",
            stringify!(pubname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePublicationStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePublicationStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreatePublicationStmt>())).tables as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePublicationStmt),
            "::",
            stringify!(tables)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreatePublicationStmt>())).for_all_tables as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreatePublicationStmt),
            "::",
            stringify!(for_all_tables)
        )
    );
}
impl Default for CreatePublicationStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterPublicationStmt {
    pub type_: NodeTag,
    pub pubname: *mut ::std::os::raw::c_char,
    #[doc = " parameters used for ALTER PUBLICATION ... WITH"]
    pub options: *mut List,
    #[doc = " parameters used for ALTER PUBLICATION ... ADD/DROP TABLE"]
    pub tables: *mut List,
    #[doc = " List of tables to add/drop"]
    pub for_all_tables: bool,
    #[doc = " Special publication for all tables in db"]
    pub tableAction: DefElemAction,
}
#[test]
fn bindgen_test_layout_AlterPublicationStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterPublicationStmt>(),
        40usize,
        concat!("Size of: ", stringify!(AlterPublicationStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterPublicationStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterPublicationStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterPublicationStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPublicationStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterPublicationStmt>())).pubname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPublicationStmt),
            "::",
            stringify!(pubname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterPublicationStmt>())).options as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPublicationStmt),
            "::",
            stringify!(options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterPublicationStmt>())).tables as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPublicationStmt),
            "::",
            stringify!(tables)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterPublicationStmt>())).for_all_tables as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPublicationStmt),
            "::",
            stringify!(for_all_tables)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterPublicationStmt>())).tableAction as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterPublicationStmt),
            "::",
            stringify!(tableAction)
        )
    );
}
impl Default for AlterPublicationStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CreateSubscriptionStmt {
    pub type_: NodeTag,
    pub subname: *mut ::std::os::raw::c_char,
    #[doc = " Name of the subscription"]
    pub conninfo: *mut ::std::os::raw::c_char,
    #[doc = " Connection string to publisher"]
    pub publication: *mut List,
    #[doc = " One or more publication to subscribe to"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_CreateSubscriptionStmt() {
    assert_eq!(
        ::std::mem::size_of::<CreateSubscriptionStmt>(),
        40usize,
        concat!("Size of: ", stringify!(CreateSubscriptionStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<CreateSubscriptionStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(CreateSubscriptionStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSubscriptionStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSubscriptionStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSubscriptionStmt>())).subname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSubscriptionStmt),
            "::",
            stringify!(subname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSubscriptionStmt>())).conninfo as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSubscriptionStmt),
            "::",
            stringify!(conninfo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<CreateSubscriptionStmt>())).publication as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSubscriptionStmt),
            "::",
            stringify!(publication)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<CreateSubscriptionStmt>())).options as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(CreateSubscriptionStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for CreateSubscriptionStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Serialize, Deserialize)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AlterSubscriptionType {
    ALTER_SUBSCRIPTION_OPTIONS = 0,
    ALTER_SUBSCRIPTION_CONNECTION = 1,
    ALTER_SUBSCRIPTION_PUBLICATION = 2,
    ALTER_SUBSCRIPTION_REFRESH = 3,
    ALTER_SUBSCRIPTION_ENABLED = 4,
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct AlterSubscriptionStmt {
    pub type_: NodeTag,
    pub kind: AlterSubscriptionType,
    #[doc = " ALTER_SUBSCRIPTION_OPTIONS, etc"]
    pub subname: *mut ::std::os::raw::c_char,
    #[doc = " Name of the subscription"]
    pub conninfo: *mut ::std::os::raw::c_char,
    #[doc = " Connection string to publisher"]
    pub publication: *mut List,
    #[doc = " One or more publication to subscribe to"]
    pub options: *mut List,
}
#[test]
fn bindgen_test_layout_AlterSubscriptionStmt() {
    assert_eq!(
        ::std::mem::size_of::<AlterSubscriptionStmt>(),
        40usize,
        concat!("Size of: ", stringify!(AlterSubscriptionStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<AlterSubscriptionStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(AlterSubscriptionStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSubscriptionStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSubscriptionStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSubscriptionStmt>())).kind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSubscriptionStmt),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSubscriptionStmt>())).subname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSubscriptionStmt),
            "::",
            stringify!(subname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSubscriptionStmt>())).conninfo as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSubscriptionStmt),
            "::",
            stringify!(conninfo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<AlterSubscriptionStmt>())).publication as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSubscriptionStmt),
            "::",
            stringify!(publication)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<AlterSubscriptionStmt>())).options as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(AlterSubscriptionStmt),
            "::",
            stringify!(options)
        )
    );
}
impl Default for AlterSubscriptionStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DropSubscriptionStmt {
    pub type_: NodeTag,
    pub subname: *mut ::std::os::raw::c_char,
    #[doc = " Name of the subscription"]
    pub missing_ok: bool,
    #[doc = " Skip error if missing?"]
    pub behavior: DropBehavior,
}
#[test]
fn bindgen_test_layout_DropSubscriptionStmt() {
    assert_eq!(
        ::std::mem::size_of::<DropSubscriptionStmt>(),
        24usize,
        concat!("Size of: ", stringify!(DropSubscriptionStmt))
    );
    assert_eq!(
        ::std::mem::align_of::<DropSubscriptionStmt>(),
        8usize,
        concat!("Alignment of ", stringify!(DropSubscriptionStmt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropSubscriptionStmt>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DropSubscriptionStmt),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropSubscriptionStmt>())).subname as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DropSubscriptionStmt),
            "::",
            stringify!(subname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropSubscriptionStmt>())).missing_ok as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DropSubscriptionStmt),
            "::",
            stringify!(missing_ok)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DropSubscriptionStmt>())).behavior as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(DropSubscriptionStmt),
            "::",
            stringify!(behavior)
        )
    );
}
impl Default for DropSubscriptionStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn raw_parser(str: *const ::std::os::raw::c_char) -> *mut List;
}
extern "C" {
    pub static mut TopMemoryContext: MemoryContext;
}
extern "C" {
    pub fn MemoryContextInit();
}
extern "C" {
    pub fn MemoryContextReset(context: MemoryContext);
}
extern "C" {
    pub fn AllocSetContextCreateInternal(
        parent: MemoryContext,
        name: *const ::std::os::raw::c_char,
        minContextSize: Size,
        initBlockSize: Size,
        maxBlockSize: Size,
    ) -> MemoryContext;
}
