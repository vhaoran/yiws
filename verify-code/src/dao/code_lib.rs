use super::*;
use crate::model::CodeLib;
use log::*;
use mongodb::bson::oid;
use r_base::rmongo;

const TB: &str = "code_lib";
r_base::mongo_base!(MONGO_DB, TB, CodeLib);

#[tokio::test]
async fn test_1() -> Result<(), Box<dyn std::error::Error>> {
    r_base::init_modules(None).await?;
    //
    let src = CodeLib::new(100);
    let r = self::insert_one(src, None).await?;
    //
    let r: Vec<CodeLib> = self::find_many(doc! {}, None).await?;
    println!("-----------{r:#?}-----------",);

    //
    Ok(())
}
