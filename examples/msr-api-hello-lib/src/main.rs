// 直接API调用示例 - 不通过Web路由
use msr_api_rs::MSRApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建默认的MSR API客户端，使用官方API地址
    let client = MSRApiClient::new();
    
    // 直接调用API方法 - 无需Web路由
    println!("直接调用API封装功能示例：");
    
    // 示例：获取歌曲信息
    match client.get_song("125042".to_string()).await {
        Ok(resp) => println!("成功获取歌曲: {:?}", resp.data.name),
        Err(e) => println!("获取歌曲失败: {}", e),
    }
    
    // 示例：获取所有专辑
    match client.get_all_albums().await {
        Ok(resp) => println!("成功获取专辑数量: {}", resp.data.len()),
        Err(e) => println!("获取专辑失败: {}", e),
    }
    
    // 示例：搜索功能
    match client.search("au".to_string()).await {
        Ok(resp) => println!("搜索结果: {} 个专辑, {} 条新闻", 
                           resp.data.albums.list.len(), 
                           resp.data.news.list.len()),
        Err(e) => println!("搜索失败: {}", e),
    }
    
    Ok(())
}
