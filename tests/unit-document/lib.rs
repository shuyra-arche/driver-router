#[cfg(test)]
mod tests {
    use router_declaration::{BUILD, UNIT};
    use unit::{document as d, record};

    const TOML: &str = include_str!("../../unit.toml");

    fn document() -> d::Unit<String> {
        toml_edit::de::from_str(TOML).unwrap()
    }

    fn assert_legacy_policy<S: AsRef<str>>(value: &d::Unit<S>) {
        assert_eq!(value.schema, 1);
        assert_eq!(value.id.as_ref(), UNIT.name);
        assert_eq!(value.loader.image.as_ref(), UNIT.loader.image);
        assert!(matches!(UNIT.loader.space, env::ProgramKind::User));
        assert_eq!(value.loader.space, d::Space::User);
        assert_eq!(
            value.loader.args,
            UNIT.loader
                .args
                .iter()
                .map(|arg| *arg as u64)
                .collect::<Vec<_>>()
        );
        assert_eq!(value.loader.stack, UNIT.loader.stack as u64);

        assert_eq!(
            value.operator.namespace.len(),
            UNIT.operator.namespace.len()
        );
        for (current, previous) in value.operator.namespace.iter().zip(UNIT.operator.namespace) {
            assert_eq!(current.key.as_ref(), previous.key.name());
            assert_eq!(current.path.as_ref(), previous.path);
            assert!(matches!(
                previous.placement,
                legacy_unit::operator::Placement::Shared
            ));
            assert_eq!(current.placement, d::Placement::Shared);
            assert!(matches!(
                previous.scope,
                legacy_unit::operator::Scope::Subtree
            ));
            assert!(matches!(current.scope, d::Scope::Subtree));
            assert_eq!(current.ops, previous.ops.bits());
            assert_eq!(previous.permit, Some(system_api::operator::Permit::Public));
            assert_eq!(current.permit, Some(d::Permit::Public));
            assert_eq!(current.source_mark, previous.source_mark);
            assert_eq!(current.bindings.len(), previous.bindings.len());
            for (binding, old_binding) in current.bindings.iter().zip(previous.bindings) {
                assert_eq!(binding.name.as_ref(), old_binding.name);
                assert_eq!(binding.mark, old_binding.mark.get());
                assert_eq!(binding.view, old_binding.view.bits().bits());
                assert_eq!(old_binding.permit, system_api::operator::Permit::Public);
                assert_eq!(binding.permit, d::Permit::Public);
                assert_eq!(binding.name.as_ref(), router_api::ENTRY_NAME);
                assert_eq!(binding.mark, router_api::ENTRY_MARK.get());
                assert_eq!(binding.view, env::Access::STORE.bits().bits());
            }
        }
        assert_eq!(
            value
                .identity
                .aliases
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>(),
            UNIT.identity.aliases
        );
        assert_eq!(
            value
                .identity
                .grants
                .iter()
                .map(|code| { system_api::identity::Grant::from_action(*code).unwrap() })
                .collect::<Vec<_>>(),
            UNIT.identity.grants
        );

        assert_eq!(value.control.creation, d::Creation::Startup);
        assert_eq!(
            UNIT.control.after.unwrap(),
            [
                legacy_unit::Dependency::Bootstrap(legacy_unit::Bootstrap::Operator),
                legacy_unit::Dependency::Unit("hub"),
            ]
        );
        // System bootstraps Operator before creating Units; Hub is a real provider.
        assert_eq!(
            value
                .control
                .requires
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>(),
            ["hub"]
        );
        assert_eq!(
            value
                .control
                .after
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>(),
            ["hub"]
        );
        match (value.control.ready, UNIT.control.ready) {
            (d::Ready::Signal(current), legacy_unit::Ready::Signal(previous)) => {
                assert_eq!(current, previous.get());
                assert_eq!(current, unit::READY_MARK.get());
                assert_eq!(current, router_api::READY_MARK.get());
            }
            _ => panic!("ready policy changed"),
        }
        assert!(matches!(
            UNIT.control.ending,
            Some(legacy_unit::Ending::Resident)
        ));
        assert_eq!(value.control.ending, d::Ending::Resident);
        assert_eq!(
            value.control.supervision.as_ref().map(AsRef::as_ref),
            UNIT.control.supervision
        );
        assert!(UNIT.control.construction.is_empty());
        assert!(value.control.construction.is_empty());
        assert!(UNIT.control.retirement.is_empty());
        assert!(value.control.retirement.is_empty());
        assert!(value.control.groups.is_empty());
        assert!(!value.control.guarded);
        assert!(UNIT.supply.bindings.is_empty());
        assert!(value.supply.bindings.is_empty());
        assert!(UNIT.valid());
        value.validate().unwrap();
    }

    #[test]
    fn toml_preserves_every_legacy_policy_field() {
        assert_legacy_policy(&document());
    }

    #[test]
    fn binary_roundtrip_preserves_every_field_and_borrows_strings() {
        let declaration = document();
        let bytes = record::encode(&declaration).unwrap();
        let decoded = record::decode(&bytes).unwrap();
        assert_legacy_policy(&decoded);
        let text = toml_edit::ser::to_string(&decoded).unwrap();
        let owned: d::Unit<String> = toml_edit::de::from_str(&text).unwrap();
        assert_eq!(owned, declaration);
        let begin = bytes.as_ptr() as usize;
        let end = begin + bytes.len();
        for text in [
            decoded.id,
            decoded.loader.image,
            decoded.operator.namespace[0].bindings[0].name,
        ] {
            let pointer = text.as_ptr() as usize;
            assert!(pointer >= begin && pointer + text.len() <= end);
        }
    }

    #[test]
    fn quoted_decimal_and_hex_marks_keep_the_native_u64_values() {
        let mut hexadecimal = TOML.to_owned();
        for mark in [router_api::ENTRY_MARK, unit::READY_MARK] {
            assert!(mark.get() > i64::MAX as u64);
            let decimal = format!("\"{}\"", mark.get());
            assert!(hexadecimal.contains(&decimal));
            hexadecimal = hexadecimal.replace(&decimal, &format!("\"0x{:016x}\"", mark.get()));
        }
        let decoded: d::Unit<String> = toml_edit::de::from_str(&hexadecimal).unwrap();
        assert_eq!(decoded, document());
        assert_legacy_policy(&record::decode(&record::encode(&decoded).unwrap()).unwrap());
    }

    #[test]
    fn image_names_the_existing_cargo_binary_and_preserves_the_build_contract() {
        assert_eq!(document().loader.image, BUILD.image);
        assert_eq!(BUILD.manifest, "server/Cargo.toml");
        let manifest: toml_edit::DocumentMut =
            include_str!("../../server/Cargo.toml").parse().unwrap();
        assert_eq!(manifest["package"]["name"].as_str(), Some(BUILD.package));
        let bins = manifest["bin"].as_array_of_tables().unwrap();
        assert_eq!(bins.len(), 1);
        let binary = bins.get(0).unwrap();
        assert_eq!(binary["name"].as_str(), Some(BUILD.bin));
        assert_eq!(BUILD.bin, BUILD.image);
        assert_eq!(binary["path"].as_str(), Some("src/main.rs"));
        assert!(include_str!("../../server/src/main.rs").contains("router_server::serve()"));
        assert!(BUILD.features.is_empty());
        assert!(BUILD.default_features);
        assert_eq!(BUILD.target, "riscv64gc-unknown-none-elf");
        assert_eq!(BUILD.profile, "release");
    }
}
