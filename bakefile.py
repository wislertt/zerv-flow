# /// script
# requires-python = ">=3.14"
# dependencies = [
#     "bakefile[lib]>=0.0.26",
# ]
# ///

from bake import command
from bakelib import RustSpace


class MyBakebook(RustSpace):
    def update(self) -> None:
        super().update()
        # TODO: move to bakefile
        self.ctx.run("bakefile lock --upgrade")
        self.ctx.run("bakefile sync")

    def lint(self) -> None:
        super().lint()
        # TODO: move to bakefile
        self.ctx.run("bakefile lint")

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
