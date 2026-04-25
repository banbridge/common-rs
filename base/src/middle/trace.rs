use fastrace_axum::{FastraceLayer, TRACEPARENT_HEADER};

pub fn get_trace_layer() -> FastraceLayer {
    FastraceLayer::default().with_span_context_extractor(|req| {
        req.headers()
            .get(TRACEPARENT_HEADER)
            .and_then(|trace_parent| {
                fastrace::collector::SpanContext::decode_w3c_traceparent(
                    trace_parent.to_str().ok()?,
                )
            })
    })
}
