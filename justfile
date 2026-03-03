# 格式化代码和配置文件
fmt:
	taplo fmt --option reorder_keys=true
	cargo fmt --all

# 运行 pre-commit 检查
check:
	pre-commit run --all-files

# 编译所有程序
build-all:
    cargo build-sbf
    just collect-artifacts

# 提取生成的 .so 文件到根目录的 artifacts 文件夹
collect-artifacts:
    mkdir -p artifacts
    cp target/deploy/*.so artifacts/
    @echo "✅ Artifacts collected in ./artifacts:"
    @ls -lh artifacts

# 编译特定程序 (更快捷)
build-escrow:
    cargo build-sbf -p pinocchio_escrow
    just collect-artifacts

build-classic:
    cargo build-sbf -p solana_pinocchio_escrow
    just collect-artifacts

# 清理所有编译产物
clean:
    cargo clean
    rm -rf artifacts
