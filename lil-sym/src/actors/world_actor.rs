
#[derive(Debug, Clone, Default)]
pub struct WorldActorResult{

}

impl WorldActorResult{
    pub fn new() -> WorldActorResult{
        WorldActorResult{}
    }
}

pub struct WorldActor{
    pub floor: ColliderHandle
}
