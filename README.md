# Deepic · 图片旋转压缩工具

一个轻量的 Windows 桌面小工具：把网页版「图片旋转压缩」打包成可双击打开的软件。

仓库地址：https://github.com/Mahoooom/Deepic

---

## 怎么下载安装

1. 打开 [Releases（发布页）](https://github.com/Mahoooom/Deepic/releases)
2. 找到最新版本，在 **Assets** 里下载：
   - `Deepic_*_x64-setup.exe`（推荐，NSIS 安装包），或
   - `Deepic_*_x64_en-US.msi`
3. 双击安装。若 Windows 提示「未知发布者 / SmartScreen」，点 **更多信息 → 仍要运行**（当前为未签名构建，属正常现象）。

装好后，开始菜单里会出现 **Deepic** / **图片旋转压缩工具**。

---

## 这个软件做什么

- 批量选图、旋转、压缩、加水印
- 把图片或整个文件夹拖进窗口，也可以点按钮选择图片、选择文件夹
- 导出为下载文件，或写入你选择的文件夹
- 无需安装 Node / 浏览器插件，用系统自带的 WebView2 显示界面

---

## 以后怎么改功能（给非开发同学）

你不需要自己写代码或本地打包。流程是：

1. **用自然语言说明想改什么**  
   例如：「默认压缩质量改成 80」「窗口再大一点」「加一个清除全部按钮」。
2. **助手改完代码并 push 到 GitHub 的 `main` 分支**。
3. **GitHub Actions 自动打包**  
   打开仓库的 [Actions](https://github.com/Mahoooom/Deepic/actions)，等绿色勾。
4. **到 Releases 下载新版本安装包**。

更短的说明见项目文档：`docs/iteration-guide.md`（在 Cursor 项目里）。

打版本标签（例如 `v0.1.1`）也会触发同样的 Windows 打包与发布。

---

## 给开发者（可选）

本仓库是 **Tauri 2 + 纯静态 HTML**，没有 Vite / webpack，前端就是 `src/index.html`。

```bash
# 需安装 Rust 与 Tauri 前置依赖（见 https://v2.tauri.app/start/prerequisites/）
cargo install tauri-cli --version "^2" --locked
cd Deepic
cargo tauri dev    # 开发
cargo tauri build  # 本地打包（Windows 上生成 exe/msi）
```

配置要点：

- 应用标识：`com.deepic.app`
- 窗口标题：图片旋转压缩工具（约 1100×750）
- `frontendDist` 指向 `../src`，`beforeBuildCommand` 为空
- CI：`.github/workflows/release.yml`（`tauri-apps/tauri-action`，仅 Windows，未签名）
