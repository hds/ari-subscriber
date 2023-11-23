use tracing::{field::Visit, span};

enum FmtFieldsKind {
    Span,
    Event(Option<String>),
}

pub(crate) struct FmtSpan {
    id: span::Id,
    name: String,
    fields: FmtFields,
    formatted: String,
}

impl FmtSpan {
    pub(crate) fn new(id: &span::Id, attrs: &span::Attributes<'_>, fields: FmtFields) -> Self {
        let mut span = Self {
            id: id.clone(),
            name: attrs.metadata().name().to_owned(),
            fields,
            formatted: String::new(),
        };
        span.format();
        span
    }

    pub(crate) fn format(&mut self) {
        self.formatted = format!(
            "{name}[{id}]{{{fields}}}",
            name = &self.name,
            id = self.id.into_u64(),
            fields = self.fields.formatted(),
        )
    }

    pub(crate) fn formatted(&self) -> &str {
        &self.formatted
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
