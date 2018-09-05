pub fn push_message_list(req: HttpRequest<AppState>) -> impl Responder {
    let ref_req = Rc::new(req);
    let req_for_pushes = Rc::clone(&ref_req);
    let req_for_err = Rc::clone(&ref_req);
    let query_message = Pagination::<push_message::GetPushMessages>::from_request(Rc::clone(&ref_req));

    req_for_pushes.state().database.send(query_message).from_err()
        .and_then(|push_messages| {
            let push_messages: PaginatedListMessage<push_message::PushMessageDisplay> = push_messages?;
            let mut list: Vec<PushMessage> = vec![];

            for item in push_messages.list {
                list.push(PushMessage {
                    id: item.id,
                    content: item.content,
                    created_at: item.created_at.format("%Y-%m-%d").to_string(),
                    created_at_o: item.created_at,
                })
            }

            let pagination = PaginatedListMessage {
                list,
                page: push_messages.page,
                per_page: push_messages.per_page,
                total: push_messages.total,
            };

            let mut context = Context::new();
            let pages = (pagination.total as f64 / pagination.per_page as f64).ceil() as i64;
            context.add("pushes", &pagination.list);
            context.add("pages", &pages);
            context.add("current", &pagination.page);
            context.extend(get_global_context());

            let rendered = match TEMPLATES.render("push-messages.html", &context) {
                Ok(r) => r,
                Err(e) => {
                    debug!("Error to render: list.html, error detail: {:?}", e);
                    "Render error".into()
                }
            };
            Ok(HttpResponse::Ok().body(rendered))
        })
        .map_err(error_handler(req_for_err))
        .responder()
}