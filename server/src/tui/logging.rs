use bevy::{
    log::{BoxedLayer, Level, tracing, tracing_subscriber},
    prelude::*,
};
use chrono::{DateTime, Local};
use crossbeam::channel::*;

pub fn update_history(mut history: ResMut<LoggedHistory>, receiver: Res<TracingReceiver>) {
    while let Ok(message) = receiver.try_recv() {
        history.insert(0, message);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct LoggedHistory(Vec<TraceMessage>);

pub fn custom_layer(app: &mut App) -> Option<BoxedLayer> {
    let (sender, receiver) = unbounded();
    app.insert_resource(LoggedHistory(Vec::new()));
    app.insert_resource(TracingReceiver(receiver));
    Some(Box::new(CustomTerminalLayer(sender)))
}

#[derive(Resource, Deref, DerefMut)]
pub struct TracingReceiver(Receiver<TraceMessage>);

pub struct TraceMessage {
    pub time: DateTime<Local>,
    pub level: Level,
    pub target: String,
    pub message: String,
}

#[derive(Deref, DerefMut)]
pub struct CustomTerminalLayer(Sender<TraceMessage>);

impl<S> tracing_subscriber::Layer<S> for CustomTerminalLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = TerminalVisitor("".to_string());
        event.record(&mut visitor);
        let time = Local::now();
        let level = *event.metadata().level();
        let target = event.metadata().target().to_string();
        let _ = self.try_send(TraceMessage {
            time,
            level,
            target,
            message: visitor.0,
        });
    }
}

struct TerminalVisitor(String);

impl tracing::field::Visit for TerminalVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.0 = format!("{:?}", value).trim_matches('"').to_string();
        }
    }
}
