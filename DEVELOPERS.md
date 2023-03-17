# tjuptattendance 开发者文档

tjuptattendance 为 [TJUPT](https://www.tjupt.org) 的签到工具，支持 **TOP10** 签到

# 目标

todo

# 项目结构

tjuptattendance 主体包含两个部分：客户端(`tjuptop`) 与 API服务器(`server`)，同时将共用部分(如：通用数据结构，通用工具函数等)放置到 `util` 库中

**servers** 采用 **逐级查询** 的方式来扩大用户可获取的数据范围，同时可以让 server 之间进行数据同步。为在一定程度上保证数据的正确性，还需进一步设计


- **客户端 - tjuptop**：作为用户直接使用的 **命令行工具** tjuptop 实现基本的(TOP10)签到功能的同时，提供丰富的个性化设置选项，并提供一个优良的预设配置。可从 `DouBanAPI` `Server` `CFWAS`  获取签到需要的信息

- **服务端 - DouBanAPI**：豆瓣官方API，作为数据的权威来源，其余所有 API 服务的数据，均直接或间接来自于此。但是具有较大使用限制：1. 短时间内最大调用 `5` 次；2. 获得的数据无法直接用来识别答案，需经过二次处理

- **服务端 - Server**：为了绕过豆瓣API的请求次数限制和加快 TOP10 签到速度，保存经过处理后的豆瓣数据(通过附加信息的方式，此附加信息可直接用于识别答案)，并开放API接口，供 tjuptop 调用


- **服务端 - CFWAS**：利用 CloudFlare Wokers 的免费资源，搭建 API 服务，可实现 `server` 相同的功能，但由于 cpu 时间限制及较大的网络延迟，`CloudFlare Wokers API Server (CFWAS)` 作为仅次于 `DouBanAPI` 的最上游


# 实现

TODO

