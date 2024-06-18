use primal_rpc::v1::ids::SourceId;
use primal_rpc::v1::types::{Package, Source};
use primal_rpc::v1::ParseSpecContext;
use std::num::NonZeroU32;

#[test]
fn test_elementary_enum_base() {
    let content = r#"
    (version "1")
    (def IdempotentHandling
      (id "60726927-5a79-485a-bf43-9b561149883a")
      (enum
       :variants
       [
       ;; accept the task even if there is one unassigned task with the
       ;; idempotent key and update the accepted_at key but ignore the payload
       (id 1)
       :Accept
       ;; reject the task if the idempotent key is already present within the unassigned task set with a 409 conflict
       (id 2)
       :Reject
       ;; accept the task but do not update the accepted_at field or the payload if there is an unassigned task with the same idempotent key
       (id 3)
       :Ignore
       ;; replace the task payload if the idempotent key is already present within the unassigned task set, this will update the accepted_at field too
       (id 4)
       :Replace
       (id 5)
       ;; only allow one task with the same idempotent key and queue name
       :Unique
       ]
       ))
    "#;
    let source_id = SourceId(NonZeroU32::new(1));
    let mut package = Package::default();
    ParseSpecContext::new(source_id, content, &mut package).expect("fail to parse");
}
