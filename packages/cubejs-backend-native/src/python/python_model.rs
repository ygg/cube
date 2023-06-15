use convert_case::{Case, Casing};
use neon::prelude::*;
use pyo3::exceptions::PyTypeError;
use pyo3::types::PyFunction;
use pyo3::{Py, PyAny, PyErr, PyResult};

use crate::python::cross::{CLRepr, CLReprObject};

pub struct CubePythonModel {
    functions: Vec<(String, Py<PyFunction>)>,
}

impl CubePythonModel {
    pub fn new(functions: Vec<(String, Py<PyFunction>)>) -> Self {
        Self { functions }
    }
}

impl Finalize for CubePythonModel {}

impl CubePythonModel {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_object<'a, C: Context<'a>>(self, cx: &mut C) -> JsResult<'a, JsValue> {
        let mut obj_rep = CLReprObject::new();

        for (fn_name, _fun) in self.functions {
            obj_rep.insert(fn_name, CLRepr::Bool(true));
        }

        let obj = CLRepr::Object(obj_rep);
        obj.into_js(cx)
    }
}
