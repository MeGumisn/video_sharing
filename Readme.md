# 运行时的目录结构说明

本项目是一个轻量级的应用结构，包含核心可执行文件、配置文件以及静态网页资源。

## 📂 目录树状图

```text
.
├── video_sharing.exe          # 核心可执行程序
├── config.json    # 配置文件, 
└── public/        # 静态资源存放目录
    └── index.html     # 前端展示页面
```

## 📄 文件功能清单

* **`./video_sharing.exe`**
    * 主运行程序。
    * 负责读取配置并启动服务。

* **`./config.json`**
    * 参数配置文件。
    * dir_path: 会展示dir_path下的视频文件或子目录。
    * port: 服务器端口号

* **`./public/index.html`**
    * 静态网页文件。
    * 用于在浏览器中进行前端界面展示。
