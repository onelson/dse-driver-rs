use dse_driver_sys::{
    cass_int64_t, dse_graph_options_free, dse_graph_options_new,
    dse_graph_options_new_from_existing, dse_graph_options_set_graph_language,
    dse_graph_options_set_graph_name, dse_graph_options_set_graph_source,
    dse_graph_options_set_read_consistency, dse_graph_options_set_request_timeout,
    DseGraphOptions as RawDseGraphOptions,
};

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
};
use std::ffi::CString;
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
    ptr: *mut RawDseGraphOptions,
}

impl DseGraphOptions {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { dse_graph_options_new() },
        }
    }
    pub fn new_from_existing(other: &DseGraphOptions) -> Self {
        Self {
            ptr: unsafe { dse_graph_options_new_from_existing(other.ptr) },
        }
    }
    pub fn set_graph_language(&mut self, lang: &str) -> &mut Self {
        let lang = CString::new(lang).unwrap();
        unsafe { dse_graph_options_set_graph_language(self.ptr, lang.as_ptr()) };
        self
    }
    pub fn set_graph_name(&mut self, name: &str) -> &mut Self {
        let name = CString::new(name).unwrap();
        unsafe { dse_graph_options_set_graph_name(self.ptr, name.as_ptr()) };
        self
    }
    pub fn set_graph_source(&mut self, source: &str) -> &mut Self {
        let source = CString::new(source).unwrap();
        unsafe { dse_graph_options_set_graph_source(self.ptr, source.as_ptr()) };
        self
    }
    pub fn set_read_consistency(&mut self, consistency: Consistency) -> &mut Self {
        unsafe { dse_graph_options_set_read_consistency(self.ptr, consistency.into()) };
        self
    }
    pub fn set_request_timeout(&mut self, timeout: Duration) -> &mut Self {
        let timeout_ms = timeout.as_millis() as cass_int64_t;
        unsafe { dse_graph_options_set_request_timeout(self.ptr, timeout_ms) };
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
        unsafe { dse_graph_options_free(self.ptr) }
    }
}
