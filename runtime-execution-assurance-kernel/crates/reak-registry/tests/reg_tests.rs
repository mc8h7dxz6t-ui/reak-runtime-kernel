use reak_registry::{ArtifactDescriptor, RegistryCatalog, RegistryError};
use reak_types::{ArtifactId, TenantId};

#[test]
fn register_and_resolve() {
    let cat = RegistryCatalog::new(TenantId::parse("t").unwrap());
    let id = ArtifactId::parse("art-1").unwrap();
    cat.register_artifact(ArtifactDescriptor {
        artifact_id: id.clone(),
        media_type: "application/json".into(),
        content_locator: "loc://a".into(),
        lineage_hint: None,
    })
    .unwrap();
    let p = cat.resolve_pointer(&id).unwrap();
    assert_eq!(p.descriptor.content_locator, "loc://a");
}

#[test]
fn missing_artifact_fails_closed() {
    let cat = RegistryCatalog::new(TenantId::parse("t").unwrap());
    let err = cat
        .resolve_pointer(&ArtifactId::parse("nope").unwrap())
        .unwrap_err();
    assert_eq!(err, RegistryError::NotFound);
}
