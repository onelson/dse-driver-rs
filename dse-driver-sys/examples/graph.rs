//! Port of
//! <https://github.com/datastax/cpp-dse-driver-examples/blob/master/dse/examples/graph/graph.c>
//! in an attempt to get a basic graph connection working.
//!

extern crate dse_driver_sys;

const GRAPH_ALLOW_SCANS: &str = "schema.config().option('graph.allow_scan').set('true')";

const GRAPH_MAKE_STRICT: &str = "\
schema.config().option('graph.schema_mode').set(com.datastax.bdp.graph.api.model.Schema.Mode.Production)";

const GRAPH_SCHEMA: &str = "
schema.propertyKey('name').Text().ifNotExists().create();
schema.propertyKey('age').Int().ifNotExists().create();
schema.propertyKey('lang').Text().ifNotExists().create();
schema.propertyKey('weight').Float().ifNotExists().create();
schema.vertexLabel('person').properties('name', 'age').ifNotExists().create();
schema.vertexLabel('software').properties('name', 'lang').ifNotExists().create();
schema.edgeLabel('created').properties('weight').connection('person', 'software').ifNotExists().create();
schema.edgeLabel('created').connection('software', 'software').add();
schema.edgeLabel('knows').properties('weight').connection('person', 'person').ifNotExists().create();
";

const GRAPH_DATA: &str = "
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
";

use dse_driver_sys::{
    cass_bool_t, CassFuture, CassSession, DseGraphObject, DseGraphOptions, DseGraphResult,
    DseGraphResultSet,
};
use std::os::raw::{c_char, c_int};

unsafe fn print_error(future: *const CassFuture) {
    unimplemented!();
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
    session: *const CassSession,
    query: *const c_char,
    options: *const DseGraphOptions,
    values: *const DseGraphObject,
    resultset: *const *const DseGraphResultSet,
) -> cass_bool_t {
    unimplemented!();
}

unsafe fn create_graph(session: *const CassSession, name: *const char) -> cass_bool_t {
    unimplemented!();
}

unsafe fn execute_graph_query_and_print(
    session: *const CassSession,
    query: *const c_char,
    options: *const DseGraphOptions,
    values: *const DseGraphObject,
) {
    unimplemented!();
}

fn main() {}
