use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    net::SocketAddr,
    path::PathBuf,
};

use crate::{
    config::{DataId, NodeId, NodeRunConfig, OperatorId},
    descriptor::{Descriptor, OperatorDefinition, ResolvedNode},
};
use dora_message::{uhlc, Metadata};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NodeConfig {
    pub dataflow_id: DataflowId,
    pub node_id: NodeId,
    pub run_config: NodeRunConfig,
    pub daemon_communication: DaemonCommunication,
    pub dataflow_descriptor: Descriptor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DaemonCommunication {
    Shmem {
        daemon_control_region_id: SharedMemoryId,
        daemon_drop_region_id: SharedMemoryId,
        daemon_events_region_id: SharedMemoryId,
        daemon_events_close_region_id: SharedMemoryId,
    },
    Tcp {
        socket_addr: SocketAddr,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RuntimeConfig {
    pub node: NodeConfig,
    pub operators: Vec<OperatorDefinition>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum DaemonRequest {
    Register {
        dataflow_id: DataflowId,
        node_id: NodeId,
        dora_version: String,
    },
    Subscribe,
    SendMessage {
        output_id: DataId,
        metadata: Metadata,
        data: Option<Data>,
    },
    CloseOutputs(Vec<DataId>),
    /// Signals that the node is finished sending outputs and that it received all
    /// required drop tokens.
    OutputsDone,
    NextEvent {
        drop_tokens: Vec<DropToken>,
    },
    ReportDropTokens {
        drop_tokens: Vec<DropToken>,
    },
    SubscribeDrop,
    NextFinishedDropTokens,
    EventStreamDropped,
}

impl DaemonRequest {
    pub fn expects_tcp_reply(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            DaemonRequest::SendMessage { .. } | DaemonRequest::ReportDropTokens { .. } => false,
            DaemonRequest::Register { .. }
            | DaemonRequest::Subscribe
            | DaemonRequest::CloseOutputs(_)
            | DaemonRequest::OutputsDone
            | DaemonRequest::NextEvent { .. }
            | DaemonRequest::SubscribeDrop
            | DaemonRequest::NextFinishedDropTokens
            | DaemonRequest::EventStreamDropped => true,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum Data {
    #[serde(with = "serde_bytes")]
    Vec(Vec<u8>),
    SharedMemory {
        shared_memory_id: String,
        len: usize,
        drop_token: DropToken,
    },
}

impl Data {
    pub fn drop_token(&self) -> Option<DropToken> {
        match self {
            Data::Vec(_) => None,
            Data::SharedMemory { drop_token, .. } => Some(*drop_token),
        }
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vec(v) => f
                .debug_struct("Vec")
                .field("len", &v.len())
                .finish_non_exhaustive(),
            Self::SharedMemory {
                shared_memory_id,
                len,
                drop_token,
            } => f
                .debug_struct("SharedMemory")
                .field("shared_memory_id", shared_memory_id)
                .field("len", len)
                .field("drop_token", drop_token)
                .finish(),
        }
    }
}

type SharedMemoryId = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[must_use]
pub enum DaemonReply {
    Result(Result<(), String>),
    PreparedMessage { shared_memory_id: SharedMemoryId },
    NextEvents(Vec<Timestamped<NodeEvent>>),
    NextDropEvents(Vec<Timestamped<NodeDropEvent>>),
    Empty,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Timestamped<T> {
    pub inner: T,
    pub timestamp: uhlc::Timestamp,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum NodeEvent {
    Stop,
    Reload {
        operator_id: Option<OperatorId>,
    },
    Input {
        id: DataId,
        metadata: Metadata,
        data: Option<Data>,
    },
    InputClosed {
        id: DataId,
    },
    AllInputsClosed,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum NodeDropEvent {
    OutputDropped { drop_token: DropToken },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DropEvent {
    pub tokens: Vec<DropToken>,
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct DropToken(Uuid);

impl DropToken {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum InputData {
    SharedMemory(SharedMemoryInput),
    Vec(Vec<u8>),
}

impl InputData {
    pub fn drop_token(&self) -> Option<DropToken> {
        match self {
            InputData::SharedMemory(data) => Some(data.drop_token),
            InputData::Vec(_) => None,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SharedMemoryInput {
    pub shared_memory_id: SharedMemoryId,
    pub len: usize,
    pub drop_token: DropToken,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum DaemonCoordinatorEvent {
    Spawn(SpawnDataflowNodes),
    AllNodesReady {
        dataflow_id: DataflowId,
        success: bool,
    },
    StopDataflow {
        dataflow_id: DataflowId,
    },
    ReloadDataflow {
        dataflow_id: DataflowId,
        node_id: NodeId,
        operator_id: Option<OperatorId>,
    },
    Logs {
        dataflow_id: DataflowId,
        node_id: NodeId,
    },
    Destroy,
    Heartbeat,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum InterDaemonEvent {
    Output {
        dataflow_id: DataflowId,
        node_id: NodeId,
        output_id: DataId,
        metadata: Metadata,
        data: Option<Vec<u8>>,
    },
    InputsClosed {
        dataflow_id: DataflowId,
        inputs: BTreeSet<(NodeId, DataId)>,
    },
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum DaemonCoordinatorReply {
    SpawnResult(Result<(), String>),
    ReloadResult(Result<(), String>),
    StopResult(Result<(), String>),
    DestroyResult(Result<(), String>),
    Logs(Result<Vec<u8>, String>),
}

pub type DataflowId = Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SpawnDataflowNodes {
    pub dataflow_id: DataflowId,
    pub working_dir: PathBuf,
    pub nodes: Vec<ResolvedNode>,
    pub machine_listen_ports: BTreeMap<String, SocketAddr>,
    pub dataflow_descriptor: Descriptor,
}
