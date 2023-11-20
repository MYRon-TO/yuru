use super::*;

use tokio::{fs::File, io::AsyncReadExt};

/// ## 博客的json文件结构
/// BlogJS:{
///    title: String,
///    desc: String,
///    has_cover: bool,
/// }
#[derive(Debug, Deserialize)]
pub struct BlogJs {
    title: String,
    desc: String,
    has_cover: bool,
}

impl BlogJs {
  pub fn get_title(&self) -> &str {
    &self.title
  }
  pub fn get_desc(&self) -> &str {
    &self.desc
  }
  pub fn get_has_cover(&self) -> bool {
    self.has_cover
  }
}

/// ### 读取博客的json文件
/// path: &str 博客的json文件路径
/// return: Result<BlogJs, std::io::Error>
///
pub async fn get_blog_json(path: &str) -> Result<BlogJs, std::io::Error> {
    // 读取文件
    let mut file = File::open(path).await?;

    let mut data = Vec::new();
    file.read_to_end(&mut data).await?;

    let blog: BlogJs = serde_json::from_reader(&data[..])?;
    return Ok(blog);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_blog_json() {
        let blog = get_blog_json("assets/blogs/消息与信道/info.json")
            .await
            .unwrap();
        assert_eq!(blog.title, "消息与信道");
        assert_eq!(blog.desc, "这是一篇计算机网络的，有关消息与信道的笔记");
        assert_eq!(blog.has_cover, true);
    }
}