// TODO:字符串转为rust结构体，以json输出
// TODO:兼容多种数据格式，如{"pserson_name": "cjx", "Age": 25, "email": "123@qq.com"}
// TODO:修改Person结构体。{"pserson_name": "cjx", "Age": "25", "email": "123@qq.com", "sex": "man"}。sex只可能是"man", "woman"
// TODO:sex传入"man", "woman"以外时, 统一变成"unkonw"
// TODO:sex变成一个可选项

use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::{Result as SerdeResult, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Sex {
    Man,
    Woman,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    #[serde(alias = "person_name")]
    name: String,
    #[serde(alias = "Age")]
    age: u8,
    email: String,
    #[serde(
        deserialize_with = "deserialize_with_sex",
        skip_serializing_if = "Option::is_none"
    )]
    sex: Option<Sex>,
}

fn deserialize_with_sex<'de, D>(deserializer: D) -> Result<Option<Sex>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;

    if v.is_string() {
        let r = v.as_str().ok_or(de::Error::custom("转换失败"))?;

        Ok(match r {
            "man" => Some(Sex::Man),
            "woman" => Some(Sex::Woman),
            _ => Some(Sex::Unknown),
        })
    } else {
        Err(de::Error::custom("转换失败"))
    }

    // Ok(Some(Sex::Man))
}

fn main() -> SerdeResult<()> {
    // // 字符串切片
    // let input = r#"{"name": "cjx", "age": 25, "email": "123@qq.com"}"#;
    // // ?语法是用来简化错误处理的
    // /*
    //  * serde_json::from_str返回一个 Result 类型的值。
    //  * ? 运算符会自动将一个 Result<T, E> 类型的值进行解包，如果这个 Result 的值是 Ok(T)，则返回 T 值；如果这个 Result 的值是 Err(E)，则将这个 Err(E) 值作为整个函数或方法的返回值。在这个过程中，如果发生了错误，函数或方法就会直接返回错误，不再执行后面的代码。
    //  */
    // // Parse the string of data into serde_json::Value.
    // let person_data: Person = serde_json::from_str(input)?;
    // // let my_data: Person = serde_json::from_str(input).expect("error");
    // let output = serde_json::to_string(&person_data)?;
    // println!("{}", output);

    let input2 = r#"{"person_name": "cjx", "Age": 25, "email": "123@qq.com","sex":"woman123"}"#;
    let person_data2: Person = serde_json::from_str(input2)?;
    let output2 = serde_json::to_string(&person_data2)?;
    println!("{}", output2);

    Ok(())
}
