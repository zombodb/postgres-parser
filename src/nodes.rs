#![doc = r" Generated types to represent a parse tree in a safe manner as returned from `parse_query()`"]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
#[doc = r" All the various Postgres parse tree nodes that can be returned upon parsing a SQL statement"]
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum Node {
    A_ArrayExpr(A_ArrayExpr),
    A_Const(A_Const),
    A_Expr(A_Expr),
    A_Indices(A_Indices),
    A_Indirection(A_Indirection),
    A_Star(A_Star),
    AccessPriv(AccessPriv),
    Aggref(Aggref),
    Alias(Alias),
    AlterCollationStmt(AlterCollationStmt),
    AlterDatabaseSetStmt(AlterDatabaseSetStmt),
    AlterDatabaseStmt(AlterDatabaseStmt),
    AlterDefaultPrivilegesStmt(AlterDefaultPrivilegesStmt),
    AlterDomainStmt(AlterDomainStmt),
    AlterEnumStmt(AlterEnumStmt),
    AlterEventTrigStmt(AlterEventTrigStmt),
    AlterExtensionContentsStmt(AlterExtensionContentsStmt),
    AlterExtensionStmt(AlterExtensionStmt),
    AlterFdwStmt(AlterFdwStmt),
    AlterForeignServerStmt(AlterForeignServerStmt),
    AlterFunctionStmt(AlterFunctionStmt),
    AlterObjectDependsStmt(AlterObjectDependsStmt),
    AlterObjectSchemaStmt(AlterObjectSchemaStmt),
    AlterOpFamilyStmt(AlterOpFamilyStmt),
    AlterOperatorStmt(AlterOperatorStmt),
    AlterOwnerStmt(AlterOwnerStmt),
    AlterPolicyStmt(AlterPolicyStmt),
    AlterPublicationStmt(AlterPublicationStmt),
    AlterRoleSetStmt(AlterRoleSetStmt),
    AlterRoleStmt(AlterRoleStmt),
    AlterSeqStmt(AlterSeqStmt),
    AlterStatsStmt(AlterStatsStmt),
    AlterSubscriptionStmt(AlterSubscriptionStmt),
    AlterSystemStmt(AlterSystemStmt),
    AlterTSConfigurationStmt(AlterTSConfigurationStmt),
    AlterTSDictionaryStmt(AlterTSDictionaryStmt),
    AlterTableCmd(AlterTableCmd),
    AlterTableMoveAllStmt(AlterTableMoveAllStmt),
    AlterTableSpaceOptionsStmt(AlterTableSpaceOptionsStmt),
    AlterTableStmt(AlterTableStmt),
    AlterTypeStmt(AlterTypeStmt),
    AlterUserMappingStmt(AlterUserMappingStmt),
    AlternativeSubPlan(AlternativeSubPlan),
    ArrayCoerceExpr(ArrayCoerceExpr),
    ArrayExpr(ArrayExpr),
    BoolExpr(BoolExpr),
    BooleanTest(BooleanTest),
    CallContext(CallContext),
    CallStmt(CallStmt),
    CaseExpr(CaseExpr),
    CaseTestExpr(CaseTestExpr),
    CaseWhen(CaseWhen),
    CheckPointStmt(CheckPointStmt),
    ClosePortalStmt(ClosePortalStmt),
    ClusterStmt(ClusterStmt),
    CoalesceExpr(CoalesceExpr),
    CoerceToDomain(CoerceToDomain),
    CoerceToDomainValue(CoerceToDomainValue),
    CoerceViaIO(CoerceViaIO),
    CollateClause(CollateClause),
    CollateExpr(CollateExpr),
    ColumnDef(ColumnDef),
    ColumnRef(ColumnRef),
    CommentStmt(CommentStmt),
    CommonTableExpr(CommonTableExpr),
    CompositeTypeStmt(CompositeTypeStmt),
    Const(Const),
    Constraint(Constraint),
    ConstraintsSetStmt(ConstraintsSetStmt),
    ConvertRowtypeExpr(ConvertRowtypeExpr),
    CopyStmt(CopyStmt),
    CreateAmStmt(CreateAmStmt),
    CreateCastStmt(CreateCastStmt),
    CreateConversionStmt(CreateConversionStmt),
    CreateDomainStmt(CreateDomainStmt),
    CreateEnumStmt(CreateEnumStmt),
    CreateEventTrigStmt(CreateEventTrigStmt),
    CreateExtensionStmt(CreateExtensionStmt),
    CreateFdwStmt(CreateFdwStmt),
    CreateForeignServerStmt(CreateForeignServerStmt),
    CreateForeignTableStmt(CreateForeignTableStmt),
    CreateFunctionStmt(CreateFunctionStmt),
    CreateOpClassItem(CreateOpClassItem),
    CreateOpClassStmt(CreateOpClassStmt),
    CreateOpFamilyStmt(CreateOpFamilyStmt),
    CreatePLangStmt(CreatePLangStmt),
    CreatePolicyStmt(CreatePolicyStmt),
    CreatePublicationStmt(CreatePublicationStmt),
    CreateRangeStmt(CreateRangeStmt),
    CreateRoleStmt(CreateRoleStmt),
    CreateSchemaStmt(CreateSchemaStmt),
    CreateSeqStmt(CreateSeqStmt),
    CreateStatsStmt(CreateStatsStmt),
    CreateStmt(CreateStmt),
    CreateSubscriptionStmt(CreateSubscriptionStmt),
    CreateTableAsStmt(CreateTableAsStmt),
    CreateTableSpaceStmt(CreateTableSpaceStmt),
    CreateTransformStmt(CreateTransformStmt),
    CreateTrigStmt(CreateTrigStmt),
    CreateUserMappingStmt(CreateUserMappingStmt),
    CreatedbStmt(CreatedbStmt),
    CurrentOfExpr(CurrentOfExpr),
    DeallocateStmt(DeallocateStmt),
    DeclareCursorStmt(DeclareCursorStmt),
    DefElem(DefElem),
    DefineStmt(DefineStmt),
    DeleteStmt(DeleteStmt),
    DiscardStmt(DiscardStmt),
    DoStmt(DoStmt),
    DropOwnedStmt(DropOwnedStmt),
    DropRoleStmt(DropRoleStmt),
    DropStmt(DropStmt),
    DropSubscriptionStmt(DropSubscriptionStmt),
    DropTableSpaceStmt(DropTableSpaceStmt),
    DropUserMappingStmt(DropUserMappingStmt),
    DropdbStmt(DropdbStmt),
    ExecuteStmt(ExecuteStmt),
    ExplainStmt(ExplainStmt),
    Expr(Box<Node>),
    FetchStmt(FetchStmt),
    FieldSelect(FieldSelect),
    FieldStore(FieldStore),
    FromExpr(FromExpr),
    FuncCall(FuncCall),
    FuncExpr(FuncExpr),
    FunctionParameter(FunctionParameter),
    GrantRoleStmt(GrantRoleStmt),
    GrantStmt(GrantStmt),
    GroupingFunc(GroupingFunc),
    GroupingSet(GroupingSet),
    ImportForeignSchemaStmt(ImportForeignSchemaStmt),
    IndexElem(IndexElem),
    IndexStmt(IndexStmt),
    InferClause(InferClause),
    InferenceElem(InferenceElem),
    InlineCodeBlock(InlineCodeBlock),
    InsertStmt(InsertStmt),
    IntoClause(IntoClause),
    JoinExpr(JoinExpr),
    List(Vec<Node>),
    ListenStmt(ListenStmt),
    LoadStmt(LoadStmt),
    LockStmt(LockStmt),
    LockingClause(LockingClause),
    MinMaxExpr(MinMaxExpr),
    MultiAssignRef(MultiAssignRef),
    NamedArgExpr(NamedArgExpr),
    NextValueExpr(NextValueExpr),
    NotifyStmt(NotifyStmt),
    NullTest(NullTest),
    ObjectWithArgs(ObjectWithArgs),
    OnConflictClause(OnConflictClause),
    OnConflictExpr(OnConflictExpr),
    OpExpr(OpExpr),
    Param(Param),
    ParamRef(ParamRef),
    PartitionBoundSpec(PartitionBoundSpec),
    PartitionCmd(PartitionCmd),
    PartitionElem(PartitionElem),
    PartitionRangeDatum(PartitionRangeDatum),
    PartitionSpec(PartitionSpec),
    PrepareStmt(PrepareStmt),
    RangeFunction(RangeFunction),
    RangeSubselect(RangeSubselect),
    RangeTableFunc(RangeTableFunc),
    RangeTableFuncCol(RangeTableFuncCol),
    RangeTableSample(RangeTableSample),
    RangeTblRef(RangeTblRef),
    RangeVar(RangeVar),
    RawStmt(RawStmt),
    ReassignOwnedStmt(ReassignOwnedStmt),
    RefreshMatViewStmt(RefreshMatViewStmt),
    ReindexStmt(ReindexStmt),
    RelabelType(RelabelType),
    RenameStmt(RenameStmt),
    ReplicaIdentityStmt(ReplicaIdentityStmt),
    ResTarget(ResTarget),
    RoleSpec(RoleSpec),
    RowCompareExpr(RowCompareExpr),
    RowExpr(RowExpr),
    RowMarkClause(RowMarkClause),
    RuleStmt(RuleStmt),
    SQLValueFunction(SQLValueFunction),
    ScalarArrayOpExpr(ScalarArrayOpExpr),
    SecLabelStmt(SecLabelStmt),
    SelectStmt(SelectStmt),
    SetOperationStmt(SetOperationStmt),
    SetToDefault(SetToDefault),
    SortBy(SortBy),
    SortGroupClause(SortGroupClause),
    SubLink(SubLink),
    SubscriptingRef(SubscriptingRef),
    TableFunc(TableFunc),
    TableLikeClause(TableLikeClause),
    TableSampleClause(TableSampleClause),
    TargetEntry(TargetEntry),
    TransactionStmt(TransactionStmt),
    TriggerTransition(TriggerTransition),
    TruncateStmt(TruncateStmt),
    TypeCast(TypeCast),
    TypeName(TypeName),
    UnlistenStmt(UnlistenStmt),
    UpdateStmt(UpdateStmt),
    VacuumRelation(VacuumRelation),
    VacuumStmt(VacuumStmt),
    Value(Value),
    Var(Var),
    VariableSetStmt(VariableSetStmt),
    VariableShowStmt(VariableShowStmt),
    ViewStmt(ViewStmt),
    WindowClause(WindowClause),
    WindowDef(WindowDef),
    WindowFunc(WindowFunc),
    WithCheckOption(WithCheckOption),
    WithClause(WithClause),
    XmlExpr(XmlExpr),
    XmlSerialize(XmlSerialize),
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Alias {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliasname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colnames: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RangeVar {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalogname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemaname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relname: Option<String>,
    pub inh: bool,
    pub relpersistence: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Box<Alias>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct TableFunc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns_uris: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns_names: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docexpr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowexpr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colnames: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coltypes: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coltypmods: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colcollations: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colexprs: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coldefexprs: Option<Vec<Node>>,
    pub ordinalitycol: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct IntoClause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colNames: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessMethod: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    pub onCommit: crate::sys::OnCommitAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tableSpaceName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewQuery: Option<Box<Node>>,
    pub skipData: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Expr {}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Var {
    pub varno: crate::sys::Index,
    pub varattno: crate::sys::AttrNumber,
    pub vartype: crate::sys::Oid,
    pub vartypmod: i32,
    pub varcollid: crate::sys::Oid,
    pub varlevelsup: crate::sys::Index,
    pub varnosyn: crate::sys::Index,
    pub varattnosyn: crate::sys::AttrNumber,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Const {
    pub consttype: crate::sys::Oid,
    pub consttypmod: i32,
    pub constcollid: crate::sys::Oid,
    pub constlen: i32,
    pub constvalue: crate::sys::Datum,
    pub constisnull: bool,
    pub constbyval: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Param {
    pub paramkind: crate::sys::ParamKind,
    pub paramid: i32,
    pub paramtype: crate::sys::Oid,
    pub paramtypmod: i32,
    pub paramcollid: crate::sys::Oid,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Aggref {
    pub aggfnoid: crate::sys::Oid,
    pub aggtype: crate::sys::Oid,
    pub aggcollid: crate::sys::Oid,
    pub inputcollid: crate::sys::Oid,
    pub aggtranstype: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggargtypes: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggdirectargs: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggorder: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggdistinct: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggfilter: Option<Box<Node>>,
    pub aggstar: bool,
    pub aggvariadic: bool,
    pub aggkind: char,
    pub agglevelsup: crate::sys::Index,
    pub aggsplit: crate::sys::AggSplit,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct GroupingFunc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cols: Option<Vec<Node>>,
    pub agglevelsup: crate::sys::Index,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct WindowFunc {
    pub winfnoid: crate::sys::Oid,
    pub wintype: crate::sys::Oid,
    pub wincollid: crate::sys::Oid,
    pub inputcollid: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggfilter: Option<Box<Node>>,
    pub winref: crate::sys::Index,
    pub winstar: bool,
    pub winagg: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct SubscriptingRef {
    pub refcontainertype: crate::sys::Oid,
    pub refelemtype: crate::sys::Oid,
    pub reftypmod: i32,
    pub refcollid: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refupperindexpr: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflowerindexpr: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refexpr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refassgnexpr: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct FuncExpr {
    pub funcid: crate::sys::Oid,
    pub funcresulttype: crate::sys::Oid,
    pub funcretset: bool,
    pub funcvariadic: bool,
    pub funcformat: crate::sys::CoercionForm,
    pub funccollid: crate::sys::Oid,
    pub inputcollid: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct NamedArgExpr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub argnumber: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct OpExpr {
    pub opno: crate::sys::Oid,
    pub opfuncid: crate::sys::Oid,
    pub opresulttype: crate::sys::Oid,
    pub opretset: bool,
    pub opcollid: crate::sys::Oid,
    pub inputcollid: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScalarArrayOpExpr {
    pub opno: crate::sys::Oid,
    pub opfuncid: crate::sys::Oid,
    pub useOr: bool,
    pub inputcollid: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct BoolExpr {
    pub boolop: crate::sys::BoolExprType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct SubLink {
    pub subLinkType: crate::sys::SubLinkType,
    pub subLinkId: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testexpr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operName: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subselect: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlternativeSubPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subplans: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct FieldSelect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    pub fieldnum: crate::sys::AttrNumber,
    pub resulttype: crate::sys::Oid,
    pub resulttypmod: i32,
    pub resultcollid: crate::sys::Oid,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct FieldStore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newvals: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fieldnums: Option<Vec<Node>>,
    pub resulttype: crate::sys::Oid,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RelabelType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    pub resulttype: crate::sys::Oid,
    pub resulttypmod: i32,
    pub resultcollid: crate::sys::Oid,
    pub relabelformat: crate::sys::CoercionForm,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CoerceViaIO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    pub resulttype: crate::sys::Oid,
    pub resultcollid: crate::sys::Oid,
    pub coerceformat: crate::sys::CoercionForm,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArrayCoerceExpr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elemexpr: Option<Box<Node>>,
    pub resulttype: crate::sys::Oid,
    pub resulttypmod: i32,
    pub resultcollid: crate::sys::Oid,
    pub coerceformat: crate::sys::CoercionForm,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConvertRowtypeExpr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    pub resulttype: crate::sys::Oid,
    pub convertformat: crate::sys::CoercionForm,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CollateExpr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    pub collOid: crate::sys::Oid,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CaseExpr {
    pub casetype: crate::sys::Oid,
    pub casecollid: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defresult: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CaseWhen {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CaseTestExpr {
    pub typeId: crate::sys::Oid,
    pub typeMod: i32,
    pub collation: crate::sys::Oid,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArrayExpr {
    pub array_typeid: crate::sys::Oid,
    pub array_collid: crate::sys::Oid,
    pub element_typeid: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<Node>>,
    pub multidims: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RowExpr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    pub row_typeid: crate::sys::Oid,
    pub row_format: crate::sys::CoercionForm,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colnames: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RowCompareExpr {
    pub rctype: crate::sys::RowCompareType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opnos: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opfamilies: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputcollids: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub largs: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rargs: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CoalesceExpr {
    pub coalescetype: crate::sys::Oid,
    pub coalescecollid: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MinMaxExpr {
    pub minmaxtype: crate::sys::Oid,
    pub minmaxcollid: crate::sys::Oid,
    pub inputcollid: crate::sys::Oid,
    pub op: crate::sys::MinMaxOp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct SQLValueFunction {
    pub op: crate::sys::SQLValueFunctionOp,
    pub typmod: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct XmlExpr {
    pub op: crate::sys::XmlExprOp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_args: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg_names: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    pub xmloption: crate::sys::XmlOptionType,
    pub typmod: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct NullTest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    pub nulltesttype: crate::sys::NullTestType,
    pub argisrow: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct BooleanTest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    pub booltesttype: crate::sys::BoolTestType,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CoerceToDomain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    pub resulttype: crate::sys::Oid,
    pub resulttypmod: i32,
    pub resultcollid: crate::sys::Oid,
    pub coercionformat: crate::sys::CoercionForm,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CoerceToDomainValue {
    pub typeId: crate::sys::Oid,
    pub typeMod: i32,
    pub collation: crate::sys::Oid,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct SetToDefault {
    pub typeId: crate::sys::Oid,
    pub typeMod: i32,
    pub collation: crate::sys::Oid,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CurrentOfExpr {
    pub cvarno: crate::sys::Index,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_name: Option<String>,
    pub cursor_param: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct NextValueExpr {
    pub seqid: crate::sys::Oid,
    pub typeId: crate::sys::Oid,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct InferenceElem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<Box<Node>>,
    pub infercollid: crate::sys::Oid,
    pub inferopclass: crate::sys::Oid,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct TargetEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<Box<Node>>,
    pub resno: crate::sys::AttrNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resname: Option<String>,
    pub ressortgroupref: crate::sys::Index,
    pub resorigtbl: crate::sys::Oid,
    pub resorigcol: crate::sys::AttrNumber,
    pub resjunk: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RangeTblRef {
    pub rtindex: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct JoinExpr {
    pub jointype: crate::sys::JoinType,
    pub isNatural: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub larg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usingClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quals: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Box<Alias>>,
    pub rtindex: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct FromExpr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromlist: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quals: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct OnConflictExpr {
    pub action: crate::sys::OnConflictAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arbiterElems: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arbiterWhere: Option<Box<Node>>,
    pub constraint: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onConflictSet: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onConflictWhere: Option<Box<Node>>,
    pub exclRelIndex: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclRelTlist: Option<Vec<Node>>,
}
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Value {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null: Option<()>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct TypeName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<Node>>,
    pub typeOid: crate::sys::Oid,
    pub setof: bool,
    pub pct_type: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typmods: Option<Vec<Node>>,
    pub typemod: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrayBounds: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ColumnRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ParamRef {
    pub number: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct A_Expr {
    pub kind: crate::sys::A_Expr_Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexpr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rexpr: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct A_Const {
    pub val: Value,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct TypeCast {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<Box<TypeName>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CollateClause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collname: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RoleSpec {
    pub roletype: crate::sys::RoleSpecType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolename: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct FuncCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funcname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_order: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_filter: Option<Box<Node>>,
    pub agg_within_group: bool,
    pub agg_star: bool,
    pub agg_distinct: bool,
    pub func_variadic: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub over: Option<Box<WindowDef>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct A_Star {}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct A_Indices {
    pub is_slice: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lidx: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uidx: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct A_Indirection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indirection: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct A_ArrayExpr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ResTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indirection: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MultiAssignRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<Node>>,
    pub colno: i32,
    pub ncolumns: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct SortBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<Box<Node>>,
    pub sortby_dir: crate::sys::SortByDir,
    pub sortby_nulls: crate::sys::SortByNulls,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useOp: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct WindowDef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitionClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderClause: Option<Vec<Node>>,
    pub frameOptions: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startOffset: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endOffset: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RangeSubselect {
    pub lateral: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subquery: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Box<Alias>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RangeFunction {
    pub lateral: bool,
    pub ordinality: bool,
    pub is_rowsfrom: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Box<Alias>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coldeflist: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RangeTableFunc {
    pub lateral: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docexpr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowexpr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Box<Alias>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RangeTableFuncCol {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<Box<TypeName>>,
    pub for_ordinality: bool,
    pub is_not_null: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colexpr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coldefexpr: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RangeTableSample {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeatable: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ColumnDef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<Box<TypeName>>,
    pub inhcount: i32,
    pub is_local: bool,
    pub is_not_null: bool,
    pub is_from_type: bool,
    pub storage: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_default: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooked_default: Option<Box<Node>>,
    pub identity: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identitySequence: Option<Box<RangeVar>>,
    pub generated: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collClause: Option<Box<CollateClause>>,
    pub collOid: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fdwoptions: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct TableLikeClause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    pub options: crate::sys::bits32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct IndexElem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexcolname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collation: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opclass: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opclassopts: Option<Vec<Node>>,
    pub ordering: crate::sys::SortByDir,
    pub nulls_ordering: crate::sys::SortByNulls,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DefElem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defnamespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Node>>,
    pub defaction: crate::sys::DefElemAction,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct LockingClause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockedRels: Option<Vec<Node>>,
    pub strength: crate::sys::LockClauseStrength,
    pub waitPolicy: crate::sys::LockWaitPolicy,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct XmlSerialize {
    pub xmloption: crate::sys::XmlOptionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<Box<TypeName>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartitionElem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collation: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opclass: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartitionSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partParams: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartitionBoundSpec {
    pub strategy: char,
    pub is_default: bool,
    pub modulus: i32,
    pub remainder: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listdatums: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowerdatums: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upperdatums: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartitionRangeDatum {
    pub kind: crate::sys::PartitionRangeDatumKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartitionCmd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound: Option<Box<PartitionBoundSpec>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct TableSampleClause {
    pub tsmhandler: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeatable: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct WithCheckOption {
    pub kind: crate::sys::WCOKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qual: Option<Box<Node>>,
    pub cascaded: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct SortGroupClause {
    pub tleSortGroupRef: crate::sys::Index,
    pub eqop: crate::sys::Oid,
    pub sortop: crate::sys::Oid,
    pub nulls_first: bool,
    pub hashable: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct GroupingSet {
    pub kind: crate::sys::GroupingSetKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct WindowClause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitionClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderClause: Option<Vec<Node>>,
    pub frameOptions: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startOffset: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endOffset: Option<Box<Node>>,
    pub startInRangeFunc: crate::sys::Oid,
    pub endInRangeFunc: crate::sys::Oid,
    pub inRangeColl: crate::sys::Oid,
    pub inRangeAsc: bool,
    pub inRangeNullsFirst: bool,
    pub winref: crate::sys::Index,
    pub copiedOrder: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RowMarkClause {
    pub rti: crate::sys::Index,
    pub strength: crate::sys::LockClauseStrength,
    pub waitPolicy: crate::sys::LockWaitPolicy,
    pub pushedDown: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct WithClause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctes: Option<Vec<Node>>,
    pub recursive: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct InferClause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexElems: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whereClause: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conname: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct OnConflictClause {
    pub action: crate::sys::OnConflictAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer: Option<Box<InferClause>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetList: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whereClause: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommonTableExpr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliascolnames: Option<Vec<Node>>,
    pub ctematerialized: crate::sys::CTEMaterialize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctequery: Option<Box<Node>>,
    pub cterecursive: bool,
    pub cterefcount: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctecolnames: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctecoltypes: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctecoltypmods: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctecolcollations: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct TriggerTransition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub isNew: bool,
    pub isTable: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RawStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stmt: Option<Box<Node>>,
    pub stmt_location: i32,
    pub stmt_len: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct InsertStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cols: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectStmt: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onConflictClause: Option<Box<OnConflictClause>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returningList: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withClause: Option<Box<WithClause>>,
    pub override_: crate::sys::OverridingKind,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DeleteStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usingClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whereClause: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returningList: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withClause: Option<Box<WithClause>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct UpdateStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetList: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whereClause: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returningList: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withClause: Option<Box<WithClause>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct SelectStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinctClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intoClause: Option<Box<IntoClause>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetList: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whereClause: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub havingClause: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windowClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valuesLists: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitOffset: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitCount: Option<Box<Node>>,
    pub limitOption: crate::sys::LimitOption,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockingClause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withClause: Option<Box<WithClause>>,
    pub op: crate::sys::SetOperation,
    pub all: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub larg: Option<Box<SelectStmt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarg: Option<Box<SelectStmt>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct SetOperationStmt {
    pub op: crate::sys::SetOperation,
    pub all: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub larg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarg: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colTypes: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colTypmods: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colCollations: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupClauses: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateSchemaStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemaname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authrole: Option<Box<RoleSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemaElts: Option<Vec<Node>>,
    pub if_not_exists: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterTableStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmds: Option<Vec<Node>>,
    pub relkind: crate::sys::ObjectType,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplicaIdentityStmt {
    pub identity_type: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterTableCmd {
    pub subtype: crate::sys::AlterTableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub num: i16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newowner: Option<Box<RoleSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def: Option<Box<Node>>,
    pub behavior: crate::sys::DropBehavior,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterCollationStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collname: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterDomainStmt {
    pub subtype: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def: Option<Box<Node>>,
    pub behavior: crate::sys::DropBehavior,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct GrantStmt {
    pub is_grant: bool,
    pub targtype: crate::sys::GrantTargetType,
    pub objtype: crate::sys::ObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileges: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantees: Option<Vec<Node>>,
    pub grant_option: bool,
    pub behavior: crate::sys::DropBehavior,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectWithArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objargs: Option<Vec<Node>>,
    pub args_unspecified: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AccessPriv {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priv_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cols: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct GrantRoleStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted_roles: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_roles: Option<Vec<Node>>,
    pub is_grant: bool,
    pub admin_opt: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantor: Option<Box<RoleSpec>>,
    pub behavior: crate::sys::DropBehavior,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterDefaultPrivilegesStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<GrantStmt>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CopyStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attlist: Option<Vec<Node>>,
    pub is_from: bool,
    pub is_program: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whereClause: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct VariableSetStmt {
    pub kind: crate::sys::VariableSetKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    pub is_local: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct VariableShowStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tableElts: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inhRelations: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partbound: Option<Box<PartitionBoundSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partspec: Option<Box<PartitionSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ofTypename: Option<Box<TypeName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    pub oncommit: crate::sys::OnCommitAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tablespacename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessMethod: Option<String>,
    pub if_not_exists: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Constraint {
    pub contype: crate::sys::ConstrType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conname: Option<String>,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub is_no_inherit: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_expr: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooked_expr: Option<String>,
    pub generated_when: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub including: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexspace: Option<String>,
    pub reset_default_tblspc: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub where_clause: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pktable: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fk_attrs: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pk_attrs: Option<Vec<Node>>,
    pub fk_matchtype: char,
    pub fk_upd_action: char,
    pub fk_del_action: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_conpfeqop: Option<Vec<Node>>,
    pub old_pktable_oid: crate::sys::Oid,
    pub skip_validation: bool,
    pub initially_valid: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateTableSpaceStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tablespacename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<RoleSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DropTableSpaceStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tablespacename: Option<String>,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterTableSpaceOptionsStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tablespacename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    pub isReset: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterTableMoveAllStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_tablespacename: Option<String>,
    pub objtype: crate::sys::ObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tablespacename: Option<String>,
    pub nowait: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateExtensionStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extname: Option<String>,
    pub if_not_exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterExtensionStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterExtensionContentsStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extname: Option<String>,
    pub action: i32,
    pub objtype: crate::sys::ObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateFdwStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fdwname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub func_options: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterFdwStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fdwname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub func_options: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateForeignServerStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servertype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fdwname: Option<String>,
    pub if_not_exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterForeignServerStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    pub has_version: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateForeignTableStmt {
    pub base: CreateStmt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateUserMappingStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<RoleSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servername: Option<String>,
    pub if_not_exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterUserMappingStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<RoleSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DropUserMappingStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<RoleSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servername: Option<String>,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ImportForeignSchemaStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_schema: Option<String>,
    pub list_type: crate::sys::ImportForeignSchemaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_list: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreatePolicyStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd_name: Option<String>,
    pub permissive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qual: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_check: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterPolicyStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qual: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_check: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateAmStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler_name: Option<Vec<Node>>,
    pub amtype: char,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateTrigStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funcname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    pub row: bool,
    pub timing: i16,
    pub events: i16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whenClause: Option<Box<Node>>,
    pub isconstraint: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitionRels: Option<Vec<Node>>,
    pub deferrable: bool,
    pub initdeferred: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constrrel: Option<Box<RangeVar>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateEventTrigStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whenclause: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funcname: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterEventTrigStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigname: Option<String>,
    pub tgenabled: char,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreatePLangStmt {
    pub replace: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plhandler: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plinline: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plvalidator: Option<Vec<Node>>,
    pub pltrusted: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateRoleStmt {
    pub stmt_type: crate::sys::RoleStmtType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterRoleStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<RoleSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    pub action: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterRoleSetStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<RoleSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setstmt: Option<Box<VariableSetStmt>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DropRoleStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Node>>,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateSeqStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    pub ownerId: crate::sys::Oid,
    pub for_identity: bool,
    pub if_not_exists: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterSeqStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    pub for_identity: bool,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DefineStmt {
    pub kind: crate::sys::ObjectType,
    pub oldstyle: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defnames: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<Node>>,
    pub if_not_exists: bool,
    pub replace: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateDomainStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<Box<TypeName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collClause: Option<Box<CollateClause>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateOpClassStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opclassname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opfamilyname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datatype: Option<Box<TypeName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Node>>,
    pub isDefault: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateOpClassItem {
    pub itemtype: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<ObjectWithArgs>>,
    pub number: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_family: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_args: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storedtype: Option<Box<TypeName>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateOpFamilyStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opfamilyname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amname: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterOpFamilyStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opfamilyname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amname: Option<String>,
    pub isDrop: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DropStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<Node>>,
    pub removeType: crate::sys::ObjectType,
    pub behavior: crate::sys::DropBehavior,
    pub missing_ok: bool,
    pub concurrent: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct TruncateStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<Node>>,
    pub restart_seqs: bool,
    pub behavior: crate::sys::DropBehavior,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommentStmt {
    pub objtype: crate::sys::ObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct SecLabelStmt {
    pub objtype: crate::sys::ObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DeclareCursorStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portalname: Option<String>,
    pub options: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClosePortalStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portalname: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct FetchStmt {
    pub direction: crate::sys::FetchDirection,
    pub howMany: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portalname: Option<String>,
    pub ismove: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct IndexStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idxname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessMethod: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tableSpace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexParams: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexIncludingParams: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whereClause: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludeOpNames: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idxcomment: Option<String>,
    pub indexOid: crate::sys::Oid,
    pub oldNode: crate::sys::Oid,
    pub oldCreateSubid: crate::sys::SubTransactionId,
    pub oldFirstRelfilenodeSubid: crate::sys::SubTransactionId,
    pub unique: bool,
    pub primary: bool,
    pub isconstraint: bool,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub transformed: bool,
    pub concurrent: bool,
    pub if_not_exists: bool,
    pub reset_default_tblspc: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateStatsStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defnames: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat_types: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exprs: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stxcomment: Option<String>,
    pub if_not_exists: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterStatsStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defnames: Option<Vec<Node>>,
    pub stxstattarget: i32,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateFunctionStmt {
    pub is_procedure: bool,
    pub replace: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funcname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnType: Option<Box<TypeName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct FunctionParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argType: Option<Box<TypeName>>,
    pub mode: crate::sys::FunctionParameterMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defexpr: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterFunctionStmt {
    pub objtype: crate::sys::ObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub func: Option<Box<ObjectWithArgs>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DoStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct InlineCodeBlock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_text: Option<String>,
    pub langOid: crate::sys::Oid,
    pub langIsTrusted: bool,
    pub atomic: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CallStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funccall: Option<Box<FuncCall>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CallContext {
    pub atomic: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RenameStmt {
    pub renameType: crate::sys::ObjectType,
    pub relationType: crate::sys::ObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newname: Option<String>,
    pub behavior: crate::sys::DropBehavior,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterObjectDependsStmt {
    pub objectType: crate::sys::ObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extname: Option<Box<Value>>,
    pub remove: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterObjectSchemaStmt {
    pub objectType: crate::sys::ObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newschema: Option<String>,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterOwnerStmt {
    pub objectType: crate::sys::ObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newowner: Option<Box<RoleSpec>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterOperatorStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opername: Option<Box<ObjectWithArgs>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterTypeStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RuleStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rulename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whereClause: Option<Box<Node>>,
    pub event: crate::sys::CmdType,
    pub instead: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Node>>,
    pub replace: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct NotifyStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditionname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ListenStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditionname: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct UnlistenStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditionname: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct TransactionStmt {
    pub kind: crate::sys::TransactionStmtKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savepoint_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    pub chain: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CompositeTypeStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typevar: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coldeflist: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateEnumStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vals: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateRangeStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterEnumStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldVal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newVal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newValNeighbor: Option<String>,
    pub newValIsAfter: bool,
    pub skipIfNewValExists: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ViewStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<Node>>,
    pub replace: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    pub withCheckOption: crate::sys::ViewCheckOption,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct LoadStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreatedbStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterDatabaseStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterDatabaseSetStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setstmt: Option<Box<VariableSetStmt>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DropdbStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbname: Option<String>,
    pub missing_ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterSystemStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setstmt: Option<Box<VariableSetStmt>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClusterStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexname: Option<String>,
    pub options: i32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct VacuumStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rels: Option<Vec<Node>>,
    pub is_vacuumcmd: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct VacuumRelation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    pub oid: crate::sys::Oid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub va_cols: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ExplainStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateTableAsStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub into: Option<Box<IntoClause>>,
    pub relkind: crate::sys::ObjectType,
    pub is_select_into: bool,
    pub if_not_exists: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RefreshMatViewStmt {
    pub concurrent: bool,
    pub skipData: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CheckPointStmt {}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DiscardStmt {
    pub target: crate::sys::DiscardMode,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct LockStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<Node>>,
    pub mode: i32,
    pub nowait: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConstraintsSetStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<Node>>,
    pub deferred: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReindexStmt {
    pub kind: crate::sys::ReindexObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Box<RangeVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub options: i32,
    pub concurrent: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateConversionStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_name: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_encoding_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_encoding_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub func_name: Option<Vec<Node>>,
    pub def: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateCastStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourcetype: Option<Box<TypeName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targettype: Option<Box<TypeName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub func: Option<Box<ObjectWithArgs>>,
    pub context: crate::sys::CoercionContext,
    pub inout: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateTransformStmt {
    pub replace: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<Box<TypeName>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromsql: Option<Box<ObjectWithArgs>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tosql: Option<Box<ObjectWithArgs>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct PrepareStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argtypes: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ExecuteStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DeallocateStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DropOwnedStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Node>>,
    pub behavior: crate::sys::DropBehavior,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReassignOwnedStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newrole: Option<Box<RoleSpec>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterTSDictionaryStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dictname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterTSConfigurationStmt {
    pub kind: crate::sys::AlterTSConfigType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfgname: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokentype: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dicts: Option<Vec<Node>>,
    pub override_: bool,
    pub replace: bool,
    pub missing_ok: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreatePublicationStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<Node>>,
    pub for_all_tables: bool,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterPublicationStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<Node>>,
    pub for_all_tables: bool,
    pub tableAction: crate::sys::DefElemAction,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CreateSubscriptionStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conninfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AlterSubscriptionStmt {
    pub kind: crate::sys::AlterSubscriptionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conninfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Node>>,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DropSubscriptionStmt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subname: Option<String>,
    pub missing_ok: bool,
    pub behavior: crate::sys::DropBehavior,
}
impl crate::convert::ConvertNode for crate::sys::Node {
    fn convert(&self) -> crate::nodes::Node {
        match self.type_ {
            crate::sys::NodeTag::T_A_ArrayExpr => {
                let ptr = self as *const _ as *const crate::sys::A_ArrayExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_A_Const => {
                let ptr = self as *const _ as *const crate::sys::A_Const;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_A_Expr => {
                let ptr = self as *const _ as *const crate::sys::A_Expr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_A_Indices => {
                let ptr = self as *const _ as *const crate::sys::A_Indices;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_A_Indirection => {
                let ptr = self as *const _ as *const crate::sys::A_Indirection;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_A_Star => {
                let ptr = self as *const _ as *const crate::sys::A_Star;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AccessPriv => {
                let ptr = self as *const _ as *const crate::sys::AccessPriv;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_Aggref => {
                let ptr = self as *const _ as *const crate::sys::Aggref;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_Alias => {
                let ptr = self as *const _ as *const crate::sys::Alias;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterCollationStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterCollationStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterDatabaseSetStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterDatabaseSetStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterDatabaseStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterDatabaseStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterDefaultPrivilegesStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterDefaultPrivilegesStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterDomainStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterDomainStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterEnumStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterEnumStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterEventTrigStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterEventTrigStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterExtensionContentsStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterExtensionContentsStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterExtensionStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterExtensionStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterFdwStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterFdwStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterForeignServerStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterForeignServerStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterFunctionStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterFunctionStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterObjectDependsStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterObjectDependsStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterObjectSchemaStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterObjectSchemaStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterOpFamilyStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterOpFamilyStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterOperatorStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterOperatorStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterOwnerStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterOwnerStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterPolicyStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterPolicyStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterPublicationStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterPublicationStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterRoleSetStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterRoleSetStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterRoleStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterRoleStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterSeqStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterSeqStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterStatsStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterStatsStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterSubscriptionStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterSubscriptionStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterSystemStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterSystemStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterTSConfigurationStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterTSConfigurationStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterTSDictionaryStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterTSDictionaryStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterTableCmd => {
                let ptr = self as *const _ as *const crate::sys::AlterTableCmd;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterTableMoveAllStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterTableMoveAllStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterTableSpaceOptionsStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterTableSpaceOptionsStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterTableStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterTableStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterTypeStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterTypeStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlterUserMappingStmt => {
                let ptr = self as *const _ as *const crate::sys::AlterUserMappingStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_AlternativeSubPlan => {
                let ptr = self as *const _ as *const crate::sys::AlternativeSubPlan;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ArrayCoerceExpr => {
                let ptr = self as *const _ as *const crate::sys::ArrayCoerceExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ArrayExpr => {
                let ptr = self as *const _ as *const crate::sys::ArrayExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_BoolExpr => {
                let ptr = self as *const _ as *const crate::sys::BoolExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_BooleanTest => {
                let ptr = self as *const _ as *const crate::sys::BooleanTest;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CallContext => {
                let ptr = self as *const _ as *const crate::sys::CallContext;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CallStmt => {
                let ptr = self as *const _ as *const crate::sys::CallStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CaseExpr => {
                let ptr = self as *const _ as *const crate::sys::CaseExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CaseTestExpr => {
                let ptr = self as *const _ as *const crate::sys::CaseTestExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CaseWhen => {
                let ptr = self as *const _ as *const crate::sys::CaseWhen;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CheckPointStmt => {
                let ptr = self as *const _ as *const crate::sys::CheckPointStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ClosePortalStmt => {
                let ptr = self as *const _ as *const crate::sys::ClosePortalStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ClusterStmt => {
                let ptr = self as *const _ as *const crate::sys::ClusterStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CoalesceExpr => {
                let ptr = self as *const _ as *const crate::sys::CoalesceExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CoerceToDomain => {
                let ptr = self as *const _ as *const crate::sys::CoerceToDomain;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CoerceToDomainValue => {
                let ptr = self as *const _ as *const crate::sys::CoerceToDomainValue;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CoerceViaIO => {
                let ptr = self as *const _ as *const crate::sys::CoerceViaIO;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CollateClause => {
                let ptr = self as *const _ as *const crate::sys::CollateClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CollateExpr => {
                let ptr = self as *const _ as *const crate::sys::CollateExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ColumnDef => {
                let ptr = self as *const _ as *const crate::sys::ColumnDef;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ColumnRef => {
                let ptr = self as *const _ as *const crate::sys::ColumnRef;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CommentStmt => {
                let ptr = self as *const _ as *const crate::sys::CommentStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CommonTableExpr => {
                let ptr = self as *const _ as *const crate::sys::CommonTableExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CompositeTypeStmt => {
                let ptr = self as *const _ as *const crate::sys::CompositeTypeStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_Const => {
                let ptr = self as *const _ as *const crate::sys::Const;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_Constraint => {
                let ptr = self as *const _ as *const crate::sys::Constraint;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ConstraintsSetStmt => {
                let ptr = self as *const _ as *const crate::sys::ConstraintsSetStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ConvertRowtypeExpr => {
                let ptr = self as *const _ as *const crate::sys::ConvertRowtypeExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CopyStmt => {
                let ptr = self as *const _ as *const crate::sys::CopyStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateAmStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateAmStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateCastStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateCastStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateConversionStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateConversionStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateDomainStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateDomainStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateEnumStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateEnumStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateEventTrigStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateEventTrigStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateExtensionStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateExtensionStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateFdwStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateFdwStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateForeignServerStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateForeignServerStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateForeignTableStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateForeignTableStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateFunctionStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateFunctionStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateOpClassItem => {
                let ptr = self as *const _ as *const crate::sys::CreateOpClassItem;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateOpClassStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateOpClassStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateOpFamilyStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateOpFamilyStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreatePLangStmt => {
                let ptr = self as *const _ as *const crate::sys::CreatePLangStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreatePolicyStmt => {
                let ptr = self as *const _ as *const crate::sys::CreatePolicyStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreatePublicationStmt => {
                let ptr = self as *const _ as *const crate::sys::CreatePublicationStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateRangeStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateRangeStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateRoleStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateRoleStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateSchemaStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateSchemaStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateSeqStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateSeqStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateStatsStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateStatsStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateSubscriptionStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateSubscriptionStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateTableAsStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateTableAsStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateTableSpaceStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateTableSpaceStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateTransformStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateTransformStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateTrigStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateTrigStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreateUserMappingStmt => {
                let ptr = self as *const _ as *const crate::sys::CreateUserMappingStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CreatedbStmt => {
                let ptr = self as *const _ as *const crate::sys::CreatedbStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_CurrentOfExpr => {
                let ptr = self as *const _ as *const crate::sys::CurrentOfExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DeallocateStmt => {
                let ptr = self as *const _ as *const crate::sys::DeallocateStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DeclareCursorStmt => {
                let ptr = self as *const _ as *const crate::sys::DeclareCursorStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DefElem => {
                let ptr = self as *const _ as *const crate::sys::DefElem;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DefineStmt => {
                let ptr = self as *const _ as *const crate::sys::DefineStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DeleteStmt => {
                let ptr = self as *const _ as *const crate::sys::DeleteStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DiscardStmt => {
                let ptr = self as *const _ as *const crate::sys::DiscardStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DoStmt => {
                let ptr = self as *const _ as *const crate::sys::DoStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DropOwnedStmt => {
                let ptr = self as *const _ as *const crate::sys::DropOwnedStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DropRoleStmt => {
                let ptr = self as *const _ as *const crate::sys::DropRoleStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DropStmt => {
                let ptr = self as *const _ as *const crate::sys::DropStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DropSubscriptionStmt => {
                let ptr = self as *const _ as *const crate::sys::DropSubscriptionStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DropTableSpaceStmt => {
                let ptr = self as *const _ as *const crate::sys::DropTableSpaceStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DropUserMappingStmt => {
                let ptr = self as *const _ as *const crate::sys::DropUserMappingStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_DropdbStmt => {
                let ptr = self as *const _ as *const crate::sys::DropdbStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ExecuteStmt => {
                let ptr = self as *const _ as *const crate::sys::ExecuteStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ExplainStmt => {
                let ptr = self as *const _ as *const crate::sys::ExplainStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_Expr => {
                let ptr = self as *const _ as *const crate::sys::Expr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_FetchStmt => {
                let ptr = self as *const _ as *const crate::sys::FetchStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_FieldSelect => {
                let ptr = self as *const _ as *const crate::sys::FieldSelect;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_FieldStore => {
                let ptr = self as *const _ as *const crate::sys::FieldStore;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_FromExpr => {
                let ptr = self as *const _ as *const crate::sys::FromExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_FuncCall => {
                let ptr = self as *const _ as *const crate::sys::FuncCall;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_FuncExpr => {
                let ptr = self as *const _ as *const crate::sys::FuncExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_FunctionParameter => {
                let ptr = self as *const _ as *const crate::sys::FunctionParameter;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_GrantRoleStmt => {
                let ptr = self as *const _ as *const crate::sys::GrantRoleStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_GrantStmt => {
                let ptr = self as *const _ as *const crate::sys::GrantStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_GroupingFunc => {
                let ptr = self as *const _ as *const crate::sys::GroupingFunc;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_GroupingSet => {
                let ptr = self as *const _ as *const crate::sys::GroupingSet;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ImportForeignSchemaStmt => {
                let ptr = self as *const _ as *const crate::sys::ImportForeignSchemaStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_IndexElem => {
                let ptr = self as *const _ as *const crate::sys::IndexElem;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_IndexStmt => {
                let ptr = self as *const _ as *const crate::sys::IndexStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_InferClause => {
                let ptr = self as *const _ as *const crate::sys::InferClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_InferenceElem => {
                let ptr = self as *const _ as *const crate::sys::InferenceElem;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_InlineCodeBlock => {
                let ptr = self as *const _ as *const crate::sys::InlineCodeBlock;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_InsertStmt => {
                let ptr = self as *const _ as *const crate::sys::InsertStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_IntoClause => {
                let ptr = self as *const _ as *const crate::sys::IntoClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_JoinExpr => {
                let ptr = self as *const _ as *const crate::sys::JoinExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_List => {
                let ptr = self as *const _ as *const crate::sys::List;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ListenStmt => {
                let ptr = self as *const _ as *const crate::sys::ListenStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_LoadStmt => {
                let ptr = self as *const _ as *const crate::sys::LoadStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_LockStmt => {
                let ptr = self as *const _ as *const crate::sys::LockStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_LockingClause => {
                let ptr = self as *const _ as *const crate::sys::LockingClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_MinMaxExpr => {
                let ptr = self as *const _ as *const crate::sys::MinMaxExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_MultiAssignRef => {
                let ptr = self as *const _ as *const crate::sys::MultiAssignRef;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_NamedArgExpr => {
                let ptr = self as *const _ as *const crate::sys::NamedArgExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_NextValueExpr => {
                let ptr = self as *const _ as *const crate::sys::NextValueExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_NotifyStmt => {
                let ptr = self as *const _ as *const crate::sys::NotifyStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_NullTest => {
                let ptr = self as *const _ as *const crate::sys::NullTest;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ObjectWithArgs => {
                let ptr = self as *const _ as *const crate::sys::ObjectWithArgs;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_OnConflictClause => {
                let ptr = self as *const _ as *const crate::sys::OnConflictClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_OnConflictExpr => {
                let ptr = self as *const _ as *const crate::sys::OnConflictExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_OpExpr => {
                let ptr = self as *const _ as *const crate::sys::OpExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_Param => {
                let ptr = self as *const _ as *const crate::sys::Param;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ParamRef => {
                let ptr = self as *const _ as *const crate::sys::ParamRef;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_PartitionBoundSpec => {
                let ptr = self as *const _ as *const crate::sys::PartitionBoundSpec;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_PartitionCmd => {
                let ptr = self as *const _ as *const crate::sys::PartitionCmd;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_PartitionElem => {
                let ptr = self as *const _ as *const crate::sys::PartitionElem;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_PartitionRangeDatum => {
                let ptr = self as *const _ as *const crate::sys::PartitionRangeDatum;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_PartitionSpec => {
                let ptr = self as *const _ as *const crate::sys::PartitionSpec;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_PrepareStmt => {
                let ptr = self as *const _ as *const crate::sys::PrepareStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RangeFunction => {
                let ptr = self as *const _ as *const crate::sys::RangeFunction;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RangeSubselect => {
                let ptr = self as *const _ as *const crate::sys::RangeSubselect;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RangeTableFunc => {
                let ptr = self as *const _ as *const crate::sys::RangeTableFunc;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RangeTableFuncCol => {
                let ptr = self as *const _ as *const crate::sys::RangeTableFuncCol;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RangeTableSample => {
                let ptr = self as *const _ as *const crate::sys::RangeTableSample;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RangeTblRef => {
                let ptr = self as *const _ as *const crate::sys::RangeTblRef;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RangeVar => {
                let ptr = self as *const _ as *const crate::sys::RangeVar;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RawStmt => {
                let ptr = self as *const _ as *const crate::sys::RawStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ReassignOwnedStmt => {
                let ptr = self as *const _ as *const crate::sys::ReassignOwnedStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RefreshMatViewStmt => {
                let ptr = self as *const _ as *const crate::sys::RefreshMatViewStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ReindexStmt => {
                let ptr = self as *const _ as *const crate::sys::ReindexStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RelabelType => {
                let ptr = self as *const _ as *const crate::sys::RelabelType;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RenameStmt => {
                let ptr = self as *const _ as *const crate::sys::RenameStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ReplicaIdentityStmt => {
                let ptr = self as *const _ as *const crate::sys::ReplicaIdentityStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ResTarget => {
                let ptr = self as *const _ as *const crate::sys::ResTarget;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RoleSpec => {
                let ptr = self as *const _ as *const crate::sys::RoleSpec;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RowCompareExpr => {
                let ptr = self as *const _ as *const crate::sys::RowCompareExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RowExpr => {
                let ptr = self as *const _ as *const crate::sys::RowExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RowMarkClause => {
                let ptr = self as *const _ as *const crate::sys::RowMarkClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_RuleStmt => {
                let ptr = self as *const _ as *const crate::sys::RuleStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_SQLValueFunction => {
                let ptr = self as *const _ as *const crate::sys::SQLValueFunction;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ScalarArrayOpExpr => {
                let ptr = self as *const _ as *const crate::sys::ScalarArrayOpExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_SecLabelStmt => {
                let ptr = self as *const _ as *const crate::sys::SecLabelStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_SelectStmt => {
                let ptr = self as *const _ as *const crate::sys::SelectStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_SetOperationStmt => {
                let ptr = self as *const _ as *const crate::sys::SetOperationStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_SetToDefault => {
                let ptr = self as *const _ as *const crate::sys::SetToDefault;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_SortBy => {
                let ptr = self as *const _ as *const crate::sys::SortBy;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_SortGroupClause => {
                let ptr = self as *const _ as *const crate::sys::SortGroupClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_SubLink => {
                let ptr = self as *const _ as *const crate::sys::SubLink;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_SubscriptingRef => {
                let ptr = self as *const _ as *const crate::sys::SubscriptingRef;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_TableFunc => {
                let ptr = self as *const _ as *const crate::sys::TableFunc;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_TableLikeClause => {
                let ptr = self as *const _ as *const crate::sys::TableLikeClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_TableSampleClause => {
                let ptr = self as *const _ as *const crate::sys::TableSampleClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_TargetEntry => {
                let ptr = self as *const _ as *const crate::sys::TargetEntry;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_TransactionStmt => {
                let ptr = self as *const _ as *const crate::sys::TransactionStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_TriggerTransition => {
                let ptr = self as *const _ as *const crate::sys::TriggerTransition;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_TruncateStmt => {
                let ptr = self as *const _ as *const crate::sys::TruncateStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_TypeCast => {
                let ptr = self as *const _ as *const crate::sys::TypeCast;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_TypeName => {
                let ptr = self as *const _ as *const crate::sys::TypeName;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_UnlistenStmt => {
                let ptr = self as *const _ as *const crate::sys::UnlistenStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_UpdateStmt => {
                let ptr = self as *const _ as *const crate::sys::UpdateStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_VacuumRelation => {
                let ptr = self as *const _ as *const crate::sys::VacuumRelation;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_VacuumStmt => {
                let ptr = self as *const _ as *const crate::sys::VacuumStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_Value => {
                let ptr = self as *const _ as *const crate::sys::Value;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_Var => {
                let ptr = self as *const _ as *const crate::sys::Var;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_VariableSetStmt => {
                let ptr = self as *const _ as *const crate::sys::VariableSetStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_VariableShowStmt => {
                let ptr = self as *const _ as *const crate::sys::VariableShowStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_ViewStmt => {
                let ptr = self as *const _ as *const crate::sys::ViewStmt;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_WindowClause => {
                let ptr = self as *const _ as *const crate::sys::WindowClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_WindowDef => {
                let ptr = self as *const _ as *const crate::sys::WindowDef;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_WindowFunc => {
                let ptr = self as *const _ as *const crate::sys::WindowFunc;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_WithCheckOption => {
                let ptr = self as *const _ as *const crate::sys::WithCheckOption;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_WithClause => {
                let ptr = self as *const _ as *const crate::sys::WithClause;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_XmlExpr => {
                let ptr = self as *const _ as *const crate::sys::XmlExpr;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_XmlSerialize => {
                let ptr = self as *const _ as *const crate::sys::XmlSerialize;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            crate::sys::NodeTag::T_String
            | crate::sys::NodeTag::T_Integer
            | crate::sys::NodeTag::T_Float
            | crate::sys::NodeTag::T_BitString
            | crate::sys::NodeTag::T_Null => {
                let ptr = self as *const _ as *const crate::sys::Value;
                unsafe { ptr.as_ref().unwrap().convert() }
            }
            _ => panic!("Unrecognized NodeTag: {:?}", self.type_),
        }
    }
}
impl crate::convert::ConvertNode for crate::sys::Alias {
    fn convert(&self) -> crate::nodes::Node {
        Node::Alias(Alias {
            aliasname: if self.aliasname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.aliasname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            colnames: if self.colnames.is_null() {
                None
            } else {
                match unsafe { self.colnames.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RangeVar {
    fn convert(&self) -> crate::nodes::Node {
        Node::RangeVar(RangeVar {
            catalogname: if self.catalogname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.catalogname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            schemaname: if self.schemaname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.schemaname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            relname: if self.relname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.relname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            inh: self.inh as bool,
            relpersistence: self.relpersistence as u8 as char,
            alias: if self.alias.is_null() {
                None
            } else {
                match unsafe { self.alias.as_ref().unwrap().convert() } {
                    crate::nodes::Node::Alias(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(alias),
                        stringify!(Alias)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::TableFunc {
    fn convert(&self) -> crate::nodes::Node {
        Node::TableFunc(TableFunc {
            ns_uris: if self.ns_uris.is_null() {
                None
            } else {
                match unsafe { self.ns_uris.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            ns_names: if self.ns_names.is_null() {
                None
            } else {
                match unsafe { self.ns_names.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            docexpr: if self.docexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.docexpr.as_ref().unwrap().convert()
                }))
            },
            rowexpr: if self.rowexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.rowexpr.as_ref().unwrap().convert()
                }))
            },
            colnames: if self.colnames.is_null() {
                None
            } else {
                match unsafe { self.colnames.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            coltypes: if self.coltypes.is_null() {
                None
            } else {
                match unsafe { self.coltypes.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            coltypmods: if self.coltypmods.is_null() {
                None
            } else {
                match unsafe { self.coltypmods.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            colcollations: if self.colcollations.is_null() {
                None
            } else {
                match unsafe { self.colcollations.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            colexprs: if self.colexprs.is_null() {
                None
            } else {
                match unsafe { self.colexprs.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            coldefexprs: if self.coldefexprs.is_null() {
                None
            } else {
                match unsafe { self.coldefexprs.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            ordinalitycol: self.ordinalitycol as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::IntoClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::IntoClause(IntoClause {
            rel: if self.rel.is_null() {
                None
            } else {
                match unsafe { self.rel.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(rel),
                        stringify!(RangeVar)
                    ),
                }
            },
            colNames: if self.colNames.is_null() {
                None
            } else {
                match unsafe { self.colNames.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            accessMethod: if self.accessMethod.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.accessMethod)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            onCommit: self.onCommit as crate::sys::OnCommitAction,
            tableSpaceName: if self.tableSpaceName.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.tableSpaceName)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            viewQuery: if self.viewQuery.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.viewQuery.as_ref().unwrap().convert()
                }))
            },
            skipData: self.skipData as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::Var {
    fn convert(&self) -> crate::nodes::Node {
        Node::Var(Var {
            varno: self.varno as crate::sys::Index,
            varattno: self.varattno as crate::sys::AttrNumber,
            vartype: self.vartype as crate::sys::Oid,
            vartypmod: self.vartypmod as i32,
            varcollid: self.varcollid as crate::sys::Oid,
            varlevelsup: self.varlevelsup as crate::sys::Index,
            varnosyn: self.varnosyn as crate::sys::Index,
            varattnosyn: self.varattnosyn as crate::sys::AttrNumber,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::Const {
    fn convert(&self) -> crate::nodes::Node {
        Node::Const(Const {
            consttype: self.consttype as crate::sys::Oid,
            consttypmod: self.consttypmod as i32,
            constcollid: self.constcollid as crate::sys::Oid,
            constlen: self.constlen as i32,
            constvalue: self.constvalue as crate::sys::Datum,
            constisnull: self.constisnull as bool,
            constbyval: self.constbyval as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::Param {
    fn convert(&self) -> crate::nodes::Node {
        Node::Param(Param {
            paramkind: self.paramkind as crate::sys::ParamKind,
            paramid: self.paramid as i32,
            paramtype: self.paramtype as crate::sys::Oid,
            paramtypmod: self.paramtypmod as i32,
            paramcollid: self.paramcollid as crate::sys::Oid,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::Aggref {
    fn convert(&self) -> crate::nodes::Node {
        Node::Aggref(Aggref {
            aggfnoid: self.aggfnoid as crate::sys::Oid,
            aggtype: self.aggtype as crate::sys::Oid,
            aggcollid: self.aggcollid as crate::sys::Oid,
            inputcollid: self.inputcollid as crate::sys::Oid,
            aggtranstype: self.aggtranstype as crate::sys::Oid,
            aggargtypes: if self.aggargtypes.is_null() {
                None
            } else {
                match unsafe { self.aggargtypes.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            aggdirectargs: if self.aggdirectargs.is_null() {
                None
            } else {
                match unsafe { self.aggdirectargs.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            aggorder: if self.aggorder.is_null() {
                None
            } else {
                match unsafe { self.aggorder.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            aggdistinct: if self.aggdistinct.is_null() {
                None
            } else {
                match unsafe { self.aggdistinct.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            aggfilter: if self.aggfilter.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.aggfilter.as_ref().unwrap().convert()
                }))
            },
            aggstar: self.aggstar as bool,
            aggvariadic: self.aggvariadic as bool,
            aggkind: self.aggkind as u8 as char,
            agglevelsup: self.agglevelsup as crate::sys::Index,
            aggsplit: self.aggsplit as crate::sys::AggSplit,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::GroupingFunc {
    fn convert(&self) -> crate::nodes::Node {
        Node::GroupingFunc(GroupingFunc {
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            refs: if self.refs.is_null() {
                None
            } else {
                match unsafe { self.refs.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            cols: if self.cols.is_null() {
                None
            } else {
                match unsafe { self.cols.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            agglevelsup: self.agglevelsup as crate::sys::Index,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::WindowFunc {
    fn convert(&self) -> crate::nodes::Node {
        Node::WindowFunc(WindowFunc {
            winfnoid: self.winfnoid as crate::sys::Oid,
            wintype: self.wintype as crate::sys::Oid,
            wincollid: self.wincollid as crate::sys::Oid,
            inputcollid: self.inputcollid as crate::sys::Oid,
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            aggfilter: if self.aggfilter.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.aggfilter.as_ref().unwrap().convert()
                }))
            },
            winref: self.winref as crate::sys::Index,
            winstar: self.winstar as bool,
            winagg: self.winagg as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::SubscriptingRef {
    fn convert(&self) -> crate::nodes::Node {
        Node::SubscriptingRef(SubscriptingRef {
            refcontainertype: self.refcontainertype as crate::sys::Oid,
            refelemtype: self.refelemtype as crate::sys::Oid,
            reftypmod: self.reftypmod as i32,
            refcollid: self.refcollid as crate::sys::Oid,
            refupperindexpr: if self.refupperindexpr.is_null() {
                None
            } else {
                match unsafe { self.refupperindexpr.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            reflowerindexpr: if self.reflowerindexpr.is_null() {
                None
            } else {
                match unsafe { self.reflowerindexpr.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            refexpr: if self.refexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.refexpr.as_ref().unwrap().convert()
                }))
            },
            refassgnexpr: if self.refassgnexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.refassgnexpr.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::FuncExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::FuncExpr(FuncExpr {
            funcid: self.funcid as crate::sys::Oid,
            funcresulttype: self.funcresulttype as crate::sys::Oid,
            funcretset: self.funcretset as bool,
            funcvariadic: self.funcvariadic as bool,
            funcformat: self.funcformat as crate::sys::CoercionForm,
            funccollid: self.funccollid as crate::sys::Oid,
            inputcollid: self.inputcollid as crate::sys::Oid,
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::NamedArgExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::NamedArgExpr(NamedArgExpr {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            argnumber: self.argnumber as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::OpExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::OpExpr(OpExpr {
            opno: self.opno as crate::sys::Oid,
            opfuncid: self.opfuncid as crate::sys::Oid,
            opresulttype: self.opresulttype as crate::sys::Oid,
            opretset: self.opretset as bool,
            opcollid: self.opcollid as crate::sys::Oid,
            inputcollid: self.inputcollid as crate::sys::Oid,
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ScalarArrayOpExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::ScalarArrayOpExpr(ScalarArrayOpExpr {
            opno: self.opno as crate::sys::Oid,
            opfuncid: self.opfuncid as crate::sys::Oid,
            useOr: self.useOr as bool,
            inputcollid: self.inputcollid as crate::sys::Oid,
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::BoolExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::BoolExpr(BoolExpr {
            boolop: self.boolop as crate::sys::BoolExprType,
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::SubLink {
    fn convert(&self) -> crate::nodes::Node {
        Node::SubLink(SubLink {
            subLinkType: self.subLinkType as crate::sys::SubLinkType,
            subLinkId: self.subLinkId as i32,
            testexpr: if self.testexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.testexpr.as_ref().unwrap().convert()
                }))
            },
            operName: if self.operName.is_null() {
                None
            } else {
                match unsafe { self.operName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            subselect: if self.subselect.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.subselect.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlternativeSubPlan {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlternativeSubPlan(AlternativeSubPlan {
            subplans: if self.subplans.is_null() {
                None
            } else {
                match unsafe { self.subplans.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::FieldSelect {
    fn convert(&self) -> crate::nodes::Node {
        Node::FieldSelect(FieldSelect {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            fieldnum: self.fieldnum as crate::sys::AttrNumber,
            resulttype: self.resulttype as crate::sys::Oid,
            resulttypmod: self.resulttypmod as i32,
            resultcollid: self.resultcollid as crate::sys::Oid,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::FieldStore {
    fn convert(&self) -> crate::nodes::Node {
        Node::FieldStore(FieldStore {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            newvals: if self.newvals.is_null() {
                None
            } else {
                match unsafe { self.newvals.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            fieldnums: if self.fieldnums.is_null() {
                None
            } else {
                match unsafe { self.fieldnums.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            resulttype: self.resulttype as crate::sys::Oid,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RelabelType {
    fn convert(&self) -> crate::nodes::Node {
        Node::RelabelType(RelabelType {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            resulttype: self.resulttype as crate::sys::Oid,
            resulttypmod: self.resulttypmod as i32,
            resultcollid: self.resultcollid as crate::sys::Oid,
            relabelformat: self.relabelformat as crate::sys::CoercionForm,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CoerceViaIO {
    fn convert(&self) -> crate::nodes::Node {
        Node::CoerceViaIO(CoerceViaIO {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            resulttype: self.resulttype as crate::sys::Oid,
            resultcollid: self.resultcollid as crate::sys::Oid,
            coerceformat: self.coerceformat as crate::sys::CoercionForm,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ArrayCoerceExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::ArrayCoerceExpr(ArrayCoerceExpr {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            elemexpr: if self.elemexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.elemexpr.as_ref().unwrap().convert()
                }))
            },
            resulttype: self.resulttype as crate::sys::Oid,
            resulttypmod: self.resulttypmod as i32,
            resultcollid: self.resultcollid as crate::sys::Oid,
            coerceformat: self.coerceformat as crate::sys::CoercionForm,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ConvertRowtypeExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::ConvertRowtypeExpr(ConvertRowtypeExpr {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            resulttype: self.resulttype as crate::sys::Oid,
            convertformat: self.convertformat as crate::sys::CoercionForm,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CollateExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::CollateExpr(CollateExpr {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            collOid: self.collOid as crate::sys::Oid,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CaseExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::CaseExpr(CaseExpr {
            casetype: self.casetype as crate::sys::Oid,
            casecollid: self.casecollid as crate::sys::Oid,
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            defresult: if self.defresult.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.defresult.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CaseWhen {
    fn convert(&self) -> crate::nodes::Node {
        Node::CaseWhen(CaseWhen {
            expr: if self.expr.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.expr.as_ref().unwrap().convert() }))
            },
            result: if self.result.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.result.as_ref().unwrap().convert() }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CaseTestExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::CaseTestExpr(CaseTestExpr {
            typeId: self.typeId as crate::sys::Oid,
            typeMod: self.typeMod as i32,
            collation: self.collation as crate::sys::Oid,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ArrayExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::ArrayExpr(ArrayExpr {
            array_typeid: self.array_typeid as crate::sys::Oid,
            array_collid: self.array_collid as crate::sys::Oid,
            element_typeid: self.element_typeid as crate::sys::Oid,
            elements: if self.elements.is_null() {
                None
            } else {
                match unsafe { self.elements.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            multidims: self.multidims as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RowExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::RowExpr(RowExpr {
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            row_typeid: self.row_typeid as crate::sys::Oid,
            row_format: self.row_format as crate::sys::CoercionForm,
            colnames: if self.colnames.is_null() {
                None
            } else {
                match unsafe { self.colnames.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RowCompareExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::RowCompareExpr(RowCompareExpr {
            rctype: self.rctype as crate::sys::RowCompareType,
            opnos: if self.opnos.is_null() {
                None
            } else {
                match unsafe { self.opnos.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            opfamilies: if self.opfamilies.is_null() {
                None
            } else {
                match unsafe { self.opfamilies.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            inputcollids: if self.inputcollids.is_null() {
                None
            } else {
                match unsafe { self.inputcollids.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            largs: if self.largs.is_null() {
                None
            } else {
                match unsafe { self.largs.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            rargs: if self.rargs.is_null() {
                None
            } else {
                match unsafe { self.rargs.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CoalesceExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::CoalesceExpr(CoalesceExpr {
            coalescetype: self.coalescetype as crate::sys::Oid,
            coalescecollid: self.coalescecollid as crate::sys::Oid,
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::MinMaxExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::MinMaxExpr(MinMaxExpr {
            minmaxtype: self.minmaxtype as crate::sys::Oid,
            minmaxcollid: self.minmaxcollid as crate::sys::Oid,
            inputcollid: self.inputcollid as crate::sys::Oid,
            op: self.op as crate::sys::MinMaxOp,
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::SQLValueFunction {
    fn convert(&self) -> crate::nodes::Node {
        Node::SQLValueFunction(SQLValueFunction {
            op: self.op as crate::sys::SQLValueFunctionOp,
            typmod: self.typmod as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::XmlExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::XmlExpr(XmlExpr {
            op: self.op as crate::sys::XmlExprOp,
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            named_args: if self.named_args.is_null() {
                None
            } else {
                match unsafe { self.named_args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            arg_names: if self.arg_names.is_null() {
                None
            } else {
                match unsafe { self.arg_names.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            xmloption: self.xmloption as crate::sys::XmlOptionType,
            typmod: self.typmod as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::NullTest {
    fn convert(&self) -> crate::nodes::Node {
        Node::NullTest(NullTest {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            nulltesttype: self.nulltesttype as crate::sys::NullTestType,
            argisrow: self.argisrow as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::BooleanTest {
    fn convert(&self) -> crate::nodes::Node {
        Node::BooleanTest(BooleanTest {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            booltesttype: self.booltesttype as crate::sys::BoolTestType,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CoerceToDomain {
    fn convert(&self) -> crate::nodes::Node {
        Node::CoerceToDomain(CoerceToDomain {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            resulttype: self.resulttype as crate::sys::Oid,
            resulttypmod: self.resulttypmod as i32,
            resultcollid: self.resultcollid as crate::sys::Oid,
            coercionformat: self.coercionformat as crate::sys::CoercionForm,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CoerceToDomainValue {
    fn convert(&self) -> crate::nodes::Node {
        Node::CoerceToDomainValue(CoerceToDomainValue {
            typeId: self.typeId as crate::sys::Oid,
            typeMod: self.typeMod as i32,
            collation: self.collation as crate::sys::Oid,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::SetToDefault {
    fn convert(&self) -> crate::nodes::Node {
        Node::SetToDefault(SetToDefault {
            typeId: self.typeId as crate::sys::Oid,
            typeMod: self.typeMod as i32,
            collation: self.collation as crate::sys::Oid,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CurrentOfExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::CurrentOfExpr(CurrentOfExpr {
            cvarno: self.cvarno as crate::sys::Index,
            cursor_name: if self.cursor_name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.cursor_name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            cursor_param: self.cursor_param as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::NextValueExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::NextValueExpr(NextValueExpr {
            seqid: self.seqid as crate::sys::Oid,
            typeId: self.typeId as crate::sys::Oid,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::InferenceElem {
    fn convert(&self) -> crate::nodes::Node {
        Node::InferenceElem(InferenceElem {
            expr: if self.expr.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.expr.as_ref().unwrap().convert() }))
            },
            infercollid: self.infercollid as crate::sys::Oid,
            inferopclass: self.inferopclass as crate::sys::Oid,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::TargetEntry {
    fn convert(&self) -> crate::nodes::Node {
        Node::TargetEntry(TargetEntry {
            expr: if self.expr.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.expr.as_ref().unwrap().convert() }))
            },
            resno: self.resno as crate::sys::AttrNumber,
            resname: if self.resname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.resname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            ressortgroupref: self.ressortgroupref as crate::sys::Index,
            resorigtbl: self.resorigtbl as crate::sys::Oid,
            resorigcol: self.resorigcol as crate::sys::AttrNumber,
            resjunk: self.resjunk as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RangeTblRef {
    fn convert(&self) -> crate::nodes::Node {
        Node::RangeTblRef(RangeTblRef {
            rtindex: self.rtindex as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::JoinExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::JoinExpr(JoinExpr {
            jointype: self.jointype as crate::sys::JoinType,
            isNatural: self.isNatural as bool,
            larg: if self.larg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.larg.as_ref().unwrap().convert() }))
            },
            rarg: if self.rarg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.rarg.as_ref().unwrap().convert() }))
            },
            usingClause: if self.usingClause.is_null() {
                None
            } else {
                match unsafe { self.usingClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            quals: if self.quals.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.quals.as_ref().unwrap().convert() }))
            },
            alias: if self.alias.is_null() {
                None
            } else {
                match unsafe { self.alias.as_ref().unwrap().convert() } {
                    crate::nodes::Node::Alias(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(alias),
                        stringify!(Alias)
                    ),
                }
            },
            rtindex: self.rtindex as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::FromExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::FromExpr(FromExpr {
            fromlist: if self.fromlist.is_null() {
                None
            } else {
                match unsafe { self.fromlist.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            quals: if self.quals.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.quals.as_ref().unwrap().convert() }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::OnConflictExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::OnConflictExpr(OnConflictExpr {
            action: self.action as crate::sys::OnConflictAction,
            arbiterElems: if self.arbiterElems.is_null() {
                None
            } else {
                match unsafe { self.arbiterElems.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            arbiterWhere: if self.arbiterWhere.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.arbiterWhere.as_ref().unwrap().convert()
                }))
            },
            constraint: self.constraint as crate::sys::Oid,
            onConflictSet: if self.onConflictSet.is_null() {
                None
            } else {
                match unsafe { self.onConflictSet.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            onConflictWhere: if self.onConflictWhere.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.onConflictWhere.as_ref().unwrap().convert()
                }))
            },
            exclRelIndex: self.exclRelIndex as i32,
            exclRelTlist: if self.exclRelTlist.is_null() {
                None
            } else {
                match unsafe { self.exclRelTlist.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::TypeName {
    fn convert(&self) -> crate::nodes::Node {
        Node::TypeName(TypeName {
            names: if self.names.is_null() {
                None
            } else {
                match unsafe { self.names.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            typeOid: self.typeOid as crate::sys::Oid,
            setof: self.setof as bool,
            pct_type: self.pct_type as bool,
            typmods: if self.typmods.is_null() {
                None
            } else {
                match unsafe { self.typmods.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            typemod: self.typemod as i32,
            arrayBounds: if self.arrayBounds.is_null() {
                None
            } else {
                match unsafe { self.arrayBounds.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ColumnRef {
    fn convert(&self) -> crate::nodes::Node {
        Node::ColumnRef(ColumnRef {
            fields: if self.fields.is_null() {
                None
            } else {
                match unsafe { self.fields.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ParamRef {
    fn convert(&self) -> crate::nodes::Node {
        Node::ParamRef(ParamRef {
            number: self.number as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::A_Expr {
    fn convert(&self) -> crate::nodes::Node {
        Node::A_Expr(A_Expr {
            kind: self.kind as crate::sys::A_Expr_Kind,
            name: if self.name.is_null() {
                None
            } else {
                match unsafe { self.name.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            lexpr: if self.lexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.lexpr.as_ref().unwrap().convert() }))
            },
            rexpr: if self.rexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.rexpr.as_ref().unwrap().convert() }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::A_Const {
    fn convert(&self) -> crate::nodes::Node {
        Node::A_Const(A_Const {
            val: match self.val.convert() {
                crate::nodes::Node::Value(value) => value,
                _ => panic!("Value didn't convert to Value"),
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::TypeCast {
    fn convert(&self) -> crate::nodes::Node {
        Node::TypeCast(TypeCast {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            typeName: if self.typeName.is_null() {
                None
            } else {
                match unsafe { self.typeName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(typeName),
                        stringify!(TypeName)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CollateClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::CollateClause(CollateClause {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            collname: if self.collname.is_null() {
                None
            } else {
                match unsafe { self.collname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RoleSpec {
    fn convert(&self) -> crate::nodes::Node {
        Node::RoleSpec(RoleSpec {
            roletype: self.roletype as crate::sys::RoleSpecType,
            rolename: if self.rolename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.rolename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::FuncCall {
    fn convert(&self) -> crate::nodes::Node {
        Node::FuncCall(FuncCall {
            funcname: if self.funcname.is_null() {
                None
            } else {
                match unsafe { self.funcname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            agg_order: if self.agg_order.is_null() {
                None
            } else {
                match unsafe { self.agg_order.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            agg_filter: if self.agg_filter.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.agg_filter.as_ref().unwrap().convert()
                }))
            },
            agg_within_group: self.agg_within_group as bool,
            agg_star: self.agg_star as bool,
            agg_distinct: self.agg_distinct as bool,
            func_variadic: self.func_variadic as bool,
            over: if self.over.is_null() {
                None
            } else {
                match unsafe { self.over.as_ref().unwrap().convert() } {
                    crate::nodes::Node::WindowDef(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(over),
                        stringify!(WindowDef)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::A_Star {
    fn convert(&self) -> crate::nodes::Node {
        Node::A_Star(A_Star {})
    }
}
impl crate::convert::ConvertNode for crate::sys::A_Indices {
    fn convert(&self) -> crate::nodes::Node {
        Node::A_Indices(A_Indices {
            is_slice: self.is_slice as bool,
            lidx: if self.lidx.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.lidx.as_ref().unwrap().convert() }))
            },
            uidx: if self.uidx.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.uidx.as_ref().unwrap().convert() }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::A_Indirection {
    fn convert(&self) -> crate::nodes::Node {
        Node::A_Indirection(A_Indirection {
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            indirection: if self.indirection.is_null() {
                None
            } else {
                match unsafe { self.indirection.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::A_ArrayExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::A_ArrayExpr(A_ArrayExpr {
            elements: if self.elements.is_null() {
                None
            } else {
                match unsafe { self.elements.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ResTarget {
    fn convert(&self) -> crate::nodes::Node {
        Node::ResTarget(ResTarget {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            indirection: if self.indirection.is_null() {
                None
            } else {
                match unsafe { self.indirection.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            val: if self.val.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.val.as_ref().unwrap().convert() }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::MultiAssignRef {
    fn convert(&self) -> crate::nodes::Node {
        Node::MultiAssignRef(MultiAssignRef {
            source: if self.source.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.source.as_ref().unwrap().convert() }))
            },
            colno: self.colno as i32,
            ncolumns: self.ncolumns as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::SortBy {
    fn convert(&self) -> crate::nodes::Node {
        Node::SortBy(SortBy {
            node: if self.node.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.node.as_ref().unwrap().convert() }))
            },
            sortby_dir: self.sortby_dir as crate::sys::SortByDir,
            sortby_nulls: self.sortby_nulls as crate::sys::SortByNulls,
            useOp: if self.useOp.is_null() {
                None
            } else {
                match unsafe { self.useOp.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::WindowDef {
    fn convert(&self) -> crate::nodes::Node {
        Node::WindowDef(WindowDef {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            refname: if self.refname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.refname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            partitionClause: if self.partitionClause.is_null() {
                None
            } else {
                match unsafe { self.partitionClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            orderClause: if self.orderClause.is_null() {
                None
            } else {
                match unsafe { self.orderClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            frameOptions: self.frameOptions as i32,
            startOffset: if self.startOffset.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.startOffset.as_ref().unwrap().convert()
                }))
            },
            endOffset: if self.endOffset.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.endOffset.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RangeSubselect {
    fn convert(&self) -> crate::nodes::Node {
        Node::RangeSubselect(RangeSubselect {
            lateral: self.lateral as bool,
            subquery: if self.subquery.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.subquery.as_ref().unwrap().convert()
                }))
            },
            alias: if self.alias.is_null() {
                None
            } else {
                match unsafe { self.alias.as_ref().unwrap().convert() } {
                    crate::nodes::Node::Alias(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(alias),
                        stringify!(Alias)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RangeFunction {
    fn convert(&self) -> crate::nodes::Node {
        Node::RangeFunction(RangeFunction {
            lateral: self.lateral as bool,
            ordinality: self.ordinality as bool,
            is_rowsfrom: self.is_rowsfrom as bool,
            functions: if self.functions.is_null() {
                None
            } else {
                match unsafe { self.functions.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            alias: if self.alias.is_null() {
                None
            } else {
                match unsafe { self.alias.as_ref().unwrap().convert() } {
                    crate::nodes::Node::Alias(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(alias),
                        stringify!(Alias)
                    ),
                }
            },
            coldeflist: if self.coldeflist.is_null() {
                None
            } else {
                match unsafe { self.coldeflist.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RangeTableFunc {
    fn convert(&self) -> crate::nodes::Node {
        Node::RangeTableFunc(RangeTableFunc {
            lateral: self.lateral as bool,
            docexpr: if self.docexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.docexpr.as_ref().unwrap().convert()
                }))
            },
            rowexpr: if self.rowexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.rowexpr.as_ref().unwrap().convert()
                }))
            },
            namespaces: if self.namespaces.is_null() {
                None
            } else {
                match unsafe { self.namespaces.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            columns: if self.columns.is_null() {
                None
            } else {
                match unsafe { self.columns.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            alias: if self.alias.is_null() {
                None
            } else {
                match unsafe { self.alias.as_ref().unwrap().convert() } {
                    crate::nodes::Node::Alias(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(alias),
                        stringify!(Alias)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RangeTableFuncCol {
    fn convert(&self) -> crate::nodes::Node {
        Node::RangeTableFuncCol(RangeTableFuncCol {
            colname: if self.colname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.colname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            typeName: if self.typeName.is_null() {
                None
            } else {
                match unsafe { self.typeName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(typeName),
                        stringify!(TypeName)
                    ),
                }
            },
            for_ordinality: self.for_ordinality as bool,
            is_not_null: self.is_not_null as bool,
            colexpr: if self.colexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.colexpr.as_ref().unwrap().convert()
                }))
            },
            coldefexpr: if self.coldefexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.coldefexpr.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RangeTableSample {
    fn convert(&self) -> crate::nodes::Node {
        Node::RangeTableSample(RangeTableSample {
            relation: if self.relation.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.relation.as_ref().unwrap().convert()
                }))
            },
            method: if self.method.is_null() {
                None
            } else {
                match unsafe { self.method.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            repeatable: if self.repeatable.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.repeatable.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ColumnDef {
    fn convert(&self) -> crate::nodes::Node {
        Node::ColumnDef(ColumnDef {
            colname: if self.colname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.colname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            typeName: if self.typeName.is_null() {
                None
            } else {
                match unsafe { self.typeName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(typeName),
                        stringify!(TypeName)
                    ),
                }
            },
            inhcount: self.inhcount as i32,
            is_local: self.is_local as bool,
            is_not_null: self.is_not_null as bool,
            is_from_type: self.is_from_type as bool,
            storage: self.storage as u8 as char,
            raw_default: if self.raw_default.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.raw_default.as_ref().unwrap().convert()
                }))
            },
            cooked_default: if self.cooked_default.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.cooked_default.as_ref().unwrap().convert()
                }))
            },
            identity: self.identity as u8 as char,
            identitySequence: if self.identitySequence.is_null() {
                None
            } else {
                match unsafe { self.identitySequence.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(identitySequence),
                        stringify!(RangeVar)
                    ),
                }
            },
            generated: self.generated as u8 as char,
            collClause: if self.collClause.is_null() {
                None
            } else {
                match unsafe { self.collClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::CollateClause(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(collClause),
                        stringify!(CollateClause)
                    ),
                }
            },
            collOid: self.collOid as crate::sys::Oid,
            constraints: if self.constraints.is_null() {
                None
            } else {
                match unsafe { self.constraints.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            fdwoptions: if self.fdwoptions.is_null() {
                None
            } else {
                match unsafe { self.fdwoptions.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::TableLikeClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::TableLikeClause(TableLikeClause {
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            options: self.options as crate::sys::bits32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::IndexElem {
    fn convert(&self) -> crate::nodes::Node {
        Node::IndexElem(IndexElem {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            expr: if self.expr.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.expr.as_ref().unwrap().convert() }))
            },
            indexcolname: if self.indexcolname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.indexcolname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            collation: if self.collation.is_null() {
                None
            } else {
                match unsafe { self.collation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            opclass: if self.opclass.is_null() {
                None
            } else {
                match unsafe { self.opclass.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            opclassopts: if self.opclassopts.is_null() {
                None
            } else {
                match unsafe { self.opclassopts.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            ordering: self.ordering as crate::sys::SortByDir,
            nulls_ordering: self.nulls_ordering as crate::sys::SortByNulls,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DefElem {
    fn convert(&self) -> crate::nodes::Node {
        Node::DefElem(DefElem {
            defnamespace: if self.defnamespace.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.defnamespace)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            defname: if self.defname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.defname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            arg: if self.arg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.arg.as_ref().unwrap().convert() }))
            },
            defaction: self.defaction as crate::sys::DefElemAction,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::LockingClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::LockingClause(LockingClause {
            lockedRels: if self.lockedRels.is_null() {
                None
            } else {
                match unsafe { self.lockedRels.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            strength: self.strength as crate::sys::LockClauseStrength,
            waitPolicy: self.waitPolicy as crate::sys::LockWaitPolicy,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::XmlSerialize {
    fn convert(&self) -> crate::nodes::Node {
        Node::XmlSerialize(XmlSerialize {
            xmloption: self.xmloption as crate::sys::XmlOptionType,
            expr: if self.expr.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.expr.as_ref().unwrap().convert() }))
            },
            typeName: if self.typeName.is_null() {
                None
            } else {
                match unsafe { self.typeName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(typeName),
                        stringify!(TypeName)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::PartitionElem {
    fn convert(&self) -> crate::nodes::Node {
        Node::PartitionElem(PartitionElem {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            expr: if self.expr.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.expr.as_ref().unwrap().convert() }))
            },
            collation: if self.collation.is_null() {
                None
            } else {
                match unsafe { self.collation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            opclass: if self.opclass.is_null() {
                None
            } else {
                match unsafe { self.opclass.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::PartitionSpec {
    fn convert(&self) -> crate::nodes::Node {
        Node::PartitionSpec(PartitionSpec {
            strategy: if self.strategy.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.strategy)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            partParams: if self.partParams.is_null() {
                None
            } else {
                match unsafe { self.partParams.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::PartitionBoundSpec {
    fn convert(&self) -> crate::nodes::Node {
        Node::PartitionBoundSpec(PartitionBoundSpec {
            strategy: self.strategy as u8 as char,
            is_default: self.is_default as bool,
            modulus: self.modulus as i32,
            remainder: self.remainder as i32,
            listdatums: if self.listdatums.is_null() {
                None
            } else {
                match unsafe { self.listdatums.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            lowerdatums: if self.lowerdatums.is_null() {
                None
            } else {
                match unsafe { self.lowerdatums.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            upperdatums: if self.upperdatums.is_null() {
                None
            } else {
                match unsafe { self.upperdatums.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::PartitionRangeDatum {
    fn convert(&self) -> crate::nodes::Node {
        Node::PartitionRangeDatum(PartitionRangeDatum {
            kind: self.kind as crate::sys::PartitionRangeDatumKind,
            value: if self.value.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.value.as_ref().unwrap().convert() }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::PartitionCmd {
    fn convert(&self) -> crate::nodes::Node {
        Node::PartitionCmd(PartitionCmd {
            name: if self.name.is_null() {
                None
            } else {
                match unsafe { self.name.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(name),
                        stringify!(RangeVar)
                    ),
                }
            },
            bound: if self.bound.is_null() {
                None
            } else {
                match unsafe { self.bound.as_ref().unwrap().convert() } {
                    crate::nodes::Node::PartitionBoundSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(bound),
                        stringify!(PartitionBoundSpec)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::TableSampleClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::TableSampleClause(TableSampleClause {
            tsmhandler: self.tsmhandler as crate::sys::Oid,
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            repeatable: if self.repeatable.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.repeatable.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::WithCheckOption {
    fn convert(&self) -> crate::nodes::Node {
        Node::WithCheckOption(WithCheckOption {
            kind: self.kind as crate::sys::WCOKind,
            relname: if self.relname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.relname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            polname: if self.polname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.polname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            qual: if self.qual.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.qual.as_ref().unwrap().convert() }))
            },
            cascaded: self.cascaded as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::SortGroupClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::SortGroupClause(SortGroupClause {
            tleSortGroupRef: self.tleSortGroupRef as crate::sys::Index,
            eqop: self.eqop as crate::sys::Oid,
            sortop: self.sortop as crate::sys::Oid,
            nulls_first: self.nulls_first as bool,
            hashable: self.hashable as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::GroupingSet {
    fn convert(&self) -> crate::nodes::Node {
        Node::GroupingSet(GroupingSet {
            kind: self.kind as crate::sys::GroupingSetKind,
            content: if self.content.is_null() {
                None
            } else {
                match unsafe { self.content.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::WindowClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::WindowClause(WindowClause {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            refname: if self.refname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.refname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            partitionClause: if self.partitionClause.is_null() {
                None
            } else {
                match unsafe { self.partitionClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            orderClause: if self.orderClause.is_null() {
                None
            } else {
                match unsafe { self.orderClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            frameOptions: self.frameOptions as i32,
            startOffset: if self.startOffset.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.startOffset.as_ref().unwrap().convert()
                }))
            },
            endOffset: if self.endOffset.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.endOffset.as_ref().unwrap().convert()
                }))
            },
            startInRangeFunc: self.startInRangeFunc as crate::sys::Oid,
            endInRangeFunc: self.endInRangeFunc as crate::sys::Oid,
            inRangeColl: self.inRangeColl as crate::sys::Oid,
            inRangeAsc: self.inRangeAsc as bool,
            inRangeNullsFirst: self.inRangeNullsFirst as bool,
            winref: self.winref as crate::sys::Index,
            copiedOrder: self.copiedOrder as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RowMarkClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::RowMarkClause(RowMarkClause {
            rti: self.rti as crate::sys::Index,
            strength: self.strength as crate::sys::LockClauseStrength,
            waitPolicy: self.waitPolicy as crate::sys::LockWaitPolicy,
            pushedDown: self.pushedDown as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::WithClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::WithClause(WithClause {
            ctes: if self.ctes.is_null() {
                None
            } else {
                match unsafe { self.ctes.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            recursive: self.recursive as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::InferClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::InferClause(InferClause {
            indexElems: if self.indexElems.is_null() {
                None
            } else {
                match unsafe { self.indexElems.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            whereClause: if self.whereClause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.whereClause.as_ref().unwrap().convert()
                }))
            },
            conname: if self.conname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.conname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::OnConflictClause {
    fn convert(&self) -> crate::nodes::Node {
        Node::OnConflictClause(OnConflictClause {
            action: self.action as crate::sys::OnConflictAction,
            infer: if self.infer.is_null() {
                None
            } else {
                match unsafe { self.infer.as_ref().unwrap().convert() } {
                    crate::nodes::Node::InferClause(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(infer),
                        stringify!(InferClause)
                    ),
                }
            },
            targetList: if self.targetList.is_null() {
                None
            } else {
                match unsafe { self.targetList.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            whereClause: if self.whereClause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.whereClause.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CommonTableExpr {
    fn convert(&self) -> crate::nodes::Node {
        Node::CommonTableExpr(CommonTableExpr {
            ctename: if self.ctename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.ctename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            aliascolnames: if self.aliascolnames.is_null() {
                None
            } else {
                match unsafe { self.aliascolnames.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            ctematerialized: self.ctematerialized as crate::sys::CTEMaterialize,
            ctequery: if self.ctequery.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.ctequery.as_ref().unwrap().convert()
                }))
            },
            cterecursive: self.cterecursive as bool,
            cterefcount: self.cterefcount as i32,
            ctecolnames: if self.ctecolnames.is_null() {
                None
            } else {
                match unsafe { self.ctecolnames.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            ctecoltypes: if self.ctecoltypes.is_null() {
                None
            } else {
                match unsafe { self.ctecoltypes.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            ctecoltypmods: if self.ctecoltypmods.is_null() {
                None
            } else {
                match unsafe { self.ctecoltypmods.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            ctecolcollations: if self.ctecolcollations.is_null() {
                None
            } else {
                match unsafe { self.ctecolcollations.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::TriggerTransition {
    fn convert(&self) -> crate::nodes::Node {
        Node::TriggerTransition(TriggerTransition {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            isNew: self.isNew as bool,
            isTable: self.isTable as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RawStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::RawStmt(RawStmt {
            stmt: if self.stmt.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.stmt.as_ref().unwrap().convert() }))
            },
            stmt_location: self.stmt_location as i32,
            stmt_len: self.stmt_len as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::InsertStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::InsertStmt(InsertStmt {
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            cols: if self.cols.is_null() {
                None
            } else {
                match unsafe { self.cols.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            selectStmt: if self.selectStmt.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.selectStmt.as_ref().unwrap().convert()
                }))
            },
            onConflictClause: if self.onConflictClause.is_null() {
                None
            } else {
                match unsafe { self.onConflictClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::OnConflictClause(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(onConflictClause),
                        stringify!(OnConflictClause)
                    ),
                }
            },
            returningList: if self.returningList.is_null() {
                None
            } else {
                match unsafe { self.returningList.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            withClause: if self.withClause.is_null() {
                None
            } else {
                match unsafe { self.withClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::WithClause(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(withClause),
                        stringify!(WithClause)
                    ),
                }
            },
            override_: self.override_ as crate::sys::OverridingKind,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DeleteStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DeleteStmt(DeleteStmt {
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            usingClause: if self.usingClause.is_null() {
                None
            } else {
                match unsafe { self.usingClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            whereClause: if self.whereClause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.whereClause.as_ref().unwrap().convert()
                }))
            },
            returningList: if self.returningList.is_null() {
                None
            } else {
                match unsafe { self.returningList.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            withClause: if self.withClause.is_null() {
                None
            } else {
                match unsafe { self.withClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::WithClause(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(withClause),
                        stringify!(WithClause)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::UpdateStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::UpdateStmt(UpdateStmt {
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            targetList: if self.targetList.is_null() {
                None
            } else {
                match unsafe { self.targetList.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            whereClause: if self.whereClause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.whereClause.as_ref().unwrap().convert()
                }))
            },
            fromClause: if self.fromClause.is_null() {
                None
            } else {
                match unsafe { self.fromClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            returningList: if self.returningList.is_null() {
                None
            } else {
                match unsafe { self.returningList.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            withClause: if self.withClause.is_null() {
                None
            } else {
                match unsafe { self.withClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::WithClause(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(withClause),
                        stringify!(WithClause)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::SelectStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::SelectStmt(SelectStmt {
            distinctClause: if self.distinctClause.is_null() {
                None
            } else {
                match unsafe { self.distinctClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            intoClause: if self.intoClause.is_null() {
                None
            } else {
                match unsafe { self.intoClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::IntoClause(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(intoClause),
                        stringify!(IntoClause)
                    ),
                }
            },
            targetList: if self.targetList.is_null() {
                None
            } else {
                match unsafe { self.targetList.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            fromClause: if self.fromClause.is_null() {
                None
            } else {
                match unsafe { self.fromClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            whereClause: if self.whereClause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.whereClause.as_ref().unwrap().convert()
                }))
            },
            groupClause: if self.groupClause.is_null() {
                None
            } else {
                match unsafe { self.groupClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            havingClause: if self.havingClause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.havingClause.as_ref().unwrap().convert()
                }))
            },
            windowClause: if self.windowClause.is_null() {
                None
            } else {
                match unsafe { self.windowClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            valuesLists: if self.valuesLists.is_null() {
                None
            } else {
                match unsafe { self.valuesLists.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            sortClause: if self.sortClause.is_null() {
                None
            } else {
                match unsafe { self.sortClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            limitOffset: if self.limitOffset.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.limitOffset.as_ref().unwrap().convert()
                }))
            },
            limitCount: if self.limitCount.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.limitCount.as_ref().unwrap().convert()
                }))
            },
            limitOption: self.limitOption as crate::sys::LimitOption,
            lockingClause: if self.lockingClause.is_null() {
                None
            } else {
                match unsafe { self.lockingClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            withClause: if self.withClause.is_null() {
                None
            } else {
                match unsafe { self.withClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::WithClause(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(withClause),
                        stringify!(WithClause)
                    ),
                }
            },
            op: self.op as crate::sys::SetOperation,
            all: self.all as bool,
            larg: if self.larg.is_null() {
                None
            } else {
                match unsafe { self.larg.as_ref().unwrap().convert() } {
                    crate::nodes::Node::SelectStmt(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(larg),
                        stringify!(SelectStmt)
                    ),
                }
            },
            rarg: if self.rarg.is_null() {
                None
            } else {
                match unsafe { self.rarg.as_ref().unwrap().convert() } {
                    crate::nodes::Node::SelectStmt(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(rarg),
                        stringify!(SelectStmt)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::SetOperationStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::SetOperationStmt(SetOperationStmt {
            op: self.op as crate::sys::SetOperation,
            all: self.all as bool,
            larg: if self.larg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.larg.as_ref().unwrap().convert() }))
            },
            rarg: if self.rarg.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.rarg.as_ref().unwrap().convert() }))
            },
            colTypes: if self.colTypes.is_null() {
                None
            } else {
                match unsafe { self.colTypes.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            colTypmods: if self.colTypmods.is_null() {
                None
            } else {
                match unsafe { self.colTypmods.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            colCollations: if self.colCollations.is_null() {
                None
            } else {
                match unsafe { self.colCollations.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            groupClauses: if self.groupClauses.is_null() {
                None
            } else {
                match unsafe { self.groupClauses.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateSchemaStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateSchemaStmt(CreateSchemaStmt {
            schemaname: if self.schemaname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.schemaname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            authrole: if self.authrole.is_null() {
                None
            } else {
                match unsafe { self.authrole.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(authrole),
                        stringify!(RoleSpec)
                    ),
                }
            },
            schemaElts: if self.schemaElts.is_null() {
                None
            } else {
                match unsafe { self.schemaElts.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            if_not_exists: self.if_not_exists as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterTableStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterTableStmt(AlterTableStmt {
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            cmds: if self.cmds.is_null() {
                None
            } else {
                match unsafe { self.cmds.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            relkind: self.relkind as crate::sys::ObjectType,
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ReplicaIdentityStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ReplicaIdentityStmt(ReplicaIdentityStmt {
            identity_type: self.identity_type as u8 as char,
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterTableCmd {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterTableCmd(AlterTableCmd {
            subtype: self.subtype as crate::sys::AlterTableType,
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            num: self.num as i16,
            newowner: if self.newowner.is_null() {
                None
            } else {
                match unsafe { self.newowner.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(newowner),
                        stringify!(RoleSpec)
                    ),
                }
            },
            def: if self.def.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.def.as_ref().unwrap().convert() }))
            },
            behavior: self.behavior as crate::sys::DropBehavior,
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterCollationStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterCollationStmt(AlterCollationStmt {
            collname: if self.collname.is_null() {
                None
            } else {
                match unsafe { self.collname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterDomainStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterDomainStmt(AlterDomainStmt {
            subtype: self.subtype as u8 as char,
            typeName: if self.typeName.is_null() {
                None
            } else {
                match unsafe { self.typeName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            def: if self.def.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.def.as_ref().unwrap().convert() }))
            },
            behavior: self.behavior as crate::sys::DropBehavior,
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::GrantStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::GrantStmt(GrantStmt {
            is_grant: self.is_grant as bool,
            targtype: self.targtype as crate::sys::GrantTargetType,
            objtype: self.objtype as crate::sys::ObjectType,
            objects: if self.objects.is_null() {
                None
            } else {
                match unsafe { self.objects.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            privileges: if self.privileges.is_null() {
                None
            } else {
                match unsafe { self.privileges.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            grantees: if self.grantees.is_null() {
                None
            } else {
                match unsafe { self.grantees.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            grant_option: self.grant_option as bool,
            behavior: self.behavior as crate::sys::DropBehavior,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ObjectWithArgs {
    fn convert(&self) -> crate::nodes::Node {
        Node::ObjectWithArgs(ObjectWithArgs {
            objname: if self.objname.is_null() {
                None
            } else {
                match unsafe { self.objname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            objargs: if self.objargs.is_null() {
                None
            } else {
                match unsafe { self.objargs.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            args_unspecified: self.args_unspecified as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AccessPriv {
    fn convert(&self) -> crate::nodes::Node {
        Node::AccessPriv(AccessPriv {
            priv_name: if self.priv_name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.priv_name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            cols: if self.cols.is_null() {
                None
            } else {
                match unsafe { self.cols.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::GrantRoleStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::GrantRoleStmt(GrantRoleStmt {
            granted_roles: if self.granted_roles.is_null() {
                None
            } else {
                match unsafe { self.granted_roles.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            grantee_roles: if self.grantee_roles.is_null() {
                None
            } else {
                match unsafe { self.grantee_roles.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            is_grant: self.is_grant as bool,
            admin_opt: self.admin_opt as bool,
            grantor: if self.grantor.is_null() {
                None
            } else {
                match unsafe { self.grantor.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(grantor),
                        stringify!(RoleSpec)
                    ),
                }
            },
            behavior: self.behavior as crate::sys::DropBehavior,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterDefaultPrivilegesStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterDefaultPrivilegesStmt(AlterDefaultPrivilegesStmt {
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            action: if self.action.is_null() {
                None
            } else {
                match unsafe { self.action.as_ref().unwrap().convert() } {
                    crate::nodes::Node::GrantStmt(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(action),
                        stringify!(GrantStmt)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CopyStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CopyStmt(CopyStmt {
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            query: if self.query.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.query.as_ref().unwrap().convert() }))
            },
            attlist: if self.attlist.is_null() {
                None
            } else {
                match unsafe { self.attlist.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            is_from: self.is_from as bool,
            is_program: self.is_program as bool,
            filename: if self.filename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.filename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            whereClause: if self.whereClause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.whereClause.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::VariableSetStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::VariableSetStmt(VariableSetStmt {
            kind: self.kind as crate::sys::VariableSetKind,
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            is_local: self.is_local as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::VariableShowStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::VariableShowStmt(VariableShowStmt {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateStmt(CreateStmt {
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            tableElts: if self.tableElts.is_null() {
                None
            } else {
                match unsafe { self.tableElts.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            inhRelations: if self.inhRelations.is_null() {
                None
            } else {
                match unsafe { self.inhRelations.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            partbound: if self.partbound.is_null() {
                None
            } else {
                match unsafe { self.partbound.as_ref().unwrap().convert() } {
                    crate::nodes::Node::PartitionBoundSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(partbound),
                        stringify!(PartitionBoundSpec)
                    ),
                }
            },
            partspec: if self.partspec.is_null() {
                None
            } else {
                match unsafe { self.partspec.as_ref().unwrap().convert() } {
                    crate::nodes::Node::PartitionSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(partspec),
                        stringify!(PartitionSpec)
                    ),
                }
            },
            ofTypename: if self.ofTypename.is_null() {
                None
            } else {
                match unsafe { self.ofTypename.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(ofTypename),
                        stringify!(TypeName)
                    ),
                }
            },
            constraints: if self.constraints.is_null() {
                None
            } else {
                match unsafe { self.constraints.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            oncommit: self.oncommit as crate::sys::OnCommitAction,
            tablespacename: if self.tablespacename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.tablespacename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            accessMethod: if self.accessMethod.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.accessMethod)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            if_not_exists: self.if_not_exists as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::Constraint {
    fn convert(&self) -> crate::nodes::Node {
        Node::Constraint(Constraint {
            contype: self.contype as crate::sys::ConstrType,
            conname: if self.conname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.conname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            deferrable: self.deferrable as bool,
            initdeferred: self.initdeferred as bool,
            is_no_inherit: self.is_no_inherit as bool,
            raw_expr: if self.raw_expr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.raw_expr.as_ref().unwrap().convert()
                }))
            },
            cooked_expr: if self.cooked_expr.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.cooked_expr)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            generated_when: self.generated_when as u8 as char,
            keys: if self.keys.is_null() {
                None
            } else {
                match unsafe { self.keys.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            including: if self.including.is_null() {
                None
            } else {
                match unsafe { self.including.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            exclusions: if self.exclusions.is_null() {
                None
            } else {
                match unsafe { self.exclusions.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            indexname: if self.indexname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.indexname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            indexspace: if self.indexspace.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.indexspace)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            reset_default_tblspc: self.reset_default_tblspc as bool,
            access_method: if self.access_method.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.access_method)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            where_clause: if self.where_clause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.where_clause.as_ref().unwrap().convert()
                }))
            },
            pktable: if self.pktable.is_null() {
                None
            } else {
                match unsafe { self.pktable.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(pktable),
                        stringify!(RangeVar)
                    ),
                }
            },
            fk_attrs: if self.fk_attrs.is_null() {
                None
            } else {
                match unsafe { self.fk_attrs.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            pk_attrs: if self.pk_attrs.is_null() {
                None
            } else {
                match unsafe { self.pk_attrs.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            fk_matchtype: self.fk_matchtype as u8 as char,
            fk_upd_action: self.fk_upd_action as u8 as char,
            fk_del_action: self.fk_del_action as u8 as char,
            old_conpfeqop: if self.old_conpfeqop.is_null() {
                None
            } else {
                match unsafe { self.old_conpfeqop.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            old_pktable_oid: self.old_pktable_oid as crate::sys::Oid,
            skip_validation: self.skip_validation as bool,
            initially_valid: self.initially_valid as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateTableSpaceStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateTableSpaceStmt(CreateTableSpaceStmt {
            tablespacename: if self.tablespacename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.tablespacename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            owner: if self.owner.is_null() {
                None
            } else {
                match unsafe { self.owner.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(owner),
                        stringify!(RoleSpec)
                    ),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DropTableSpaceStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DropTableSpaceStmt(DropTableSpaceStmt {
            tablespacename: if self.tablespacename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.tablespacename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterTableSpaceOptionsStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterTableSpaceOptionsStmt(AlterTableSpaceOptionsStmt {
            tablespacename: if self.tablespacename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.tablespacename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            isReset: self.isReset as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterTableMoveAllStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterTableMoveAllStmt(AlterTableMoveAllStmt {
            orig_tablespacename: if self.orig_tablespacename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.orig_tablespacename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            objtype: self.objtype as crate::sys::ObjectType,
            roles: if self.roles.is_null() {
                None
            } else {
                match unsafe { self.roles.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            new_tablespacename: if self.new_tablespacename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.new_tablespacename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            nowait: self.nowait as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateExtensionStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateExtensionStmt(CreateExtensionStmt {
            extname: if self.extname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.extname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            if_not_exists: self.if_not_exists as bool,
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterExtensionStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterExtensionStmt(AlterExtensionStmt {
            extname: if self.extname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.extname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterExtensionContentsStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterExtensionContentsStmt(AlterExtensionContentsStmt {
            extname: if self.extname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.extname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            action: self.action as i32,
            objtype: self.objtype as crate::sys::ObjectType,
            object: if self.object.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.object.as_ref().unwrap().convert() }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateFdwStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateFdwStmt(CreateFdwStmt {
            fdwname: if self.fdwname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.fdwname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            func_options: if self.func_options.is_null() {
                None
            } else {
                match unsafe { self.func_options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterFdwStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterFdwStmt(AlterFdwStmt {
            fdwname: if self.fdwname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.fdwname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            func_options: if self.func_options.is_null() {
                None
            } else {
                match unsafe { self.func_options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateForeignServerStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateForeignServerStmt(CreateForeignServerStmt {
            servername: if self.servername.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.servername)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            servertype: if self.servertype.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.servertype)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            version: if self.version.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.version)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            fdwname: if self.fdwname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.fdwname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            if_not_exists: self.if_not_exists as bool,
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterForeignServerStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterForeignServerStmt(AlterForeignServerStmt {
            servername: if self.servername.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.servername)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            version: if self.version.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.version)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            has_version: self.has_version as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateUserMappingStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateUserMappingStmt(CreateUserMappingStmt {
            user: if self.user.is_null() {
                None
            } else {
                match unsafe { self.user.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(user),
                        stringify!(RoleSpec)
                    ),
                }
            },
            servername: if self.servername.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.servername)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            if_not_exists: self.if_not_exists as bool,
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterUserMappingStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterUserMappingStmt(AlterUserMappingStmt {
            user: if self.user.is_null() {
                None
            } else {
                match unsafe { self.user.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(user),
                        stringify!(RoleSpec)
                    ),
                }
            },
            servername: if self.servername.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.servername)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DropUserMappingStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DropUserMappingStmt(DropUserMappingStmt {
            user: if self.user.is_null() {
                None
            } else {
                match unsafe { self.user.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(user),
                        stringify!(RoleSpec)
                    ),
                }
            },
            servername: if self.servername.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.servername)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ImportForeignSchemaStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ImportForeignSchemaStmt(ImportForeignSchemaStmt {
            server_name: if self.server_name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.server_name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            remote_schema: if self.remote_schema.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.remote_schema)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            local_schema: if self.local_schema.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.local_schema)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            list_type: self.list_type as crate::sys::ImportForeignSchemaType,
            table_list: if self.table_list.is_null() {
                None
            } else {
                match unsafe { self.table_list.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreatePolicyStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreatePolicyStmt(CreatePolicyStmt {
            policy_name: if self.policy_name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.policy_name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            table: if self.table.is_null() {
                None
            } else {
                match unsafe { self.table.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(table),
                        stringify!(RangeVar)
                    ),
                }
            },
            cmd_name: if self.cmd_name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.cmd_name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            permissive: self.permissive as bool,
            roles: if self.roles.is_null() {
                None
            } else {
                match unsafe { self.roles.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            qual: if self.qual.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.qual.as_ref().unwrap().convert() }))
            },
            with_check: if self.with_check.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.with_check.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterPolicyStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterPolicyStmt(AlterPolicyStmt {
            policy_name: if self.policy_name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.policy_name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            table: if self.table.is_null() {
                None
            } else {
                match unsafe { self.table.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(table),
                        stringify!(RangeVar)
                    ),
                }
            },
            roles: if self.roles.is_null() {
                None
            } else {
                match unsafe { self.roles.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            qual: if self.qual.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.qual.as_ref().unwrap().convert() }))
            },
            with_check: if self.with_check.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.with_check.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateAmStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateAmStmt(CreateAmStmt {
            amname: if self.amname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.amname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            handler_name: if self.handler_name.is_null() {
                None
            } else {
                match unsafe { self.handler_name.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            amtype: self.amtype as u8 as char,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateTrigStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateTrigStmt(CreateTrigStmt {
            trigname: if self.trigname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.trigname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            funcname: if self.funcname.is_null() {
                None
            } else {
                match unsafe { self.funcname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            row: self.row as bool,
            timing: self.timing as i16,
            events: self.events as i16,
            columns: if self.columns.is_null() {
                None
            } else {
                match unsafe { self.columns.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            whenClause: if self.whenClause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.whenClause.as_ref().unwrap().convert()
                }))
            },
            isconstraint: self.isconstraint as bool,
            transitionRels: if self.transitionRels.is_null() {
                None
            } else {
                match unsafe { self.transitionRels.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            deferrable: self.deferrable as bool,
            initdeferred: self.initdeferred as bool,
            constrrel: if self.constrrel.is_null() {
                None
            } else {
                match unsafe { self.constrrel.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(constrrel),
                        stringify!(RangeVar)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateEventTrigStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateEventTrigStmt(CreateEventTrigStmt {
            trigname: if self.trigname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.trigname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            eventname: if self.eventname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.eventname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            whenclause: if self.whenclause.is_null() {
                None
            } else {
                match unsafe { self.whenclause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            funcname: if self.funcname.is_null() {
                None
            } else {
                match unsafe { self.funcname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterEventTrigStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterEventTrigStmt(AlterEventTrigStmt {
            trigname: if self.trigname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.trigname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            tgenabled: self.tgenabled as u8 as char,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreatePLangStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreatePLangStmt(CreatePLangStmt {
            replace: self.replace as bool,
            plname: if self.plname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.plname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            plhandler: if self.plhandler.is_null() {
                None
            } else {
                match unsafe { self.plhandler.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            plinline: if self.plinline.is_null() {
                None
            } else {
                match unsafe { self.plinline.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            plvalidator: if self.plvalidator.is_null() {
                None
            } else {
                match unsafe { self.plvalidator.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            pltrusted: self.pltrusted as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateRoleStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateRoleStmt(CreateRoleStmt {
            stmt_type: self.stmt_type as crate::sys::RoleStmtType,
            role: if self.role.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.role)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterRoleStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterRoleStmt(AlterRoleStmt {
            role: if self.role.is_null() {
                None
            } else {
                match unsafe { self.role.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(role),
                        stringify!(RoleSpec)
                    ),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            action: self.action as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterRoleSetStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterRoleSetStmt(AlterRoleSetStmt {
            role: if self.role.is_null() {
                None
            } else {
                match unsafe { self.role.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(role),
                        stringify!(RoleSpec)
                    ),
                }
            },
            database: if self.database.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.database)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            setstmt: if self.setstmt.is_null() {
                None
            } else {
                match unsafe { self.setstmt.as_ref().unwrap().convert() } {
                    crate::nodes::Node::VariableSetStmt(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(setstmt),
                        stringify!(VariableSetStmt)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DropRoleStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DropRoleStmt(DropRoleStmt {
            roles: if self.roles.is_null() {
                None
            } else {
                match unsafe { self.roles.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateSeqStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateSeqStmt(CreateSeqStmt {
            sequence: if self.sequence.is_null() {
                None
            } else {
                match unsafe { self.sequence.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(sequence),
                        stringify!(RangeVar)
                    ),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            ownerId: self.ownerId as crate::sys::Oid,
            for_identity: self.for_identity as bool,
            if_not_exists: self.if_not_exists as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterSeqStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterSeqStmt(AlterSeqStmt {
            sequence: if self.sequence.is_null() {
                None
            } else {
                match unsafe { self.sequence.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(sequence),
                        stringify!(RangeVar)
                    ),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            for_identity: self.for_identity as bool,
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DefineStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DefineStmt(DefineStmt {
            kind: self.kind as crate::sys::ObjectType,
            oldstyle: self.oldstyle as bool,
            defnames: if self.defnames.is_null() {
                None
            } else {
                match unsafe { self.defnames.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            definition: if self.definition.is_null() {
                None
            } else {
                match unsafe { self.definition.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            if_not_exists: self.if_not_exists as bool,
            replace: self.replace as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateDomainStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateDomainStmt(CreateDomainStmt {
            domainname: if self.domainname.is_null() {
                None
            } else {
                match unsafe { self.domainname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            typeName: if self.typeName.is_null() {
                None
            } else {
                match unsafe { self.typeName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(typeName),
                        stringify!(TypeName)
                    ),
                }
            },
            collClause: if self.collClause.is_null() {
                None
            } else {
                match unsafe { self.collClause.as_ref().unwrap().convert() } {
                    crate::nodes::Node::CollateClause(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(collClause),
                        stringify!(CollateClause)
                    ),
                }
            },
            constraints: if self.constraints.is_null() {
                None
            } else {
                match unsafe { self.constraints.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateOpClassStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateOpClassStmt(CreateOpClassStmt {
            opclassname: if self.opclassname.is_null() {
                None
            } else {
                match unsafe { self.opclassname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            opfamilyname: if self.opfamilyname.is_null() {
                None
            } else {
                match unsafe { self.opfamilyname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            amname: if self.amname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.amname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            datatype: if self.datatype.is_null() {
                None
            } else {
                match unsafe { self.datatype.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(datatype),
                        stringify!(TypeName)
                    ),
                }
            },
            items: if self.items.is_null() {
                None
            } else {
                match unsafe { self.items.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            isDefault: self.isDefault as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateOpClassItem {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateOpClassItem(CreateOpClassItem {
            itemtype: self.itemtype as i32,
            name: if self.name.is_null() {
                None
            } else {
                match unsafe { self.name.as_ref().unwrap().convert() } {
                    crate::nodes::Node::ObjectWithArgs(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(name),
                        stringify!(ObjectWithArgs)
                    ),
                }
            },
            number: self.number as i32,
            order_family: if self.order_family.is_null() {
                None
            } else {
                match unsafe { self.order_family.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            class_args: if self.class_args.is_null() {
                None
            } else {
                match unsafe { self.class_args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            storedtype: if self.storedtype.is_null() {
                None
            } else {
                match unsafe { self.storedtype.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(storedtype),
                        stringify!(TypeName)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateOpFamilyStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateOpFamilyStmt(CreateOpFamilyStmt {
            opfamilyname: if self.opfamilyname.is_null() {
                None
            } else {
                match unsafe { self.opfamilyname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            amname: if self.amname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.amname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterOpFamilyStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterOpFamilyStmt(AlterOpFamilyStmt {
            opfamilyname: if self.opfamilyname.is_null() {
                None
            } else {
                match unsafe { self.opfamilyname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            amname: if self.amname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.amname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            isDrop: self.isDrop as bool,
            items: if self.items.is_null() {
                None
            } else {
                match unsafe { self.items.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DropStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DropStmt(DropStmt {
            objects: if self.objects.is_null() {
                None
            } else {
                match unsafe { self.objects.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            removeType: self.removeType as crate::sys::ObjectType,
            behavior: self.behavior as crate::sys::DropBehavior,
            missing_ok: self.missing_ok as bool,
            concurrent: self.concurrent as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::TruncateStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::TruncateStmt(TruncateStmt {
            relations: if self.relations.is_null() {
                None
            } else {
                match unsafe { self.relations.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            restart_seqs: self.restart_seqs as bool,
            behavior: self.behavior as crate::sys::DropBehavior,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CommentStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CommentStmt(CommentStmt {
            objtype: self.objtype as crate::sys::ObjectType,
            object: if self.object.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.object.as_ref().unwrap().convert() }))
            },
            comment: if self.comment.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.comment)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::SecLabelStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::SecLabelStmt(SecLabelStmt {
            objtype: self.objtype as crate::sys::ObjectType,
            object: if self.object.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.object.as_ref().unwrap().convert() }))
            },
            provider: if self.provider.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.provider)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            label: if self.label.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.label)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DeclareCursorStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DeclareCursorStmt(DeclareCursorStmt {
            portalname: if self.portalname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.portalname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: self.options as i32,
            query: if self.query.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.query.as_ref().unwrap().convert() }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ClosePortalStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ClosePortalStmt(ClosePortalStmt {
            portalname: if self.portalname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.portalname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::FetchStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::FetchStmt(FetchStmt {
            direction: self.direction as crate::sys::FetchDirection,
            howMany: self.howMany as i64,
            portalname: if self.portalname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.portalname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            ismove: self.ismove as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::IndexStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::IndexStmt(IndexStmt {
            idxname: if self.idxname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.idxname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            accessMethod: if self.accessMethod.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.accessMethod)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            tableSpace: if self.tableSpace.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.tableSpace)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            indexParams: if self.indexParams.is_null() {
                None
            } else {
                match unsafe { self.indexParams.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            indexIncludingParams: if self.indexIncludingParams.is_null() {
                None
            } else {
                match unsafe { self.indexIncludingParams.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            whereClause: if self.whereClause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.whereClause.as_ref().unwrap().convert()
                }))
            },
            excludeOpNames: if self.excludeOpNames.is_null() {
                None
            } else {
                match unsafe { self.excludeOpNames.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            idxcomment: if self.idxcomment.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.idxcomment)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            indexOid: self.indexOid as crate::sys::Oid,
            oldNode: self.oldNode as crate::sys::Oid,
            oldCreateSubid: self.oldCreateSubid as crate::sys::SubTransactionId,
            oldFirstRelfilenodeSubid: self.oldFirstRelfilenodeSubid as crate::sys::SubTransactionId,
            unique: self.unique as bool,
            primary: self.primary as bool,
            isconstraint: self.isconstraint as bool,
            deferrable: self.deferrable as bool,
            initdeferred: self.initdeferred as bool,
            transformed: self.transformed as bool,
            concurrent: self.concurrent as bool,
            if_not_exists: self.if_not_exists as bool,
            reset_default_tblspc: self.reset_default_tblspc as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateStatsStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateStatsStmt(CreateStatsStmt {
            defnames: if self.defnames.is_null() {
                None
            } else {
                match unsafe { self.defnames.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            stat_types: if self.stat_types.is_null() {
                None
            } else {
                match unsafe { self.stat_types.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            exprs: if self.exprs.is_null() {
                None
            } else {
                match unsafe { self.exprs.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            relations: if self.relations.is_null() {
                None
            } else {
                match unsafe { self.relations.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            stxcomment: if self.stxcomment.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.stxcomment)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            if_not_exists: self.if_not_exists as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterStatsStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterStatsStmt(AlterStatsStmt {
            defnames: if self.defnames.is_null() {
                None
            } else {
                match unsafe { self.defnames.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            stxstattarget: self.stxstattarget as i32,
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateFunctionStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateFunctionStmt(CreateFunctionStmt {
            is_procedure: self.is_procedure as bool,
            replace: self.replace as bool,
            funcname: if self.funcname.is_null() {
                None
            } else {
                match unsafe { self.funcname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            parameters: if self.parameters.is_null() {
                None
            } else {
                match unsafe { self.parameters.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            returnType: if self.returnType.is_null() {
                None
            } else {
                match unsafe { self.returnType.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(returnType),
                        stringify!(TypeName)
                    ),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::FunctionParameter {
    fn convert(&self) -> crate::nodes::Node {
        Node::FunctionParameter(FunctionParameter {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            argType: if self.argType.is_null() {
                None
            } else {
                match unsafe { self.argType.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(argType),
                        stringify!(TypeName)
                    ),
                }
            },
            mode: self.mode as crate::sys::FunctionParameterMode,
            defexpr: if self.defexpr.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.defexpr.as_ref().unwrap().convert()
                }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterFunctionStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterFunctionStmt(AlterFunctionStmt {
            objtype: self.objtype as crate::sys::ObjectType,
            func: if self.func.is_null() {
                None
            } else {
                match unsafe { self.func.as_ref().unwrap().convert() } {
                    crate::nodes::Node::ObjectWithArgs(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(func),
                        stringify!(ObjectWithArgs)
                    ),
                }
            },
            actions: if self.actions.is_null() {
                None
            } else {
                match unsafe { self.actions.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DoStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DoStmt(DoStmt {
            args: if self.args.is_null() {
                None
            } else {
                match unsafe { self.args.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::InlineCodeBlock {
    fn convert(&self) -> crate::nodes::Node {
        Node::InlineCodeBlock(InlineCodeBlock {
            source_text: if self.source_text.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.source_text)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            langOid: self.langOid as crate::sys::Oid,
            langIsTrusted: self.langIsTrusted as bool,
            atomic: self.atomic as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CallStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CallStmt(CallStmt {
            funccall: if self.funccall.is_null() {
                None
            } else {
                match unsafe { self.funccall.as_ref().unwrap().convert() } {
                    crate::nodes::Node::FuncCall(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(funccall),
                        stringify!(FuncCall)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CallContext {
    fn convert(&self) -> crate::nodes::Node {
        Node::CallContext(CallContext {
            atomic: self.atomic as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RenameStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::RenameStmt(RenameStmt {
            renameType: self.renameType as crate::sys::ObjectType,
            relationType: self.relationType as crate::sys::ObjectType,
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            object: if self.object.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.object.as_ref().unwrap().convert() }))
            },
            subname: if self.subname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.subname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            newname: if self.newname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.newname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            behavior: self.behavior as crate::sys::DropBehavior,
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterObjectDependsStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterObjectDependsStmt(AlterObjectDependsStmt {
            objectType: self.objectType as crate::sys::ObjectType,
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            object: if self.object.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.object.as_ref().unwrap().convert() }))
            },
            extname: if self.extname.is_null() {
                None
            } else {
                match unsafe { self.extname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::Value(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(extname),
                        stringify!(Value)
                    ),
                }
            },
            remove: self.remove as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterObjectSchemaStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterObjectSchemaStmt(AlterObjectSchemaStmt {
            objectType: self.objectType as crate::sys::ObjectType,
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            object: if self.object.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.object.as_ref().unwrap().convert() }))
            },
            newschema: if self.newschema.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.newschema)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterOwnerStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterOwnerStmt(AlterOwnerStmt {
            objectType: self.objectType as crate::sys::ObjectType,
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            object: if self.object.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.object.as_ref().unwrap().convert() }))
            },
            newowner: if self.newowner.is_null() {
                None
            } else {
                match unsafe { self.newowner.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(newowner),
                        stringify!(RoleSpec)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterOperatorStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterOperatorStmt(AlterOperatorStmt {
            opername: if self.opername.is_null() {
                None
            } else {
                match unsafe { self.opername.as_ref().unwrap().convert() } {
                    crate::nodes::Node::ObjectWithArgs(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(opername),
                        stringify!(ObjectWithArgs)
                    ),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterTypeStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterTypeStmt(AlterTypeStmt {
            typeName: if self.typeName.is_null() {
                None
            } else {
                match unsafe { self.typeName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RuleStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::RuleStmt(RuleStmt {
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            rulename: if self.rulename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.rulename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            whereClause: if self.whereClause.is_null() {
                None
            } else {
                Some(Box::new(unsafe {
                    self.whereClause.as_ref().unwrap().convert()
                }))
            },
            event: self.event as crate::sys::CmdType,
            instead: self.instead as bool,
            actions: if self.actions.is_null() {
                None
            } else {
                match unsafe { self.actions.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            replace: self.replace as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::NotifyStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::NotifyStmt(NotifyStmt {
            conditionname: if self.conditionname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.conditionname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            payload: if self.payload.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.payload)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ListenStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ListenStmt(ListenStmt {
            conditionname: if self.conditionname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.conditionname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::UnlistenStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::UnlistenStmt(UnlistenStmt {
            conditionname: if self.conditionname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.conditionname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::TransactionStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::TransactionStmt(TransactionStmt {
            kind: self.kind as crate::sys::TransactionStmtKind,
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            savepoint_name: if self.savepoint_name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.savepoint_name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            gid: if self.gid.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.gid)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            chain: self.chain as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CompositeTypeStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CompositeTypeStmt(CompositeTypeStmt {
            typevar: if self.typevar.is_null() {
                None
            } else {
                match unsafe { self.typevar.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(typevar),
                        stringify!(RangeVar)
                    ),
                }
            },
            coldeflist: if self.coldeflist.is_null() {
                None
            } else {
                match unsafe { self.coldeflist.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateEnumStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateEnumStmt(CreateEnumStmt {
            typeName: if self.typeName.is_null() {
                None
            } else {
                match unsafe { self.typeName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            vals: if self.vals.is_null() {
                None
            } else {
                match unsafe { self.vals.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateRangeStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateRangeStmt(CreateRangeStmt {
            typeName: if self.typeName.is_null() {
                None
            } else {
                match unsafe { self.typeName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            params: if self.params.is_null() {
                None
            } else {
                match unsafe { self.params.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterEnumStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterEnumStmt(AlterEnumStmt {
            typeName: if self.typeName.is_null() {
                None
            } else {
                match unsafe { self.typeName.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            oldVal: if self.oldVal.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.oldVal)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            newVal: if self.newVal.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.newVal)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            newValNeighbor: if self.newValNeighbor.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.newValNeighbor)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            newValIsAfter: self.newValIsAfter as bool,
            skipIfNewValExists: self.skipIfNewValExists as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ViewStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ViewStmt(ViewStmt {
            view: if self.view.is_null() {
                None
            } else {
                match unsafe { self.view.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(view),
                        stringify!(RangeVar)
                    ),
                }
            },
            aliases: if self.aliases.is_null() {
                None
            } else {
                match unsafe { self.aliases.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            query: if self.query.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.query.as_ref().unwrap().convert() }))
            },
            replace: self.replace as bool,
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            withCheckOption: self.withCheckOption as crate::sys::ViewCheckOption,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::LoadStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::LoadStmt(LoadStmt {
            filename: if self.filename.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.filename)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreatedbStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreatedbStmt(CreatedbStmt {
            dbname: if self.dbname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.dbname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterDatabaseStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterDatabaseStmt(AlterDatabaseStmt {
            dbname: if self.dbname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.dbname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterDatabaseSetStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterDatabaseSetStmt(AlterDatabaseSetStmt {
            dbname: if self.dbname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.dbname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            setstmt: if self.setstmt.is_null() {
                None
            } else {
                match unsafe { self.setstmt.as_ref().unwrap().convert() } {
                    crate::nodes::Node::VariableSetStmt(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(setstmt),
                        stringify!(VariableSetStmt)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DropdbStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DropdbStmt(DropdbStmt {
            dbname: if self.dbname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.dbname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            missing_ok: self.missing_ok as bool,
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterSystemStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterSystemStmt(AlterSystemStmt {
            setstmt: if self.setstmt.is_null() {
                None
            } else {
                match unsafe { self.setstmt.as_ref().unwrap().convert() } {
                    crate::nodes::Node::VariableSetStmt(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(setstmt),
                        stringify!(VariableSetStmt)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ClusterStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ClusterStmt(ClusterStmt {
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            indexname: if self.indexname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.indexname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: self.options as i32,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::VacuumStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::VacuumStmt(VacuumStmt {
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            rels: if self.rels.is_null() {
                None
            } else {
                match unsafe { self.rels.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            is_vacuumcmd: self.is_vacuumcmd as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::VacuumRelation {
    fn convert(&self) -> crate::nodes::Node {
        Node::VacuumRelation(VacuumRelation {
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            oid: self.oid as crate::sys::Oid,
            va_cols: if self.va_cols.is_null() {
                None
            } else {
                match unsafe { self.va_cols.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ExplainStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ExplainStmt(ExplainStmt {
            query: if self.query.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.query.as_ref().unwrap().convert() }))
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateTableAsStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateTableAsStmt(CreateTableAsStmt {
            query: if self.query.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.query.as_ref().unwrap().convert() }))
            },
            into: if self.into.is_null() {
                None
            } else {
                match unsafe { self.into.as_ref().unwrap().convert() } {
                    crate::nodes::Node::IntoClause(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(into),
                        stringify!(IntoClause)
                    ),
                }
            },
            relkind: self.relkind as crate::sys::ObjectType,
            is_select_into: self.is_select_into as bool,
            if_not_exists: self.if_not_exists as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::RefreshMatViewStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::RefreshMatViewStmt(RefreshMatViewStmt {
            concurrent: self.concurrent as bool,
            skipData: self.skipData as bool,
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CheckPointStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CheckPointStmt(CheckPointStmt {})
    }
}
impl crate::convert::ConvertNode for crate::sys::DiscardStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DiscardStmt(DiscardStmt {
            target: self.target as crate::sys::DiscardMode,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::LockStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::LockStmt(LockStmt {
            relations: if self.relations.is_null() {
                None
            } else {
                match unsafe { self.relations.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            mode: self.mode as i32,
            nowait: self.nowait as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ConstraintsSetStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ConstraintsSetStmt(ConstraintsSetStmt {
            constraints: if self.constraints.is_null() {
                None
            } else {
                match unsafe { self.constraints.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            deferred: self.deferred as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ReindexStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ReindexStmt(ReindexStmt {
            kind: self.kind as crate::sys::ReindexObjectType,
            relation: if self.relation.is_null() {
                None
            } else {
                match unsafe { self.relation.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RangeVar(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(relation),
                        stringify!(RangeVar)
                    ),
                }
            },
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: self.options as i32,
            concurrent: self.concurrent as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateConversionStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateConversionStmt(CreateConversionStmt {
            conversion_name: if self.conversion_name.is_null() {
                None
            } else {
                match unsafe { self.conversion_name.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            for_encoding_name: if self.for_encoding_name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.for_encoding_name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            to_encoding_name: if self.to_encoding_name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.to_encoding_name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            func_name: if self.func_name.is_null() {
                None
            } else {
                match unsafe { self.func_name.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            def: self.def as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateCastStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateCastStmt(CreateCastStmt {
            sourcetype: if self.sourcetype.is_null() {
                None
            } else {
                match unsafe { self.sourcetype.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(sourcetype),
                        stringify!(TypeName)
                    ),
                }
            },
            targettype: if self.targettype.is_null() {
                None
            } else {
                match unsafe { self.targettype.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(targettype),
                        stringify!(TypeName)
                    ),
                }
            },
            func: if self.func.is_null() {
                None
            } else {
                match unsafe { self.func.as_ref().unwrap().convert() } {
                    crate::nodes::Node::ObjectWithArgs(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(func),
                        stringify!(ObjectWithArgs)
                    ),
                }
            },
            context: self.context as crate::sys::CoercionContext,
            inout: self.inout as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateTransformStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateTransformStmt(CreateTransformStmt {
            replace: self.replace as bool,
            type_name: if self.type_name.is_null() {
                None
            } else {
                match unsafe { self.type_name.as_ref().unwrap().convert() } {
                    crate::nodes::Node::TypeName(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(type_name),
                        stringify!(TypeName)
                    ),
                }
            },
            lang: if self.lang.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.lang)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            fromsql: if self.fromsql.is_null() {
                None
            } else {
                match unsafe { self.fromsql.as_ref().unwrap().convert() } {
                    crate::nodes::Node::ObjectWithArgs(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(fromsql),
                        stringify!(ObjectWithArgs)
                    ),
                }
            },
            tosql: if self.tosql.is_null() {
                None
            } else {
                match unsafe { self.tosql.as_ref().unwrap().convert() } {
                    crate::nodes::Node::ObjectWithArgs(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(tosql),
                        stringify!(ObjectWithArgs)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::PrepareStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::PrepareStmt(PrepareStmt {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            argtypes: if self.argtypes.is_null() {
                None
            } else {
                match unsafe { self.argtypes.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            query: if self.query.is_null() {
                None
            } else {
                Some(Box::new(unsafe { self.query.as_ref().unwrap().convert() }))
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ExecuteStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ExecuteStmt(ExecuteStmt {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            params: if self.params.is_null() {
                None
            } else {
                match unsafe { self.params.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DeallocateStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DeallocateStmt(DeallocateStmt {
            name: if self.name.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.name)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DropOwnedStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DropOwnedStmt(DropOwnedStmt {
            roles: if self.roles.is_null() {
                None
            } else {
                match unsafe { self.roles.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            behavior: self.behavior as crate::sys::DropBehavior,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::ReassignOwnedStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::ReassignOwnedStmt(ReassignOwnedStmt {
            roles: if self.roles.is_null() {
                None
            } else {
                match unsafe { self.roles.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            newrole: if self.newrole.is_null() {
                None
            } else {
                match unsafe { self.newrole.as_ref().unwrap().convert() } {
                    crate::nodes::Node::RoleSpec(value) => Some(Box::new(value)),
                    _ => panic!(
                        "{} didn't convert to {}",
                        stringify!(newrole),
                        stringify!(RoleSpec)
                    ),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterTSDictionaryStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterTSDictionaryStmt(AlterTSDictionaryStmt {
            dictname: if self.dictname.is_null() {
                None
            } else {
                match unsafe { self.dictname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterTSConfigurationStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterTSConfigurationStmt(AlterTSConfigurationStmt {
            kind: self.kind as crate::sys::AlterTSConfigType,
            cfgname: if self.cfgname.is_null() {
                None
            } else {
                match unsafe { self.cfgname.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            tokentype: if self.tokentype.is_null() {
                None
            } else {
                match unsafe { self.tokentype.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            dicts: if self.dicts.is_null() {
                None
            } else {
                match unsafe { self.dicts.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            override_: self.override_ as bool,
            replace: self.replace as bool,
            missing_ok: self.missing_ok as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreatePublicationStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreatePublicationStmt(CreatePublicationStmt {
            pubname: if self.pubname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.pubname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            tables: if self.tables.is_null() {
                None
            } else {
                match unsafe { self.tables.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            for_all_tables: self.for_all_tables as bool,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterPublicationStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterPublicationStmt(AlterPublicationStmt {
            pubname: if self.pubname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.pubname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            tables: if self.tables.is_null() {
                None
            } else {
                match unsafe { self.tables.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            for_all_tables: self.for_all_tables as bool,
            tableAction: self.tableAction as crate::sys::DefElemAction,
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::CreateSubscriptionStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::CreateSubscriptionStmt(CreateSubscriptionStmt {
            subname: if self.subname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.subname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            conninfo: if self.conninfo.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.conninfo)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            publication: if self.publication.is_null() {
                None
            } else {
                match unsafe { self.publication.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::AlterSubscriptionStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::AlterSubscriptionStmt(AlterSubscriptionStmt {
            kind: self.kind as crate::sys::AlterSubscriptionType,
            subname: if self.subname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.subname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            conninfo: if self.conninfo.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.conninfo)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            publication: if self.publication.is_null() {
                None
            } else {
                match unsafe { self.publication.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
            options: if self.options.is_null() {
                None
            } else {
                match unsafe { self.options.as_ref().unwrap().convert() } {
                    crate::nodes::Node::List(list) => Some(list),
                    _ => panic!("not a List!"),
                }
            },
        })
    }
}
impl crate::convert::ConvertNode for crate::sys::DropSubscriptionStmt {
    fn convert(&self) -> crate::nodes::Node {
        Node::DropSubscriptionStmt(DropSubscriptionStmt {
            subname: if self.subname.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(self.subname)
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
            },
            missing_ok: self.missing_ok as bool,
            behavior: self.behavior as crate::sys::DropBehavior,
        })
    }
}
