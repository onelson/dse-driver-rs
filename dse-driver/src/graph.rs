// Plain imports
use dse_driver_sys::{
    cass_int64_t, cass_session_close, cass_session_connect, cass_session_connect_keyspace,
    cass_session_execute, cass_session_execute_batch, cass_session_execute_dse_graph,
    cass_session_free, cass_session_get_client_id, cass_session_get_metrics,
    cass_session_get_schema_meta, cass_session_get_speculative_execution_metrics, cass_session_new,
    cass_session_prepare, cass_session_prepare_from_existing, dse_graph_options_free,
    dse_graph_options_new, dse_graph_options_new_from_existing,
    dse_graph_options_set_graph_language, dse_graph_options_set_graph_name,
    dse_graph_options_set_graph_source, dse_graph_options_set_read_consistency,
    dse_graph_options_set_request_timeout, CassBatch, CassCluster, CassMetrics, CassSchemaMeta,
    CassSession, CassSpeculativeExecutionMetrics, CassStatement, CassUuid,
};

// Aliased imports.
use dse_driver_sys::{
    CassConsistency, CassConsistency__CASS_CONSISTENCY_ALL as CASS_CONSISTENCY_ALL,
    CassConsistency__CASS_CONSISTENCY_ANY as CASS_CONSISTENCY_ANY,
    CassConsistency__CASS_CONSISTENCY_EACH_QUORUM as CASS_CONSISTENCY_EACH_QUORUM,
    CassConsistency__CASS_CONSISTENCY_LOCAL_ONE as CASS_CONSISTENCY_LOCAL_ONE,
    CassConsistency__CASS_CONSISTENCY_LOCAL_QUORUM as CASS_CONSISTENCY_LOCAL_QUORUM,
    CassConsistency__CASS_CONSISTENCY_LOCAL_SERIAL as CASS_CONSISTENCY_LOCAL_SERIAL,
    CassConsistency__CASS_CONSISTENCY_ONE as CASS_CONSISTENCY_ONE,
    CassConsistency__CASS_CONSISTENCY_QUORUM as CASS_CONSISTENCY_QUORUM,
    CassConsistency__CASS_CONSISTENCY_SERIAL as CASS_CONSISTENCY_SERIAL,
    CassConsistency__CASS_CONSISTENCY_THREE as CASS_CONSISTENCY_THREE,
    CassConsistency__CASS_CONSISTENCY_TWO as CASS_CONSISTENCY_TWO,
    CassConsistency__CASS_CONSISTENCY_UNKNOWN as CASS_CONSISTENCY_UNKNOWN,
    DseGraphOptions as _DseGraphOptions, DseGraphStatement as _DseGraphStatement,
};

use std::ffi::CString;
use std::mem::MaybeUninit;
use std::time::Duration;

pub enum Consistency {
    Unknown,
    Any,
    One,
    Two,
    Three,
    Quorum,
    All,
    LocalQuorum,
    EachQuorum,
    Serial,
    LocalSerial,
    LocalOne,
}

impl From<Consistency> for CassConsistency {
    fn from(value: Consistency) -> Self {
        use Consistency::*;
        match value {
            Unknown => CASS_CONSISTENCY_UNKNOWN,
            Any => CASS_CONSISTENCY_ANY,
            One => CASS_CONSISTENCY_ONE,
            Two => CASS_CONSISTENCY_TWO,
            Three => CASS_CONSISTENCY_THREE,
            Quorum => CASS_CONSISTENCY_QUORUM,
            All => CASS_CONSISTENCY_ALL,
            LocalQuorum => CASS_CONSISTENCY_LOCAL_QUORUM,
            EachQuorum => CASS_CONSISTENCY_EACH_QUORUM,
            Serial => CASS_CONSISTENCY_SERIAL,
            LocalSerial => CASS_CONSISTENCY_LOCAL_SERIAL,
            LocalOne => CASS_CONSISTENCY_LOCAL_ONE,
        }
    }
}

trait PtrProxy {
    type T;
    fn ptr(&self) -> *const Self::T;
    fn ptr_mut(&mut self) -> *mut Self::T {
        self.ptr() as *mut Self::T
    }
}

/// Configuration options for a cluster connection.
///
/// ```
/// use dse_driver::graph::{Consistency, DseGraphOptions};
/// use std::time::Duration;
/// let mut options = DseGraphOptions::new();
/// options.set_read_consistency(Consistency::Quorum)
///     .set_graph_language("groovy")
///     .set_graph_name("pokemon")
///     .set_request_timeout(Duration::from_secs(45));
/// ```
pub struct DseGraphOptions {
    _ptr: *mut _DseGraphOptions,
}

pub struct DseGraphStatement {
    _ptr: *mut _DseGraphStatement,
}

pub struct Cluster {
    _ptr: *mut CassCluster,
}

pub struct Session {
    _ptr: *mut CassSession,
}

pub struct Statement {
    _ptr: *mut CassStatement,
}

pub struct Batch {
    _ptr: *mut CassBatch,
}

impl PtrProxy for Cluster {
    type T = CassCluster;
    fn ptr(&self) -> *const Self::T {
        self._ptr
    }
}

impl PtrProxy for Session {
    type T = CassSession;
    fn ptr(&self) -> *const Self::T {
        self._ptr
    }
}

impl PtrProxy for DseGraphOptions {
    type T = _DseGraphOptions;
    fn ptr(&self) -> *const Self::T {
        self._ptr
    }
}

impl PtrProxy for Statement {
    type T = CassStatement;
    fn ptr(&self) -> *const Self::T {
        self._ptr
    }
}

impl PtrProxy for DseGraphStatement {
    type T = _DseGraphStatement;
    fn ptr(&self) -> *const Self::T {
        self._ptr
    }
}

impl PtrProxy for Batch {
    type T = CassBatch;
    fn ptr(&self) -> *const Self::T {
        self._ptr
    }
}

impl Session {
    pub fn new() -> Self {
        Self {
            _ptr: unsafe { cass_session_new() },
        }
    }

    pub fn connect(&mut self, cluster: &Cluster) {
        let _fut = unsafe { cass_session_connect(self.ptr_mut(), cluster.ptr()) };
        unimplemented!();
    }
    pub fn connect_keyspace(&mut self, cluster: &Cluster, keyspace: &str) {
        let keyspace = CString::new(keyspace).unwrap();
        let _fut = unsafe {
            cass_session_connect_keyspace(self.ptr_mut(), cluster.ptr(), keyspace.as_ptr())
        };
        unimplemented!();
    }
    pub fn close(&mut self) {
        let _fut = unsafe { cass_session_close(self.ptr_mut()) };
        unimplemented!();
    }
    pub fn prepare(&mut self, query: &str) {
        let query = CString::new(query).unwrap();
        let _fut = unsafe { cass_session_prepare(self.ptr_mut(), query.as_ptr()) };
        unimplemented!();
    }
    pub fn prepare_from_existing(&mut self, statement: &mut Statement) {
        let _fut =
            unsafe { cass_session_prepare_from_existing(self.ptr_mut(), statement.ptr_mut()) };
        unimplemented!();
    }

    pub fn execute(&mut self, statement: &Statement) {
        let _fut = unsafe { cass_session_execute(self.ptr_mut(), statement.ptr()) };
        unimplemented!();
    }

    pub fn execute_batch(&mut self, batch: &Batch) {
        let _fut = unsafe { cass_session_execute_batch(self.ptr_mut(), batch.ptr()) };
        unimplemented!();
    }
    pub fn get_schema_meta(&self) {
        let _meta: *const CassSchemaMeta = unsafe { cass_session_get_schema_meta(self.ptr()) };
        // FIXME: not sure of what to do with the const pointer we get back from this
        unimplemented!()
    }

    pub fn get_metrics(&mut self) -> CassMetrics {
        let mut metrics = MaybeUninit::uninit();
        unsafe {
            cass_session_get_metrics(self.ptr(), metrics.as_mut_ptr());
            metrics.assume_init()
        }
    }

    pub fn get_speculative_execution_metrics(&mut self) -> CassSpeculativeExecutionMetrics {
        let mut se_metrics = MaybeUninit::uninit();
        unsafe {
            cass_session_get_speculative_execution_metrics(self.ptr(), se_metrics.as_mut_ptr());
            se_metrics.assume_init()
        }
    }

    pub fn execute_dse_graph(&mut self, statement: &DseGraphStatement) {
        let _fut = unsafe { cass_session_execute_dse_graph(self.ptr_mut(), statement.ptr()) };
        unimplemented!();
    }
    pub fn get_client_id(&mut self) -> CassUuid {
        unsafe { cass_session_get_client_id(self.ptr_mut()) }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { cass_session_free(self.ptr_mut()) }
    }
}

impl DseGraphOptions {
    pub fn new() -> Self {
        Self {
            _ptr: unsafe { dse_graph_options_new() },
        }
    }
    pub fn new_from_existing(other: &DseGraphOptions) -> Self {
        Self {
            _ptr: unsafe { dse_graph_options_new_from_existing(other.ptr()) },
        }
    }
    pub fn set_graph_language(&mut self, lang: &str) -> &mut Self {
        let lang = CString::new(lang).unwrap();
        unsafe { dse_graph_options_set_graph_language(self.ptr_mut(), lang.as_ptr()) };
        self
    }
    pub fn set_graph_name(&mut self, name: &str) -> &mut Self {
        let name = CString::new(name).unwrap();
        unsafe { dse_graph_options_set_graph_name(self.ptr_mut(), name.as_ptr()) };
        self
    }
    pub fn set_graph_source(&mut self, source: &str) -> &mut Self {
        let source = CString::new(source).unwrap();
        unsafe { dse_graph_options_set_graph_source(self.ptr_mut(), source.as_ptr()) };
        self
    }
    pub fn set_read_consistency(&mut self, consistency: Consistency) -> &mut Self {
        unsafe { dse_graph_options_set_read_consistency(self.ptr_mut(), consistency.into()) };
        self
    }
    pub fn set_request_timeout(&mut self, timeout: Duration) -> &mut Self {
        let timeout_ms = timeout.as_millis() as cass_int64_t;
        unsafe { dse_graph_options_set_request_timeout(self.ptr_mut(), timeout_ms) };
        self
    }
}

impl Clone for DseGraphOptions {
    fn clone(&self) -> Self {
        DseGraphOptions::new_from_existing(self)
    }
}

impl Drop for DseGraphOptions {
    fn drop(&mut self) {
        unsafe { dse_graph_options_free(self.ptr_mut()) }
    }
}
