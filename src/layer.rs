//! A [`tracing-subscriber`] Layer which outputs to `stdout`.
//!
//! See the documentation on [`Layer`] for more details.
//!
//! [`tracing-subscriber`]: tracing_subscriber
use chrono::{DateTime, Utc};
use tracing::{span, subscriber::Interest, Subscriber};
use tracing_subscriber::registry::LookupSpan;

use crate::fmt::{FmtEvent, FmtFields, FmtSpan};

/// Creates a new [`Layer`].
///
/// See the [`Layer`] documentation for details on customization.
///
/// # Examples
///
/// ```rust
/// use tracing_subscriber::prelude::*;
///
/// let layer = aeiou::layer();
/// tracing_subscriber::registry().with(layer).init();
///
/// // Will be printed by `aeiou`
/// tracing::info!("nice!");
/// ```
#[must_use = "A Layer does nothing if it is not added to a registry."]
pub fn layer() -> Layer {
    Layer::new()
}

/// A [`tracing-subscriber`] Layer which outputs to `stdout`.
///
/// The layer can be added to a [`Registry`] and will output trace information
/// to `stdout`.
///
/// [`tracing-subscriber`]: tracing_subscriber
/// [`Registry`]: struct@tracing_subscriber::Registry

pub struct Layer {}

impl Layer {
    /// Creates a new [`Layer`].
    ///
    /// Currently, no customization is possible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tracing_subscriber::prelude::*;
    ///
    /// use aeiou::Layer;
    ///
    /// let layer = Layer::new();
    /// tracing_subscriber::registry().with(layer).init();
    ///
    /// // Will be printed by `aeiou`
    /// tracing::info!("wonderful!");
    /// ```
    #[must_use = "A Layer does nothing if it is not added to a registry."]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Layer {
    #[must_use = "A Layer does nothing if it is not added to a registry."]
    fn default() -> Self {
        Self::new()
    }
}

impl Layer {
    // Self kept because it will be needed with pending changes.
    #[allow(clippy::unused_self)]
    fn write_event(&self, fmt_event: &mut FmtEvent) {
        println!("{}", fmt_event.formatted());
    }

    fn span_event<S>(
        &self,
        now: DateTime<Utc>,
        id: &span::Id,
        ctx: &tracing_subscriber::layer::Context<'_, S>,
        message: String,
    ) where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let extensions = span.extensions();
        let Some(fmt_span) = extensions.get::<FmtSpan>() else {
            // We can't print anything if the fmt_span isn't present.
            return;
        };

        let formatted_scope = ctx
            .span_scope(id)
            .map(|scope| self.formatted_scope(scope))
            .unwrap_or_default();
        let mut fmt_event =
            FmtEvent::new_span_event(now, fmt_span, span.metadata(), &formatted_scope, message);

        self.write_event(&mut fmt_event);
    }

    // Self kept because it will be needed with pending changes.
    #[allow(clippy::unused_self)]
    fn formatted_scope<S>(&self, scope: tracing_subscriber::registry::Scope<'_, S>) -> String
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let mut formatted_scope = String::new();
        for span in scope.from_root() {
            let extensions = span.extensions();
            let span = extensions
                .get::<FmtSpan>()
                .expect("cannot get fields for in-scope span. This is a bug!");
            formatted_scope.push_str(&format!("{span} ", span = span.formatted()));
        }
        formatted_scope
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
        let now = Utc::now();
        let span = ctx.span(id).expect("Span not found, this is a bug");
        {
            let mut extensions = span.extensions_mut();

            if extensions.get_mut::<FmtSpan>().is_none() {
                let mut fields = FmtFields::new_span();
                attrs.record(&mut fields);
                fields.format();
                let span = FmtSpan::new(id, attrs, fields);
                extensions.insert(span);
            }
        }

        self.span_event(now, id, &ctx, "new".into());
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let now = Utc::now();

        let mut fields = FmtFields::new_event();
        event.record(&mut fields);

        let formatted_scope = ctx
            .event_scope(event)
            .map(|scope| self.formatted_scope(scope))
            .unwrap_or_default();

        let mut fmt_event = FmtEvent::new(now, event.metadata(), &formatted_scope, fields);
        self.write_event(&mut fmt_event);
    }

    fn on_enter(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let now: DateTime<Utc> = Utc::now();
        self.span_event(now, id, &ctx, "enter".into());
    }

    fn on_exit(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let now: DateTime<Utc> = Utc::now();
        self.span_event(now, id, &ctx, "exit".into());
    }

    fn on_close(&self, id: span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let now: DateTime<Utc> = Utc::now();
        self.span_event(now, &id, &ctx, "close".into());
    }
}
