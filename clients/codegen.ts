import { createFromRoot } from 'codama'
import { rootNodeFromAnchor } from "@codama/nodes-from-anchor"
import { renderVisitor as renderJavaScriptVisitor } from "@codama/renderers-js"
import { renderVisitor as renderRustVisitor } from "@codama/renderers-rust"
import * as fs from "fs"
import * as path from "path"
import { fileURLToPath } from 'url'

// 兼容性处理
const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
async function main() {
    const projectRoot = path.resolve(__dirname, "..")
    const idlPath = path.join(projectRoot, "idl", "blueshift_vault.json")
    // 统一输出路径
    const outputBaseDir = path.join(__dirname, "src", "generated")
    const outputTsPath = path.join(outputBaseDir, "js")
    const outputRsPath = path.join(outputBaseDir, "rust")

    console.log(`🚀 正在从 Shank IDL 生成 SDK...`)

    try {
        // 1. 读取 Shank 生成的 IDL
        if (!fs.existsSync(idlPath)) {
            throw new Error(`找不到 IDL 文件: ${idlPath}。请先运行 shank idl。`)
        }
        const idl = JSON.parse(fs.readFileSync(idlPath, "utf-8"))

        // 2. 转换 IDL
        console.log(`🚀 正在解析 IDL...`)
        const codama = createFromRoot(rootNodeFromAnchor(idl))

        // 确保目录存在
        if (!fs.existsSync(outputBaseDir)) {
            fs.mkdirSync(outputBaseDir, { recursive: true })
        }

        // 3. 生成 TypeScript 客户端
        console.log(`📦 生成 TypeScript 客户端...`)
        codama.accept(
            renderJavaScriptVisitor(outputTsPath, {
                formatCode: true,
                deleteFolderBeforeRendering: true,
            })
        )
        console.log(`✅ TypeScript SDK 已生成: ${outputTsPath}`)

        // 4. 生成 Rust 客户端
        console.log(`🦀 生成 Rust 客户端...`)
        codama.accept(renderRustVisitor(outputRsPath, {
            formatCode: true,
            anchorTraits: false,
            deleteFolderBeforeRendering: true,
        }))

        console.log(`\n✨ 全部生成成功！位置: ${outputBaseDir}`)
    } catch (error) {
        console.error(`❌ 生成失败:`, error)
        process.exit(1)
    }
}

main()
