//! Port of
//! <https://github.com/datastax/cpp-dse-driver-examples/blob/master/dse/examples/graph/graph.c>
//! in an attempt to get a basic graph connection working.
//!

extern crate dse_driver_sys;

static GRAPH_ALLOW_SCANS: &[u8] = b"schema.config().option('graph.allow_scan').set('true')\0";

static GRAPH_MAKE_STRICT: &[u8] = b"\
schema.config().option('graph.schema_mode').set(com.datastax.bdp.graph.api.model.Schema.Mode.Production)\0";

static GRAPH_SCHEMA: &[u8] = b"
schema.propertyKey('name').Text().ifNotExists().create();
schema.propertyKey('age').Int().ifNotExists().create();
schema.propertyKey('lang').Text().ifNotExists().create();
schema.propertyKey('weight').Float().ifNotExists().create();
schema.vertexLabel('person').properties('name', 'age').ifNotExists().create();
schema.vertexLabel('software').properties('name', 'lang').ifNotExists().create();
schema.edgeLabel('created').properties('weight').connection('person', 'software').ifNotExists().create();
schema.edgeLabel('created').connection('software', 'software').add();
schema.edgeLabel('knows').properties('weight').connection('person', 'person').ifNotExists().create();
\0";

static GRAPH_DATA: &[u8] = b"
Vertex marko = graph.addVertex(label, 'person', 'name', 'marko', 'age', 29);
Vertex vadas = graph.addVertex(label, 'person', 'name', 'vadas', 'age', 27);
Vertex lop = graph.addVertex(label, 'software', 'name', 'lop', 'lang', 'java');
Vertex josh = graph.addVertex(label, 'person', 'name', 'josh', 'age', 32);
Vertex ripple = graph.addVertex(label, 'software', 'name', 'ripple', 'lang', 'java');
Vertex peter = graph.addVertex(label, 'person', 'name', 'peter', 'age', 35);
marko.addEdge('knows', vadas, 'weight', 0.5f);
marko.addEdge('knows', josh, 'weight', 1.0f);
marko.addEdge('created', lop, 'weight', 0.4f);
josh.addEdge('created', ripple, 'weight', 1.0f);
josh.addEdge('created', lop, 'weight', 0.4f);
peter.addEdge('created', lop, 'weight', 0.2f);
\0";

use dse_driver_sys::{
    cass_bool_t, cass_bool_t_cass_false, cass_bool_t_cass_true, cass_cluster_free,
    cass_cluster_new_dse, cass_cluster_set_contact_points, cass_future_error_code,
    cass_future_error_message, cass_future_free, cass_future_get_dse_graph_resultset,
    cass_future_wait, cass_log_set_level, cass_session_close, cass_session_connect,
    cass_session_execute_dse_graph, cass_session_free, cass_session_new,
    dse_graph_object_add_string, dse_graph_object_finish, dse_graph_object_free,
    dse_graph_object_new, dse_graph_options_free, dse_graph_options_new,
    dse_graph_options_set_graph_name, dse_graph_result_get_bool, dse_graph_result_is_bool,
    dse_graph_resultset_count, dse_graph_resultset_free, dse_graph_resultset_next,
    dse_graph_statement_bind_values, dse_graph_statement_free, dse_graph_statement_new,
    CassCluster, CassError__CASS_OK, CassFuture, CassLogLevel__CASS_LOG_INFO, CassSession,
    DseGraphObject, DseGraphOptions, DseGraphResult, DseGraphResultSet,
};
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_int};
use std::ptr;

unsafe fn print_error(future: *mut CassFuture) {
    let mut msg = MaybeUninit::uninit();
    let mut len = MaybeUninit::uninit();
    cass_future_error_message(future, msg.as_mut_ptr(), len.as_mut_ptr());
    let msg = msg.assume_init();
    let len = len.assume_init();
    eprint!("Error:{} {}", len, CStr::from_ptr(msg).to_str().unwrap());
}

// FIXME: in the C code, this function is variadic, accepting positional args to print.
//  Once we look at the usage, we might replace it with some println or something.
unsafe fn print_indented(indent: c_int, format: *const c_char) {
    unimplemented!();
}

unsafe fn print_graph_result(indent: c_int, result: *const DseGraphResult) {
    unimplemented!();
}

unsafe fn print_graph_resultset(resultset: *const DseGraphResultSet) {
    unimplemented!();
}

unsafe fn execute_graph_query(
    session: *mut CassSession,
    query: *const c_char,
    options: *const DseGraphOptions,
    values: *const DseGraphObject,
    resultset: *mut *mut DseGraphResultSet,
) -> cass_bool_t {
    let mut is_success: cass_bool_t = cass_bool_t_cass_false;

    let mut statement = dse_graph_statement_new(query, options);
    dse_graph_statement_bind_values(statement, values);

    let mut future = cass_session_execute_dse_graph(session, statement);

    if cass_future_error_code(future) == CassError__CASS_OK {
        let rs = cass_future_get_dse_graph_resultset(future);
        if !resultset.is_null() {
            resultset.write(rs);
        } else {
            dse_graph_resultset_free(rs);
        }
        is_success = cass_bool_t_cass_true;
    } else {
        print_error(future);
    }
    cass_future_free(future);
    dse_graph_statement_free(statement);
    is_success
}

unsafe fn create_graph(session: *mut CassSession, name: *const c_char) -> cass_bool_t {
    let mut i: usize = 0;
    let mut is_success: cass_bool_t = cass_bool_t_cass_false;
    let mut values: *mut DseGraphObject = dse_graph_object_new();

    dse_graph_object_add_string(
        values,
        CStr::from_bytes_with_nul_unchecked(b"name\0").as_ptr(),
        name,
    );
    dse_graph_object_finish(values);

    let query = CStr::from_bytes_with_nul_unchecked(
        b"graph = system.graph(name); \
         if (graph.exists()) graph.drop(); \
         graph.create();\0",
    );

    if execute_graph_query(
        session,
        query.as_ptr(),
        ptr::null(),
        values,
        ptr::null_mut(),
    ) == cass_bool_t_cass_true
    {
        while i < 10 {
            i += 1;

            // This uninitialized memory gets passed into the C lib so it has
            // some place to write data to. We're just reserving a chunk of memory.
            let mut resultset = std::mem::MaybeUninit::uninit();
            if execute_graph_query(
                session,
                CStr::from_bytes_with_nul_unchecked(b"system.graph(name).exists()\0").as_ptr(),
                ptr::null(),
                values,
                resultset.as_mut_ptr(),
            ) == cass_bool_t_cass_true
            {
                // The value should be safe to use at now.
                let resultset = resultset.assume_init();

                if dse_graph_resultset_count(resultset) > 0 {
                    let result = dse_graph_resultset_next(resultset);
                    if dse_graph_result_is_bool(result) == cass_bool_t_cass_true
                        && dse_graph_result_get_bool(result) == cass_bool_t_cass_true
                    {
                        is_success = cass_bool_t_cass_true;
                        dse_graph_resultset_free(resultset);
                        break;
                    }
                }

                std::thread::sleep(std::time::Duration::from_secs(1));
                dse_graph_resultset_free(resultset);
            }
        }
    }

    dse_graph_object_free(values);
    is_success
}

unsafe fn execute_graph_query_and_print(
    session: *mut CassSession,
    query: *const c_char,
    options: *const DseGraphOptions,
    values: *const DseGraphObject,
) {
    unimplemented!();
}

fn main() {
    unsafe {
        let hosts = CString::new(
            std::env::args()
                .nth(1)
                .unwrap_or_else(|| "127.0.0.1".to_string()),
        )
        .unwrap();

        let cluster: *mut CassCluster = cass_cluster_new_dse();
        let session: *mut CassSession = cass_session_new();

        let graph_name = CStr::from_bytes_with_nul_unchecked(b"classic\0").as_ptr();
        let allow_scans = CStr::from_bytes_with_nul_unchecked(GRAPH_ALLOW_SCANS).as_ptr();
        let make_strict = CStr::from_bytes_with_nul_unchecked(GRAPH_MAKE_STRICT).as_ptr();
        let schema = CStr::from_bytes_with_nul_unchecked(GRAPH_SCHEMA).as_ptr();
        let data = CStr::from_bytes_with_nul_unchecked(GRAPH_DATA).as_ptr();

        cass_log_set_level(CassLogLevel__CASS_LOG_INFO);
        cass_cluster_set_contact_points(cluster, hosts.as_ptr());
        let connect_future = cass_session_connect(session, cluster);
        if cass_future_error_code(connect_future) == CassError__CASS_OK {
            let mut options: *mut DseGraphOptions = dse_graph_options_new();

            dse_graph_options_set_graph_name(options, graph_name);

            if create_graph(session, graph_name) == cass_bool_t_cass_true {
                execute_graph_query(session, allow_scans, options, ptr::null(), ptr::null_mut());
                execute_graph_query(session, make_strict, options, ptr::null(), ptr::null_mut());
                execute_graph_query(session, schema, options, ptr::null(), ptr::null_mut());
                execute_graph_query(session, data, options, ptr::null(), ptr::null_mut());
            }

            dse_graph_options_free(options);

            let close_future = cass_session_close(session);
            cass_future_wait(close_future);
            cass_future_free(close_future);
        }

        cass_future_free(connect_future);
        cass_cluster_free(cluster);
        cass_session_free(session);
    }
}
