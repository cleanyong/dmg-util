# dmg-util

Small CLI wrapper around `hdiutil` for creating encrypted APFS disk images on macOS.

## Usage

Run via cargo:

```
cargo run --release -- \
  --size 100m \
  --filesystem APFS \
  --image-type UDIF \
  --encryption AES-256 \
  --volume-name "MyVolume" \
  --output myvolume.dmg
```

You will be prompted (hidden input) to enter the passphrase, which is placed on
`hdiutil`'s command line via `-passphrase <value>`. The arguments map directly to
the underlying `hdiutil create` call:

```
hdiutil create -size 100m -fs APFS -type UDIF -encryption AES-256 \
  -volname "MyVolume" -passphrase "<your-pass>" myvolume.dmg
```

Note: `-passphrase` is considered insecure because the value is visible to other
processes. This tool now uses it by default to mirror the working manual command.
