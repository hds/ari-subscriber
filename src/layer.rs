use chrono::{DateTime, Utc};
use tracing::{span, subscriber::Interest, Subscriber};
use tracing_subscriber::registry::LookupSpan;

use crate::fmt::{FmtFields, FmtSpan};

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

    fn on_new_span(
        &self,
        attrs: &span::Attributes<'_>,
        id: &span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if extensions.get_mut::<FmtSpan>().is_none() {
            let mut fields = FmtFields::new_span();
            attrs.record(&mut fields);
            fields.format();
            let span = FmtSpan::new(id, attrs, fields);
            extensions.insert(span);
        }
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let now: DateTime<Utc> = Utc::now();
        let mut fields = FmtFields::new_event();
        event.record(&mut fields);

        let meta = event.metadata();
        let level = meta.level().as_str();

        let mut formatted_scope = String::new();
        if let Some(scope) = ctx.event_scope(event) {
            for span in scope.from_root() {
                let extensions = span.extensions();
                let span = extensions
                    .get::<FmtSpan>()
                    .expect("cannot get fields for in-scope span. This is a bug!");
                formatted_scope.push_str(&format!("{span} ", span = span.formatted()));
            }
        }

        let timestamp = now.format("%Y-%m-%dT%H:%M:%S%.6fZ");
        println!(
            "{timestamp} {level:>5} {formatted_scope}{formatted}",
            formatted = fields.formatted_updated()
        );
    }

    fn on_enter(&self, _id: &span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {}

    fn on_exit(&self, _id: &span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {}

    fn on_close(&self, _id: span::Id, _ctx: tracing_subscriber::layer::Context<'_, S>) {}
}
