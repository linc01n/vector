use futures::{
    future,
    sink::Sink,
    sync::mpsc::{Receiver, Sender},
    Future, Stream,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::{Arc, Mutex};
use vector::buffers::Acker;
use vector::event::{Event, Metric, ValueKind, MESSAGE};
use vector::sinks::{util::SinkExt, Healthcheck, RouterSink};
use vector::sources::Source;
use vector::topology::config::{
    DataType, GlobalOptions, SinkConfig, SourceConfig, TransformConfig,
};
use vector::transforms::Transform;

pub fn sink() -> (Receiver<Event>, MockSinkConfig) {
    let (tx, rx) = futures::sync::mpsc::channel(10);
    let sink = MockSinkConfig::new(tx, true);
    (rx, sink)
}

pub fn sink_failing_healthcheck() -> (Receiver<Event>, MockSinkConfig) {
    let (tx, rx) = futures::sync::mpsc::channel(10);
    let sink = MockSinkConfig::new(tx, false);
    (rx, sink)
}

pub fn source() -> (Sender<Event>, MockSourceConfig) {
    let (tx, rx) = futures::sync::mpsc::channel(10);
    let source = MockSourceConfig::new(rx);
    (tx, source)
}

pub fn transform(suffix: &str, increase: f64) -> MockTransformConfig {
    MockTransformConfig::new(suffix.to_owned(), increase)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MockSourceConfig {
    #[serde(skip)]
    receiver: Arc<Mutex<Option<Receiver<Event>>>>,
}

impl MockSourceConfig {
    pub fn new(receiver: Receiver<Event>) -> Self {
        Self {
            receiver: Arc::new(Mutex::new(Some(receiver))),
        }
    }
}

#[typetag::serde(name = "mock")]
impl SourceConfig for MockSourceConfig {
    fn build(
        &self,
        _name: &str,
        _globals: &GlobalOptions,
        out: Sender<Event>,
    ) -> Result<Source, vector::Error> {
        let wrapped = self.receiver.clone();
        let source = future::lazy(move || {
            wrapped
                .lock()
                .unwrap()
                .take()
                .unwrap()
                .forward(out.sink_map_err(|e| error!("Error sending in sink {}", e)))
                .map(|_| info!("finished sending"))
        });
        Ok(Box::new(source))
    }

    fn output_type(&self) -> DataType {
        DataType::Any
    }
}

pub struct MockTransform {
    suffix: String,
    increase: f64,
}

impl Transform for MockTransform {
    fn transform(&mut self, mut event: Event) -> Option<Event> {
        match &mut event {
            Event::Log(log) => {
                let mut v = log.get(&MESSAGE).unwrap().to_string_lossy();
                v.push_str(&self.suffix);
                log.insert_explicit(MESSAGE.clone(), ValueKind::from(v));
            }
            Event::Metric(Metric::Counter {
                name: _,
                val,
                timestamp: _,
                tags: _,
            }) => {
                *val += self.increase;
            }
            Event::Metric(Metric::Histogram {
                name: _,
                val,
                sample_rate: _,
                timestamp: _,
                tags: _,
            }) => {
                *val += self.increase;
            }
            Event::Metric(Metric::Gauge {
                name: _,
                val,
                direction: _,
                timestamp: _,
                tags: _,
            }) => {
                *val += self.increase;
            }
            Event::Metric(Metric::Set {
                name: _,
                val,
                timestamp: _,
                tags: _,
            }) => {
                val.push_str(&self.suffix);
            }
        };
        Some(event)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MockTransformConfig {
    suffix: String,
    increase: f64,
}

impl MockTransformConfig {
    pub fn new(suffix: String, increase: f64) -> Self {
        Self { suffix, increase }
    }
}

#[typetag::serde(name = "mock")]
impl TransformConfig for MockTransformConfig {
    fn build(&self) -> Result<Box<dyn Transform>, vector::Error> {
        Ok(Box::new(MockTransform {
            suffix: self.suffix.clone(),
            increase: self.increase,
        }))
    }

    fn input_type(&self) -> DataType {
        DataType::Any
    }

    fn output_type(&self) -> DataType {
        DataType::Any
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MockSinkConfig {
    #[serde(skip)]
    sender: Option<Sender<Event>>,
    #[serde(skip)]
    healthy: bool,
}

impl MockSinkConfig {
    pub fn new(sender: Sender<Event>, healthy: bool) -> Self {
        Self {
            sender: Some(sender),
            healthy,
        }
    }
}

#[derive(Debug, Snafu)]
enum HealthcheckError {
    #[snafu(display("unhealthy"))]
    Unhealthy,
}

#[typetag::serde(name = "mock")]
impl SinkConfig for MockSinkConfig {
    fn build(&self, acker: Acker) -> Result<(RouterSink, Healthcheck), vector::Error> {
        let sink = self
            .sender
            .clone()
            .unwrap()
            .stream_ack(acker)
            .sink_map_err(|e| error!("Error sending in sink {}", e));
        let healthcheck = match self.healthy {
            true => future::ok(()),
            false => future::err(HealthcheckError::Unhealthy.into()),
        };
        Ok((Box::new(sink), Box::new(healthcheck)))
    }

    fn input_type(&self) -> DataType {
        DataType::Any
    }
}
