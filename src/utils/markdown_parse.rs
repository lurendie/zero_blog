use pulldown_cmark::{html,Parser};
pub(crate) struct MarkdownParse;

impl MarkdownParse{
    //Markdown
    pub(crate) fn to_html(markdown_input :&String)->String{
        // 设置HTML解析器
   let parser = Parser::new(markdown_input.as_str());

   // 将HTML转换为markdown
   let mut html_output = String::new();
   html::push_html(&mut html_output, parser);

   // 打印出HTML输出
   //println!("{}", html_output);
   html_output
   }
}

#[cfg(test)]
mod test {
    use super::MarkdownParse;
    #[test]
    fn test_to_html(){
        let html=MarkdownParse::to_html(&"Hello world, this is a ~~complicated~~ *simple* example..".to_string());
        println!("{:?}",html)
    }
}