
pub fn bridge<FF,NF,MF>(jni_func:FF, native_func:NF, mapper_func:MF)
where MF: Fn(FF) -> NF 
{
    mapper_func(jni_func)
}