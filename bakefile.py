# /// script
# requires-python = ">=3.14"
# dependencies = [
#     "bakefile[lib]>=0.0.26",
# ]
#
# # Debug with local bakefile in editable mode
# # [tool.uv.sources]
# # bakefile = { path = "../bakefile", editable = true }
# ///

from bake import command
from bakelib import GitHubActionsTools, RustSpace


class MyBakebook(GitHubActionsTools, RustSpace):
    def test(self) -> None:
        env: dict[str, str] = {}
        env["RUST_LOG"] = "cargo_tarpaulin=off"
        self.ctx.run(
            "cargo tarpaulin --out Xml --out Html --out Lcov "
            "--output-dir coverage --exclude-files 'src/main.rs' "
            "-- --quiet",
            env=env,
        )

    @command()
    def run(self) -> None:
        self.ctx.run("cargo run")

    @command()
    def open_coverage(self):
        self.ctx.run("open coverage/tarpaulin-report.html")


bakebook = MyBakebook()
