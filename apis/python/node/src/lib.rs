use dora_node_api::{config::NodeId, DoraNode, Input};
use eyre::{Context, Result};
use flume::Receiver;
use pyo3::{prelude::*, types::PyBytes};

#[pyclass]
pub struct Node {
    id: NodeId,
    inputs: Receiver<Input>,
    node: DoraNode,
}

pub struct PyInput(Input);

impl IntoPy<PyObject> for PyInput {
    fn into_py(self, py: Python) -> PyObject {
        (self.0.id.to_string(), PyBytes::new(py, &self.0.data)).into_py(py)
    }
}

#[pymethods]
impl Node {
    #[new]
    pub fn new() -> Result<Self> {
        let id = {
            let raw =
                std::env::var("DORA_NODE_ID").wrap_err("env variable DORA_NODE_ID must be set")?;
            serde_yaml::from_str(&raw).context("failed to deserialize operator config")?
        };

        let mut node = DoraNode::init_from_env()?;
        let inputs = node.inputs()?;

        Ok(Node { id, inputs, node })
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> PyResult<Option<PyInput>> {
        self.__next__()
    }

    pub fn __next__(&mut self) -> PyResult<Option<PyInput>> {
        Ok(self.inputs.recv().ok().map(PyInput))
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn send_output(&mut self, output_id: String, data: &PyBytes) -> Result<()> {
        self.node
            .send_output(&output_id.into(), data.as_bytes())
            .wrap_err("Could not send output")
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

#[pymodule]
fn dora(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Node>().unwrap();
    Ok(())
}
