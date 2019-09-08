workflow "Run tests" {
  resolves = ["First interaction"]
  on = "push"
}

action "First interaction" {
  uses = "actions/checkout@v1"
  runs = "cargo test"
}
