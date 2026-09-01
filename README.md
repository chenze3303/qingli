# 清理 Qingli

一个轻量、免安装的 Windows 垃圾清理工具。它只处理预设的低风险缓存与临时文件；每次操作均需先扫描、再由你确认。

> 仍处于开发阶段，请先在测试电脑上使用，并检查扫描结果。

## 特性

- 单文件便携版：下载后双击 `qingli.exe` 即可打开
- 简体中文、轻快而克制的界面
- 扫描后显示每一类可释放空间，用户自行选择再清理
- 用“空间体检”概览展示可整理空间、已选项目和每项的低风险说明
- 支持用户临时文件、缩略图缓存、错误报告，以及 Chrome、Edge、Firefox 缓存
- 只在本机保存清理历史；不需要账号、不上传数据、不含广告或后台服务

## 不会做什么

- 不清理浏览器书签、密码、Cookie、登录状态或历史记录
- 不修改注册表、不卸载软件、不管理启动项
- 不处理个人文档、照片、下载文件或磁盘根目录

## 下载与使用

请在右侧 **Releases** 下载 `qingli.exe`。最新代码的构建会自动刷新到“最新开发版”；带 `v` 版本号的 Release 是可回溯的稳定版本。Windows 10/11 通常自带所需的 WebView2 组件；若程序提示缺失，请按引导安装 Microsoft Edge WebView2 Runtime。

1. 双击打开 `qingli.exe`。
2. 点击“开始扫描”。
3. 查看分类和空间大小，取消不想处理的项目。
4. 点击“清理”并确认。

被占用或权限不足的文件会自动跳过，并显示在本次结果中。

## 从源码构建

构建环境：Windows 10/11、[Rust stable](https://rustup.rs/)、Node.js 20+、Microsoft C++ Build Tools。

```powershell
npm install
npm run build:portable
```

产物位于 `src-tauri/target/release/qingli.exe`。正式发布前，建议对 `.exe` 做代码签名，并在干净的 Windows 10/11 虚拟机测试权限不足、文件被占用和 WebView2 缺失的情况。

## 项目维护

- 每次功能、清理范围、构建方式或发布流程变更，都应同步更新本 README、[更新日志](CHANGELOG.md) 和自动构建配置。
- 每次推送 main 都必须由 GitHub Actions 构建并更新“最新开发版” Release，确保 GitHub 始终有可直接下载的 `.exe`。
- 发布标签采用 `v主版本.次版本.修订版本` 格式；推送标签后 GitHub Actions 会自动构建 Windows 便携版并创建独立的版本 Release。

## 安全设计

- 前端无法传入任意文件路径；Rust 引擎只执行内置白名单规则。
- `Windows`、`Program Files`、`ProgramData` 与默认用户目录会被拒绝处理。
- 历史记录仅保存在 `%LOCALAPPDATA%\\Qingli\\history.json`。

## 许可证

暂未指定。发布正式版本前请补充许可证。
