use neon::prelude::*;

pub struct Memory{
  pub total_memory: String,
  pub free_memory: String,
  pub available_memory: String,
  pub used_memory: String
}

impl Memory{
  pub fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a ,JsObject>{
    let obj = cx.empty_object();

    let total_memory = cx.string(&self.total_memory);
    obj.set(cx, "totalMemory", total_memory);

    let free_memory = cx.string(&self.free_memory);
    obj.set(cx, "freeMemory", free_memory);

    let available_memory = cx.string(&self.available_memory);
    obj.set(cx, "availableMemory", available_memory);

    let used_memory = cx.string(&self.used_memory);
    obj.set(cx, "usedMemory", used_memory);

    Ok(obj)
  }
}