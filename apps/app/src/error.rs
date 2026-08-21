use tracing_error::ExtractSpanTrace;

pub fn display_tracing_error(err: &theseus::Error) {
    match err.raw.as_ref() {
        theseus::ErrorKind::StdIOError(io_err)
            if io_err.kind() == std::io::ErrorKind::ConnectionRefused
                || io_err.kind() == std::io::ErrorKind::ConnectionReset
                || io_err.kind() == std::io::ErrorKind::TimedOut =>
        {
            tracing::debug!(error = %err);
        }
        _ => match get_span_trace(err) {
            Some(span_trace) => {
                tracing::error!(error = %err, span_trace = %span_trace);
            }
            None => {
                tracing::error!(error = %err);
            }
        },
    }
}

pub fn get_span_trace<'a>(
    error: &'a (dyn std::error::Error + 'static),
) -> Option<&'a tracing_error::SpanTrace> {
    error.source().and_then(|e| e.span_trace())
}
