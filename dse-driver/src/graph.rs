use crate::{Ptr, PtrMut};
use dse_driver_derive::{Ptr, PtrMut};
// Plain imports
use dse_driver_sys::{
    cass_future_error_code, cass_future_free, cass_int64_t, cass_session_close,
    cass_session_connect, cass_session_connect_keyspace, cass_session_execute,
    cass_session_execute_batch, cass_session_execute_dse_graph, cass_session_free,
    cass_session_get_client_id, cass_session_get_metrics, cass_session_get_schema_meta,
    cass_session_get_speculative_execution_metrics, cass_session_new, cass_session_prepare,
    cass_session_prepare_from_existing, dse_graph_options_free, dse_graph_options_new,
    dse_graph_options_new_from_existing, dse_graph_options_set_graph_language,
    dse_graph_options_set_graph_name, dse_graph_options_set_graph_source,
    dse_graph_options_set_read_consistency, dse_graph_options_set_request_timeout, CassBatch,
    CassCluster, CassFuture, CassMetrics, CassSchemaMeta, CassSession,
    CassSpeculativeExecutionMetrics, CassStatement, CassUuid, DseGraphObject,
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
    CassError__CASS_OK as CASS_OK, DseGraphOptions as _DseGraphOptions,
    DseGraphStatement as _DseGraphStatement,
};

use std::ffi::CString;
use std::mem::MaybeUninit;
use std::time::Duration;

type GraphError = ();

type Result<T> = std::result::Result<T, GraphError>;

/// Checks the outcome of a `CassFuture`.
fn future_result(fut: *mut CassFuture) -> Result<()> {
    if unsafe { cass_future_error_code(fut) } == CASS_OK {
        Ok(())
    } else {
        Err(())
    }
}

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
#[derive(Ptr, PtrMut)]
#[ptr_type(_DseGraphOptions)]
pub struct DseGraphOptions {
    _ptr: *mut _DseGraphOptions,
}

#[derive(Ptr)]
#[ptr_type(_DseGraphStatement)]
pub struct DseGraphStatement {
    _ptr: *mut _DseGraphStatement,
}

#[derive(Ptr)]
#[ptr_type(CassCluster)]
pub struct Cluster {
    _ptr: *mut CassCluster,
}

impl Cluster {
    pub fn connect(&self) -> Result<Session> {
        Session::new(self.ptr())
    }

    pub fn connect_with_keyspace(&self, keyspace: &str) -> Result<Session> {
        Session::new_with_keyspace(self.ptr(), keyspace)
    }
}

#[derive(Ptr, PtrMut)]
#[ptr_type(CassSession)]
pub struct Session {
    // FIXME: seems like we should also store a `DseGraphOptions` in here.
    _ptr: *mut CassSession,
    /// `CassFutures` which should be freed during `Drop`.
    futures: Vec<*mut CassFuture>,
}

#[derive(Ptr, PtrMut)]
#[ptr_type(CassStatement)]
pub struct Statement {
    _ptr: *mut CassStatement,
}

#[derive(Ptr)]
#[ptr_type(CassBatch)]
pub struct Batch {
    _ptr: *mut CassBatch,
}

#[derive(Ptr)]
#[ptr_type(CassSchemaMeta)]
pub struct SchemaMeta {
    _ptr: *const CassSchemaMeta,
}

/// Note that `new()` and `new_with_keyspace()` are private to help ensure a
/// `Cluster` is handling the init for a `Session`.
impl Session {
    /// Create a session connected to a cluster.
    fn new(cluster: *const CassCluster) -> Result<Self> {
        let mut sess = Self {
            _ptr: unsafe { cass_session_new() },
            futures: vec![],
        };
        sess.connect(cluster).and_then(|_| Ok(sess))
    }
    /// Same as `Session::new()` but also sets the default keyspace queries made
    /// through this `Session`.
    fn new_with_keyspace(cluster: *const CassCluster, keyspace: &str) -> Result<Self> {
        let mut sess = Self {
            _ptr: unsafe { cass_session_new() },
            futures: vec![],
        };

        sess.connect_keyspace(cluster, keyspace)
            .and_then(|_| Ok(sess))
    }

    fn connect(&mut self, cluster: *const CassCluster) -> Result<()> {
        let fut = unsafe { cass_session_connect(self.ptr_mut(), cluster) };
        // FIXME: check to see if we can free this future immediately...
        self.futures.push(fut);
        future_result(fut)
    }

    fn connect_keyspace(&mut self, cluster: *const CassCluster, keyspace: &str) -> Result<()> {
        let keyspace = CString::new(keyspace).unwrap();
        let fut =
            unsafe { cass_session_connect_keyspace(self.ptr_mut(), cluster, keyspace.as_ptr()) };
        self.futures.push(fut);
        future_result(fut)
    }

    pub fn close(&mut self) {
        let _fut = unsafe { cass_session_close(self.ptr_mut()) };
        unimplemented!();
    }

    fn prepare(&mut self, query: &str) {
        let query = CString::new(query).unwrap();
        let _fut = unsafe { cass_session_prepare(self.ptr_mut(), query.as_ptr()) };
        unimplemented!();
    }

    fn prepare_from_existing(&mut self, statement: &mut Statement) {
        let _fut =
            unsafe { cass_session_prepare_from_existing(self.ptr_mut(), statement.ptr_mut()) };
        unimplemented!();
    }

    fn execute(&mut self, statement: &Statement) {
        let _fut = unsafe { cass_session_execute(self.ptr_mut(), statement.ptr()) };
        unimplemented!();
    }

    fn execute_batch(&mut self, batch: &Batch) {
        let _fut = unsafe { cass_session_execute_batch(self.ptr_mut(), batch.ptr()) };
        unimplemented!();
    }

    fn get_schema_meta(&self) -> SchemaMeta {
        let meta: *const CassSchemaMeta = unsafe { cass_session_get_schema_meta(self.ptr()) };
        SchemaMeta { _ptr: meta }
    }

    fn get_metrics(&mut self) -> CassMetrics {
        let mut metrics = MaybeUninit::uninit();
        unsafe {
            cass_session_get_metrics(self.ptr(), metrics.as_mut_ptr());
            metrics.assume_init()
        }
    }

    fn get_speculative_execution_metrics(&mut self) -> CassSpeculativeExecutionMetrics {
        let mut se_metrics = MaybeUninit::uninit();
        unsafe {
            cass_session_get_speculative_execution_metrics(self.ptr(), se_metrics.as_mut_ptr());
            se_metrics.assume_init()
        }
    }

    pub fn execute_graph<V>(&mut self, query: &str, values: V) -> Result<()>
    where
        V: AsRef<DseGraphObject>, // FIXME: need to use a wrapper for the object type
    {
        Err(())
    }

    fn execute_dse_graph(&mut self, statement: &DseGraphStatement) {
        let _fut = unsafe { cass_session_execute_dse_graph(self.ptr_mut(), statement.ptr()) };
        unimplemented!();
    }

    pub fn get_client_id(&mut self) -> CassUuid {
        unsafe { cass_session_get_client_id(self.ptr_mut()) }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            for fut in self.futures.drain(..) {
                cass_future_free(fut);
            }
            cass_session_free(self.ptr_mut());
        }
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
