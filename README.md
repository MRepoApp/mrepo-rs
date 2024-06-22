# MRepo
A manager for building modules repository

## Build
```shell
cargo install --git https://github.com/MRepoApp/mrepo-rs.git
```

## Build (with git)
> [!WARNING]
> Unstable, only confirmed to work on MacOS
### Dependencies
- OpenSSL 1.0.1 - 3.x.x (or LibreSSL 2.5 - 3.7.x)
```shell
cargo install --git https://github.com/MRepoApp/mrepo-rs.git --features git
```

## Data structure
### config.json
```json
{
  "log": {
    "disabled": false,
    "level": "debug",
    "output": "",
    "timestamp": true
  },
  "repository": {
    "name": "Sanmer Magisk Repo (demo)",
    "metadata": {
      "homepage": "https://demo-repo.sanmer.app",
      "donate": "",
      "support": ""
    },
    "setting": {
      "base_url": "https://demo-repo.sanmer.app",
      "keep_size": 3
    }
  },
  "modules": [
    {
      "id": "zygisk_lsposed",
      "kind": "update-json",
      "provider": "https://lsposed.github.io/LSPosed/release/zygisk.json",
      "changelog": "",
      "metadata": {
        "license": "GPL-3.0",
        "homepage": "https://lsposed.org",
        "source": "https://github.com/LSPosed/LSPosed.git",
        "donate": "https://github.com/sponsors/LSPosed",
        "support": "https://github.com/LSPosed/LSPosed/issues"
      },
      "setting": {
        "disabled": false,
        "keep_size": 6
      }
    }
  ]
}
```

### modules.json
```json
{
  "name": "Sanmer Magisk Repo (demo)",
  "timestamp": 1718781429070,
  "metadata": {
    "homepage": "https://demo-repo.sanmer.app",
    "donate": "",
    "support": ""
  },
  "modules": [
    {
      "id": "zygisk_lsposed",
      "name": "Zygisk - LSPosed",
      "version": "v1.9.2 (7024)",
      "version_code": 7024,
      "author": "LSPosed Developers",
      "description": "Another enhanced implementation of Xposed Framework. Supports Android 8.1 ~ 14. Requires Magisk 24.0+ and Zygisk enabled.",
      "metadata": {
        "license": "GPL-3.0",
        "homepage": "https://lsposed.org",
        "source": "https://github.com/LSPosed/LSPosed.git",
        "donate": "https://github.com/sponsors/LSPosed",
        "support": "https://github.com/LSPosed/LSPosed/issues"
      },
      "versions": [
        {
          "timestamp": 1697034252000,
          "version": "v1.9.2 (7024)",
          "version_code": 7024,
          "zip_url": "https://demo-repo.sanmer.app/modules/zygisk_lsposed/7024.zip",
          "changelog": "https://demo-repo.sanmer.app/modules/zygisk_lsposed/7024.md"
        }
      ]
    }
  ]
}
```

### track.json (internal)
```json
{
  "module": {
    "id": "zygisk_lsposed",
    "name": "Zygisk - LSPosed",
    "version": "v1.9.2 (7024)",
    "version_code": 7024,
    "author": "LSPosed Developers",
    "description": "Another enhanced implementation of Xposed Framework. Supports Android 8.1 ~ 14. Requires Magisk 24.0+ and Zygisk enabled."
  },
  "versions": [
    {
      "timestamp": 1697034252000,
      "version": "v1.9.2 (7024)",
      "version_code": 7024,
      "zip_file": "7024.zip",
      "changelog": "7024.md"
    }
  ]
}
```
