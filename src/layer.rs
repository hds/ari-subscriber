use chrono::{DateTime, Utc};
use tracing::{field::Visit, subscriber::Interest, Subscriber};
use tracing_subscriber::registry::LookupSpan;

struct FmtFields {
    message: Option<String>,
    fields: Vec<(String, String)>,
    dirty: bool,
    formatted: String,
}

impl FmtFields {
    fn new() -> Self {
        Self {
            message: None,
            fields: Vec::new(),
            dirty: false,
            formatted: String::new(),
        }
    }

    fn formatted(&mut self) -> &str {
        if self.dirty {
            let mut formatted = self
                .fields
                .iter()
                .filter_map(|(name, value)| {
                    if name == "message" {
                        self.message = Some(value.clone());
                        None
                    } else {
                        Some(format!("{name}={value}"))
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");
            if let Some(message) = &self.message {
                if !formatted.is_empty() {
                    formatted.push(' ');
                }
                formatted.push_str(message);
            }
            self.formatted = formatted;
            self.dirty = false;
        }

        &self.formatted
    }
}

impl Visit for FmtFields {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.fields
            .push((field.name().into(), format!("{value:?}")));
        self.dirty = true;
    }
}

pub struct Layer {}

impl Layer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> tracing_subscriber::Layer<S> for Layer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn register_callsite(&self, _metadata: &'static tracing::Metadata<'static>) -> Interest {
        Interest::always()
    }

    fn enabled(
        &self,
        _metadata: &tracing::Metadata<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        true
    }

    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let now: DateTime<Utc> = Utc::now();
        let mut fields = FmtFields::new();
        event.record(&mut fields);

        let meta = event.metadata();
        let level = meta.level().as_str();

        let timestamp = now.format("%Y-%m-%dT%H:%M:%S%.6fZ");
        println!(
            "{timestamp} {level:>5} {formatted}",
            formatted = fields.formatted()
        );
    }
}
