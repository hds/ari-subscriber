use chrono::{DateTime, Utc};
use colored::{Color, Colorize};
use tracing::{field::Visit, span, Metadata};

mod color;
use color::{
    BLUE, BLUE_BOLD, GREEN, GREEN_BOLD, ORANGE, ORANGE_BOLD, PINK, PINK_BOLD, PURPLE, PURPLE_BOLD,
    RED, RED_BOLD, TURQUOISE, TURQUOISE_BOLD, YELLOW, YELLOW_BOLD,
};

enum FmtFieldsKind {
    Span,
    Event(Option<String>),
}

#[derive(Clone)]
enum SpanKind {
    Unknown,
    Spawn,
    Resource,
    AsyncOp,
    AsyncOpPoll,
}

impl SpanKind {
    fn get_colors(&self) -> (Color, Color) {
        match self {
            Self::Spawn => (GREEN, GREEN_BOLD),
            Self::Resource => (RED, RED_BOLD),
            Self::AsyncOp => (BLUE, BLUE_BOLD),
            Self::AsyncOpPoll => (YELLOW, YELLOW_BOLD),
            Self::Unknown => (Color::White, Color::White),
        }
    }
}

pub(crate) struct FmtSpan {
    id: span::Id,
    kind: SpanKind,
    name: String,
    fields: FmtFields,
    formatted: String,
}

impl FmtSpan {
    pub(crate) fn new(id: &span::Id, attrs: &span::Attributes<'_>, fields: FmtFields) -> Self {
        let meta = attrs.metadata();
        let kind = match (meta.name(), meta.target()) {
            ("runtime.spawn", _) | ("task", "tokio::task") => SpanKind::Spawn,
            ("runtime.resource", _) => SpanKind::Resource,
            ("runtime.resource.async_op", _) => SpanKind::AsyncOp,
            ("runtime.resource.async_op.poll", _) => SpanKind::AsyncOpPoll,
            _ => SpanKind::Unknown,
        };

        let mut span = Self {
            id: id.clone(),
            kind,
            name: meta.name().to_owned(),
            fields,
            formatted: String::new(),
        };
        span.format();
        span
    }

    pub(crate) fn format(&mut self) {
        let (color, bold) = self.kind.get_colors();

        self.formatted = format!(
            "{name}[{id}]{{{fields}}}",
            name = &self.name,
            id = self.id.into_u64().to_string().color(bold).bold(),
            fields = self.fields.formatted(),
        )
        .color(color)
        .to_string();
    }

    pub(crate) fn formatted(&self) -> &str {
        &self.formatted
    }
}

pub(crate) struct FmtEvent<'a> {
    timestamp: DateTime<Utc>,
    kind: EventKind,
    meta: &'a Metadata<'a>,
    scope: &'a str,
    fields: FmtFields,
}

impl<'a> FmtEvent<'a> {
    pub(crate) fn new(
        timestamp: DateTime<Utc>,
        meta: &'a Metadata<'a>,
        scope: &'a str,
        fields: FmtFields,
    ) -> Self {
        let kind = match meta.target() {
            "runtime::waker" | "tokio::task::waker" => EventKind::Waker,
            "runtime::resource::poll_op" => EventKind::PollOp,
            "runtime::resource::state_update" => EventKind::ResourceStateUpdate,
            "runtime::resource::async_op::state_update" => EventKind::AsyncOpUpdate,
            _ => EventKind::Unknown,
        };

        Self {
            timestamp,
            kind,
            meta,
            scope,
            fields,
        }
    }

    pub(crate) fn new_span_event(
        timestamp: DateTime<Utc>,
        span: &FmtSpan,
        meta: &'a Metadata<'a>,
        scope: &'a str,
        message: String,
    ) -> Self {
        Self {
            timestamp,
            kind: EventKind::SpanEvent(span.kind.clone()),
            meta,
            scope,
            fields: FmtFields::new_message(message),
        }
    }

    pub(crate) fn formatted(&mut self) -> String {
        let (color, bold) = self.kind.get_colors();

        let timestamp_format = format!(
            "{date}T{time}.{subsec}",
            date = "%Y-%m-%d".white().bold(),
            time = "%H:%M:%S".white().bold(),
            subsec = "%6fZ"
        );
        let timestamp = self
            .timestamp
            .format(&timestamp_format)
            .to_string()
            .dimmed();
        if matches!(&self.kind, EventKind::SpanEvent(_)) {
            format!(
                "{timestamp} {level:>5} {scope}{formatted}",
                level = format_level(*self.meta.level()),
                scope = self.scope,
                formatted = self
                    .fields
                    .formatted_updated()
                    .color(bold)
                    .underline()
                    .bold()
            )
        } else {
            format!(
                "{timestamp} {level:>5} {scope}{target}: {formatted}",
                level = format_level(*self.meta.level()),
                scope = self.scope,
                target = self.meta.target().color(bold).bold(),
                formatted = self.fields.formatted_updated().color(color)
            )
        }
    }
}

fn format_level(level: tracing::Level) -> String {
    match level {
        tracing::Level::TRACE => "TRACE".color(PURPLE),
        tracing::Level::DEBUG => "DEBUG".color(BLUE),
        tracing::Level::INFO => " INFO".color(GREEN),
        tracing::Level::WARN => " WARN".color(YELLOW),
        tracing::Level::ERROR => "ERROR".color(RED),
    }
    .to_string()
}

enum EventKind {
    Unknown,
    Waker,
    PollOp,
    ResourceStateUpdate,
    AsyncOpUpdate,
    SpanEvent(SpanKind),
}

impl EventKind {
    fn get_colors(&self) -> (Color, Color) {
        match self {
            Self::Waker => (PURPLE, PURPLE_BOLD),
            Self::PollOp => (ORANGE, ORANGE_BOLD),
            Self::ResourceStateUpdate => (PINK, PINK_BOLD),
            Self::AsyncOpUpdate => (TURQUOISE, TURQUOISE_BOLD),
            Self::Unknown => (Color::White, Color::White),
            Self::SpanEvent(span_kind) => span_kind.get_colors(),
        }
    }
}

pub(crate) struct FmtFields {
    kind: FmtFieldsKind,
    fields: Vec<(String, String)>,
    dirty: bool,
    formatted: String,
}

impl FmtFields {
    pub(crate) fn new_event() -> Self {
        Self {
            kind: FmtFieldsKind::Event(None),
            fields: Vec::new(),
            dirty: false,
            formatted: String::new(),
        }
    }

    pub(crate) fn new_span() -> Self {
        Self {
            kind: FmtFieldsKind::Span,
            fields: Vec::new(),
            dirty: false,
            formatted: String::new(),
        }
    }

    pub(crate) fn new_message(message: String) -> Self {
        let mut fields = Self {
            kind: FmtFieldsKind::Event(Some(message)),
            fields: Vec::new(),
            dirty: true,
            formatted: String::new(),
        };
        fields.format();
        fields
    }

    pub(crate) fn formatted(&self) -> &str {
        &self.formatted
    }

    pub(crate) fn formatted_updated(&mut self) -> &str {
        if self.dirty {
            self.format();
        }

        self.formatted()
    }

    pub(crate) fn format(&mut self) {
        let mut formatted = self
            .fields
            .iter()
            .filter_map(|(name, value)| {
                if name == "message" {
                    if let FmtFieldsKind::Event(_) = self.kind {
                        self.kind = FmtFieldsKind::Event(Some(value.clone()));
                        return None;
                    }
                }
                Some(format!("{name}={value}"))
            })
            .collect::<Vec<String>>()
            .join(", ");
        if let FmtFieldsKind::Event(Some(message)) = &self.kind {
            if !formatted.is_empty() {
                formatted.push(' ');
            }
            formatted.push_str(message);
        }
        self.formatted = formatted;
        self.dirty = false;
    }
}

impl Visit for FmtFields {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.fields
            .push((field.name().into(), format!("{value:?}")));
        self.dirty = true;
    }
}
