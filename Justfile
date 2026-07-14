git_revision := `git rev-parse --short HEAD`
app_version := `awk -F'"' '/^\[package\]/{p=1} p && /^version *=/{print $2; exit}' Cargo.toml`
build_date := `date -u +%Y-%m-%dT%H:%M:%SZ`
container_runner := "docker"
container_image := "ghcr.io/nomuraKIDS/squirrelt"

# 手元の環境で確認するためのビルドタスク
container-local:
    {{container_runner}} build \
      --build-arg GIT_REVISION={{git_revision}} \
      --build-arg BUILD_DATE={{build_date}} \
      --build-arg VERSION={{app_version}} \
      -t {{container_image}}:latest -t {{container_image}}:{{app_version}} \
      -f Containerfile \
      .

# GitHub Actionsで実行するためのマルチプラットフォーム向けビルドタスク
container:
    {{container_runner}} buildx build --push \
      --platform linux/amd64,linux/arm64 \
      --build-arg GIT_REVISION={{git_revision}} \
      --build-arg BUILD_DATE={{build_date}} \
      --build-arg VERSION={{app_version}} \
      -t {{container_image}}:latest -t {{container_image}}:{{app_version}} \
      -f Containerfile \
      .