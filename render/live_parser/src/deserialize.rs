use crate::liveregistry::LiveRegistry;
use crate::livenode::LiveValue;
use crate::livenode::LiveNode;
use crate::id::Id;
use crate::id::IdType;
use std::any::Any;

pub trait DeLiveFactory{
    fn de_live_any(&self, lr: &LiveRegistry, file: usize, level: usize, start: usize) -> Result<Box<dyn Any>,DeLiveErr>;
}

#[derive(Clone, Debug)]
pub struct DeLiveErr {
    pub msg: String,
    pub file: usize,
    pub level: usize,
    pub index: usize,
}

pub trait DeLive: Sized {
    fn de_live(lr: &LiveRegistry, file: usize, level: usize, index: usize) -> Result<Self,
    DeLiveErr>;
}

impl DeLiveErr {
    pub fn unk_prop(cls: Id, prop: Id, file: usize, level: usize, index: usize) -> DeLiveErr {
        DeLiveErr {
            msg: format!("Unknown property {} for type {}", prop, cls),
            file,
            level,
            index
        }
    }
    pub fn miss_prop(cls: Id, prop: &str, file: usize, level: usize, index: usize) -> DeLiveErr {
        DeLiveErr {
            msg: format!("Missing property {} for type {}", prop, cls),
            file,
            level,
            index
        }
    }

    pub fn enum_notfound(enm: Id, prop: Id, file: usize, level: usize, index: usize) -> DeLiveErr {
        DeLiveErr {
            msg: format!("Cannot find enum id {} for prop {}", enm, prop),
            file,
            level,
            index
        }
    }
    pub fn not_class(node: &LiveNode, file: usize, level: usize, index: usize) -> DeLiveErr {
        DeLiveErr {
            msg: format!("Live node not a class {:?} for type", node),
            file,
            level,
            index
        }
    }
    pub fn not_struct(node: &LiveNode, file: usize, level: usize, index: usize) -> DeLiveErr {
        DeLiveErr {
            msg: format!("Live node not a struct {:?}", node),
            file,
            level,
            index
        }
    }

    pub fn arg_count(cls: Id, got:usize, req:usize, file: usize, level: usize, index: usize) -> DeLiveErr {
        DeLiveErr {
            msg: format!("Not enough args {} got {} req {}", cls, got, req),
            file,
            level,
            index
        }
    }
    pub fn incompat_value(ty:&str, lv: &LiveValue, file: usize, level: usize, index: usize) -> DeLiveErr {
        DeLiveErr {
            msg: format!("Incompatible value {:?} for {}", lv, ty),
            file,
            level,
            index
        }
    }

}

impl DeLive for f32 {
    fn de_live(lr: &LiveRegistry, file: usize, level: usize, index: usize) -> Result<Self,
    DeLiveErr> {
        let node = &lr.expanded[file].nodes[level][index];
        match node.value {
            LiveValue::Id(id)=>{// it should be a pointer
                if let IdType::NodePtr{file_id, ptr} = id.to_type(){
                    return DeLive::de_live(lr, file_id.to_index(), ptr.level, ptr.index)
                }
            }
            LiveValue::Int(v) => return Ok(v as f32),
            LiveValue::Float(v) => return Ok(v as f32),
            _ =>()
        }
        Err(DeLiveErr::incompat_value("f32", &node.value, file, level, index))
    }
}

impl DeLive for u32 {
    fn de_live(lr: &LiveRegistry, file: usize, level: usize, index: usize) -> Result<Self,
    DeLiveErr> {
        let node = &lr.expanded[file].nodes[level][index];
        match node.value {
            LiveValue::Id(id)=>{// it should be a pointer
                if let IdType::NodePtr{file_id, ptr} = id.to_type(){
                    return DeLive::de_live(lr, file_id.to_index(), ptr.level, ptr.index)
                }
            }
            LiveValue::Int(v) => return Ok(v as u32),
            LiveValue::Float(v) => return Ok(v as u32),
            _ =>()
        }
        Err(DeLiveErr::incompat_value("f32", &node.value, file, level, index))
    }
}
