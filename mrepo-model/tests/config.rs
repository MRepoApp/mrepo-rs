use mrepo_model::config::*;

#[test]
fn log() {
    let json = r#"
    {
      "disabled": false,
      "level": "trace",
      "output": "mrepo.log",
      "timestamp": true
    }"#;
    assert_eq!(
        serde_json::from_str::<Log>(json).unwrap(),
        Log::new(false, "trace", "mrepo.log", true)
    );

    let json = r#"
    {
      "level": "info"
    }"#;
    assert_eq!(
        serde_json::from_str::<Log>(json).unwrap(),
        Log::new(false, "info", "", true)
    );
}

#[test]
fn repository() {
    let json = r#"
    {
      "name": "Test Repo",
      "setting": {
        "base_url": "https://repo.test.app"
      }
    }"#;
    assert_eq!(
        serde_json::from_str::<Repository>(json).unwrap(),
        Repository::new(
            "Test Repo",
            None,
            RepositorySetting::new("https://repo.test.app", 3)
        )
    );

    let json = r#"
    {
      "name": "Test Repo",
      "metadata": {
        "homepage": "https://repo.test.app/",
        "donate": "https://repo.test.app/donate",
        "support": "https://repo.test.app/support"
      },
      "setting": {
        "base_url": "https://repo.test.app",
        "keep_size": 10
      }
    }"#;
    assert_eq!(
        serde_json::from_str::<Repository>(json).unwrap(),
        Repository::new(
            "Test Repo",
            RepositoryMetadata::new(
                "https://repo.test.app/",
                "https://repo.test.app/donate",
                "https://repo.test.app/support"
            ),
            RepositorySetting::new("https://repo.test.app", 10)
        )
    );

    let json = r#"
    {
      "name": "Test Repo",
      "metadata": {
        "homepage": "https://repo.test.app/"
      },
      "setting": {
        "base_url": "https://repo.test.app",
        "keep_size": 10
      }
    }"#;
    assert_eq!(
        serde_json::from_str::<Repository>(json).unwrap(),
        Repository::new(
            "Test Repo",
            RepositoryMetadata::new("https://repo.test.app/", "", ""),
            RepositorySetting::new("https://repo.test.app", 10)
        )
    );
}

#[test]
fn module() {
    let json = r#"
    {
      "id": "test",
      "kind": "update-json",
      "provider": "https://test.app/update.json"
    }"#;
    assert_eq!(
        serde_json::from_str::<Module>(json).unwrap(),
        Module::new(
            "test",
            ProviderKind::UpdateJson,
            "https://test.app/update.json",
            "",
            None,
            None
        )
    );

    let json = r#"
    {
      "id": "test",
      "kind": "zip-url",
      "provider": "https://test.app/test.zip",
      "changelog": "https://test.app/changelog.md",
      "metadata": {
        "license": "MIT",
        "homepage": "https://test.app/",
        "donate": "https://test.app/donate",
        "support": "https://test.app/support",
        "source": "https://test.app/source"
      },
      "setting": {
        "disabled": false,
         "keep_size": 10
      }
    }"#;
    assert_eq!(
        serde_json::from_str::<Module>(json).unwrap(),
        Module::new(
            "test",
            ProviderKind::ZipUrl,
            "https://test.app/test.zip",
            "https://test.app/changelog.md",
            ModuleMetadata::new(
                "MIT",
                "https://test.app/",
                "https://test.app/donate",
                "https://test.app/support",
                "https://test.app/source"
            ),
            ModuleSetting::new(false, 10)
        )
    );

    let json = r#"
    {
      "id": "test",
      "kind": "zip-url",
      "provider": "https://test.app/test.zip",
      "changelog": "https://test.app/changelog.md",
      "metadata": {
        "license": "MIT",
        "homepage": "https://test.app/"
      },
      "setting": {
        "keep_size": 10
      }
    }"#;
    assert_eq!(
        serde_json::from_str::<Module>(json).unwrap(),
        Module::new(
            "test",
            ProviderKind::ZipUrl,
            "https://test.app/test.zip",
            "https://test.app/changelog.md",
            ModuleMetadata::new("MIT", "https://test.app/", "", "", ""),
            ModuleSetting::new(false, 10)
        )
    );
}
