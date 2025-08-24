# MSR API 分类文档

## 项目概述
本项目是一个Rust实现的Monster Siren Records API封装服务，提供对官方API的代理访问。

## 已实现API端点

### 1. 歌曲相关接口

#### 获取歌曲详情
- **端点**: `GET /song/{cid}`
- **描述**: 获取指定歌曲的详细信息，包括音频URL、歌词URL等
- **参数**: 
  - `cid` (路径参数): 歌曲ID，例如125079
- **响应示例**:
```json
{
  "code": 0,
  "msg": "",
  "data": {
    "cid": "125079",
    "name": "Settle Into Ash (Instrumental)",
    "albumCid": "6664",
    "sourceUrl": "https://res01.hycdn.cn/.../xxx.wav",
    "lyricUrl": null,
    "mvUrl": null,
    "mvCoverUrl": null,
    "artists": ["塞壬唱片-MSR"]
  }
}
```

### 2. 用户相关接口（示例/测试用）

#### 获取用户信息
- **端点**: `GET /users/{id}`
- **描述**: 获取指定用户的信息（示例接口）
- **参数**: 
  - `id` (路径参数): 用户ID

#### 创建用户
- **端点**: `POST /users`
- **描述**: 创建新用户（示例接口）
- **请求体**:
```json
{
  "name": "用户名"
}
```

## 待实现API端点

### 1. 搜索相关接口

#### 基本搜索
- **端点**: `GET /search?keyword={关键词}`
- **描述**: 搜索专辑与动向
- **参数**: 
  - `keyword` (查询参数): 搜索关键词

#### 专辑搜索
- **端点**: `GET /search/album?keyword={关键词}[&lastCid={最后ID}]`
- **描述**: 通过关键词获取专辑列表
- **参数**: 
  - `keyword` (查询参数): 搜索关键词
  - `lastCid` (可选查询参数): 分页用最后ID

#### 新闻搜索
- **端点**: `GET /search/news?keyword={关键词}[&lastCid={最后ID}]`
- **描述**: 独立新闻搜索接口
- **参数**: 
  - `keyword` (查询参数): 搜索关键词
  - `lastCid` (可选查询参数): 分页用最后ID

### 2. 专辑相关接口

#### 获取所有专辑
- **端点**: `GET /albums`
- **描述**: 获取所有专辑列表，包含专辑封面

#### 获取专辑基本信息
- **端点**: `GET /album/{albumId}/data`
- **描述**: 获取专辑的基本信息（不含歌曲列表）
- **参数**: 
  - `albumId` (路径参数): 专辑ID

#### 获取专辑详细信息
- **端点**: `GET /album/{albumId}/detail`
- **描述**: 获取专辑详细信息，包含所有歌曲
- **参数**: 
  - `albumId` (路径参数): 专辑ID

### 3. 歌曲列表接口

#### 获取所有歌曲
- **端点**: `GET /songs`
- **描述**: 获取所有歌曲列表

### 4. 动向相关接口

#### 获取轮播图
- **端点**: `GET /recommends`
- **描述**: 获取动向左侧轮播图数据

#### 获取新闻列表
- **端点**: `GET /news[?lastCid={最后ID}]`
- **描述**: 获取新闻列表，支持分页
- **参数**: 
  - `lastCid` (可选查询参数): 分页用最后ID

#### 获取新闻详情
- **端点**: `GET /news/{newsId}`
- **描述**: 获取指定新闻的详细信息
- **参数**: 
  - `newsId` (路径参数): 新闻ID

### 5. 杂项接口

#### 获取字体配置
- **端点**: `GET /fontset`
- **描述**: 获取网页用字体文件配置

## 错误处理

### 标准错误响应
```json
{
  "code": 非0值,
  "msg": "错误描述"
}
```

### 参数验证错误
```json
{
  "msg": "[{\"message\":\"should not be empty\",\"code\":\"invalid\",\"field\":\"keyword\"}]"
}
```

## 使用说明

### 本地开发
1. 启动服务: `cargo run --bin server`
2. 访问地址: `http://localhost:8080`
3. API文档: `http://localhost:8080/swagger-ui`

### 环境配置
- 端口配置: 修改 `.env` 文件中的 `SERVER_PORT`
- 远程API: 修改 `.env` 文件中的 `REMOTE_BASE`

## 实现优先级建议

1. **高优先级**: 歌曲相关接口（已实现）
2. **中优先级**: 专辑相关接口
3. **低优先级**: 搜索和新闻接口
4. **可选**: 用户相关接口（目前为示例用）
