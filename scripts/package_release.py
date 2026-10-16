"""Package the wasm-pack web build for a GitHub release (no dependencies)."""
import hashlib
import shutil
import sys
import tomllib
import zipfile
from pathlib import Path


def main():
    tag = sys.argv[1]
    version = tomllib.loads(Path("wasm/Cargo.toml").read_text())["package"]["version"]
    if tag != f"v{version}":
        raise SystemExit(f"Tag {tag} does not match WASM package version {version}")
    package = Path("target/release-package/pkg")
    output = Path("target/release-package/dist")
    output.mkdir(parents=True, exist_ok=True)
    files = [path for path in package.iterdir() if path.is_file() and not path.name.startswith(".")]
    for name in ["lifelike_wasm.js", "lifelike_wasm_bg.wasm", "lifelike_wasm.d.ts"]:
        if not (package / name).is_file():
            raise SystemExit(f"Missing build output: {name}")
    with zipfile.ZipFile(output / f"lifelike-wasm-{tag}.zip", "w", zipfile.ZIP_DEFLATED) as archive:
        for path in sorted(files):
            archive.write(path, path.name)
        archive.write("LICENSE", "LICENSE")
    for path in files:
        if path.suffix in {".js", ".wasm", ".ts"}:
            shutil.copy2(path, output / path.name)
    assets = sorted(path for path in output.iterdir() if path.name != "SHA256SUMS")
    (output / "SHA256SUMS").write_text("".join(
        f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.name}\n" for path in assets
    ))
    print(f"Packaged {len(assets)} release assets for {tag}")


if __name__ == "__main__":
    main()
