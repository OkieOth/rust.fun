
use crate::layer::{Layer};
use uuid::Uuid;

#[test]
fn test_init_layer() {
    let id: Uuid = Uuid::nil();
    let name = "dummy".to_string();
    let l = Layer::new(id, "dummy".to_string());
    assert_eq!(l.id, id);
    assert_eq!(l.name, name);
}
