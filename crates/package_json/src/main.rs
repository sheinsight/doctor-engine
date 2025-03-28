use biome_deserialize::{
  Deserializable, DeserializableValue, DeserializationDiagnostic,
  json::{DeserializationResult, deserialize_from_json_str},
};
use biome_deserialize_macros::Deserializable;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::TextRange;
use std::ops::Range;

// 最简单的Spanned实现，只包含值和位置
#[derive(Debug)]
struct Spanned<T> {
  value: T,
  range: Range<usize>,
}

// 实现Deserializable trait，这是位置计算的核心
impl<T: Deserializable> Deserializable for Spanned<T> {
  fn deserialize(
    value: &impl DeserializableValue,
    name: &str,
    diagnostics: &mut Vec<DeserializationDiagnostic>,
  ) -> Option<Self> {
    // 核心位置计算: 获取值的TextRange
    let text_range = value.range();

    // 将TextRange转换为标准的Range<usize>
    let start: usize = text_range.start().into();
    let end: usize = text_range.end().into();
    let range = start..end;

    // 解析实际值
    let value = T::deserialize(value, name, diagnostics)?;

    Some(Spanned { value, range })
  }
}

// 简单的测试结构
#[derive(Debug, Deserializable)]
struct Person {
  name: Spanned<String>,
  age: Spanned<i32>,
  hobbies: Spanned<Vec<Spanned<String>>>,
  address: Spanned<Address>,
}

#[derive(Debug, Deserializable)]
struct Address {
  city: Spanned<String>,
  zip: Spanned<String>,
}

// 打印结果
fn print_json_with_ranges(json: &str, person: &Person) {
  println!("原始JSON:\n{}", json);
  println!("\n字段位置信息:");

  println!(
    "name: \"{}\" - 位置: {}..{} - 文本: \"{}\"",
    person.name.value,
    person.name.range.start,
    person.name.range.end,
    &json[person.name.range.clone()]
  );

  println!(
    "age: {} - 位置: {}..{} - 文本: \"{}\"",
    person.age.value,
    person.age.range.start,
    person.age.range.end,
    &json[person.age.range.clone()]
  );

  println!(
    "\nhobbies (数组): 位置: {}..{} - 文本: \"{}\"",
    person.hobbies.range.start,
    person.hobbies.range.end,
    &json[person.hobbies.range.clone()]
  );

  for (i, hobby) in person.hobbies.value.iter().enumerate() {
    println!(
      "  hobby[{}]: \"{}\" - 位置: {}..{} - 文本: \"{}\"",
      i,
      hobby.value,
      hobby.range.start,
      hobby.range.end,
      &json[hobby.range.clone()]
    );
  }

  println!(
    "\naddress (对象): 位置: {}..{} - 文本: \"{}\"",
    person.address.range.start,
    person.address.range.end,
    &json[person.address.range.clone()]
  );

  println!(
    "  city: \"{}\" - 位置: {}..{} - 文本: \"{}\"",
    person.address.value.city.value,
    person.address.value.city.range.start,
    person.address.value.city.range.end,
    &json[person.address.value.city.range.clone()]
  );

  println!(
    "  zip: \"{}\" - 位置: {}..{} - 文本: \"{}\"",
    person.address.value.zip.value,
    person.address.value.zip.range.start,
    person.address.value.zip.range.end,
    &json[person.address.value.zip.range.clone()]
  );

  // 可视化范围
  println!("\n位置可视化:");
  let lines: Vec<&str> = json.lines().collect();
  for (i, line) in lines.iter().enumerate() {
    println!("{:2}: {}", i + 1, line);
  }
}

fn main() {
  // 测试JSON
  let json = r#"{
"name": "John Doe",
"age": 30,
"hobbies": [
  "reading",
  "coding",
  "hiking"
],
"address": {
  "city": "San Francisco",
  "zip": "94105"
}
}"#;

  let result = deserialize_from_json_str::<Person>(
    json,
    JsonParserOptions::default().with_allow_comments(),
    "example.json",
  );

  // 检查错误
  // if !result.diagnostics().is_empty() {
  //   println!("解析错误:");
  //   for diagnostic in result.diagnostics() {
  //     println!("  - {}", diagnostic.message());
  //   }
  //   return;
  // }

  // // 打印结果
  // if let Some(person) = result.into_deserialized() {
  //   print_json_with_ranges(json, &person);
  // } else {
  //   println!("解析失败，但无错误");
  // }
}
